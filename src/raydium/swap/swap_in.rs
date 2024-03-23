use std::sync::Arc;

use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{bs58, signature::Keypair};

use crate::{
    env::load_settings,
    raydium::pool_searcher::amm_keys::pool_keys_fetcher,
    rpc::HTTP_CLIENT,
    user_inputs::{
        amounts::{amount_percentage, priority_fee, sol_amount},
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
    let priority_fee = priority_fee().await;
    let token_out = token_env("Pool Address").await;

    let private_key = Keypair::from_bytes(&bs58::decode(&args.payer_keypair).into_vec().unwrap())?;
    let rpc_client = RpcClient::new(args.rpc_url.to_string());

    let pool_keys = pool_keys_fetcher(token_out).await?;

    let swap = match raydium_in(
        &Arc::new(private_key),
        pool_keys,
        sol_amount,
        0,
        priority_fee,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
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
    let priority_fee = priority_fee().await;

    let private_key = Keypair::from_bytes(&bs58::decode(&args.payer_keypair).into_vec().unwrap())?;
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let pool_keys = pool_keys_fetcher(token_out).await?;

    let swap = match raydium_txn_backrun(rpc_client, &Arc::new(private_key), pool_keys, sol_amount)
        .await
    {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
}
