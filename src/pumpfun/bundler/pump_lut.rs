use std::{str::FromStr, sync::Arc};

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_address_lookup_table_program::instruction::extend_lookup_table;
use solana_program::pubkey;
use solana_sdk::{
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    env::minter::{load_minter_settings, PoolDataSettings},
    instruction::instruction::SOL_MINT,
    liquidity::{
        lut::create_lut::create_lut,
        option::wallet_gen::load_wallets,
        utils::{tip_account, tip_txn},
    },
    pumpfun::{
        bundler::ix_accounts::create_ix_accounts,
        instructions::pumpfun_program::instructions::CreateKeys,
    },
    raydium_amm::swap::swapper::auth_keypair,
    rpc::HTTP_CLIENT,
};

pub const METAPLEX_METADATA: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
pub const MINT_AUTH: Pubkey = pubkey!("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM");

pub async fn pumpkeys_lut(
    pumpfun_keys: CreateKeys,
    lut: Pubkey,
    server_data: PoolDataSettings,
) -> eyre::Result<Instruction> {
    let buyer_wallet = Keypair::from_base58_string(&server_data.buyer_key);

    let mut keys = vec![];
    keys.push(pumpfun_keys.mint);
    keys.push(pumpfun_keys.mint_authority);
    keys.push(pumpfun_keys.bonding_curve);
    keys.push(pumpfun_keys.associated_bonding_curve);
    keys.push(pumpfun_keys.global);
    keys.push(pumpfun_keys.mpl_token_metadata);
    keys.push(pumpfun_keys.metadata);
    keys.push(pumpfun_keys.user);
    keys.push(pumpfun_keys.system_program);
    keys.push(pumpfun_keys.token_program);
    keys.push(pumpfun_keys.associated_token_program);
    keys.push(pumpfun_keys.rent);
    keys.push(pumpfun_keys.event_authority);
    keys.push(pumpfun_keys.program);

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

pub async fn lut_caller(
    server_data: PoolDataSettings,
    pump_keys: CreateKeys,
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
    let pool_lut = pumpkeys_lut(pump_keys, lut_account, server_data.clone()).await?;
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

    if extendlut_ixs.len() > 2 {
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

pub async fn pump_lut_main() -> eyre::Result<()> {
    let pool_data = load_minter_settings().await?;

    let mint = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")?;
    let wallet = Keypair::from_base58_string(&pool_data.deployer_key);

    // generate amm keys
    let pump_keys = match create_ix_accounts(mint, wallet.pubkey()) {
        Ok(keys) => keys,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let wallets: Vec<Keypair> = match load_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    match lut_caller(
        pool_data,
        pump_keys,
        wallets.iter().map(|x| x.pubkey()).collect::<Vec<Pubkey>>(),
    )
    .await
    {
        Ok(lut) => lut,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    Ok(())
}
