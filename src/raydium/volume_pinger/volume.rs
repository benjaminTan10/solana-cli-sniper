use std::{
    error::Error,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use demand::Input;
use log::{error, info};
use rand::Rng;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig,
    rpc_request::TokenAccountsFilter,
};
use solana_sdk::{
    commitment_config::CommitmentLevel,
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    native_token::sol_to_lamports,
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    app::{priority_fee, theme, token_env},
    env::load_settings,
    raydium::{
        bundles::swap_instructions::volume_swap_base_in,
        pool_searcher::amm_keys::pool_keys_fetcher,
        subscribe::PoolKeysSniper,
        swap::instructions::{
            swap_base_out, token_price_data, AmmInstruction, SwapInstructionBaseIn, SOLC_MINT,
        },
        utils::utils::LIQUIDITY_STATE_LAYOUT_V4,
    },
};

pub struct VolumeBotSettings {
    pub buy_amount: u64,
    pub priority_fee: u64,
    pub wallet: Keypair,
}

pub async fn buy_amount(input: &str) -> Result<u64, Box<dyn Error>> {
    let theme = theme();
    let t = Input::new(format!("{}:", input))
        .placeholder("0.01")
        .theme(&theme)
        .prompt("Input: ");

    let string = t.run().expect("error running input");

    let amount = sol_to_lamports(string.parse::<f64>()?);

    Ok(amount)
}

pub async fn generate_volume() -> Result<(), Box<dyn Error>> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };
    let min_amount = buy_amount("Min Amount").await?;
    let max_amount = buy_amount("Max Amount").await?;

    let priority_fee = priority_fee().await?;
    let pool_address = token_env().await?;

    let secret_key = bs58::decode(args.payer_keypair.clone()).into_vec()?;

    let (pool_keys, amm_info) = pool_keys_fetcher(pool_address.to_string()).await?;

    info!(
        "Pool Keys: {}",
        serde_json::to_string_pretty(&pool_keys).unwrap()
    );
    for _ in 0..4 {
        let wallet = Keypair::from_bytes(&secret_key)?;
        let rpc_client = RpcClient::new(args.rpc_url.to_string());

        let mut rng = rand::thread_rng();
        let buy_amount: u64 = rng.gen_range(min_amount..=max_amount);
        let volume_settings = VolumeBotSettings {
            buy_amount,
            priority_fee,
            wallet,
        };

        let _ = match volume_round(
            Arc::new(rpc_client),
            pool_keys.clone(),
            amm_info.clone(),
            volume_settings,
        )
        .await
        {
            Ok(x) => x,
            Err(e) => {
                error!("Error: {:?}", e);
                return Ok(());
            }
        };
    }
    Ok(())
}

pub async fn volume_round(
    rpc_client: Arc<RpcClient>,
    pool_keys: PoolKeysSniper,
    amm_info: LIQUIDITY_STATE_LAYOUT_V4,
    volume_bot: VolumeBotSettings,
) -> Result<(), Box<dyn Error>> {
    let wallet = Arc::new(&volume_bot.wallet);
    let user_source_owner = wallet.pubkey();

    let token_address = if pool_keys.base_mint == SOLC_MINT.to_string() {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };
    let tokens_amount = token_price_data(
        rpc_client.clone(),
        pool_keys.clone(),
        wallet.clone(),
        volume_bot.buy_amount,
    )
    .await?;

    let tokens_amount = tokens_amount * 999 / 1000;

    info!("Swap amount out: {}", tokens_amount);

    let transaction_main_instructions = volume_swap_base_in(
        &Pubkey::from_str(&pool_keys.program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.id).unwrap(),
        &Pubkey::from_str(&pool_keys.authority).unwrap(),
        &Pubkey::from_str(&pool_keys.open_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.target_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_bids).unwrap(),
        &Pubkey::from_str(&pool_keys.market_asks).unwrap(),
        &Pubkey::from_str(&pool_keys.market_event_queue).unwrap(),
        &Pubkey::from_str(&pool_keys.market_base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_authority).unwrap(),
        &user_source_owner,
        &user_source_owner,
        &Pubkey::from_str(&token_address).unwrap(),
        volume_bot.buy_amount,
        tokens_amount as u64,
        volume_bot.priority_fee,
        rpc_client.clone(),
    )
    .await?;

    // transaction_main_instructions.extend(swap_out_instructions);

    let config = CommitmentLevel::Confirmed;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &transaction_main_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let frontrun_tx = match VersionedTransaction::try_new(
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
    let txn = rpc_client
        .send_transaction_with_config(&frontrun_tx, config)
        .await;

    match txn {
        Ok(x) => {
            info!("Swap in Transaction: {:?}", x);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }

    // let mut token_balance = 0;
    // let start = Instant::now();
    // while start.elapsed() < Duration::from_secs(60) {
    //     let token_accounts = rpc_client
    //         .get_token_accounts_by_owner(
    //             &wallet.pubkey(),
    //             TokenAccountsFilter::Mint(Pubkey::from_str(&pool_keys.base_mint).unwrap()),
    //         )
    //         .await?;

    //     for rpc_keyed_account in &token_accounts {
    //         let pubkey = &rpc_keyed_account.pubkey;
    //         //convert to pubkey
    //         let pubkey = Pubkey::from_str(&pubkey)?;

    //         let balance = rpc_client.get_token_account_balance(&pubkey).await?;
    //         let lamports = match balance.amount.parse::<u64>() {
    //             Ok(lamports) => lamports,
    //             Err(e) => {
    //                 eprintln!("Failed to parse balance: {}", e);
    //                 break;
    //             }
    //         };

    //         token_balance = lamports;

    //         if lamports != 0 {
    //             break;
    //         }
    //     }

    //     if token_balance != 0 {
    //         info!("Token Balance: {:?}", token_balance);
    //         break;
    //     }
    // }
    let swap_out_instructions = swap_base_out(
        &Pubkey::from_str(&pool_keys.program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.id).unwrap(),
        &Pubkey::from_str(&pool_keys.authority).unwrap(),
        &Pubkey::from_str(&pool_keys.open_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.target_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_bids).unwrap(),
        &Pubkey::from_str(&pool_keys.market_asks).unwrap(),
        &Pubkey::from_str(&pool_keys.market_event_queue).unwrap(),
        &Pubkey::from_str(&pool_keys.market_base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_authority).unwrap(),
        &user_source_owner,
        &user_source_owner,
        &Pubkey::from_str(&token_address).unwrap(),
        tokens_amount as u64,
        0,
        volume_bot.priority_fee,
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
        &swap_out_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };
    let frontrun_tx = match VersionedTransaction::try_new(
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
    let txn = rpc_client
        .send_transaction_with_config(&frontrun_tx, config)
        .await;

    match txn {
        Ok(x) => {
            info!("Swap out Transaction: {:?}", x);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
    Ok(())
}
