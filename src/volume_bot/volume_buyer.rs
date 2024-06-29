use crate::liquidity::utils::tip_account;
use crate::volume_bot::tip_txn;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::VersionedTransaction,
};
use spl_token::instruction::sync_native;
use std::{str::FromStr, sync::Arc};

use crate::{
    env::{load_settings, minter::load_minter_settings},
    instruction::instruction::{get_keys_for_market, load_amm_keys},
    liquidity::{option::wallet_gen::list_folders, pool_ixs::AMM_PROGRAM, swap_ixs::swap_ixs},
    raydium_amm::swap::{instructions::SOLC_MINT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
};

pub async fn volume_wallets_buyer(out: bool) -> eyre::Result<()> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let data = load_minter_settings().await?;
    let engine = load_settings().await?;

    let pool_id = match Pubkey::from_str(&data.pool_id) {
        Ok(pool_id) => pool_id,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    // let pool_keys = pool_keys_fetcher(pool_id).await?;
    let amm_keys = load_amm_keys(&connection, &AMM_PROGRAM, &pool_id).await?;
    let market_keys =
        get_keys_for_market(&connection, &amm_keys.market_program, &amm_keys.market).await?;

    let buyer_key = Arc::new(Keypair::from_base58_string(&data.buyer_key));

    let (_, wallets) = match list_folders().await {
        Ok(wallets) => wallets,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let mut client =
        get_searcher_client(&engine.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let token_mint = Pubkey::from_str(&data.token_mint)?;

    for wallet in wallets {
        let mut current_instructions = vec![];

        current_instructions.push(
            spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                &wallet.pubkey(),
                &wallet.pubkey(),
                &SOLC_MINT,
                &spl_token::id(),
            ),
        );
        current_instructions.push(
            spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                &wallet.pubkey(),
                &wallet.pubkey(),
                &token_mint,
                &spl_token::id(),
            ),
        );

        let user_token_source = spl_associated_token_account::get_associated_token_address(
            &wallet.pubkey(),
            &SOLC_MINT,
        );

        let balance = connection.get_balance(&wallet.pubkey()).await?;

        if balance < sol_to_lamports(0.01) {
            continue;
        }

        println!("Balance: {} SOL", lamports_to_sol(balance));

        current_instructions.push(system_instruction::transfer(
            &wallet.pubkey(),
            &user_token_source,
            balance - sol_to_lamports(0.006),
        ));

        let sync_native = match sync_native(&spl_token::id(), &user_token_source) {
            Ok(sync_native) => sync_native,
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };

        current_instructions.push(sync_native);

        let swap = match swap_ixs(
            data.clone(),
            amm_keys,
            market_keys.clone(),
            &wallet,
            balance - sol_to_lamports(0.006),
            out,
            user_token_source,
        ) {
            Ok(swap) => swap,
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };

        current_instructions.push(swap);

        let versioned_msg = VersionedMessage::V0(
            match Message::try_compile(
                &wallet.pubkey(),
                &current_instructions,
                &[],
                connection.get_latest_blockhash().await?,
            ) {
                Ok(versioned_msg) => versioned_msg,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    continue;
                }
            },
        );

        let transaction = match VersionedTransaction::try_new(versioned_msg, &[&wallet]) {
            Ok(transaction) => transaction,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        let tip = tip_txn(buyer_key.pubkey(), tip_account(), sol_to_lamports(0.0001));

        let versioned_msg = VersionedMessage::V0(
            match Message::try_compile(
                &buyer_key.pubkey(),
                &[tip],
                &[],
                connection.get_latest_blockhash().await?,
            ) {
                Ok(versioned_msg) => versioned_msg,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    continue;
                }
            },
        );

        let tip_txn = match VersionedTransaction::try_new(versioned_msg, &[&buyer_key]) {
            Ok(transaction) => transaction,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        let _ = match send_bundle_with_confirmation(
            &[transaction, tip_txn],
            &connection,
            &mut client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Volume Swap Error: {}", e);
                continue;
            }
        };
    }

    Ok(())
}
