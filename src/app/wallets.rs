use std::fs;

use log::info;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{bs58, signature::Keypair, signer::Signer};

pub fn private_key(wallet: String) -> Keypair {
    let key = &private_key_env(wallet);
    let secret_key = bs58::decode(key).into_vec().unwrap();
    let secret_key = secret_key.as_slice();
    let secret_key = Keypair::from_bytes(secret_key).unwrap();
    return secret_key;
}

//read the private key from the json file
fn private_key_env(wallet: String) -> String {
    let private_key = fs::read_to_string(format!("./wallets/{}.json", wallet))
        .expect("Unable to read file, Check the file name and try again.");

    private_key.to_string()
}

pub async fn wallet_logger(wallet: String) {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let key = &private_key(wallet);
    let pubkey = key.pubkey();
    let balance = rpc_client.get_balance(&pubkey).await.unwrap();
    info!("Wallet: {} Balance: {}", pubkey, balance);
}
