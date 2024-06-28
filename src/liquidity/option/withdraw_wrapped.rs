use std::{error::Error, str::FromStr, sync::Arc};

use bincode::serialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::close_account;

use crate::{
    env::{
        load_settings,
        minter::{load_minter_settings, PoolDataSettings},
    },
    liquidity::{
        option::wallet_gen::load_wallets,
        utils::{tip_account, tip_txn},
    },
    raydium::swap::{instructions::SOLC_MINT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
    user_inputs::amounts::bundle_priority_tip,
};

pub async fn withdraw_wrapped_sol() -> Result<(), Box<dyn Error + Send>> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let data = match load_minter_settings().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let settings = match load_settings().await {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let mut client =
        match get_searcher_client(&settings.block_engine_url, &Arc::new(auth_keypair())).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let bundle_txn = match withdraw_wsol(data).await {
        Ok(bundle_txn) => bundle_txn,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let bundle = match send_bundle_with_confirmation(
        &bundle_txn,
        &connection,
        &mut client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            return Ok(());
        }
    };

    Ok(())
}

pub async fn withdraw_wsol(
    pool_data: PoolDataSettings,
) -> Result<Vec<VersionedTransaction>, Box<dyn std::error::Error + Send>> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let wallets: Vec<Keypair> = match load_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let buyer_wallet = Keypair::from_base58_string(&pool_data.buyer_key);

    let balance = connection
        .get_balance(&buyer_wallet.pubkey())
        .await
        .unwrap();

    println!("Buyer Balance: {} SOL", lamports_to_sol(balance));

    let mint = Pubkey::from_str(&pool_data.token_mint).unwrap();

    let recent_blockhash = match connection.get_latest_blockhash().await {
        Ok(recent_blockhash) => recent_blockhash,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let wallet_chunks: Vec<_> = wallets.chunks(6).collect();
    let mut txns_chunk = Vec::new();

    for (chunk_index, wallet_chunk) in wallet_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();
        let mut current_wallets = Vec::new();

        for (i, wallet) in wallet_chunk.iter().enumerate() {
            let user_token_source = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

            let destination_wallet =
                get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);

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

            // current_instructions.push(system_instruction::transfer(
            //     &user_token_source,
            //     &buyer_wallet.pubkey(),
            //     balance,
            // ));

            current_instructions.push(
                close_account(
                    &spl_token::id(),
                    &user_token_source,
                    &buyer_wallet.pubkey(),
                    &wallet.pubkey(),
                    &[&wallet.pubkey()],
                )
                .unwrap(),
            );

            current_wallets.push(wallet);
        }

        println!(
            "Chunk {}: {} instructions",
            chunk_index,
            current_instructions.len()
        );

        println!("Chunk {}: {} wallets", chunk_index, current_wallets.len());

        if current_instructions.len() == 0 {
            continue;
        }

        current_wallets.push(&buyer_wallet);

        if current_instructions.len() < 13 {
            let tip = tip_txn(buyer_wallet.pubkey(), tip_account(), sol_to_lamports(0.001));
            current_instructions.push(tip);
        }

        let versioned_msg = VersionedMessage::V0(
            Message::try_compile(
                &buyer_wallet.pubkey(),
                &current_instructions,
                &[],
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

    println!("txn: {:?}", txns_chunk.len());

    let txn_size: Vec<_> = txns_chunk
        .iter()
        .map(|x| {
            let serialized_x = serialize(x).unwrap();
            serialized_x.len()
        })
        .collect();

    println!("txn_size: {:?}", txn_size);

    Ok(txns_chunk)
}
