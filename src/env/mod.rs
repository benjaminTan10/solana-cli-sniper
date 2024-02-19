use std::{fs, path::PathBuf, str::FromStr};

use serde::Deserialize;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

pub mod env_loader;

#[derive(Debug, Clone)]
pub struct EngineSettings {
    /// URL of the block engine.
    /// See: https://jito-labs.gitbook.io/mev/searcher-resources/block-engine#connection-details
    pub block_engine_url: String,

    /// Account pubkeys to backrun
    pub backrun_accounts: Vec<Pubkey>,

    /// Path to keypair file used to sign and pay for transactions
    // pub payer_keypair: Keypair,

    /// Path to keypair file used to authenticate with the Jito Block Engine
    /// See: https://jito-labs.gitbook.io/mev/searcher-resources/getting-started#block-engine-api-key
    // pub auth_keypair: String,

    /// RPC Websocket URL.
    /// See: https://solana.com/docs/rpc/websocket
    /// Note that this RPC server must have --rpc-pubsub-enable-block-subscription enabled
    pub pubsub_url: String,

    /// RPC HTTP URL.
    pub rpc_url: String,

    /// Message to pass into the memo program as part of a bundle.
    pub message: String,

    /// Tip payment program public key
    /// See: https://jito-foundation.gitbook.io/mev/mev-payment-and-distribution/on-chain-addresses
    pub tip_program_id: Pubkey,

    /// Comma-separated list of regions to request cross-region data from.
    /// If no region specified, then default to the currently connected block engine's region.
    /// Details: https://jito-labs.gitbook.io/mev/searcher-services/recommendations#cross-region
    /// Available regions: https://jito-labs.gitbook.io/mev/searcher-resources/block-engine#connection-details
    pub regions: Vec<String>,

    /// Subscribe and print bundle results.
    pub subscribe_bundle_results: bool,
}

#[derive(Deserialize)]
struct HelperSettings {
    auth_keypair: String,
    // whitelisted_keypair: String,
    pubsub_url: String,
    rpc_url: String,
    block_engine_url: String,
    backrun_accounts: Vec<String>,
    message: String,
    tip_program_id: String,
    regions: Vec<String>,
    subscribe_bundle_results: bool,
}

pub async fn load_settings() -> eyre::Result<EngineSettings> {
    let args = match fs::read_to_string("settings.json") {
        Ok(args) => args,
        Err(e) => {
            println!("Error reading settings.json: {}", e);
            std::process::exit(1);
        }
    };

    let helper_settings: HelperSettings = match serde_json::from_str(&args) {
        Ok(settings) => settings,
        Err(e) => {
            println!("Error parsing settings.json: {}", e);
            std::process::exit(1);
        }
    };

    let tip_program_id = match Pubkey::from_str(&helper_settings.tip_program_id) {
        Ok(pubkey) => pubkey,
        Err(e) => {
            println!("Error parsing tip_program_id: {}", e);
            std::process::exit(1);
        }
    };

    let settings = EngineSettings {
        block_engine_url: helper_settings.block_engine_url,
        backrun_accounts: helper_settings
            .backrun_accounts
            .iter()
            .map(|account| match Pubkey::from_str(account) {
                Ok(pubkey) => pubkey,
                Err(e) => {
                    println!("Error parsing backrun_accounts: {}", e);
                    std::process::exit(1);
                }
            })
            .collect(),
        // payer_keypair,
        // auth_keypair,
        pubsub_url: helper_settings.pubsub_url,
        rpc_url: helper_settings.rpc_url,
        message: helper_settings.message,
        tip_program_id,
        regions: helper_settings.regions,
        subscribe_bundle_results: helper_settings.subscribe_bundle_results,
    };

    Ok(settings)
}
