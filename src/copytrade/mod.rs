use colorize::AnsiColor;
use crossterm::style::Stylize;
use log::{error, info};
use subscription::copytrading_grpc;

use crate::{
    app::MevApe,
    env::load_config,
    raydium_amm::{
        subscribe::auto_sniper_stream,
        swap::{metadata::decode_metadata, swap_in::PriorityTip},
    },
    router::SniperRoute,
    user_inputs::{
        amounts::{bundle_priority_tip, priority_fee, sol_amount},
        tokens::token_env,
    },
};

pub mod copytrading_decoder;
pub mod subscription;

pub async fn copytrade() -> eyre::Result<()> {
    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let sol_amount = sol_amount("Snipe Amount:").await;

    let token;

    let mut bundle_tip = 0;
    let priority_fee_value;

    if args.engine.use_bundles {
        priority_fee_value = priority_fee().await;
        bundle_tip = bundle_priority_tip().await;
    } else {
        priority_fee_value = priority_fee().await;
    }

    if args.trading.copytrade_accounts.is_empty() {
        token = Some(token_env("Copytrade Account: ").await);

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

    let addresses = args.trading.copytrade_accounts.clone();

    let _ = match copytrading_grpc(mev_ape, args, addresses.into()).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}
