use std::{str::FromStr, sync::Arc};

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
    system_instruction,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::sync_native;

use crate::{
    env::{
        load_settings,
        minter::{load_minter_settings, PoolDataSettings},
    },
    liquidity::{
        option::wallet_gen::load_wallets,
        utils::{tip_account, tip_txn},
    },
    raydium_amm::swap::{instructions::SOLC_MINT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
};

pub async fn wsol(
    pool_data: PoolDataSettings,
    wallets: Vec<&Keypair>,
) -> Result<Vec<VersionedTransaction>, Box<dyn std::error::Error + Send>> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let lut_creation = match Pubkey::from_str(&pool_data.lut_key) {
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

    let address_lookup_table = match AddressLookupTable::deserialize(&raw_account.data) {
        Ok(address_lookup_table) => address_lookup_table,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };
    let address_lookup_table_account = AddressLookupTableAccount {
        key: lut_creation,
        addresses: address_lookup_table.addresses.to_vec(),
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

    let wallet_chunks: Vec<_> = wallets.chunks(3).collect();
    let mut txns_chunk = Vec::new();

    for (chunk_index, wallet_chunk) in wallet_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();
        let mut current_wallets = Vec::new();

        for wallet in wallet_chunk.iter() {
            let user_token_source = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

            let balance = connection.get_balance(&wallet.pubkey()).await.unwrap();

            //if the balance is less than 0.00203928 SOL, skip the wallet
            if balance < sol_to_lamports(0.02) {
                continue;
            }

            println!("Balance: {} SOL", lamports_to_sol(balance));

            current_instructions.push(
                spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                    &wallet.pubkey(),
                    &wallet.pubkey(),
                    &SOLC_MINT,
                    &spl_token::id(),
                ),
            );
            current_instructions.push(
                spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                    &wallet.pubkey(),
                    &wallet.pubkey(),
                    &mint,
                    &spl_token::id(),
                ),
            );
            current_instructions.push(system_instruction::transfer(
                &wallet.pubkey(),
                &user_token_source,
                balance - sol_to_lamports(0.006),
            ));

            let sync_native = match sync_native(&spl_token::id(), &user_token_source) {
                Ok(sync_native) => sync_native,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    panic!("Error: {}", e);
                }
            };
            if chunk_index == wallet_chunks.len() - 1 && wallet == wallet_chunk.last().unwrap() {
                let tip = tip_txn(buyer_wallet.pubkey(), tip_account(), sol_to_lamports(0.001));
                current_instructions.push(tip);
            }
            current_instructions.push(sync_native);

            current_wallets.push(*wallet);
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

        let versioned_msg = VersionedMessage::V0(
            Message::try_compile(
                &buyer_wallet.pubkey(),
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

pub async fn sol_wrap() -> Result<(), Box<dyn std::error::Error>> {
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

    let wallets: Vec<Keypair> = match load_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let wallet_chunks: Vec<_> = wallets.chunks(14).collect();

    for (chunk_index, wallet_chunk) in wallet_chunks.iter().enumerate() {
        let wallets: Vec<&Keypair> = wallet_chunk.iter().map(|x| x).collect();

        let wrap = match wsol(data.clone(), wallets).await {
            Ok(wrap) => wrap,
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };

        let mut client =
            get_searcher_client(&settings.block_engine_url, &Arc::new(auth_keypair())).await?;

        let mut bundle_results_subscription = client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        let bundle = match send_bundle_with_confirmation(
            &wrap,
            &connection,
            &mut client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Distribution Error: {}", e);
                panic!("Error: {}", e);
            }
        };
    }

    Ok(())
}
