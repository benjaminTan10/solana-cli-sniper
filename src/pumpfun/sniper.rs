use std::sync::Arc;

use colorize::AnsiColor;
use crossterm::style::Stylize;
use futures::{channel::mpsc::SendError, Sink};
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeUpdateTransaction};

use crate::{
    app::config_init::update_config_field,
    env::{load_config, SettingsConfig},
    pumpfun::instructions::pumpfun_program::instructions::CreateIxData,
    raydium_amm::{
        subscribe::auto_sniper_stream,
        swap::{
            metadata::decode_metadata, raydium_amm_sniper::RAYDIUM_AMM_FEE_COLLECTOR,
        },
    },
    router::{grpc_pair_sub, SniperRoute},
    user_inputs::{
        amounts::sol_amount,
        tokens::token_env,
    },
};

pub const PUMPFUN_CONTRACT: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
pub const PUMPFUN_MIGRATION: &str = "39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg";

pub async fn pumpfun_sniper(manual_snipe: bool, route: SniperRoute) -> eyre::Result<()> {
    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    if args.network.grpc_url.is_empty() {
        let _ = auto_sniper_stream(manual_snipe).await?;
        return Ok(());
    }
    let sol_amount = sol_amount("Snipe Amount:").await;

    update_config_field(|c| &mut c.trading.buy_amount, lamports_to_sol(sol_amount)).await?;

    let token;

    if manual_snipe {
        token = Some(token_env("Base Mint").await);

        let (token, data) = mpl_token_metadata::accounts::Metadata::find_pda(&token.unwrap());
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

        println!(
            "Name: {}\nSymb: {}",
            colorize::AnsiColor::bold(token_name.to_string()).white(),
            colorize::AnsiColor::bold(token_name.to_string()).b_cyan(),
        );

        info!("Listening for the Launch...")
    } else {
        token = None;
    }

    let contract = if route == SniperRoute::PumpFun {
        PUMPFUN_CONTRACT
    } else if route == SniperRoute::RaydiumAMM {
        RAYDIUM_AMM_FEE_COLLECTOR
    } else if route == SniperRoute::PumpFunMigration {
        PUMPFUN_MIGRATION
    } else {
        RAYDIUM_AMM_FEE_COLLECTOR
    };

    let _ = match grpc_pair_sub(args, manual_snipe, token, contract.into(), route).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}

pub async fn pumpfun_parser(
    rpc_client: Arc<RpcClient>,
    args: SettingsConfig,
    tx: SubscribeUpdateTransaction,
    manual_snipe: bool,
    base_mint: Option<Pubkey>,
    subscribe_tx: tokio::sync::MutexGuard<
        '_,
        impl Sink<SubscribeRequest, Error = SendError> + std::marker::Unpin,
    >,
) -> eyre::Result<()> {
    let info = tx.clone().transaction.unwrap_or_default();
    let accounts = info
        .transaction
        .clone()
        .unwrap_or_default()
        .message
        .unwrap_or_default()
        .account_keys
        .iter()
        .map(|i| {
            let mut array = [0; 32];
            let bytes = &i[..array.len()]; // panics if not enough data
            array.copy_from_slice(bytes);
            Pubkey::new_from_array(array)
        })
        .collect::<Vec<Pubkey>>();
    let outer_instructions = {
        let transaction = info.transaction.clone().unwrap_or_default();
        let message = transaction.message.unwrap_or_default();
        let instructions = message.instructions.iter();
        instructions.cloned().collect::<Vec<_>>()
    };

    let inner_instructions = {
        let transaction = info.meta.unwrap_or_default();
        let message = transaction.inner_instructions;
        message
    };

    let mut coin_found = false;

    let mut coin_args: Option<CreateIxData> = None;
    for instructions in outer_instructions.iter() {
        match CreateIxData::deserialize(&instructions.data) {
            Ok(decode_new_coin) => {
                coin_found = true;
                coin_args = Some(decode_new_coin);
                break;
            }
            Err(_) => {
                continue;
            }
        };
    }

    if !coin_found {
        return Ok(());
    }

    let signature = bs58::encode(&info.signature).into_string();

    if base_mint.is_some() {
        if accounts[1] != base_mint.unwrap() {
            return Ok(());
        }
    }

    info!(
        "Transaction: {}\nCoin: {:?}\nMaker: {}\nMint: {}",
        &signature.to_string(),
        coin_args.as_ref().unwrap().0,
        accounts[0],
        accounts[1]
    );

    // match pump_swap(&Arc::new(wallet), args, PumpFunDirection::Buy).await {
    //     Ok(s) => s,
    //     Err(e) => {
    //         log::error!("Error: {}", e);
    //         return Ok(());
    //     }
    // };

    Ok(())
}
