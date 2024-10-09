use std::sync::Arc;

use log::error;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    env::load_config,
    input::{amount_input, mint_input, percentage_input},
};

use super::{executor::pump_swap, pump_interface::builder::PumpFunDirection};

pub async fn pump_swap_in() -> eyre::Result<()> {
    let settings = match load_config().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let amount = amount_input("Input Sol: ").await;

    let wallet = Keypair::from_base58_string(&settings.engine.payer_keypair);
    let token = mint_input("Base Mint: ").await;
    match pump_swap(
        &Arc::new(wallet),
        settings,
        PumpFunDirection::Buy,
        token,
        amount,
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    Ok(())
}

pub async fn pump_swap_out() -> eyre::Result<()> {
    let settings = match load_config().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let rpc_client = RpcClient::new(settings.clone().network.rpc_url);
    let wallet = Keypair::from_base58_string(&settings.engine.payer_keypair);
    let token_address = mint_input("Base Mint: ").await;

    let tokens = percentage_input().await as u64;
    let tokens_amount = rpc_client
        .get_token_account_balance(&get_associated_token_address(
            &wallet.pubkey(),
            &token_address,
        ))
        .await?;

    let token_amount = match tokens_amount.amount.parse::<u64>() {
        Ok(a) => a,
        Err(e) => {
            error!("Error: {}", e);
            return Ok(());
        }
    };

    let amount = token_amount * tokens / 100;

    let wallet = Keypair::from_base58_string(&settings.engine.payer_keypair);
    match pump_swap(
        &Arc::new(wallet),
        settings,
        PumpFunDirection::Sell,
        token_address,
        amount,
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    Ok(())
}
