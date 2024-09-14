use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, info};
use num_traits::Zero;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::native_token::{lamports_to_sol, sol_to_lamports};
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use solana_sdk::{signature::Keypair, signer::Signer};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::{self};
use tokio::time::{self, sleep};

use crate::app::config_init::get_config;
use crate::env::SettingsConfig;
use crate::instruction::instruction::InitializePoolAccounts;
use crate::liquidity::utils::tip_account;
use crate::pumpfun::instructions::instructions::calculate_sell_price;
use crate::pumpfun::token_data::bonding_curve_fetcher;
use crate::raydium_amm::subscribe::PoolKeysSniper;
use crate::raydium_amm::swap::instructions::{swap_base_in, SwapDirection, SOLC_MINT};
use crate::raydium_amm::swap::raydium_amm_sniper::clear_previous_line;
use crate::raydium_amm::swap::swapper::auth_keypair;
use crate::router::SniperRoute;
use crate::rpc::HTTP_CLIENT;
use crate::utils::read_single_key_impl;

use super::instructions::token_price_data;
use super::raydium_swap_out::raydium_out;
use super::swap_in::PriorityTip;

pub async fn raydium_in(
    pool_keys: PoolKeysSniper,
    init_amount_in: Option<u64>,
    amount_out: u64,
) -> eyre::Result<()> {
    let config = get_config().await?;

    let wallet = Keypair::from_base58_string(&config.engine.payer_keypair);

    let amount_in = init_amount_in.unwrap_or_else(|| sol_to_lamports(config.trading.buy_amount));

    let user_source_owner = wallet.pubkey();
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let mut searcher_client =
        get_searcher_client(&config.network.block_engine_url, &Arc::new(auth_keypair())).await?;

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
        sol_to_lamports(config.trading.priority_fee),
        config.clone(),
    )
    .await?;

    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: CommitmentLevel::Processed,
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

    if config.engine.use_bundles {
        info!("Building Bundle");

        let tip_txn = VersionedTransaction::from(Transaction::new_signed_with_payer(
            &[transfer(
                &wallet.pubkey(),
                &tip_account,
                sol_to_lamports(config.trading.bundle_tip),
            )],
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
        let transaction_flight = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

        if config.trading.spam {
            let mut counter = 0;
            while counter < config.trading.spam_count {
                let result = match rpc_client
                    .send_transaction_with_config(&transaction, transaction_flight)
                    .await
                {
                    Ok(x) => x,
                    Err(e) => {
                        error!("Error: {:?}", e);
                        return Ok(());
                    }
                };

                info!("Transaction Sent {:?}", result);
                info!("Transaction Sent {:?}", result);
                counter += 1;
            }
        } else {
            let result = match rpc_client
                .send_transaction_with_config(&transaction, transaction_flight)
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            };

            info!("Transaction Sent {:?}", result);
            info!("Transaction Sent {:?}", result);
        }
    }
    let pool_keys_clone = pool_keys.clone();

    let (mut stop_tx, mut stop_rx) = tokio::sync::mpsc::channel::<()>(100);

    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            read_single_key_impl(&mut stop_tx, pool_keys_clone)
                .await
                .unwrap();
        });
    });

    price_logger(
        &mut stop_rx,
        amount_in,
        Some(pool_keys),
        None,
        SniperRoute::RaydiumAMM,
    )
    .await;

    handle.join().unwrap();
    Ok(())
}

pub async fn price_logger(
    stop_rx: &mut mpsc::Receiver<()>,
    amount_in: u64,
    pool_keys: Option<PoolKeysSniper>,
    mint: Option<Pubkey>,
    snipe_route: SniperRoute,
) {
    sleep(Duration::from_secs(5)).await;
    let config = get_config().await.unwrap();
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    loop {
        let wallet = Arc::new(Keypair::from_base58_string(&config.engine.payer_keypair));

        if let Ok(_) = stop_rx.try_recv() {
            sleep(Duration::from_secs(10)).await;
            break;
        }

        let mut token_balance = 0;
        let rpc_client_clone = rpc_client.clone();
        let pool_keys_clone = pool_keys.clone().unwrap().clone();
        let token_accounts = match rpc_client_clone
            .get_token_accounts_by_owner(
                &wallet.pubkey(),
                TokenAccountsFilter::Mint(mint.unwrap_or(pool_keys_clone.base_mint)),
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

        let mut price = 0;
        if snipe_route == SniperRoute::RaydiumAMM {
            price = match token_price_data(
                rpc_client_clone,
                pool_keys_clone.clone(),
                wallet,
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
        } else if snipe_route == SniperRoute::PumpFun {
            price = bonding_curve_fetcher(mint.unwrap(), token_balance)
                .await
                .unwrap();
        }

        let total_value = lamports_to_sol(price as u64);
        let profit_percentage =
            ((total_value - lamports_to_sol(amount_in)) / lamports_to_sol(amount_in)) * 100.0;

        clear_previous_line();
        info!(
            "Aped: {:.3} Sol | Worth {:.4} Sol | Profit {:.2}%",
            lamports_to_sol(amount_in),
            total_value,
            profit_percentage
        );

        // if token_balance.is_zero() {
        //     break;
        // }

        // if profit_percentage >= config.trading.profit_threshold_percentage {
        //     let _ = match sell_tokens(100, pool_keys_clone.clone()).await {
        //         Ok(_) => {}
        //         Err(e) => {
        //             info!("Error: {}", e);
        //         }
        //     };

        //     break;
        // } else if profit_percentage <= config.trading.loss_threshold_percentage {
        //     let _ = match sell_tokens(100, pool_keys_clone).await {
        //         Ok(_) => {}
        //         Err(e) => {
        //             info!("Error: {}", e);
        //         }
        //     };
        //     break;
        // }

        // Sleep for a while
        time::sleep(Duration::from_millis(200)).await;
    }
}

pub async fn sell_tokens(amount: u64, pool_keys: PoolKeysSniper) -> eyre::Result<()> {
    let config = get_config().await?;

    let wallet = Keypair::from_base58_string(&config.engine.payer_keypair);
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let mut token_balance = 0;
    let rpc_client_clone = rpc_client.clone();
    let pool_keys_clone = pool_keys.clone();
    let token_accounts = rpc_client_clone
        .get_token_accounts_by_owner(
            &wallet.pubkey(),
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

    let _ = match raydium_out(pool_keys, token_balance, 0).await {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
        }
    };

    Ok(())
}
