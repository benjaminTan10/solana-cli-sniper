use std::sync::Arc;

use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

use crate::{
    app::config_init::get_config,
    raydium_amm::{
        pool_searcher::amm_keys::pool_keys_fetcher, swap::raydium_swap_in::price_logger,
    },
    user_inputs::{amounts::sol_amount, tokens::token_env},
    utils::read_single_key_impl,
};

pub async fn track_trades() -> eyre::Result<()> {
    let amount_in = sol_amount("Track Trade Amount: ").await;

    let token_out = token_env("Pool Address").await;

    info!("Fetching pool keys...");
    let pool_keys = pool_keys_fetcher(token_out).await?;

    let (mut stop_tx, mut stop_rx) = tokio::sync::mpsc::channel::<()>(100);

    let pool_keys_clone = pool_keys.clone();
    tokio::spawn(async move {
        let config = get_config().await.unwrap();
        match read_single_key_impl(
            &Arc::new(RpcClient::new(config.clone().network.rpc_url)),
            &mut stop_tx,
            pool_keys_clone,
            config.clone(),
            &Arc::new(Keypair::from_base58_string(&config.engine.payer_keypair)),
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                error!("Error: {}", e);
            }
        };
    });

    price_logger(
        &mut stop_rx,
        amount_in,
        Some(pool_keys.clone()),
        None,
        crate::router::SniperRoute::RaydiumAMM,
    )
    .await;
    Ok(())
}
