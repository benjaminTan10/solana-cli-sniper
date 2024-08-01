use std::{
    fs::{self, File},
    io::Write,
    rc::Rc,
    sync::Arc,
};

use anchor_client::{Client, Cluster};
use console::Key;
use log::info;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::{app::private_key_env, user_inputs::tokens::token_env};

#[derive(Debug, Clone)]
pub struct BackrunAccount {
    pub id: String,
    pub account: Pubkey,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolDataSettings {
    #[serde(rename = "TOKEN-MINT")]
    pub token_mint: String,

    #[serde(rename = "MARKET-ADDRESS")]
    pub market_id: String,

    #[serde(rename = "POOL-ID")]
    pub pool_id: String,

    #[serde(rename = "DEPLOYER-PRIVATE-KEY")]
    pub deployer_key: String,

    #[serde(rename = "BUYER-PRIVATE-KEY")]
    pub buyer_key: String,

    #[serde(rename = "LUT-KEY")]
    pub lut_key: String,

    #[serde(rename = "VOLUME-LUT-KEY")]
    pub volume_lut_key: String,
}

#[derive(Deserialize, Serialize, Clone, Default)]
struct HelperSettings {
    #[serde(rename = "TOKEN-MINT")]
    token_mint: String,

    #[serde(rename = "MARKET-ADDRESS")]
    market_id: String,

    #[serde(rename = "POOL-ID")]
    pool_id: String,

    #[serde(rename = "DEPLOYER-PRIVATE-KEY")]
    deployer_key: String,

    #[serde(rename = "BUYER-PRIVATE-KEY")]
    buyer_key: String,

    #[serde(rename = "LUT-KEY")]
    lut_key: String,

    #[serde(rename = "VOLUME-LUT-KEY")]
    volume_lut_key: String,
}

pub async fn load_minter_settings() -> eyre::Result<PoolDataSettings> {
    let args = match fs::read_to_string("bundler_settings.json") {
        Ok(args) => args,
        Err(_) => {
            info!("Settings file not found, creating a new one");
            // Create a new settings.json file with default settings
            let default_settings = HelperSettings {
                market_id: "".to_string(),
                token_mint: "".to_string(),
                deployer_key: "".to_string(),
                buyer_key: "".to_string(),
                pool_id: "".to_string(),
                lut_key: "".to_string(),
                volume_lut_key: "".to_string(),
            };
            let default_settings_json = serde_json::to_string(&default_settings).unwrap();
            let mut file = File::create("bundler_settings.json").unwrap();
            file.write_all(default_settings_json.as_bytes()).unwrap();

            "".to_string() // Return an empty string if the file does not exist
        }
    };

    let mut helper_settings: HelperSettings = match serde_json::from_str(&args) {
        Ok(settings) => settings,
        Err(_) => {
            // If the file is empty, use default settings
            HelperSettings::default()
        }
    };

    // If any field is empty, ask the user to fill it
    if helper_settings.deployer_key.is_empty() {
        helper_settings.deployer_key = private_key_env("Deployer Private Key").await.unwrap();
    }
    if helper_settings.buyer_key.is_empty() {
        helper_settings.buyer_key = private_key_env("Buyer Private Key").await.unwrap();
    }

    // Save the updated settings to the file
    let default_settings_json = serde_json::to_string_pretty(&helper_settings).unwrap();
    let mut file = File::create("bundler_settings.json").unwrap();
    file.write_all(default_settings_json.as_bytes()).unwrap();

    Ok(PoolDataSettings {
        market_id: helper_settings.market_id,
        token_mint: helper_settings.token_mint,
        deployer_key: helper_settings.deployer_key,
        buyer_key: helper_settings.buyer_key,
        pool_id: helper_settings.pool_id,
        lut_key: helper_settings.lut_key,
        volume_lut_key: helper_settings.volume_lut_key,
    })
}

pub fn anchor_cluster(wallet: Arc<Keypair>) -> Client<Arc<Keypair>> {
    let url = Cluster::Custom(
        String::from(
            "https://mainnet.helius-rpc.com/?api-key=d9e80c44-bc75-4139-8cc7-084cefe506c7",
        ),
        String::from("wss://mainnet.helius-rpc.com/?api-key=d9e80c44-bc75-4139-8cc7-084cefe506c7"),
    );

    let client = Client::new(url, wallet);

    client
}
