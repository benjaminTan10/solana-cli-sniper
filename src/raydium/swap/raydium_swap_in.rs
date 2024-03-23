use log::{error, info};
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::{signature::Keypair, signer::Signer};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::swap::instructions::{swap_base_in, SOLC_MINT};
use crate::rpc::HTTP_CLIENT;

use super::instructions::token_price_data;

pub async fn raydium_in(
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    amount_in: u64,
    amount_out: u64,
    priority_fee: u64,
) -> eyre::Result<()> {
    let user_source_owner = wallet.pubkey();
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

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
        amount_in,
        amount_out,
        priority_fee,
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

    info!("Transaction Signature: {:?}", result.to_string());

    // let rpc_client_1 = rpc_client.clone();
    // tokio::spawn(async move {
    //     let _ = match rpc_client_1
    //         .confirm_transaction_with_spinner(
    //             &result,
    //             &rpc_client_1.get_latest_blockhash().await.unwrap(),
    //             solana_sdk::commitment_config::CommitmentConfig::processed(),
    //         )
    //         .await
    //     {
    //         Ok(x) => x,
    //         Err(e) => {
    //             error!("Error: {:?}", e);
    //         }
    //     };
    // });

    let pool_keys_clone = pool_keys.clone();
    let wallet_clone = wallet.clone();
    let _ = price_logger(pool_keys_clone, &wallet_clone).await;
    Ok(())
}

async fn price_logger(
    //mut stop_rx: mpsc::Receiver<()>,
    pool_keys: PoolKeysSniper,
    wallet: &Arc<Keypair>,
) -> eyre::Result<()> {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    loop {
        // if let Ok(_) = stop_rx.try_recv() {
        //     break;
        // }

        let mut token_balance = 0;
        let rpc_client_clone = rpc_client.clone();
        let pool_keys_clone = pool_keys.clone();
        let wallet_clone = Arc::clone(wallet);
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
            info!("balance: {:?}", balance.ui_amount_string);
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

        let price = token_price_data(
            rpc_client_clone,
            pool_keys_clone,
            wallet_clone,
            token_balance,
        )
        .await?;

        info!("Worth: {:?} Sol", lamports_to_sol(price as u64));
        // Sleep for a while
        time::sleep(Duration::from_secs(1)).await;
    }
}
