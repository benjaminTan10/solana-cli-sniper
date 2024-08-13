use std::sync::Arc;

use super::instructions::instructions::{
    generate_pump_buy_ix, generate_pump_sell_ix, PumpFunDirection,
};
use crate::{
    env::{utils::read_keys, EngineSettings},
    liquidity::{pool_ixs::token_percentage, utils::tip_account},
    raydium_amm::{swap::swapper::auth_keypair, volume_pinger::volume::buy_amount},
    rpc::HTTP_CLIENT,
    user_inputs::{amounts::bundle_priority_tip, tokens::token_env},
};
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::system_instruction::transfer;
use solana_sdk::{
    commitment_config::CommitmentLevel,
    signature::Keypair,
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token_client::token;

pub async fn pump_swap(
    wallet: &Arc<Keypair>,
    args: EngineSettings,
    direction: PumpFunDirection,
) -> eyre::Result<()> {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let token_address = token_env("Token Address: ").await;

    let mut amount = 0;
    if direction == PumpFunDirection::Buy {
        amount = match buy_amount("Swap Amount: ").await {
            Ok(a) => a,
            Err(e) => {
                log::error!("Error: {}", e);
                return Ok(());
            }
        };
    } else if direction == PumpFunDirection::Sell {
        let tokens = (token_percentage() * 100.0).round() as u64;
        let tokens_amount = rpc_client
            .get_token_account_balance(&get_associated_token_address(
                &wallet.pubkey(),
                &token_address,
            ))
            .await?;
        let token_amount = match tokens_amount.amount.parse::<u64>() {
            Ok(a) => a,
            Err(e) => {
                error!("Error: {}", e);
                return Ok(());
            }
        };

        amount = token_amount * tokens / 100;
    }

    info!("Tokens Amount: {}", amount);

    let mut bundle_tip = 0;
    if args.use_bundles {
        bundle_tip = bundle_priority_tip().await;
    }

    let user_source_owner = wallet.pubkey();

    let mut searcher_client =
        get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;

    let tip_account = tip_account();

    let mut swap_instructions = vec![];

    let create_account = create_associated_token_account_idempotent(
        &wallet.pubkey(),
        &wallet.pubkey(),
        &token_address,
        &spl_token::id(),
    );

    swap_instructions.push(create_account);

    if direction == PumpFunDirection::Buy {
        let buy_ix =
            generate_pump_buy_ix(rpc_client.clone(), token_address, amount, wallet.clone())
                .await
                .unwrap();
        swap_instructions.extend(buy_ix);
    } else {
        let sell_ix = generate_pump_sell_ix(token_address, amount, wallet.clone())
            .await
            .unwrap();
        swap_instructions.extend(sell_ix);
    }

    let config = CommitmentLevel::Finalized;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let transaction = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    if args.use_bundles {
        info!("Building Bundle");

        let tip_txn = VersionedTransaction::from(Transaction::new_signed_with_payer(
            &[transfer(&wallet.pubkey(), &tip_account, bundle_tip)],
            Some(&wallet.pubkey()),
            &[&wallet],
            rpc_client.get_latest_blockhash().await.unwrap(),
        ));

        let bundle_txn = vec![transaction, tip_txn];

        let mut bundle_results_subscription = searcher_client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        match send_bundle_with_confirmation(
            &bundle_txn,
            &rpc_client,
            &mut searcher_client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: {}", e);
            }
        }

        std::mem::drop(bundle_results_subscription);
    } else {
        info!("Sending Transaction");
        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

        if args.spam {
            let mut counter = 0;
            while counter < args.spam_count {
                let result = match rpc_client
                    .send_transaction_with_config(&transaction, config)
                    .await
                {
                    Ok(x) => x,
                    Err(e) => {
                        error!("Error: {:?}", e);
                        return Ok(());
                    }
                };

                info!("Transaction Sent {:?}", result);
                counter += 1;
            }
        } else {
            let result = match rpc_client
                .send_transaction_with_config(&transaction, config)
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            };

            info!("Transaction Sent {:?}", result);
        }
    }

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

    // price_logger(&mut stop_rx, amount, pool_keys, wallet.clone()).await;

    // handle.join().unwrap();

    let _ = read_keys();

    Ok(())
}
