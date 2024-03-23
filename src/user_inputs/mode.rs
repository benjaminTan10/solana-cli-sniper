use std::{error::Error, sync::Arc};

use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use tokio::sync::mpsc::channel;

use crate::{
    app::MevApe,
    env::load_settings,
    plugins::jito_plugin::event_loop::bundle_results_loop,
    raydium::swap::{grpc_new_pairs::grpc_pair_sub, instructions::wrap_sol, swapper::auth_keypair},
};

use super::{
    amounts::{bundle_priority_tip, priority_fee, sol_amount},
    tokens::token_env,
};

pub async fn wrap_sol_call() -> Result<(), Box<dyn Error>> {
    let sol_amount = sol_amount().await;
    // let wallet = private_key_env().await?;

    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };
    let private_key = Keypair::from_bytes(&bs58::decode(args.payer_keypair).into_vec().unwrap())?;
    let rpc_client = RpcClient::new(args.rpc_url.to_string());

    let _ = match wrap_sol(Arc::new(rpc_client), &private_key, sol_amount).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}

pub async fn automatic_snipe(snipe_automatic: bool) -> eyre::Result<()> {
    let sol_amount = sol_amount().await;
    let priority_fee = priority_fee().await;
    let bundle_tip = bundle_priority_tip().await;
    let mut token = Pubkey::default();

    if !snipe_automatic {
        token = token_env("Base Mint").await;
    } else {
        token = Pubkey::default();
    }

    // let wallet = private_key_env().await?;

    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };
    // let private_key = Keypair::from_bytes(&bs58::decode(args.payer_keypair).into_vec().unwrap())?;
    // let rpc_client = RpcClient::new(args.rpc_url.to_string());

    let mev_ape = MevApe {
        sol_amount,
        priority_fee,
        // bundle_tip,
        wallet: args.payer_keypair.clone(),
    };

    let (bundle_results_sender, bundle_results_receiver) = channel(100);

    tokio::spawn(bundle_results_loop(
        args.block_engine_url.clone(),
        Arc::new(auth_keypair()),
        bundle_results_sender,
    ));

    let _ = match grpc_pair_sub(mev_ape, args, bundle_results_receiver).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}
