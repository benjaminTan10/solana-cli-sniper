use demand::Input;
use futures::stream::StreamExt;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    message::{v0::Message, VersionedMessage},
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    env::{env_loader::tip_account, load_settings, minter::load_minter_settings},
    instruction::instruction::{get_keys_for_market, load_amm_keys},
    liquidity::{
        option::wallet_gen::list_folders,
        pool_ixs::AMM_PROGRAM,
        swap_ixs::{self, swap_ixs},
        utils::tip_txn,
    },
    raydium::swap::{
        instructions::{SOLC_MINT, TAX_ACCOUNT},
        swapper::auth_keypair,
    },
    rpc::HTTP_CLIENT,
    user_inputs::tokens::token_env,
};

pub async fn sell_specific(percentage: bool) -> eyre::Result<()> {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let mut percentage_tokens = 1.0;
    if percentage {
        let t = Input::new("Token Percentage:")
            .placeholder("eg. 50%...")
            .prompt("Input: ");

        let percentage = t.run().expect("error running input");

        //parse the percentage
        percentage_tokens = percentage.parse::<f64>()?;
    }

    let engine = load_settings().await?;

    let (_, wallets) = match list_folders().await {
        Ok((folder_name, wallets)) => (folder_name, wallets),
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    let wallets = &Arc::new(wallets);

    let data = load_minter_settings().await?;
    let buyer_wallet = Keypair::from_base58_string(&data.buyer_key);
    let lut_key = Pubkey::from_str(&data.lut_key)?;

    let mut pool_id = Pubkey::from_str(&data.pool_id).unwrap();
    if data.pool_id.is_empty() {
        pool_id = token_env("Pool ID: ").await;
    }

    //-------------------Load Pool Keys-------------------
    let amm_keys = match load_amm_keys(&rpc_client.clone(), &AMM_PROGRAM, &pool_id).await {
        Ok(amm_keys) => amm_keys,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    let market_keys = match get_keys_for_market(
        &rpc_client.clone(),
        &amm_keys.market_program,
        &amm_keys.market,
    )
    .await
    {
        Ok(market_keys) => market_keys,
        Err(e) => {
            eprintln!("Market Error: {}", e);
            return Ok(());
        }
    };

    let balance_amounts = Arc::new(Mutex::new(vec![]));

    let mut raw_account = None;

    while raw_account.is_none() {
        match rpc_client.get_account(&lut_key).await {
            Ok(account) => raw_account = Some(account),
            Err(e) => {
                eprintln!("Error getting LUT account: {}, retrying...", e);
            }
        }
    }

    let raw_account = raw_account.unwrap();
    let address_lookup_table = AddressLookupTable::deserialize(&raw_account.data)?;
    let address_lookup_table_account = AddressLookupTableAccount {
        key: lut_key,
        addresses: address_lookup_table.addresses.to_vec(),
    };

    let wallets_chunks = wallets.chunks(7).collect::<Vec<_>>();
    let mut bundle_txn = vec![];

    let recent_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

    for (chunk_index, wallet_chunk) in wallets_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();
        let mut current_wallets = Vec::new();

        for (i, wallet) in wallet_chunk.iter().enumerate() {
            let token_account = get_associated_token_address(
                &wallet.pubkey(),
                &Pubkey::from_str(&data.token_mint).unwrap(),
            );

            let balance = sol_to_lamports(
                lamports_to_sol(
                    rpc_client
                        .get_token_account_balance(&token_account)
                        .await
                        .unwrap()
                        .amount
                        .parse::<u64>()
                        .unwrap(),
                ) * percentage_tokens,
            );

            let swap_ixs = swap_ixs(
                data.clone(),
                amm_keys,
                market_keys.clone(),
                &wallet,
                balance,
                true,
            )
            .unwrap();

            if chunk_index == wallets_chunks.len() - 1 && i == wallet_chunk.len() - 1 {
                let total_amount = sol_to_lamports(
                    lamports_to_sol(balance_amounts.lock().unwrap().iter().sum::<u64>()) * 0.1,
                );

                let mut tip = vec![tip_txn(
                    buyer_wallet.pubkey(),
                    tip_account(),
                    sol_to_lamports(0.001),
                )];
                let source_ata = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);
                let tax_destination = get_associated_token_address(&TAX_ACCOUNT, &SOLC_MINT);

                let tax = spl_token::instruction::transfer(
                    &spl_token::id(),
                    &source_ata,
                    &tax_destination,
                    &buyer_wallet.pubkey(),
                    &[&buyer_wallet.pubkey()],
                    total_amount,
                )?;

                tip.push(tax);
            }
            current_wallets.push(wallet);
            current_instructions.push(swap_ixs);
        }
        let versioned_tx = match VersionedTransaction::try_new(
            VersionedMessage::V0(
                Message::try_compile(
                    &buyer_wallet.pubkey(),
                    &current_instructions,
                    &[address_lookup_table_account.clone()],
                    recent_blockhash,
                )
                .unwrap(),
            ),
            &current_wallets,
        ) {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("Error creating pool transaction: {}", e);
                return Err(e.into());
            }
        };

        bundle_txn.push(versioned_tx);
    }

    let mut client =
        get_searcher_client(&engine.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let bundle_results = match send_bundle_with_confirmation(
        &bundle_txn,
        &rpc_client,
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

    Ok(())
}
