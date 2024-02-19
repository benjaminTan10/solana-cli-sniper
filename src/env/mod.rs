use std::path::PathBuf;

use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

pub mod env_loader;

#[derive(Debug)]
pub struct Settings {
    /// URL of the block engine.
    /// See: https://jito-labs.gitbook.io/mev/searcher-resources/block-engine#connection-details
    pub block_engine_url: String,

    /// Account pubkeys to backrun
    pub backrun_accounts: Vec<Pubkey>,

    /// Path to keypair file used to sign and pay for transactions
    pub payer_keypair: Keypair,

    /// Path to keypair file used to authenticate with the Jito Block Engine
    /// See: https://jito-labs.gitbook.io/mev/searcher-resources/getting-started#block-engine-api-key
    pub auth_keypair: PathBuf,

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
