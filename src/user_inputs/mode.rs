use std::{error::Error, sync::Arc};

use colorize::AnsiColor;
use crossterm::style::Stylize;
use demand::{DemandOption, Select};
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::{
    app::{theme, MevApe},
    env::load_config,
    raydium_amm::{
        subscribe::auto_sniper_stream,
        swap::{
            instructions::{unwrap_sol, wrap_sol},
            metadata::decode_metadata,
            raydium_amm_sniper::RAYDIUM_AMM_FEE_COLLECTOR,
            swap_in::PriorityTip,
        },
    },
    router::{grpc_pair_sub, SniperRoute},
};

use super::{
    amounts::{bundle_priority_tip, priority_fee, sol_amount},
    tokens::token_env,
};

pub async fn wrap_sol_call() -> Result<(), Box<dyn Error>> {
    let sol_amount = sol_amount("Wrap Amount: ").await;
    // let wallet = private_key_env().await?;

    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };
    let private_key =
        Keypair::from_bytes(&bs58::decode(args.engine.payer_keypair).into_vec().unwrap())?;
    let rpc_client = RpcClient::new(args.network.rpc_url.to_string());

    let _ = match wrap_sol(Arc::new(rpc_client), &private_key, sol_amount).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}

pub async fn unwrap_sol_call() -> Result<(), Box<dyn Error>> {
    let theme = theme();
    let ms = Select::new("Unwrap Wallet")
        .description("Select the Wallet to Unwrap")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("deployerwallets").label("🧨 Deployer Wallets"))
        .option(DemandOption::new("folder_deployerwallets").label("🗃️  Sniper Wallet"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "deployerwallets" => {
            let _ = unwrap_sol(true).await;
        }
        "folder_deployerwallets" => {
            let _ = unwrap_sol(false).await;
        }
        _ => {}
    }

    Ok(())
}

pub async fn automatic_snipe(manual_snipe: bool) -> eyre::Result<()> {
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

    let mut token;

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
        RAYDIUM_AMM_FEE_COLLECTOR.into(),
        SniperRoute::RaydiumAMM,
    )
    .await
    {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}
