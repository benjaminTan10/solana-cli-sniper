use std::sync::Arc;

use log::{error, info};
use solana_sdk::signature::Keypair;

use crate::{
    env::load_settings,
    raydium::{pool_searcher::amm_keys::pool_keys_fetcher, swap::raydium_swap_in::price_logger},
    user_inputs::{amounts::sol_amount, tokens::token_env},
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
    let token_out = token_env("Pool Address").await;

    let private_key = Keypair::from_bytes(&bs58::decode(&args.payer_keypair).into_vec().unwrap())?;

    info!("Fetching pool keys...");
    let pool_keys = pool_keys_fetcher(token_out).await?;

    let trades = match price_logger(amount_in, pool_keys, &Arc::new(private_key)).await {
        Ok(_) => {}
        Err(e) => error!("{}", e),
    };

    Ok(())
}
