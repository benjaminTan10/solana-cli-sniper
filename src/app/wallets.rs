use std::fs;

use log::{error, info, warn};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{bs58, native_token::lamports_to_sol, signature::Keypair, signer::Signer};

pub fn private_keys() -> io::Result<Vec<(String, Keypair)>> {
    let private_keys = private_keys_loader()?;

    Ok(private_keys
        .iter()
        .map(|(name, value)| {
            let key = value.as_str();

            let secret_key = match bs58::decode(key).into_vec() {
                Ok(v) => v,
                Err(e) => {
                    panic!("Error: {}", e);
                }
            };
            let secret_key = secret_key.as_slice();
            let secret_key = match Keypair::from_bytes(secret_key) {
                Ok(v) => v,
                Err(e) => {
                    panic!("Error: {}", e);
                }
            };

            (name.clone(), secret_key)
        })
        .collect())
}

use serde_json::Value;
use std::io;

use crate::env::load_settings;

fn private_keys_loader() -> io::Result<Vec<(String, String)>> {
    let mut keys = Vec::new();

    let entries = fs::read_dir("./wallets")?;
    for entry in entries {
        let entry = entry?.path();

        if !entry.is_file() || entry.extension() != Some("json".as_ref()) {
            continue;
        }

        let data = fs::read_to_string(&entry)?;
        let json: Value = serde_json::from_str(&data)?;

        if let Some(key) = json["wallet"].as_str() {
            if let Some(name) = entry
                .file_name()
                .and_then(|n| n.to_str().map(str::to_string))
            {
                keys.push((name, key.to_owned()));
            } else {
                warn!("Unable to get file name");
            }
        } else {
            warn!("Invalid wallet key");
        }
    }

    Ok(keys)
}
pub async fn wallet_logger() -> io::Result<()> {
    info!("Loading details...");
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };

    let rpc_client = RpcClient::new(args.rpc_url.to_string());
    let secret_key = bs58::decode(args.payer_keypair.clone()).into_vec().unwrap();
    let wallet = match Keypair::from_bytes(&secret_key) {
        Ok(wallet) => wallet,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };

    let balance = rpc_client.get_balance(&wallet.pubkey()).await;
    match balance {
        Ok(balance) => {
            info!("Wallet: {}", wallet.pubkey());
            info!("Balance: {:.5} Sol", lamports_to_sol(balance));
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }

    Ok(())
}

/* ----------------------Task Selected Wallet---------------------- */

pub fn private_key(wallet: String) -> Keypair {
    let key = match custom_private_key(wallet) {
        Some(key) => key,
        None => {
            panic!("Wallet not found");
        }
    };

    let decoded = match bs58::decode(key).into_vec() {
        Ok(v) => v,
        Err(e) => panic!("Decode error: {}", e),
    };

    let parsed = match Keypair::from_bytes(&decoded[..]) {
        Ok(v) => v,
        Err(e) => panic!("Parse error: {}", e),
    };

    parsed
}

//read the private key from the json file
fn custom_private_key(wallet: String) -> Option<String> {
    let path = format!("./wallets/{}.json", wallet);

    let data = match fs::read_to_string(&path) {
        Ok(data) => data,
        Err(e) => {
            warn!("Unable to read file {}: {}", path, e);
            return None;
        }
    };

    let json: Result<Value, _> = serde_json::from_str(&data);
    if let Ok(json) = json {
        if let Some(key) = json["wallet"].as_str() {
            return Some(key.to_owned());
        } else {
            warn!("Invalid wallet key in {}", path);
        }
    } else {
        warn!("Unable to parse JSON file {}", path);
    }

    None
}
