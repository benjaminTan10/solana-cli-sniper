use std::env;

use log::info;

pub fn rpc_key() -> String {
    let rpc = read_json()["RPC"].as_str().unwrap().to_string();
    rpc
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
