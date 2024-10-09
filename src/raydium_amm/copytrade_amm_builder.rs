use {
    super::{
        subscribe::PoolKeysSniper,
        swap::raydium_swap_in::{raydium_in, TradeDirection},
    },
    crate::{
        app::config_init::get_config,
        copytrade::copytrading_decoder::MevApe,
        raydium_amm::swap::{metadata::decode_metadata, raydium_amm_sniper::clear_previous_line},
    },
    anchor_lang::prelude::{AccountMeta, ProgramError},
    chrono::{DateTime, Utc},
    colorize::AnsiColor,
    crossterm::style::Stylize,
    eyre::Context,
    jito_protos::searcher::SubscribeBundleResultsRequest,
    jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation},
    log::{debug, error, info, warn},
    once_cell::sync::Lazy,
    serum_dex::instruction::MarketInstruction,
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig},
    solana_sdk::{
        compute_budget::ComputeBudgetInstruction,
        instruction::Instruction,
        native_token::sol_to_lamports,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_instruction::transfer,
        transaction::{Transaction, VersionedTransaction},
    },
    spl_associated_token_account::get_associated_token_address,
    spl_token::instruction::TokenInstruction,
    std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::{time::sleep, try_join},
};

pub async fn copytrade_raydium_amm_builder(
    pool_keys: PoolKeysSniper,
    sleep_duration: u64,
    inputs: Arc<MevApe>,
    datetime: chrono::DateTime<Utc>,
) -> eyre::Result<()> {
    let config = get_config().await?;
    let rpc_client = Arc::new(RpcClient::new(config.network.rpc_url.clone()));
    let params_clone = config.clone();

    tokio::spawn(async move {
        let (token, _) = mpl_token_metadata::accounts::Metadata::find_pda(&pool_keys.base_mint);
        let metadata = match decode_metadata(&token).await {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                error!("Error: {:?}", e);
                None
            }
        };

        let token_name = metadata
            .clone()
            .and_then(|m| Some(m.name))
            .unwrap_or_else(|| "Unknown".to_string());
        let token_symbol = metadata
            .clone()
            .and_then(|m| Some(m.symbol))
            .unwrap_or_else(|| "Unknown".to_string());

        clear_previous_line();
        println!(
            "Name: {}\nSymbol: {}\nBase Mint: {}\nPool ID: {}",
            colorize::AnsiColor::bold(token_name.to_string()).white(),
            colorize::AnsiColor::bold(token_symbol.to_string()).b_cyan(),
            pool_keys.base_mint.to_string(),
            pool_keys.id.to_string(),
        );
        println!("Open Time: {}", datetime.to_string());
    });

    let config = get_config().await?;

    let private_key = &inputs.wallet;
    let secret_key = bs58::decode(private_key.clone()).into_vec()?;

    let wallet = Keypair::from_bytes(&secret_key)?;
    let amount_in = &inputs.sol_amount;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs();

    let sleep_duration = if sleep_duration > current_time {
        info!(
            "Sleep Duration: {:?} Mins",
            (sleep_duration - current_time) / 60
        );
        Duration::from_secs(sleep_duration - current_time)
    } else {
        Duration::from_secs(0)
    };

    sleep(sleep_duration).await;

    let _ = match raydium_in(
        &rpc_client,
        &Arc::new(wallet),
        pool_keys.clone(),
        *amount_in,
        1,
        config.clone(),
        TradeDirection::Buy,
    )
    .await
    {
        Ok(tx) => tx,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(eyre::eyre!("Error: {:?}", e));
        }
    };

    Ok(())
}
