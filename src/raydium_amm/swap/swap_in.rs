use std::sync::Arc;

use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{bs58, signature::Keypair};

use crate::{
    env::load_config,
    input::{amount_input, mint_input, percentage_input},
    raydium_amm::{
        pool_searcher::amm_keys::pool_keys_fetcher, swap::raydium_swap_in::TradeDirection,
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

    let rpc_client = &Arc::new(RpcClient::new(args.network.rpc_url.clone()));

    let sol_amount = amount_input("Swap Amount:").await;

    let token_out = mint_input("Pool Address").await;

    let private_key =
        Keypair::from_bytes(&bs58::decode(&args.engine.payer_keypair).into_vec().unwrap())?;

    info!("Fetching pool keys...");
    let pool_keys = pool_keys_fetcher(token_out).await?;

    info!("---------------------------------------------------");

    let _swap = match raydium_in(
        rpc_client,
        &Arc::new(private_key),
        pool_keys,
        sol_amount,
        0,
        args,
        TradeDirection::Buy,
    )
    .await
    {
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

    let rpc_client = &Arc::new(RpcClient::new(args.network.rpc_url.clone()));

    let token_out = mint_input("Pool Address").await;

    let private_key =
        Keypair::from_bytes(&bs58::decode(&args.engine.payer_keypair).into_vec().unwrap())?;

    let pool_keys = pool_keys_fetcher(token_out).await?;

    let percentage = percentage_input().await;

    let _swap = match raydium_txn_backrun(
        rpc_client,
        &Arc::new(private_key),
        pool_keys,
        percentage as u64,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
}
