use colored::Colorize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::native_token::sol_to_lamports;
use std::env;

use solana_sdk::bs58;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;

pub fn private_key() -> Keypair {
    let key = &private_key_env().unwrap();
    let secret_key = bs58::decode(key).into_vec().unwrap();
    let secret_key = secret_key.as_slice();
    let secret_key = Keypair::from_bytes(secret_key).unwrap();
    return secret_key;
}

// pub fn connection() -> RpcClient {
//     let rpc_client = RpcClient::new(rpc_key());
//     return rpc_client;
// }

// pub fn rpc_key() -> String {
//     let rpc_url = env::var("RPC_ENDPOINT").unwrap_or("http://64.176.215.55:8899/".to_string());
//     return rpc_url;
// }

pub fn private_key_env() -> Result<String, env::VarError> {
    env::var("PRIVATE_KEY_WALLET")
}
pub fn mode() -> Result<String, env::VarError> {
    match env::var("MODE") {
        Ok(e) => Ok(e),
        Err(e) => Err(e),
    }
}

pub fn amount_in() -> u64 {
    let amount = env::var("AMOUNT_IN_SOL").unwrap_or("0.1".into());
    let amount = amount.parse::<f64>().unwrap();
    let lamports = sol_to_lamports(amount);
    return lamports;
}

pub fn priority_fee() -> u64 {
    let fee_input = env::var("PRIORITY_FEE").unwrap_or("0.0001".into());
    let fee_in = fee_input.parse::<f64>().unwrap();
    let fee = sol_to_lamports(fee_in);
    return fee;
}

pub fn x_token() -> String {
    let x_token = env::var("X_TOKEN").unwrap_or("00000000-0000-0000-0000-000000000000".to_string());
    return x_token;
}

pub fn endpoint() -> String {
    let endpoint = env::var("GRPC_ENDPOINT").unwrap_or("http://64.176.215.55:10000".to_string());
    return endpoint;
}
