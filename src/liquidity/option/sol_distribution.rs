use std::sync::Arc;

use bincode::serialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use solana_sdk::{
    instruction::Instruction, message::VersionedMessage, pubkey::Pubkey, signature::Keypair,
    signer::Signer, system_instruction, transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

use crate::{
    env::{
        load_settings,
        minter::{load_minter_settings, PoolDataSettings},
    },
    instruction::instruction::SOL_MINT,
    liquidity::{
        option::wallet_gen::load_wallets,
        utils::{tip_account, tip_txn},
    },
    raydium::swap::swapper::auth_keypair,
    rpc::HTTP_CLIENT,
    user_inputs::amounts::{bundle_priority_tip, sol_amount},
    utils::rand::distribute_randomly,
};

pub async fn sol_distribution(
    server_data: PoolDataSettings,
    wallets: &&[Keypair],
    total_amount: u64,
    min_amount: u64,
    max_amount: u64,
    bundle_tip: u64,
) -> eyre::Result<(Vec<u64>, Vec<VersionedTransaction>)> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let buyer_wallet = Arc::new(Keypair::from_base58_string(&server_data.buyer_key));

    let rand_amount = distribute_randomly(total_amount, wallets.len(), min_amount, max_amount);

    let wallet_chunks: Vec<_> = wallets.chunks(21).collect();
    let mut bundle_txns = vec![];

    let recent_blockhash = connection.get_latest_blockhash().await?;

    for (index, wallet_chunk) in wallet_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();

        for (i, wallet) in wallet_chunk.iter().enumerate() {
            let transfer_instruction = system_instruction::transfer(
                &buyer_wallet.pubkey(),
                &wallet.pubkey(),
                rand_amount[index],
            );

            current_instructions.push(transfer_instruction);

            if index == wallet_chunks.len() - 1 && i == wallet_chunk.len() - 1 {
                info!("Adding tip to last transaction");
                let tip = tip_txn(buyer_wallet.pubkey(), tip_account(), bundle_tip);
                current_instructions.push(tip);
            }
        }

        let versioned_msg = VersionedMessage::V0(
            match solana_sdk::message::v0::Message::try_compile(
                &buyer_wallet.pubkey(),
                &current_instructions,
                &[],
                recent_blockhash,
            ) {
                Ok(message) => message,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    panic!("Error: {}", e);
                }
            },
        );

        let transaction = VersionedTransaction::try_new(versioned_msg, &[&buyer_wallet])?;

        bundle_txns.push(transaction);
    }

    let mut sum = 0;
    let txn_size: Vec<_> = bundle_txns
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

    println!("Generated transactions: {}", bundle_txns.len());

    Ok((rand_amount, bundle_txns))
}

//server_data.BlockEngineSelections
pub async fn atas_creation(
    wallets: Vec<Pubkey>,
    buyer_key: Arc<Keypair>,
    mint: Pubkey,
) -> eyre::Result<(Vec<Instruction>, Pubkey, Pubkey)> {
    let mut mint_ata = Pubkey::default();
    let mut sol_ata = Pubkey::default();
    let mut instructions = vec![];
    for wallet in wallets {
        mint_ata = get_associated_token_address(&wallet, &mint);
        sol_ata = get_associated_token_address(&wallet, &SOL_MINT);

        let create_mint_ata =
            create_associated_token_account(&buyer_key.pubkey(), &wallet, &mint, &spl_token::id());
        let create_sol_ata = create_associated_token_account(
            &buyer_key.pubkey(),
            &wallet,
            &SOL_MINT,
            &spl_token::id(),
        );

        instructions.push(create_mint_ata);
        instructions.push(create_sol_ata);
    }

    Ok((instructions, mint_ata, sol_ata))
}

pub async fn distributor() -> eyre::Result<()> {
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

    let settings = load_settings().await?;

    let mut client =
        get_searcher_client(&settings.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let wallets: Vec<Keypair> = match load_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };
    let total_amount = sol_amount("Total Amount:").await;
    let max_amount = sol_amount("Max Distribution Amount:").await;

    let min_amount = sol_amount("Min Distribution Amount:").await;
    let bundle_tip = bundle_priority_tip().await;

    let wallet_chunks = wallets.chunks(104).collect::<Vec<_>>();

    for (index, wallet_chunk) in wallet_chunks.iter().enumerate() {
        let (amounts, transactions_1) = match sol_distribution(
            data.clone(),
            wallet_chunk,
            total_amount,
            min_amount,
            max_amount,
            bundle_tip,
        )
        .await
        {
            Ok((amounts, transactions_1)) => (amounts, transactions_1),
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };

        info!("Sending Bundle");

        let bundle = match send_bundle_with_confirmation(
            &transactions_1,
            &connection,
            &mut client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Distribution Error: {}", e);
                return Ok(());
            }
        };
    }

    Ok(())
}
