use std::sync::Arc;

use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{bs58, signature::Keypair};

use crate::{
    env::load_config,
    raydium_amm::pool_searcher::amm_keys::pool_keys_fetcher,
    rpc::HTTP_CLIENT,
    user_inputs::{
        amounts::{amount_percentage, bundle_priority_tip, priority_fee, sol_amount},
        tokens::token_env,
    },
};

use super::{raydium_swap_in::raydium_in, raydium_swap_out::raydium_txn_backrun};

pub async fn swap_in() -> Result<(), Box<dyn std::error::Error>> {
    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let sol_amount = sol_amount("Swap Amount:").await;

    let mut bundle_tip = 0;
    let mut priority_fee_value = 0;

    if args.engine.use_bundles {
        priority_fee_value = priority_fee().await;
        bundle_tip = bundle_priority_tip().await;
    } else {
        priority_fee_value = priority_fee().await;
    }

    let token_out = token_env("Pool Address").await;

    let private_key =
        Keypair::from_bytes(&bs58::decode(&args.engine.payer_keypair).into_vec().unwrap())?;

    let rpc_client = Arc::new(RpcClient::new(args.clone().network.rpc_url));

    info!("Fetching pool keys...");
    let pool_keys = pool_keys_fetcher(token_out, rpc_client).await?;

    let fees = PriorityTip {
        bundle_tip,
        priority_fee_value,
    };

    info!("---------------------------------------------------");

    let _swap = match raydium_in(pool_keys, sol_amount.into(), 0).await {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
}

#[derive(Debug, Clone)]
pub struct PriorityTip {
    pub bundle_tip: u64,
    pub priority_fee_value: u64,
}

pub async fn swap_out() -> Result<(), Box<dyn std::error::Error>> {
    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let token_out = token_env("Pool Address").await;
    let sol_amount = amount_percentage().await;

    let private_key =
        Keypair::from_bytes(&bs58::decode(&args.engine.payer_keypair).into_vec().unwrap())?;
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let pool_keys = pool_keys_fetcher(token_out, rpc_client).await?;

    let swap = match raydium_txn_backrun(pool_keys, sol_amount).await {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
}
