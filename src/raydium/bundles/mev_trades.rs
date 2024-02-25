use log::{error, info};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::app::{bundle_priority_tip, priority_fee};
use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::utils::utils::LIQUIDITY_STATE_LAYOUT_V4;
use crate::raydium::volume_pinger::volume::buy_amount;
use crate::{
    env::load_settings, plugins::jito_plugin::lib::backrun_jito,
    raydium::pool_searcher::amm_keys::pool_keys_fetcher,
};

use futures::stream::StreamExt;

pub static POOL_KEYS: Lazy<Mutex<HashMap<String, PoolKeysSniper>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct MEVBotSettings {
    pub min_amount: u64,
    pub max_amount: u64,
    pub priority_fee: u64,
    pub bundle_tip: u64,
}

pub async fn mev_trades() -> Result<(), Box<dyn Error>> {
    let min_amount = buy_amount("Min Amount").await?;
    let max_amount = buy_amount("Max Amount").await?;
    let priority_fee = priority_fee().await?;
    let bundle_tip = bundle_priority_tip().await?;

    let settings = MEVBotSettings {
        min_amount,
        max_amount,
        priority_fee,
        bundle_tip,
    };

    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let backrun = args.clone().backrun_accounts;
    let fetches = backrun.into_iter().map(|account| async move {
        let (pool_keys, _) = match pool_keys_fetcher(account.id.to_string()).await {
            Ok(pool_keys) => pool_keys,
            Err(e) => {
                error!("Error: {:?}", e);
                (
                    PoolKeysSniper::default(),
                    LIQUIDITY_STATE_LAYOUT_V4::default(),
                )
            }
        };
        (account, pool_keys)
    });

    let mut map = POOL_KEYS.lock().unwrap();
    futures::stream::iter(fetches)
        .buffer_unordered(100)
        .for_each(|(account, pool_keys)| {
            map.insert(account.id.to_string(), pool_keys.clone());
            info!("Fetched keys for account {}: {:?}", account.id, pool_keys);
            futures::future::ready(())
        })
        .await;

    //release the lock
    drop(map);

    let _ = match backrun_jito(args, Arc::new(settings)).await {
        Ok(_) => info!("Jito Started"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}
