use std::sync::Arc;

use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{bs58, signature::Keypair};
use tokio::sync::mpsc::channel;

use crate::{
    env::load_settings,
    plugins::jito_plugin::event_loop::bundle_results_loop,
    raydium::{pool_searcher::amm_keys::pool_keys_fetcher, swap::swapper::auth_keypair},
    rpc::HTTP_CLIENT,
    user_inputs::{
        amounts::{amount_percentage, bundle_priority_tip, priority_fee, sol_amount},
        tokens::token_env,
    },
};

use super::{
    raydium_swap_in::raydium_in,
    raydium_swap_out::{raydium_out, raydium_txn_backrun},
};

pub async fn swap_in() -> Result<(), Box<dyn std::error::Error>> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let sol_amount = sol_amount().await;

    let mut bundle_tip = 0;
    let mut priority_fee_value = 0;
    let (bundle_results_sender, bundle_results_receiver) = channel(100);

    if args.use_bundles {
        tokio::spawn(bundle_results_loop(
            args.block_engine_url.clone(),
            Arc::new(auth_keypair()),
            bundle_results_sender,
        ));
        priority_fee_value = priority_fee().await;
        bundle_tip = bundle_priority_tip().await;
    } else {
        priority_fee_value = priority_fee().await;
    }

    let token_out = token_env("Pool Address").await;

    let private_key = Keypair::from_bytes(&bs58::decode(&args.payer_keypair).into_vec().unwrap())?;

    info!("Fetching pool keys...");
    let pool_keys = pool_keys_fetcher(token_out).await?;

    let fees = PriorityTip {
        bundle_tip,
        priority_fee_value,
    };

    info!("---------------------------------------------------");

    let _swap = match raydium_in(
        &Arc::new(private_key),
        pool_keys,
        sol_amount,
        0,
        fees,
        args,
        bundle_results_receiver,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
}

pub struct PriorityTip {
    pub bundle_tip: u64,
    pub priority_fee_value: u64,
}

pub async fn swap_out() -> Result<(), Box<dyn std::error::Error>> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let token_out = token_env("Pool Address").await;
    let sol_amount = amount_percentage().await;
    let mut bundle_tip = 0;
    let mut priority_fee_value = 0;
    if args.use_bundles {
        bundle_tip = bundle_priority_tip().await;
        priority_fee_value = priority_fee().await;
    } else {
        priority_fee_value = priority_fee().await;
    }
    let private_key = Keypair::from_bytes(&bs58::decode(&args.payer_keypair).into_vec().unwrap())?;
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let pool_keys = pool_keys_fetcher(token_out).await?;

    let fees = PriorityTip {
        bundle_tip,
        priority_fee_value,
    };

    let swap = match raydium_txn_backrun(
        rpc_client,
        &Arc::new(private_key),
        pool_keys,
        sol_amount,
        fees,
        args,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
}
