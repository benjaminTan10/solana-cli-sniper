use log::{error, info};
use once_cell::sync::Lazy;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::fs::write;

use crate::app::private_key_env;
use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::utils::utils::LIQUIDITY_STATE_LAYOUT_V4;
use crate::raydium::volume_pinger::volume::buy_amount;
use crate::user_inputs::amounts::{bundle_priority_tip, priority_fee};
use crate::{
    env::load_settings, plugins::jito_plugin::lib::backrun_jito,
    raydium::pool_searcher::amm_keys::pool_keys_fetcher,
};

use futures::stream::StreamExt;

use super::raydiumupdate::{load_json_to_hashmap, update_raydium};

pub static POOL_KEYS: Lazy<Mutex<HashMap<Pubkey, PoolKeysSniper>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct MEVBotSettings {
    pub min_amount: u64,
    pub max_amount: u64,
    pub priority_fee: u64,
    pub bundle_tip: u64,
    pub wallet: String,
}

pub async fn mev_trades() -> Result<(), Box<dyn Error>> {
    let backrun_keys = match update_raydium().await {
        Ok(keys) => keys,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };

    let min_amount = buy_amount("Min Amount").await?;
    let max_amount = buy_amount("Max Amount").await?;
    let priority_fee = priority_fee().await;
    let bundle_tip = bundle_priority_tip().await;
    let wallet = private_key_env().await?;

    let settings = MEVBotSettings {
        min_amount,
        max_amount,
        priority_fee,
        bundle_tip,
        wallet,
    };

    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    // let fetches = backrun.into_iter().map(|account| async move {
    //     let (pool_keys, _) = match pool_keys_fetcher(account).await {
    //         Ok(pool_keys) => (pool_keys, LIQUIDITY_STATE_LAYOUT_V4::default()),
    //         Err(e) => {
    //             error!("Error: {:?}", e);
    //             (
    //                 PoolKeysSniper::default(),
    //                 LIQUIDITY_STATE_LAYOUT_V4::default(),
    //             )
    //         }
    //     };
    //     (account, pool_keys)
    // });

    // let mut map = POOL_KEYS.lock().unwrap();
    // futures::stream::iter(fetches)
    //     .buffer_unordered(100)
    //     .for_each(|(account, pool_keys)| {
    //         map.insert(account, pool_keys.clone());
    //         info!("Fetched keys for account {}: {:?}", account, pool_keys);
    //         futures::future::ready(())
    //     })
    //     .await;

    // drop(map);

    // // args.backrun_accounts = backrun_keys;

    // let _ = match backrun_jito(args, Arc::new(settings)).await {
    //     Ok(_) => info!("Jito Started"),
    //     Err(e) => error!("{}", e),
    // };

    Ok(())
}
