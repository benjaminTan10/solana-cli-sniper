use jito_protos::bundle::BundleResult;
use jito_searcher_client::get_searcher_client;
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_memo::build_memo;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::Receiver;

use crate::env::env_loader::tip_account;
use crate::env::EngineSettings;
use crate::plugins::jito_plugin::lib::{send_bundles, BundledTransactions};
use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::swap::instructions::{swap_base_in, swap_base_out, SOLC_MINT};
use crate::rpc::{rpc_key, HTTP_CLIENT};

use super::swap_in::PriorityTip;
use super::swapper::auth_keypair;

pub async fn raydium_txn_backrun(
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    token_amount: u64,
    fees: PriorityTip,
    args: EngineSettings,
    mut bundle_results_receiver: Receiver<BundleResult>,
) -> eyre::Result<()> {
    let start = Instant::now();
    let mut token_balance = 0;
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    info!("BaseMint: {:?}", pool_keys.base_mint);
    while start.elapsed() < Duration::from_secs(15) {
        let token_accounts = rpc_client
            .get_token_accounts_by_owner(
                &wallet.pubkey(),
                TokenAccountsFilter::Mint(pool_keys.base_mint),
            )
            .await?;

        for rpc_keyed_account in &token_accounts {
            let pubkey = &rpc_keyed_account.pubkey;
            //convert to pubkey
            let pubkey = Pubkey::from_str(&pubkey)?;

            let balance = rpc_client.get_token_account_balance(&pubkey).await?;

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

        if token_balance != 0 {
            break;
        }
    }

    if token_balance == 0 {
        error!("No tokens found");
        return Ok(());
    }

    let percentage = 0.5;
    let token_amount = (token_balance * 1);

    info!("Token Amount: {:?}", token_amount);

    info!("Tokens: {:?}", token_balance);

    let _ = raydium_out(
        wallet,
        pool_keys.clone(),
        token_amount,
        1,
        fees,
        args,
        bundle_results_receiver,
    )
    .await?;

    Ok(())
}

pub async fn raydium_out(
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    amount_in: u64,
    amount_out: u64,
    fees: PriorityTip,
    args: EngineSettings,
    mut bundle_results_receiver: Receiver<BundleResult>,
) -> eyre::Result<()> {
    info!("Building Bundle...");

    let user_source_owner = wallet.pubkey();
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let mut searcher_client =
        get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;

    let tip_account = tip_account().await;

    let rpc_client = Arc::new(rpc_client);

    let token_address = if pool_keys.base_mint == SOLC_MINT {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };

    let swap_instructions = swap_base_out(
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
        &pool_keys.base_mint,
        amount_in,
        amount_out,
        fees.priority_fee_value,
    )
    .await?;

    let config = CommitmentLevel::Confirmed;
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
            &[
                build_memo(
                    format!(
                        "{}: {:?}",
                        "{args.message}",
                        transaction.signatures[0].to_string(),
                        // Fix: Add a placeholder for the missing argument
                    )
                    .as_bytes(),
                    &[],
                ),
                transfer(&wallet.pubkey(), &tip_account, fees.bundle_tip),
            ],
            Some(&wallet.pubkey()),
            &[&wallet],
            rpc_client.get_latest_blockhash().await.unwrap(),
        ));

        let bundle_txn = BundledTransactions {
            mempool_txs: vec![transaction],
            middle_txs: vec![],
            backrun_txs: vec![tip_txn],
        };

        let mut results = send_bundles(&mut searcher_client, &[bundle_txn]).await?;

        if let Ok(response) = results.remove(0) {
            let message = response.into_inner();
            let uuid = &message.uuid;
            info!("UUID: {}", uuid);
        }

        info!("Fetching Bundle Result...");

        // Receive bundle results
        let mut logs = Vec::new();
        let start_time = std::time::Instant::now();
        while let Some(bundle_result) = bundle_results_receiver.recv().await {
            info!(
                "received bundle_result: [bundle_id={:?}, result={:?}]",
                bundle_result.bundle_id, bundle_result.result
            );

            logs.push(bundle_result.clone());

            // Check if logs stop coming
            if bundle_result.result.is_none() {
                break;
            }

            if start_time.elapsed().as_secs() >= 2 {
                break;
            }
            if logs.len() >= 2 {
                break;
            }
        }
    } else {
        info!("Sending Transaction");
        let config = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

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

        info!("Transaction sent: {:?}", result);
    }

    Ok(())
}
