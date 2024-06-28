use std::{rc::Rc, sync::Arc};

use super::instructions::instructions::generate_pump_buy_ix;
use crate::{
    env::EngineSettings,
    liquidity::utils::tip_account,
    raydium::{
        swap::{swap_in::PriorityTip, swapper::auth_keypair},
        volume_pinger::volume::buy_amount,
    },
    rpc::HTTP_CLIENT,
    user_inputs::{
        amounts::{bundle_priority_tip, priority_fee},
        tokens::token_env,
    },
};
use anchor_client::{Client, Cluster};
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::system_instruction::transfer;
use solana_sdk::{
    commitment_config::CommitmentLevel,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};

pub async fn pump_in(wallet: &Arc<Keypair>, args: EngineSettings) -> eyre::Result<()> {
    let token_address = token_env("Token Address: ").await;

    let amount_in = match buy_amount("Swap Amount: ").await {
        Ok(a) => a,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let mut bundle_tip = 0;
    let mut priority_fee_value = 0;

    if args.use_bundles {
        priority_fee_value = priority_fee().await;
        bundle_tip = bundle_priority_tip().await;
    } else {
        priority_fee_value = priority_fee().await;
    }

    let user_source_owner = wallet.pubkey();

    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let mut searcher_client =
        get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;

    let tip_account = tip_account();

    let swap_instructions =
        generate_pump_buy_ix(rpc_client, token_address, amount_in, wallet.clone())
            .await
            .unwrap();

    // let config = CommitmentLevel::Finalized;
    // let (latest_blockhash, _) = rpc_client
    //     .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
    //         commitment: config,
    //     })
    //     .await?;

    // let message = match solana_program::message::v0::Message::try_compile(
    //     &user_source_owner,
    //     &swap_instructions,
    //     &[],
    //     latest_blockhash,
    // ) {
    //     Ok(x) => x,
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //         return Ok(());
    //     }
    // };

    // let transaction = match VersionedTransaction::try_new(
    //     solana_program::message::VersionedMessage::V0(message),
    //     &[&wallet],
    // ) {
    //     Ok(x) => x,
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //         return Ok(());
    //     }
    // };

    // if args.use_bundles {
    //     info!("Building Bundle");

    //     let tip_txn = VersionedTransaction::from(Transaction::new_signed_with_payer(
    //         &[transfer(&wallet.pubkey(), &tip_account, bundle_tip)],
    //         Some(&wallet.pubkey()),
    //         &[&wallet],
    //         rpc_client.get_latest_blockhash().await.unwrap(),
    //     ));

    //     let bundle_txn = vec![transaction, tip_txn];

    //     let mut bundle_results_subscription = searcher_client
    //         .subscribe_bundle_results(SubscribeBundleResultsRequest {})
    //         .await
    //         .expect("subscribe to bundle results")
    //         .into_inner();

    //     let bundle = match send_bundle_with_confirmation(
    //         &bundle_txn,
    //         &rpc_client,
    //         &mut searcher_client,
    //         &mut bundle_results_subscription,
    //     )
    //     .await
    //     {
    //         Ok(_) => {}
    //         Err(e) => {
    //             panic!("Error: {}", e);
    //         }
    //     };

    //     std::mem::drop(bundle_results_subscription);
    // } else {
    //     info!("Sending Transaction");
    //     let config = RpcSendTransactionConfig {
    //         skip_preflight: true,
    //         ..Default::default()
    //     };

    //     if args.spam {
    //         let mut counter = 0;
    //         while counter < args.spam_count {
    //             let result = match rpc_client
    //                 .send_transaction_with_config(&transaction, config)
    //                 .await
    //             {
    //                 Ok(x) => x,
    //                 Err(e) => {
    //                     error!("Error: {:?}", e);
    //                     return Ok(());
    //                 }
    //             };

    //             info!("Transaction Sent {:?}", result);
    //             counter += 1;
    //         }
    //     } else {
    //         let result = match rpc_client
    //             .send_transaction_with_config(&transaction, config)
    //             .await
    //         {
    //             Ok(x) => x,
    //             Err(e) => {
    //                 error!("Error: {:?}", e);
    //                 return Ok(());
    //             }
    //         };

    //         info!("Transaction Sent {:?}", result);
    //     }
    // }
    // let pool_keys_clone = pool_keys.clone();
    // let args_clone = args.clone();
    // let fees_clone = fees.clone();
    // let wallet_clone = Arc::clone(&wallet);
    // let (mut stop_tx, mut stop_rx) = tokio::sync::mpsc::channel::<()>(100);

    // let handle = thread::spawn(move || {
    //     let runtime = tokio::runtime::Runtime::new().unwrap();
    //     runtime.block_on(async {
    //         read_single_key_impl(
    //             &mut stop_tx,
    //             pool_keys_clone,
    //             args_clone,
    //             fees_clone,
    //             &wallet_clone,
    //         )
    //         .await
    //         .unwrap();
    //     });
    // });

    // price_logger(&mut stop_rx, amount_in, pool_keys, wallet.clone()).await;

    // handle.join().unwrap();
    Ok(())
}
