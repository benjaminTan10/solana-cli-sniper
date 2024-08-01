use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    process::exit,
};

use demand::Input;
use log::info;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use crate::app::private_key_env;

pub mod input;
pub mod minter;
pub mod utils;
pub mod vanity;

#[derive(Debug, Clone)]
pub struct BackrunAccount {
    pub id: String,
    pub account: Pubkey,
}

#[derive(Debug, Clone)]
pub struct EngineSettings {
    /// URL of the block engine.
    /// See: https://jito-labs.gitbook.io/mev/searcher-resources/block-engine#connection-details
    pub block_engine_url: String,

    /// Account pubkeys to backrun
    // pub backrun_accounts: Vec<Pubkey>,

    /// Path to keypair file used to sign and pay for transactions
    pub payer_keypair: String,

    /// Path to keypair file used to authenticate with the Jito Block Engine
    /// See: https://jito-labs.gitbook.io/mev/searcher-resources/getting-started#block-engine-api-key
    // pub auth_keypair: Vec<u8>,

    /// RPC Websocket URL.
    /// See: https://solana.com/docs/rpc/websocket
    /// Note that this RPC server must have --rpc-pubsub-enable-block-subscription enabled
    pub pubsub_url: String,

    /// RPC HTTP URL.
    pub rpc_url: String,

    /// GRPC URL.
    pub grpc_url: String,

    /// Message to pass into the memo program as part of a bundle.
    pub message: String,

    /// License
    pub license_key: String,

    /// Discord Username
    pub username: String,

    /// Tip payment program public key
    /// See: https://jito-foundation.gitbook.io/mev/mev-payment-and-distribution/on-chain-addresses
    // pub tip_program_id: Pubkey,

    /// Comma-separated list of regions to request cross-region data from.
    /// If no region specified, then default to the currently connected block engine's region.
    /// Details: https://jito-labs.gitbook.io/mev/searcher-services/recommendations#cross-region
    /// Available regions: https://jito-labs.gitbook.io/mev/searcher-resources/block-engine#connection-details
    pub regions: Vec<String>,

    /// Subscribe and print bundle results.
    pub subscribe_bundle_results: bool,

    /// Use Bundles for Buy & Sell
    pub use_bundles: bool,

    pub spam: bool,
    pub spam_count: i32,

    //pub txn_level: i32,
    pub slippage: bool,
}

#[derive(Deserialize, Clone)]
struct HelperBackrunAccount {
    id: String,
    account: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct HelperSettings {
    #[serde(rename = "USERNAME")]
    username: String,

    #[serde(rename = "LICENSE-KEY")]
    license_key: String,

    #[serde(rename = "SIGNER-PRIVATE-KEY")]
    buy_wallet: String,

    #[serde(rename = "WEBSOCKET-URL")]
    pubsub_url: String,

    #[serde(rename = "RPC-URL")]
    rpc_url: String,

    #[serde(rename = "YELLOWSTONE-GRPC-URL")]
    grpc_url: String,

    #[serde(rename = "BLOCK-ENGINE-URL")]
    block_engine_url: String,

    #[serde(rename = "USE-BUNDLES")]
    use_bundles: bool,

    #[serde(rename = "SPAM")]
    spam: bool,

    #[serde(rename = "SPAM-COUNT")]
    spam_count: i32,

    // #[serde(rename = "Transaction-Level")]
    // txn_level: i32,
    #[serde(rename = "SLIPPAGE")]
    slippage: bool,
}

pub async fn load_settings() -> eyre::Result<EngineSettings> {
    let args = fs::read_to_string("settings.json").unwrap_or_else(|_| {
        info!("Settings file not found, creating a new one");
        // Create a new settings.json file with default settings
        let default_settings = HelperSettings {
            username: "".to_string(),
            license_key: "".to_string(),
            block_engine_url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
            buy_wallet: "".to_string(),
            grpc_url: "".to_string(),
            pubsub_url:
                "wss://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                    .to_string(),
            rpc_url: "https://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                .to_string(),
            use_bundles: true,
            spam: false,
            spam_count: 15,
            // txn_level: 1,
            slippage: false,
        };
        let default_settings_json = serde_json::to_string(&default_settings).unwrap();
        let mut file = File::create("settings.json").unwrap();
        file.write_all(default_settings_json.as_bytes()).unwrap();

        return default_settings_json;
    });

    let mut helper_settings: HelperSettings = serde_json::from_str(&args).unwrap_or_else(|_| {
        info!("Settings file not found, creating a new one");
        // Create a new settings.json file with default settings
        let default_settings = HelperSettings {
            username: "".to_string(),
            license_key: "".to_string(),
            block_engine_url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
            buy_wallet: "".to_string(),
            grpc_url: "".to_string(),
            pubsub_url:
                "wss://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                    .to_string(),
            rpc_url: "https://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                .to_string(),
            use_bundles: true,
            spam: false,
            spam_count: 15,
            // txn_level: 1,
            slippage: false,
        };
        let default_settings_json = serde_json::to_string(&default_settings).unwrap();
        let mut file = File::create("settings.json").unwrap();
        file.write_all(default_settings_json.as_bytes()).unwrap();

        return default_settings;
    });

    if helper_settings.username.is_empty() {
        helper_settings.username = register_sims("Enter Discord Username: ", "popuy...")
            .await
            .unwrap();
    }

    if helper_settings.license_key.is_empty() {
        helper_settings.license_key = register_sims("Enter License Key: ", "MEVA........ImCh")
            .await
            .unwrap();
    }

    if helper_settings.buy_wallet.is_empty() {
        helper_settings.buy_wallet = private_key_env("Enter Wallet Private-Key: ").await.unwrap();
    }

    let mut file = File::create("settings.json").unwrap();
    file.write(serde_json::to_string(&helper_settings).unwrap().as_bytes())
        .unwrap();

    let settings = EngineSettings {
        block_engine_url: helper_settings.block_engine_url,
        payer_keypair: helper_settings.buy_wallet,
        grpc_url: helper_settings.grpc_url,
        pubsub_url: helper_settings.pubsub_url,
        rpc_url: helper_settings.rpc_url,
        message: "hello".to_string(),
        username: helper_settings.username,
        license_key: helper_settings.license_key,
        regions: ["ny".to_string()].into(),
        subscribe_bundle_results: false,
        use_bundles: helper_settings.use_bundles,
        spam: helper_settings.spam,
        spam_count: helper_settings.spam_count,
        //   txn_level: helper_settings.txn_level,
        slippage: false,
    };

    Ok(settings)
}

pub async fn register_sims(key: &str, place_holder: &str) -> Result<String, Box<dyn Error>> {
    let t = Input::new(key).placeholder(place_holder).prompt("Input: ");

    let string = t.run().expect("error running input");

    // Check if the private key is valid

    Ok(string)
}
