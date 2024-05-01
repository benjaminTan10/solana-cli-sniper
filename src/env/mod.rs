use std::{
    fs::{self, File},
    io::Write,
    os::unix::process,
    process::exit,
};

use log::info;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

pub mod env_loader;

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

    /// Bot password
    pub bot_auth: String,

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
}

#[derive(Deserialize, Clone)]
struct HelperBackrunAccount {
    id: String,
    account: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct HelperSettings {
    // auth_keypair: Vec<u8>,
    // whitelisted_keypair: String,
    pubsub_url: String,
    rpc_url: String,
    block_engine_url: String,
    message: String,
    grpc_url: String,
    buy_wallet: String,

    #[serde(rename = "bot:auth")]
    bot_auth: String,
    // tip_program_id: String,
    regions: Vec<String>,
    subscribe_bundle_results: bool,
    use_bundles: bool,
    spam: bool,
    spam_count: i32,
}

pub async fn load_settings() -> eyre::Result<EngineSettings> {
    let args = match fs::read_to_string("settings.json") {
        Ok(args) => args,
        Err(_) => {
            info!("Settings file not found, creating a new one");
            info!("Please fill in the settings.json file with your settings and restart the bot.");
            // Create a new settings.json file with default settings
            let default_settings = HelperSettings {
                block_engine_url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
                buy_wallet: "".to_string(),
                grpc_url: "http://205.209.109.10:10000/".to_string(),
                pubsub_url:
                    "wss://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                rpc_url:
                    "https://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                message: "Jito Tip Message".to_string(),
                bot_auth: "".to_string(),
                // tip_program_id: "".to_string(),
                regions: vec!["ny".to_string()],
                subscribe_bundle_results: false,
                // auth_keypair: vec![],
                use_bundles: true,
                spam: false,
                spam_count: 15,
            };
            let default_settings_json = serde_json::to_string(&default_settings).unwrap();
            let mut file = File::create("settings.json").unwrap();
            file.write_all(default_settings_json.as_bytes()).unwrap();

            exit(1);
        }
    };

    let helper_settings: HelperSettings = match serde_json::from_str(&args) {
        Ok(settings) => settings,
        Err(_) => {
            info!("Settings file not found, creating a new one");
            info!("Please fill in the settings.json file with your settings and restart the bot.");
            // Create a new settings.json file with default settings
            let default_settings = HelperSettings {
                block_engine_url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
                buy_wallet: "".to_string(),
                grpc_url: "http://205.209.109.10:10000/".to_string(),
                pubsub_url:
                    "wss://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                rpc_url:
                    "https://mainnet.helius-rpc.com/?api-key=0b99078c-7247-47ad-8cf8-35cbfc021667"
                        .to_string(),
                message: "Jito Tip Message".to_string(),
                bot_auth: "".to_string(),
                // tip_program_id: "".to_string(),
                regions: vec!["ny".to_string()],
                subscribe_bundle_results: false,
                // auth_keypair: vec![],
                use_bundles: true,
                spam: false,
                spam_count: 15,
            };
            let default_settings_json = serde_json::to_string(&default_settings).unwrap();
            let mut file = File::create("settings.json").unwrap();
            file.write_all(default_settings_json.as_bytes()).unwrap();

            exit(1);
        }
    };

    let settings = EngineSettings {
        block_engine_url: helper_settings.block_engine_url,
        payer_keypair: helper_settings.buy_wallet,
        grpc_url: helper_settings.grpc_url,
        pubsub_url: helper_settings.pubsub_url,
        rpc_url: helper_settings.rpc_url,
        message: helper_settings.message,
        bot_auth: helper_settings.bot_auth,
        // tip_program_id,
        regions: helper_settings.regions,
        subscribe_bundle_results: helper_settings.subscribe_bundle_results,
        // auth_keypair: helper_settings.auth_keypair,
        use_bundles: helper_settings.use_bundles,
        spam: helper_settings.spam,
        spam_count: helper_settings.spam_count,
    };

    Ok(settings)
}
