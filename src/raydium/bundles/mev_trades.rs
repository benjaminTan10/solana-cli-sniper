use std::error::Error;

use log::{error, info};

use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::utils::utils::LIQUIDITY_STATE_LAYOUT_V4;
use crate::{
    env::load_settings, plugins::jito_plugin::lib::backrun_jito,
    raydium::pool_searcher::amm_keys::pool_keys_fetcher,
};

pub async fn mev_trades() -> Result<(), Box<dyn Error>> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    let backrun = args.clone().backrun_accounts;
    for account in backrun {
        let (pool_keys, _) = match pool_keys_fetcher(account.to_string()).await {
            Ok(pool_keys) => pool_keys,
            Err(e) => {
                error!("Error: {:?}", e);
                (
                    PoolKeysSniper::default(),
                    LIQUIDITY_STATE_LAYOUT_V4::default(),
                )
            }
        };
        info!("index {:?}", pool_keys);
        let mut map = POOL_KEYS.lock().unwrap();
        map.insert(account.to_string(), pool_keys);
    }
    let _ = match backrun_jito(args).await {
        Ok(_) => info!("Jito Started"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static POOL_KEYS: Lazy<Mutex<HashMap<String, PoolKeysSniper>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
