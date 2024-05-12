use std::sync::Arc;

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

use crate::{
    env::{load_settings, minter::load_minter_settings},
    raydium::swap::{instructions::TAX_ACCOUNT, swapper::auth_keypair},
    rpc::{rpc_key, HTTP_CLIENT},
    user_inputs::amounts::{bundle_priority_tip, sol_amount},
};

use super::{
    pool_27::PoolDeployResponse,
    pool_ixs::pool_ixs,
    swap_ixs::{load_pool_keys, swap_ixs},
    utils::{tip_account, tip_txn, JitoPoolData},
};

pub async fn single_pool() -> eyre::Result<PoolDeployResponse> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let bundle_tip = bundle_priority_tip().await;

    let server_data = load_minter_settings().await?;
    let engine = load_settings().await?;
    let deployer_key = Keypair::from_base58_string(&server_data.deployer_key);
    let buyer_key = Keypair::from_base58_string(&server_data.buyer_key);
    let buy_amount = sol_amount().await;

    let mut bundle_txn = vec![];

    // -------------------Pool Creation Instructions--------------------------
    let (create_pool_ixs, amm_pool, amm_keys) = match pool_ixs(server_data.clone()).await {
        Ok(ixs) => ixs,
        Err(e) => {
            eprintln!("Error creating pool IXs: {}", e);
            return Err(e);
        }
    };

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &[create_pool_ixs],
            &[],
            connection.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let pool_create_tx = match VersionedTransaction::try_new(versioned_msg, &[&deployer_key]) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error creating pool transaction: {}", e);
            return Err(e.into());
        }
    };

    bundle_txn.push(pool_create_tx);

    // -------------------Pool Keys------------------------------------------
    let market_keys = load_pool_keys(amm_pool, amm_keys).await?;

    let swap_ixs = swap_ixs(
        server_data.clone(),
        amm_keys,
        market_keys.clone(),
        &buyer_key,
        buy_amount,
        false,
    )
    .unwrap();

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &[swap_ixs],
            &[],
            connection.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let swap_tx = match VersionedTransaction::try_new(versioned_msg, &[&deployer_key]) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error creating pool transaction: {}", e);
            return Err(e.into());
        }
    };

    bundle_txn.push(swap_tx);

    let jito_txn = tip_txn(buyer_key.pubkey(), tip_account(), bundle_tip);

    let tax_txn = tip_txn(deployer_key.pubkey(), TAX_ACCOUNT, sol_to_lamports(0.3));

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &[jito_txn, tax_txn],
            &[],
            connection.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let tip_tx = match VersionedTransaction::try_new(versioned_msg, &[&deployer_key]) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error creating pool transaction: {}", e);
            return Err(e.into());
        }
    };

    bundle_txn.push(tip_tx);

    let mut client =
        get_searcher_client(&engine.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let bundle_results = match send_bundle_with_confirmation(
        &bundle_txn,
        &connection,
        &mut client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(bundle_results) => bundle_results,
        Err(e) => {
            eprintln!("Error sending bundle: {}", e);
        }
    };

    Ok(PoolDeployResponse {
        wallets: vec![],
        amm_pool,
    })
}
