use std::{io::Write, str::FromStr, sync::Arc};

use bincode::serialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    message::{v0::Message, VersionedMessage},
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    env::{load_settings, minter::load_minter_settings},
    instruction::instruction::SOL_MINT,
    liquidity::{
        option::wallet_gen::{list_folders, list_json_wallets},
        utils::{tip_account, tip_txn},
    },
    pumpfun::{
        bundler::{create_metadata::metadata_json, ix_accounts::token_create_ix},
        instructions::{
            instructions::{
                calculate_buy_price, generate_pump_buy_ix, generate_pump_multi_buy_ix, GLOBAL_STATE,
            },
            pumpfun_program::{
                accounts::{BondingCurve, GlobalAccount},
                instructions::CreateIxArgs,
            },
        },
    },
    raydium_amm::swap::{
        instructions::{SOLC_MINT, TAX_ACCOUNT},
        swapper::auth_keypair,
    },
    rpc::HTTP_CLIENT,
    user_inputs::amounts::{bundle_priority_tip, sol_amount},
};

#[derive(Debug, serde::Serialize)]
pub struct PoolDeployResponse {
    pub wallets: Vec<String>,
    pub amm_pool: Pubkey,
}

pub async fn multi_wallet_token() -> eyre::Result<()> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let mut data = load_minter_settings().await?;
    let engine = load_settings().await?;

    let (_, wallets) = match list_folders().await {
        Ok(folders) => folders,
        Err(e) => {
            eprintln!("Error listing folders: {}", e);
            return Ok(());
        }
    };

    let deployer_key = Keypair::from_base58_string(&data.deployer_key);
    let buyer_key = Keypair::from_base58_string(&data.buyer_key);

    // -------------------Pool Creation Instructions--------------------------
    println!("Creating Pool Transaction");

    let create_metadata = match metadata_json().await {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error creating metadata: {}", e);
            return Ok(());
        }
    };

    let args = CreateIxArgs {
        name: create_metadata.name,
        symbol: create_metadata.symbol,
        uri: create_metadata.image,
    };
    let sol_amount = sol_amount("Buy SOL Amount:").await;
    let mint = match list_json_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    let token_ixs = token_create_ix(mint[0].pubkey(), deployer_key.pubkey(), args);

    let mut file = std::fs::File::create("bundler_settings.json").unwrap();
    file.write_all(serde_json::to_string(&data)?.as_bytes())?;

    let bundle_tip = bundle_priority_tip().await;

    // -------------------LUT Account------------------------------------------

    let lut_creation = match Pubkey::from_str(&data.lut_key) {
        Ok(lut) => lut,
        Err(e) => {
            panic!("LUT key not Found in Settings: {}", e);
        }
    };

    let mut raw_account = None;

    while raw_account.is_none() {
        match connection.get_account(&lut_creation).await {
            Ok(account) => raw_account = Some(account),
            Err(e) => {
                eprintln!("Error getting LUT account: {}, retrying...", e);
            }
        }
    }

    let raw_account = raw_account.unwrap();

    let address_lookup_table = AddressLookupTable::deserialize(&raw_account.data)?;
    let address_lookup_table_account = AddressLookupTableAccount {
        key: lut_creation,
        addresses: address_lookup_table.addresses.to_vec(),
    };

    let recent_blockhash = connection.get_latest_blockhash().await?;

    //-------------------Pool Transaction---------------------------------------
    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &token_ixs, /* , tax_txn*/
            &[address_lookup_table_account.clone()],
            recent_blockhash,
        )
        .unwrap(),
    );

    let versioned_tx =
        match VersionedTransaction::try_new(versioned_msg, &[&deployer_key, &mint[0]]) {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("Error creating pool transaction: {}", e);
                return Err(e.into());
            }
        };

    // -------------------Swap Instructions---------------------------------------

    let wallets_chunks = wallets.chunks(7).collect::<Vec<_>>();
    let mut txns_chunk = Vec::new();

    txns_chunk.push(versioned_tx);

    let account_data = connection.get_account_data(&GLOBAL_STATE).await?;

    let sliced_data: &mut &[u8] = &mut account_data.as_slice();

    let mut reserves = GlobalAccount::deserialize(sliced_data)?.0;

    for (chunk_index, wallet_chunk) in wallets_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();
        let mut current_wallets = Vec::new();

        for (i, wallet) in wallet_chunk.iter().enumerate() {
            let user_token_source = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

            let balance = match connection
                .get_token_account_balance(&user_token_source)
                .await
            {
                Ok(balance) => balance.amount.parse::<u64>().unwrap(),
                Err(e) => {
                    eprintln!("Error getting token account balance: {}", e);
                    continue;
                }
            };

            println!("Balance: {} SOL", lamports_to_sol(balance));

            let user_token_source = get_associated_token_address(&wallet.pubkey(), &SOL_MINT);
            let keypair = Keypair::to_base58_string(wallet);

            let reserves_tuple: (u128, u128, u128) = (
                reserves.initial_virtual_sol_reserves as u128,
                reserves.initial_virtual_token_reserves as u128,
                reserves.initial_real_token_reserves as u128,
            );

            let price: (u128, (u128, u128, u128)) =
                calculate_buy_price(balance as u128, reserves_tuple);

            let swap_ixs = generate_pump_multi_buy_ix(
                connection.clone(),
                mint[0].pubkey(),
                sol_amount,
                Arc::new(Keypair::from_base58_string(&keypair)),
                reserves_tuple,
            )
            .await
            .unwrap();

            current_instructions.extend(swap_ixs);
            current_wallets.push(wallet);

            // Update reserves after transaction
            reserves.initial_virtual_sol_reserves = price.1 .0 as u64;
            reserves.initial_virtual_token_reserves = price.1 .1 as u64;
            reserves.initial_real_token_reserves = price.1 .2 as u64;

            if chunk_index == wallets_chunks.len() - 1 && i == wallet_chunk.len() - 1 {
                let tip = tip_txn(buyer_key.pubkey(), tip_account(), bundle_tip);
                current_instructions.push(tip);
            }
        }

        println!("Tx-{}: {} wallets", chunk_index + 1, current_wallets.len());

        current_wallets.push(&buyer_key);

        let versioned_msg = VersionedMessage::V0(
            Message::try_compile(
                &buyer_key.pubkey(),
                &current_instructions,
                &[address_lookup_table_account.clone()],
                recent_blockhash,
            )
            .unwrap(),
        );

        let versioned_tx = match VersionedTransaction::try_new(versioned_msg, &current_wallets) {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("Error creating pool transaction: {}", e);
                panic!("Error: {}", e);
            }
        };

        txns_chunk.push(versioned_tx);
        // Now you can use chunk_index, current_wallets, and current_instructions
    }

    txns_chunk.iter().for_each(|tx| {
        println!("Txn: {:?}", tx.signatures);
    });

    let txn_size: Vec<_> = txns_chunk
        .iter()
        .map(|x| {
            let serialized_x = serialize(x).unwrap();
            serialized_x.len()
        })
        .collect();

    println!("txn_size: {:?}", txn_size);

    // -------------------Subscribe to Bundle Results---------------------------------------

    let mut client =
        get_searcher_client(&engine.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    if txns_chunk.len() > 5 {
        eprintln!("Too many transactions to send in one bundle");
        return Err(eyre::eyre!("Too many transactions to send in one bundle"));
    }

    let connection = &Arc::new(connection);

    let bundle_results = match send_bundle_with_confirmation(
        &txns_chunk,
        connection,
        &mut client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(bundle_results) => bundle_results,
        Err(e) => {
            eprintln!("Error sending bundle: {}", e);
        }
    };

    Ok(())
}
