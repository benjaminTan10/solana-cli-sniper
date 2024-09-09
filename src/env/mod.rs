use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

use demand::Input;
use log::info;
use serde::{Deserialize, Serialize};
use toml;

use crate::app::private_key_env;

pub mod input;
pub mod minter;
pub mod utils;
pub mod vanity;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingsConfig {
    pub user: UserSettings,
    pub network: NetworkSettings,
    pub engine: EngineSettings,
    pub trading: TradingSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSettings {
    pub username: String,
    pub license_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkSettings {
    pub block_engine_url: String,
    pub pubsub_url: String,
    pub rpc_url: String,
    pub grpc_url: String,
    pub regions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EngineSettings {
    pub payer_keypair: String,
    pub subscribe_bundle_results: bool,
    pub use_bundles: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradingSettings {
    pub spam: bool,
    pub spam_count: i32,
    pub slippage: bool,
    pub bundle_tip: f64,
    pub copytrade_accounts: Vec<String>,
    pub loss_threshold_percentage: f64,
    pub profit_threshold_percentage: f64,
}

pub async fn load_config() -> eyre::Result<SettingsConfig> {
    let config_content = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        info!("Config file not found, creating a new one");
        let default_config = SettingsConfig {
            user: UserSettings {
                username: String::new(),
                license_key: String::new(),
            },
            network: NetworkSettings {
                block_engine_url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
                pubsub_url:
                    "wss://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                rpc_url:
                    "https://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                grpc_url: String::new(),
                regions: vec!["ny".to_string()],
            },
            engine: EngineSettings {
                payer_keypair: String::new(),
                subscribe_bundle_results: false,
                use_bundles: true,
            },
            trading: TradingSettings {
                spam: false,
                spam_count: 15,
                slippage: false,
                bundle_tip: 0.0,
                copytrade_accounts: vec![],
                loss_threshold_percentage: 50.0,
                profit_threshold_percentage: 100.0,
            },
        };
        let default_toml = toml::to_string(&default_config).unwrap();
        let mut file = File::create("config.toml").unwrap();
        file.write_all(default_toml.as_bytes()).unwrap();
        default_toml
    });

    let mut config: SettingsConfig = toml::from_str(&config_content).unwrap_or_else(|_| {
        info!("Invalid config file, creating a new one");
        let default_config = SettingsConfig {
            user: UserSettings {
                username: String::new(),
                license_key: String::new(),
            },
            network: NetworkSettings {
                block_engine_url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
                pubsub_url:
                    "wss://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                rpc_url:
                    "https://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                grpc_url: String::new(),
                regions: vec!["ny".to_string()],
            },
            engine: EngineSettings {
                payer_keypair: String::new(),
                subscribe_bundle_results: false,
                use_bundles: true,
            },
            trading: TradingSettings {
                spam: false,
                spam_count: 15,
                slippage: false,
                bundle_tip: 0.0,
                copytrade_accounts: vec![],
                loss_threshold_percentage: 50.0,
                profit_threshold_percentage: 100.0,
            },
        };
        let default_toml = toml::to_string(&default_config).unwrap();
        let mut file = File::create("config.toml").unwrap();
        file.write_all(default_toml.as_bytes()).unwrap();
        default_config
    });

    if config.user.username.is_empty() {
        config.user.username = register_sims("Enter Discord Username: ", "popuy...")
            .await
            .unwrap();
    }

    if config.user.license_key.is_empty() {
        config.user.license_key = register_sims("Enter License Key: ", "MEVA........ImCh")
            .await
            .unwrap();
    }

    if config.engine.payer_keypair.is_empty() {
        config.engine.payer_keypair = private_key_env("Enter Wallet Private-Key: ").await.unwrap();
    }

    let updated_config = toml::to_string_pretty(&config).unwrap();
    fs::write("config.toml", updated_config)?;

    Ok(config)
}

pub async fn register_sims(key: &str, place_holder: &str) -> Result<String, Box<dyn Error>> {
    let t = Input::new(key).placeholder(place_holder).prompt("Input: ");
    let string = t.run().expect("error running input");
    Ok(string)
}
