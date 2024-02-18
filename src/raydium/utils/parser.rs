use log::debug;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{
    EncodedTransaction, UiTransactionEncoding, UiTransactionStatusMeta,
};
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use crate::rpc::rpc_key;

pub async fn parse_signatures(
    confirmed_sigs: &String,
) -> Option<(UiTransactionStatusMeta, EncodedTransaction)> {
    let rpc_client = RpcClient::new(rpc_key());
    let encoding_1 = UiTransactionEncoding::JsonParsed;

    let config = RpcTransactionConfig {
        encoding: Some(encoding_1),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let start_time = Instant::now();
    loop {
        match rpc_client.get_transaction_with_config(
            &solana_sdk::signature::Signature::from_str(&confirmed_sigs).unwrap(),
            // encoding_1,
            config,
        ) {
            Ok(signs) => {
                if let Some(transaction_meta) = signs.transaction.meta {
                    let transaction = signs.transaction.transaction;
                    debug!("Transaction: {:?}", transaction_meta);
                    return Some((transaction_meta, transaction));
                } else {
                    println!("Transaction is null");
                    return None;
                }
            }
            Err(err) => {
                if start_time.elapsed() >= Duration::from_secs(60) {
                    println!(
                        "Error getting transaction after 60 seconds: {:?} - Signature {}",
                        err, confirmed_sigs
                    );
                    return None;
                }
            }
        }
    }
}
