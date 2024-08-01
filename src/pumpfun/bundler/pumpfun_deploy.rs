use std::{fs::File, io::Write, sync::Arc};

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
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
    liquidity::{
        option::wallet_gen::list_json_wallets,
        utils::{tip_account, tip_txn},
    },
    pumpfun::instructions::{
        instructions::{generate_pump_multi_buy_ix, GLOBAL_STATE},
        pumpfun_program::{accounts::GlobalAccount, instructions::CreateIxArgs},
    },
    raydium_amm::swap::{instructions::SOLC_MINT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
    user_inputs::amounts::{bundle_priority_tip, sol_amount},
};

use super::{create_metadata::metadata_json, ix_accounts::token_create_ix};

pub async fn one_pumpfun_deploy() -> eyre::Result<()> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let mint = match list_json_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    let mut server_data = load_minter_settings().await?;
    let engine = load_settings().await?;
    let deployer_key = Keypair::from_base58_string(&server_data.deployer_key);
    let buyer_key = Arc::new(Keypair::from_base58_string(&server_data.buyer_key));

    let create_metadata = match metadata_json().await {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Error creating metadata: {}", e);
            return Ok(());
        }
    };

    let args = CreateIxArgs {
        name: create_metadata.name,
        symbol: create_metadata.symbol,
        uri: create_metadata.image,
    };
    let buy_amount = sol_amount("Wallet Buy Amount:").await;
    let bundle_tip = bundle_priority_tip().await;

    println!("Bundle Tip: {}", bundle_tip);

    let mut bundle_txn = vec![];

    //-------------------Pool Creation Instructions--------------------------
    let create_ixs = token_create_ix(mint[0].pubkey(), deployer_key.pubkey(), args);

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &create_ixs,
            &[],
            connection.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let pool_create_tx =
        match VersionedTransaction::try_new(versioned_msg, &[&deployer_key, &mint[0]]) {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("Error creating pool transaction: {}", e);
                return Err(e.into());
            }
        };

    bundle_txn.push(pool_create_tx);

    // -------------------Pool Keys------------------------------------------
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
        &mint[0].pubkey(),
        &spl_token::id(),
    );
    let account_data = connection.get_account_data(&GLOBAL_STATE).await?;

    let sliced_data: &mut &[u8] = &mut account_data.as_slice();

    let reserves = GlobalAccount::deserialize(sliced_data)?.0;

    let reserves_tuple: (u128, u128, u128) = (
        reserves.initial_virtual_sol_reserves as u128,
        reserves.initial_virtual_token_reserves as u128,
        reserves.initial_real_token_reserves as u128,
    );

    let swap_ixs = generate_pump_multi_buy_ix(
        connection.clone(),
        mint[0].pubkey(),
        buy_amount,
        buyer_key.clone(),
        reserves_tuple,
    )
    .await
    .unwrap();

    let mut instructions = vec![create_source, transfer, sync_native, create_destination];

    // Append all instructions from swap_ixs
    instructions.extend(swap_ixs);

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &buyer_key.pubkey(),
            &instructions,
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

    // let tax_txn = tip_txn(deployer_key.pubkey(), TAX_ACCOUNT, sol_to_lamports(0.25));

    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &[jito_txn],
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

    println!(
        "Signatures: {:?}",
        bundle_txn
            .iter()
            .map(|x| x.signatures[0])
            .collect::<Vec<_>>()
    );

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

    server_data.pool_id = mint[0].pubkey().to_string();
    let mut file = File::create("bundler_settings.json").unwrap();
    file.write_all(serde_json::to_string(&server_data)?.as_bytes())?;

    Ok(())
}
