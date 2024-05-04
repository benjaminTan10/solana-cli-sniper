use std::{str::FromStr, sync::Arc};

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use solana_address_lookup_table_program::instruction::extend_lookup_table;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use tokio::sync::mpsc::channel;

use crate::{
    env::minter::PoolDataSettings,
    instruction::instruction::{AmmKeys, MarketPubkeys, PoolKeysSniper, SOL_MINT},
    liquidity::utils::{tip_account, tip_txn, JitoPoolData},
    plugins::jito_plugin::{
        event_loop::bundle_results_loop,
        lib::{send_bundles, BundledTransactions},
    },
    raydium::swap::swapper::auth_keypair,
    rpc::HTTP_CLIENT,
};

use super::create_lut::create_lut;

pub async fn poolkeys_lut(
    amm_keys: AmmKeys,
    market_keys: MarketPubkeys,
    lut: Pubkey,
    server_data: PoolDataSettings,
) -> eyre::Result<Instruction> {
    let buyer_wallet = Keypair::from_base58_string(&server_data.buyer_key);

    let mut keys = vec![];
    keys.push(amm_keys.amm_pool);
    keys.push(amm_keys.amm_coin_mint);
    keys.push(amm_keys.amm_pc_mint);
    keys.push(amm_keys.amm_lp_mint);
    keys.push(amm_keys.amm_authority);
    keys.push(amm_keys.amm_open_order);
    keys.push(amm_keys.amm_target);
    keys.push(amm_keys.amm_coin_vault);
    keys.push(amm_keys.amm_pc_vault);
    keys.push(amm_keys.market_program);
    keys.push(amm_keys.market);
    keys.push(*market_keys.market);
    keys.push(*market_keys.req_q);
    keys.push(*market_keys.event_q);
    keys.push(*market_keys.bids);
    keys.push(*market_keys.asks);
    keys.push(*market_keys.coin_vault);
    keys.push(*market_keys.pc_vault);
    keys.push(*market_keys.vault_signer_key);
    keys.push(*market_keys.coin_mint);
    keys.push(*market_keys.pc_mint);

    let add_accounts = extend_lookup_table(
        lut,
        buyer_wallet.pubkey(),
        Some(buyer_wallet.pubkey()),
        keys,
    );

    Ok(add_accounts)
}

pub async fn accountatas_lut(
    lut: Pubkey,
    server_data: PoolDataSettings,
    wallets: Vec<Pubkey>,
) -> eyre::Result<Vec<Instruction>> {
    let buyer_wallet = Keypair::from_base58_string(&server_data.buyer_key);
    let mint = Pubkey::from_str(&server_data.token_mint)?;

    let mut atas: Vec<Pubkey> = vec![];

    let buyer_ata = get_associated_token_address(&buyer_wallet.pubkey(), &mint);
    let buyer_sol_ata = get_associated_token_address(&buyer_wallet.pubkey(), &SOL_MINT);

    atas.push(buyer_ata);
    atas.push(buyer_sol_ata);

    for wallet in wallets {
        let mint_ata = get_associated_token_address(&wallet, &mint);
        let sol_ata = get_associated_token_address(&wallet, &SOL_MINT);

        atas.push(mint_ata);
        atas.push(sol_ata);
        atas.push(wallet)
    }

    //divide atas into 19 chunks
    let mut chunks: Vec<Vec<Pubkey>> = vec![];
    let mut chunk: Vec<Pubkey> = vec![];
    for ata in atas {
        chunk.push(ata);
        if chunk.len() == 30 {
            chunks.push(chunk);
            chunk = vec![];
        }
    }
    if chunk.len() > 0 {
        chunks.push(chunk);
    }

    let mut add_accounts = vec![];
    for chunk in chunks {
        let extend_lut = extend_lookup_table(
            lut,
            buyer_wallet.pubkey(),
            Some(buyer_wallet.pubkey()),
            chunk,
        );

        add_accounts.push(extend_lut);
    }

    Ok(add_accounts)
}

pub async fn lut_main(
    server_data: PoolDataSettings,
    amm_keys: AmmKeys,
    market_keys: MarketPubkeys,
    wallets: Vec<Pubkey>,
) -> eyre::Result<Pubkey> {
    let buyer_wallet = Keypair::from_base58_string(&server_data.buyer_key);

    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let (lut_inx, lut_account) = create_lut(server_data.clone()).await?;

    let mut extendlut_ixs: Vec<Instruction> = vec![];

    extendlut_ixs.push(lut_inx);
    let pool_lut = poolkeys_lut(amm_keys, market_keys, lut_account, server_data.clone()).await?;
    let ata_lut = accountatas_lut(lut_account, server_data.clone(), wallets.clone()).await?;

    extendlut_ixs.push(pool_lut);
    extendlut_ixs.extend(ata_lut);

    let tip = tip_txn(buyer_wallet.pubkey(), tip_account(), sol_to_lamports(0.005));
    extendlut_ixs.push(tip);

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let mut versioned_txns: Vec<VersionedTransaction> = vec![];

    if extendlut_ixs.len() >= 2 {
        let versioned_msg = VersionedMessage::V0(Message::try_compile(
            &buyer_wallet.pubkey(),
            &extendlut_ixs[0..2], // Include the first two instructions in the message
            &[],
            recent_blockhash,
        )?);

        let transaction = VersionedTransaction::try_new(versioned_msg, &[&buyer_wallet])?;

        versioned_txns.push(transaction);
    }

    for ix in &extendlut_ixs[2..] {
        let versioned_msg = VersionedMessage::V0(Message::try_compile(
            &buyer_wallet.pubkey(),
            &[ix.clone()],
            &[],
            recent_blockhash,
        )?);

        let transaction = VersionedTransaction::try_new(versioned_msg, &[&buyer_wallet])?;

        versioned_txns.push(transaction);
    }

    let mut sum = 0;
    let txn_size: Vec<_> = versioned_txns
        .iter()
        .map(|x| {
            let serialized_x = serialize(x).unwrap();
            //sum all of them
            sum += serialized_x.len();
            serialized_x.len()
        })
        .collect();

    println!("Sum: {:?}", sum);
    println!("txn_size: {:?}", txn_size);

    println!("{}", versioned_txns.len());
    if versioned_txns.len() > 5 {
        println!("{}", versioned_txns.len());
        return Err(eyre::eyre!("Too many transactions"));
    }

    let mut client = get_searcher_client(
        "https://ny.mainnet.block-engine.jito.wtf",
        &Arc::new(auth_keypair()),
    )
    .await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    use bincode::serialize;

    let _ = match send_bundle_with_confirmation(
        &versioned_txns,
        &connection,
        &mut client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(results) => results,
        Err(e) => {
            return Err(eyre::eyre!("Error sending bundle: {:?}", e));
        }
    };

    Ok(lut_account)
}
