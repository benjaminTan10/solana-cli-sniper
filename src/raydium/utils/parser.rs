use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{
    EncodedTransaction, UiTransactionEncoding, UiTransactionStatusMeta,
};
use std::str::FromStr;

pub async fn parse_signatures(
    rpc_client: &RpcClient,
    confirmed_sigs: &String,
) -> Option<(UiTransactionStatusMeta, EncodedTransaction)> {
    let encoding_1 = UiTransactionEncoding::JsonParsed;

    let config = RpcTransactionConfig {
        encoding: Some(encoding_1),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let mut attempts = 0;
    loop {
        match rpc_client
            .get_transaction_with_config(
                &solana_sdk::signature::Signature::from_str(&confirmed_sigs).unwrap(),
                config,
            )
            .await
        {
            Ok(signs) => {
                if let Some(transaction_meta) = signs.transaction.meta {
                    let transaction = signs.transaction.transaction;
                    return Some((transaction_meta, transaction));
                } else {
                    println!("Transaction is null");
                    return None;
                }
            }
            Err(err) => {
                attempts += 1;
                if attempts >= 15 {
                    println!(
                        "Error getting transaction after {} attempts: {:?} - Signature {}",
                        attempts, err, confirmed_sigs
                    );
                    return None;
                }
            }
        }
    }
}
