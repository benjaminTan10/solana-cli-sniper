use mpl_token_metadata::accounts::Metadata;

use solana_client::{client_error::ClientErrorKind, rpc_client::RpcClient};
use solana_program::pubkey::Pubkey;
use thiserror::Error;

use crate::rpc::HTTP_CLIENT;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("no account data found")]
    MissingAccount(String),

    #[error("failed to get account data")]
    ClientError(String),

    #[error("failed to parse string into Pubkey")]
    PubkeyParseFailed(String),

    #[error("failed to decode metadata")]
    DecodeMetadataFailed(String),

    #[error("failed to decode account data")]
    DecodeDataFailed(String),

    #[error("failed to deserialize account data: {0}")]
    DeserializationFailed(String),

    #[error("General error: {0}")]
    GeneralError(String),

    #[error("RuleSet revision not available")]
    RuleSetRevisionNotAvailable,

    #[error("Numerical overflow")]
    NumericalOverflow,
}

pub async fn decode_metadata(pubkey: &Pubkey) -> Result<Metadata, DecodeError> {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let account_data = rpc_client
        .get_account_data(pubkey)
        .await
        .map_err(|e| DecodeError::ClientError(e.to_string()))?;

    Metadata::safe_deserialize(&mut account_data.as_ref())
        .map_err(|e| DecodeError::DecodeMetadataFailed(e.to_string()))
}
