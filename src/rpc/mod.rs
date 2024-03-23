use log::info;
use once_cell::sync::Lazy;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

pub static HTTP_CLIENT: Lazy<Mutex<HashMap<&str, Arc<RpcClient>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn rpc_key(rpc_client: String) -> Arc<RpcClient> {
    let rpc_client_instance =
        RpcClient::new_with_commitment(rpc_client.clone(), CommitmentConfig::confirmed());
    let rpc_client_instance = Arc::new(rpc_client_instance);
    let mut http_client = HTTP_CLIENT.lock().unwrap();
    http_client.insert("http_client", Arc::clone(&rpc_client_instance));
    rpc_client_instance
}

pub fn wss_key() -> String {
    let wss = read_json()["WSS"].as_str().unwrap().to_string();
    wss
}

//function to read json from path
pub fn read_json() -> serde_json::Value {
    let data = std::fs::read_to_string("settings.json").unwrap();
    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    v
}
