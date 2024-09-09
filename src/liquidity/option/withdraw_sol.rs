use std::{error::Error, sync::Arc};

use bincode::serialize;
use colored::Colorize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    native_token::lamports_to_sol,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::VersionedTransaction,
};

use crate::{
    env::{
        load_config,
        minter::{load_minter_settings, PoolDataSettings},
    },
    liquidity::{
        option::wallet_gen::{list_folders, load_wallets},
        utils::{tip_account, tip_txn},
    },
    raydium_amm::swap::swapper::auth_keypair,
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

    let settings = match load_config().await {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let mut client = match get_searcher_client(
        &settings.network.block_engine_url,
        &Arc::new(auth_keypair()),
    )
    .await
    {
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

    match send_bundle_with_confirmation(
        &bundle_txn,
        &connection,
        &mut client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(_) => {}
        Err(_) => {
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

    let recent_blockhash = connection.get_latest_blockhash().await?;

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

    println!(
        "{}: {}\n{} SOL",
        "Deployer Wallet".bold().green(),
        deployer_wallet.pubkey(),
        lamports_to_sol(balance)
    );

    let balance = connection
        .get_balance(&buyer_wallet.pubkey())
        .await
        .unwrap();

    println!(
        "{}: {}\n{} SOL",
        "Buyer Wallet".bold().cyan(),
        buyer_wallet.pubkey(),
        lamports_to_sol(balance)
    );

    Ok(())
}
pub async fn folder_deployer_details() -> Result<(), Box<dyn Error + Send>> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let (_, wallets) = match list_folders().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    for (index, wallet) in wallets.iter().enumerate() {
        let balance = connection.get_balance(&wallet.pubkey()).await.unwrap();
        println!(
            "Wallet [{}]: {}\n{} SOL",
            index + 1,
            wallet.pubkey(),
            lamports_to_sol(balance)
        );
    }

    Ok(())
}
