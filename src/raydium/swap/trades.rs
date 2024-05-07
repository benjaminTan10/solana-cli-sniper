use std::sync::Arc;

use log::{error, info};
use solana_sdk::signature::Keypair;
use tokio::sync::mpsc::channel;

use crate::{
    env::load_settings,
    plugins::jito_plugin::event_loop::bundle_results_loop,
    raydium::{
        pool_searcher::amm_keys::pool_keys_fetcher,
        swap::{raydium_swap_in::price_logger, swap_in::PriorityTip, swapper::auth_keypair},
    },
    user_inputs::{
        amounts::{bundle_priority_tip, priority_fee, sol_amount},
        tokens::token_env,
    },
    utils::read_single_key_impl,
};

pub async fn track_trades() -> eyre::Result<()> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let amount_in = sol_amount().await;
    let mut bundle_tip = 0;
    let mut priority_fee_value = 0;

    if args.use_bundles {
        priority_fee_value = priority_fee().await;
        bundle_tip = bundle_priority_tip().await;
    } else {
        priority_fee_value = priority_fee().await;
    }
    let token_out = token_env("Pool Address").await;

    info!("Fetching pool keys...");
    let pool_keys = pool_keys_fetcher(token_out).await?;

    let (mut stop_tx, mut stop_rx) = tokio::sync::mpsc::channel::<()>(100);

    let fees = PriorityTip {
        bundle_tip,
        priority_fee_value,
    };
    let private_key = Keypair::from_bytes(&bs58::decode(&args.payer_keypair).into_vec().unwrap())?;
    let wallet = Arc::new(private_key);
    let pool_keys_clone = pool_keys.clone();
    let wallet_clone = wallet.clone();
    tokio::spawn(async move {
        match read_single_key_impl(&mut stop_tx, pool_keys_clone, args, fees, &wallet_clone).await {
            Ok(_) => {}
            Err(e) => {
                error!("Error: {}", e);
            }
        };
    });

    price_logger(&mut stop_rx, amount_in, pool_keys.clone(), wallet).await;
    Ok(())
}
