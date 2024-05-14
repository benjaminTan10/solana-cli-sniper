use std::{error::Error, str::FromStr, sync::Arc};

use bincode::serialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
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

use crate::{
    env::{
        env_loader::tip_account,
        load_settings,
        minter::{load_minter_settings, PoolDataSettings},
    },
    liquidity::{option::wallet_gen::load_wallets, utils::tip_txn},
    raydium::swap::{instructions::SOLC_MINT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
    user_inputs::amounts::bundle_priority_tip,
};

pub async fn withdraw_sol() -> Result<(), Box<dyn Error + Send>> {
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

    let bundle_txn = match withdraw_sol_wallets(data).await {
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

pub async fn withdraw_sol_wallets(
    data: PoolDataSettings,
) -> Result<Vec<VersionedTransaction>, Box<dyn Error>> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let buyer_wallet = Arc::new(Keypair::from_base58_string(&data.buyer_key));

    let wallets: Vec<Keypair> = match load_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let bundle_tip = bundle_priority_tip().await;

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

    let mut distribution_ix: Vec<Instruction> = vec![];

    //-----------------------------------------------------------------------------------------------

    // for (index, wallet) in wallets.iter().enumerate() {
    //     let balance = connection.get_balance(&wallet.pubkey()).await.unwrap();
    //     println!("Wallet {}: {} SOL", index, lamports_to_sol(balance));
    //     let transfer_instruction =
    //         system_instruction::transfer(&wallet.pubkey(), &buyer_wallet.pubkey(), balance);

    //     distribution_ix.push(transfer_instruction);
    // }

    // let tip_ix = tip_txn(buyer_wallet.pubkey(), tip_account(), bundle_tip);

    // distribution_ix.push(tip_ix);

    // let distribution_ix_chunks: Vec<_> = distribution_ix.chunks(21).collect();

    // //----------------------------------------------------------------------------------

    let recent_blockhash = connection.get_latest_blockhash().await?;

    // let mut transactions: Vec<VersionedTransaction> = vec![];

    // for (i, chunk) in distribution_ix_chunks.iter().enumerate() {
    //     let versioned_msg = VersionedMessage::V0(
    //         match solana_sdk::message::v0::Message::try_compile(
    //             &buyer_wallet.pubkey(),
    //             chunk,
    //             &[address_lookup_table_account.clone()],
    //             recent_blockhash,
    //         ) {
    //             Ok(message) => message,
    //             Err(e) => {
    //                 eprintln!("Error: {}", e);
    //                 panic!("Error: {}", e);
    //             }
    //         },
    //     );

    //     let transaction = VersionedTransaction::try_new(versioned_msg, &[&buyer_wallet])?;

    //     println!("Chunk {}: {} instructions", i, chunk.len());

    //     transactions.push(transaction);
    // }

    let wallet_chunks: Vec<_> = wallets.chunks(6).collect();
    let mut txns_chunk = Vec::new();

    for (chunk_index, wallet_chunk) in wallet_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();
        let mut current_wallets = Vec::new();

        for (i, wallet) in wallet_chunk.iter().enumerate() {
            let balance = connection.get_balance(&wallet.pubkey()).await.unwrap();

            println!("Wallet {}: {} SOL", i, lamports_to_sol(balance));
            let transfer_instruction =
                system_instruction::transfer(&wallet.pubkey(), &buyer_wallet.pubkey(), balance);

            current_instructions.push(transfer_instruction);
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
            let tip = tip_txn(buyer_wallet.pubkey(), tip_account(), bundle_tip);
            current_instructions.push(tip);
        }

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

    let mut sum = 0;
    let txn_size: Vec<_> = txns_chunk
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

    println!("Generated transactions: {}", txn_size.len());

    Ok(txns_chunk)
}

pub async fn deployer_details() -> Result<(), Box<dyn Error + Send>> {
    let settings = match load_minter_settings().await {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let buyer_wallet = Arc::new(Keypair::from_base58_string(&settings.buyer_key));
    let deployer_wallet = Arc::new(Keypair::from_base58_string(&settings.deployer_key));

    let balance = connection
        .get_balance(&deployer_wallet.pubkey())
        .await
        .unwrap();

    info!("Deployer Wallet: {} SOL", lamports_to_sol(balance));

    let balance = connection
        .get_balance(&buyer_wallet.pubkey())
        .await
        .unwrap();

    info!("Buyer Wallet: {} SOL", lamports_to_sol(balance));

    let w_sol_buyer = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);
    let w_sol_deployer = get_associated_token_address(&deployer_wallet.pubkey(), &SOLC_MINT);

    for wallet in vec![w_sol_buyer, w_sol_deployer] {
        let balance = match connection.get_token_account_balance(&wallet).await {
            Ok(balance) => balance,
            Err(e) => {
                error!("No Token Account Found on Wallet: {}", wallet);

                continue;
            }
        };

        info!(
            "Balance: {} SOL",
            lamports_to_sol(balance.amount.parse::<u64>().unwrap())
        );
    }

    Ok(())
}
