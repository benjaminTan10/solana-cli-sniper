use std::{
    fs::{self, File},
    io::Write,
};

use log::info;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use crate::{app::private_key_env, user_inputs::tokens::token_env};

#[derive(Debug, Clone)]
pub struct BackrunAccount {
    pub id: String,
    pub account: Pubkey,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolDataSettings {
    pub market_id: String,
    pub token_mint: String,
    pub deployer_key: String,
    pub buyer_key: String,
    pub pool_id: String,
    pub lut_key: String,
}

#[derive(Deserialize, Serialize, Clone, Default)]
struct HelperSettings {
    market_id: String,
    token_mint: String,
    deployer_key: String,
    buyer_key: String,
    pool_id: String,
    lut_key: String,
}

pub async fn load_minter_settings() -> eyre::Result<PoolDataSettings> {
    let args = match fs::read_to_string("mintor_settings.json") {
        Ok(args) => args,
        Err(_) => {
            info!("Settings file not found, creating a new one");
            // Create a new settings.json file with default settings
            let default_settings = PoolDataSettings {
                market_id: "".to_string(),
                token_mint: "".to_string(),
                deployer_key: "".to_string(),
                buyer_key: "".to_string(),
                pool_id: "".to_string(),
                lut_key: "".to_string(),
            };
            let default_settings_json = serde_json::to_string(&default_settings).unwrap();
            let mut file = File::create("mintor_settings.json").unwrap();
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
    if helper_settings.buyer_key.is_empty() {
        helper_settings.buyer_key = private_key_env("Deployer Private Key").await.unwrap();
    }
    if helper_settings.deployer_key.is_empty() {
        helper_settings.deployer_key = private_key_env("Buyer Private Key").await.unwrap();
    }
    if helper_settings.market_id.is_empty() {
        helper_settings.market_id = (token_env("Market ID").await).to_string();
    }
    if helper_settings.token_mint.is_empty() {
        helper_settings.token_mint = (token_env("Token Mint").await).to_string();
    }

    // Save the updated settings to the file
    let default_settings_json = serde_json::to_string_pretty(&helper_settings).unwrap();
    let mut file = File::create("mintor_settings.json").unwrap();
    file.write_all(default_settings_json.as_bytes()).unwrap();

    Ok(PoolDataSettings {
        market_id: helper_settings.market_id,
        token_mint: helper_settings.token_mint,
        deployer_key: helper_settings.deployer_key,
        buyer_key: helper_settings.buyer_key,
        pool_id: "".to_string(),
        lut_key: "".to_string(),
    })
}
