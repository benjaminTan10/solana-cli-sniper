use std::{str::FromStr, sync::Arc};

use crate::{
    app::theme,
    env::{env_loader::tip_account, load_settings, minter::load_minter_settings},
    instruction::instruction::load_amm_keys,
    liquidity::{
        lut::{create_lut::create_lut, extend_lut::poolkeys_lut},
        option::sol_distribution::atas_creation,
        pool_ixs::AMM_PROGRAM,
        swap_ixs::load_pool_keys,
        utils::tip_txn,
    },
    raydium::swap::{instructions::TAX_ACCOUNT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
    user_inputs::tokens::token_env,
};
use async_recursion::async_recursion;
use demand::{DemandOption, Select};
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use solana_sdk::{
    instruction::{AccountMeta},
    message::{v0::Message, VersionedMessage},
    native_token::{sol_to_lamports},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

#[async_recursion]
pub async fn volume_menu() -> eyre::Result<()> {
    let theme = theme();
    let ms = Select::new("Volume Bot")
        .description("Select the Volume Bot")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("LookupTable").label("[1] Create LUT"))
        .option(DemandOption::new("Volume").label("[2] Generate Volume"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "LookupTable" => {
            let _ = volume_lut().await;
            println!("-------------------Returning to Main Menu-------------------");
            volume_menu().await?;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub async fn volume_lut() -> eyre::Result<()> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let data = load_minter_settings().await?;
    let engine = load_settings().await?;

    let pool_id = token_env("Pool ID:").await;

    let buyer_key = Arc::new(Keypair::from_base58_string(&data.buyer_key));

    let mut lut_ix = vec![];

    let (ix, lut_key) = create_lut(data.clone()).await?;
    lut_ix.push(ix);

    let amm_keys = load_amm_keys(&connection, &AMM_PROGRAM, &pool_id).await?;
    let market_keys = load_pool_keys(pool_id, amm_keys).await?;

    let mut pool_ex_lut = poolkeys_lut(amm_keys, market_keys, lut_key, data.clone()).await?;

    let (_, mint_ata, sol_ata) = atas_creation(
        vec![buyer_key.pubkey()],
        buyer_key.clone(),
        Pubkey::from_str(&data.token_mint)?,
    )
    .await?;

    pool_ex_lut.accounts.push(AccountMeta::new(mint_ata, false));
    pool_ex_lut.accounts.push(AccountMeta::new(sol_ata, false));

    lut_ix.push(pool_ex_lut);

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let tax = tip_txn(buyer_key.pubkey(), TAX_ACCOUNT, sol_to_lamports(0.1));
    let tip = tip_txn(buyer_key.pubkey(), tip_account(), sol_to_lamports(0.001));

    lut_ix.push(tip);
    lut_ix.push(tax);

    info!("Building transaction...");

    let mut txns = vec![];

    for ix in &lut_ix {
        let versioned_msg = VersionedMessage::V0(Message::try_compile(
            &buyer_key.pubkey(),
            &[ix.clone()],
            &[],
            recent_blockhash,
        )?);

        let transaction = VersionedTransaction::try_new(versioned_msg, &[&buyer_key])?;

        txns.push(transaction);
    }

    let mut search =
        get_searcher_client(&engine.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = search
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    info!("Subscribed to bundle results");

    let bundle = match send_bundle_with_confirmation(
        &txns,
        &connection,
        &mut search,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Distribution Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    Ok(())
}
