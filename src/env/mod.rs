use std::env;

pub fn rpc_key() -> String {
    env::var("RPC_ENDPOINT").unwrap()
}
