use std::sync::Arc;

use borsh::BorshDeserialize;
use colorize::AnsiColor;
use crossterm::style::Stylize;
use futures::{channel::mpsc::SendError, Sink};
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::geyser::{SubscribeRequest, SubscribeUpdateTransaction};

use crate::{
    app::MevApe,
    env::load_config,
    moonshot::instructions::instructions::TokenMintIxData,
    raydium_amm::{
        subscribe::auto_sniper_stream,
        swap::{metadata::decode_metadata, swap_in::PriorityTip},
    },
    router::{grpc_pair_sub, SniperRoute},
    user_inputs::{
        amounts::{bundle_priority_tip, priority_fee, sol_amount},
        tokens::token_env,
    },
};

pub const MOONSHOT_CONTRACT: &str = "MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG";

pub async fn moonshot_sniper(manual_snipe: bool) -> eyre::Result<()> {
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

    let token;

    let mut bundle_tip = 0;
    let mut priority_fee_value = 0;

    if args.engine.use_bundles {
        priority_fee_value = priority_fee().await;
        bundle_tip = bundle_priority_tip().await;
    } else {
        priority_fee_value = priority_fee().await;
    }

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

    // let wallet = private_key_env().await?;

    let fees = PriorityTip {
        priority_fee_value,
        bundle_tip,
    };

    let mev_ape = MevApe {
        sol_amount,
        fee: fees,
        // bundle_tip,
        wallet: args.engine.payer_keypair.clone(),
    };

    let _ = match grpc_pair_sub(
        mev_ape,
        args,
        manual_snipe,
        token,
        MOONSHOT_CONTRACT.into(),
        SniperRoute::MoonShot,
    )
    .await
    {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}

pub async fn moonshot_parser(
    rpc_client: Arc<RpcClient>,
    tx: SubscribeUpdateTransaction,
    manual_snipe: bool,
    mev_ape: Arc<MevApe>,
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

    let mut coin_args: Option<TokenMintIxData> = None;
    for instructions in outer_instructions.iter() {
        match TokenMintIxData::deserialize(&instructions.data) {
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

    info!(
        "Transaction: {}\nCoin: {:?}\nMaker: {}\nMint: {}\nConfig: {}",
        &signature.to_string(),
        coin_args.as_ref().unwrap().0,
        accounts[0],
        accounts[1],
        accounts[2]
    );

    println!("Account: {:?}", accounts);

    Ok(())
}
