use std::{fs::File, io::Write, sync::Arc};

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::{create_associated_token_account, create_associated_token_account_idempotent},
};
use spl_token::instruction::sync_native;

use crate::{
    env::{load_settings, minter::load_minter_settings},
    raydium::swap::{
        instructions::{SOLC_MINT, TAX_ACCOUNT},
        swapper::auth_keypair,
    },
    rpc::HTTP_CLIENT,
    user_inputs::amounts::{bundle_priority_tip, sol_amount},
};

use super::{
    option::wallet_gen::list_folders,
    pool_27::PoolDeployResponse,
    pool_ixs::pool_ixs,
    swap_ixs::{load_pool_keys, swap_ixs},
    utils::{tip_account, tip_txn},
};

pub async fn single_pool() -> eyre::Result<PoolDeployResponse> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let bundle_tip = bundle_priority_tip().await;

    let mut server_data = load_minter_settings().await?;
    let engine = load_settings().await?;
    let deployer_key = Keypair::from_base58_string(&server_data.deployer_key);
    let buyer_key = Keypair::from_base58_string(&server_data.buyer_key);
    let buy_amount = sol_amount("Wallet Buy Amount:").await;

    let (_, mut wallets) = match list_folders().await {
        Ok(wallets) => wallets,
        Err(e) => {
            panic!("Error: {}", e)
        }
    };

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
            &create_pool_ixs,
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

    let user_token_source = get_associated_token_address(&buyer_key.pubkey(), &SOLC_MINT);

    let create_source = create_associated_token_account_idempotent(
        &buyer_key.pubkey(),
        &buyer_key.pubkey(),
        &SOLC_MINT,
        &spl_token::id(),
    );

    let transfer =
        system_instruction::transfer(&buyer_key.pubkey(), &user_token_source, buy_amount);

    let sync_native = match sync_native(&spl_token::id(), &user_token_source) {
        Ok(sync_native) => sync_native,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };
    let create_destination = create_associated_token_account(
        &buyer_key.pubkey(),
        &buyer_key.pubkey(),
        &amm_keys.amm_coin_mint,
        &spl_token::id(),
    );

    let swap_ixs = swap_ixs(
        server_data.clone(),
        amm_keys,
        market_keys.clone(),
        &buyer_key,
        buy_amount,
        false,
        user_token_source,
    )
    .unwrap();

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &buyer_key.pubkey(),
            &[
                create_source,
                transfer,
                sync_native,
                create_destination,
                swap_ixs,
            ],
            &[],
            connection.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let swap_tx = match VersionedTransaction::try_new(versioned_msg, &[&buyer_key]) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error creating pool transaction: {}", e);
            return Err(e.into());
        }
    };

    bundle_txn.push(swap_tx);

    let jito_txn = tip_txn(deployer_key.pubkey(), tip_account(), bundle_tip);

    let tax_txn = tip_txn(deployer_key.pubkey(), TAX_ACCOUNT, sol_to_lamports(0.25));

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &[tax_txn, jito_txn],
            &[],
            connection.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let tip_tx = match VersionedTransaction::try_new(versioned_msg, &[&deployer_key]) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error creating tip transaction: {}", e);
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

    wallets.push(deployer_key);
    wallets.push(buyer_key);

    let mut associated_accounts = vec![];
    wallets.iter().for_each(|wallet| {
        associated_accounts.push(get_associated_token_address(
            &wallet.pubkey(),
            &amm_keys.amm_coin_mint,
        ));
    });

    // let client_clone = client.clone();
    // tokio::spawn(async move {
    //     info!("Account Freeze Thread Activated!");
    //     let _ = freeze_sells(Arc::new(associated_accounts), client_clone).await;
    // });

    let _ = match send_bundle_with_confirmation(
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

    server_data.pool_id = amm_pool.to_string();
    let mut file = File::create("mintor_settings.json").unwrap();
    file.write_all(serde_json::to_string(&server_data)?.as_bytes())?;

    Ok(PoolDeployResponse {
        wallets: vec![],
        amm_pool,
    })
}
