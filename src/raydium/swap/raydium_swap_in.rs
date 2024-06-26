use jito_protos::bundle::BundleResult;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use solana_sdk::{signature::Keypair, signer::Signer};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{self, sleep};

use crate::env::env_loader::tip_account;
use crate::env::EngineSettings;
use crate::plugins::jito_plugin::lib::{send_bundles, BundledTransactions};
use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::swap::grpc_new_pairs::clear_previous_line;
use crate::raydium::swap::instructions::{swap_base_in, SwapDirection, SOLC_MINT};
use crate::raydium::swap::swapper::auth_keypair;
use crate::rpc::HTTP_CLIENT;
use crate::utils::read_single_key_impl;

use super::instructions::token_price_data;
use super::raydium_swap_out::raydium_out;
use super::swap_in::PriorityTip;

pub async fn raydium_in(
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    amount_in: u64,
    amount_out: u64,
    fees: PriorityTip,
    args: EngineSettings,
) -> eyre::Result<()> {
    let user_source_owner = wallet.pubkey();
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let mut searcher_client =
        get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;

    let tip_account = tip_account();

    let rpc_client = Arc::new(rpc_client);

    let token_address = if pool_keys.base_mint == SOLC_MINT {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };

    let swap_instructions = swap_base_in(
        &pool_keys.program_id,
        &pool_keys.id,
        &pool_keys.authority,
        &pool_keys.open_orders,
        &pool_keys.target_orders,
        &pool_keys.base_vault,
        &pool_keys.quote_vault,
        &pool_keys.market_program_id,
        &pool_keys.market_id,
        &pool_keys.market_bids,
        &pool_keys.market_asks,
        &pool_keys.market_event_queue,
        &pool_keys.market_base_vault,
        &pool_keys.market_quote_vault,
        &pool_keys.market_authority,
        &user_source_owner,
        &user_source_owner,
        &user_source_owner,
        &token_address,
        amount_in.clone(),
        amount_out,
        fees.priority_fee_value,
        args.clone(),
    )
    .await?;

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
            &[transfer(&wallet.pubkey(), &tip_account, fees.bundle_tip)],
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

        let bundle = match send_bundle_with_confirmation(
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
        };

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
    let pool_keys_clone = pool_keys.clone();
    let args_clone = args.clone();
    let fees_clone = fees.clone();
    let wallet_clone = Arc::clone(&wallet);
    let (mut stop_tx, mut stop_rx) = tokio::sync::mpsc::channel::<()>(100);

    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            read_single_key_impl(
                &mut stop_tx,
                pool_keys_clone,
                args_clone,
                fees_clone,
                &wallet_clone,
            )
            .await
            .unwrap();
        });
    });

    price_logger(&mut stop_rx, amount_in, pool_keys, wallet.clone()).await;

    handle.join().unwrap();
    Ok(())
}

pub async fn price_logger(
    stop_rx: &mut mpsc::Receiver<()>,
    amount_in: u64,
    pool_keys: PoolKeysSniper,
    wallet: Arc<Keypair>,
) {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    loop {
        if let Ok(_) = stop_rx.try_recv() {
            sleep(Duration::from_secs(10)).await;
            break;
        }

        let mut token_balance = 0;
        let rpc_client_clone = rpc_client.clone();
        let pool_keys_clone = pool_keys.clone();
        let wallet_clone = Arc::clone(&wallet);
        let token_accounts = match rpc_client_clone
            .get_token_accounts_by_owner(
                &wallet_clone.pubkey(),
                TokenAccountsFilter::Mint(pool_keys_clone.base_mint),
            )
            .await
        {
            Ok(token_accounts) => token_accounts,
            Err(e) => {
                error!("Error: {:?}", e);
                break;
            }
        };

        for rpc_keyed_account in &token_accounts {
            let pubkey = &rpc_keyed_account.pubkey;
            //convert to pubkey
            let pubkey = match Pubkey::from_str(&pubkey) {
                Ok(pubkey) => pubkey,
                Err(e) => {
                    eprintln!("Failed to parse pubkey: {}", e);
                    break;
                }
            };

            let balance = match rpc_client_clone.get_token_account_balance(&pubkey).await {
                Ok(balance) => balance,
                Err(e) => {
                    eprintln!("Failed to get token account balance: {}", e);
                    break;
                }
            };
            let lamports = match balance.amount.parse::<u64>() {
                Ok(lamports) => lamports,
                Err(e) => {
                    eprintln!("Failed to parse balance: {}", e);
                    break;
                }
            };

            token_balance = lamports;

            if lamports != 0 {
                break;
            }

            std::thread::sleep(Duration::from_secs(1));
        }

        let price = match token_price_data(
            rpc_client_clone,
            pool_keys_clone,
            wallet_clone,
            token_balance,
            SwapDirection::PC2Coin,
        )
        .await
        {
            Ok(price) => price,
            Err(e) => {
                error!("Error: {:?}", e);
                break;
            }
        };

        let total_value = lamports_to_sol(price as u64);
        let profit_percentage =
            ((total_value - lamports_to_sol(amount_in)) / lamports_to_sol(amount_in)) * 100.0;

        let terminal = clear_previous_line().unwrap();
        info!(
            "Aped: {:.3} Sol | Worth {:.4} Sol | Profit {:.2}%",
            lamports_to_sol(amount_in),
            total_value,
            profit_percentage
        );

        // Sleep for a while
        time::sleep(Duration::from_millis(500)).await;
    }
}

pub async fn sell_tokens(
    amount: u64,
    pool_keys: PoolKeysSniper,
    wallet: Arc<Keypair>,
    fees: PriorityTip,
    args: EngineSettings,
) -> eyre::Result<()> {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let mut token_balance = 0;
    let rpc_client_clone = rpc_client.clone();
    let pool_keys_clone = pool_keys.clone();
    let wallet_clone = Arc::clone(&wallet);
    let token_accounts = rpc_client_clone
        .get_token_accounts_by_owner(
            &wallet_clone.pubkey(),
            TokenAccountsFilter::Mint(pool_keys_clone.base_mint),
        )
        .await?;

    for rpc_keyed_account in &token_accounts {
        let pubkey = &rpc_keyed_account.pubkey;
        //convert to pubkey
        let pubkey = Pubkey::from_str(&pubkey)?;

        let balance = rpc_client_clone.get_token_account_balance(&pubkey).await?;
        let lamports = match balance.amount.parse::<u64>() {
            Ok(lamports) => lamports,
            Err(e) => {
                eprintln!("Failed to parse balance: {}", e);
                break;
            }
        };

        token_balance = lamports;

        if lamports != 0 {
            break;
        }

        std::thread::sleep(Duration::from_secs(1));
    }

    info!("Token Balance: {:?}", token_balance);

    let _ = match raydium_out(&wallet, pool_keys, token_balance, 0, fees, args).await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    };

    Ok(())
}
