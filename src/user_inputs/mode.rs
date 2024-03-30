use std::{error::Error, sync::Arc};

use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use tokio::sync::mpsc::channel;

use crate::{
    app::MevApe,
    env::load_settings,
    plugins::jito_plugin::event_loop::bundle_results_loop,
    raydium::swap::{
        grpc_new_pairs::grpc_pair_sub, instructions::wrap_sol, swap_in::PriorityTip,
        swapper::auth_keypair,
    },
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

pub async fn automatic_snipe(manual_snipe: bool) -> eyre::Result<()> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };
    let sol_amount = sol_amount().await;

    let mut token = Pubkey::default();

    let mut bundle_tip = 0;
    let mut priority_fee_value = 0;
    let (bundle_results_sender, bundle_results_receiver) = channel(100);

    if args.use_bundles {
        tokio::spawn(bundle_results_loop(
            args.block_engine_url.clone(),
            Arc::new(auth_keypair()),
            bundle_results_sender,
        ));
        priority_fee_value = priority_fee().await;
        bundle_tip = bundle_priority_tip().await;
    } else {
        priority_fee_value = priority_fee().await;
    }

    if manual_snipe {
        token = token_env("Base Mint").await;
    } else {
        token = Pubkey::default();
    }

    // let wallet = private_key_env().await?;

    let fees = PriorityTip {
        priority_fee_value,
        bundle_tip,
    };

    let mev_ape = MevApe {
        sol_amount,
        fee: fees,
        // bundle_tip,
        wallet: args.payer_keypair.clone(),
    };

    let _ = match grpc_pair_sub(mev_ape, args, manual_snipe, token, bundle_results_receiver).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}
