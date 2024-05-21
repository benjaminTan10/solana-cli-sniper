use demand::Input;
use futures::stream::StreamExt;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
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
    system_instruction,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};

use crate::{
    env::{env_loader::tip_account, load_settings, minter::load_minter_settings},
    instruction::instruction::{get_keys_for_market, load_amm_keys},
    liquidity::{
        option::wallet_gen::list_folders,
        pool_ixs::AMM_PROGRAM,
        swap_ixs::{self, swap_ixs},
        utils::tip_txn,
    },
    raydium::{
        pool_searcher::amm_keys::pool_keys_fetcher,
        swap::{
            instructions::{
                swap_amount_out, token_price_data, SwapDirection, SOLC_MINT, TAX_ACCOUNT,
            },
            swapper::auth_keypair,
        },
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
    let address_lookup_table = match AddressLookupTable::deserialize(&raw_account.data) {
        Ok(address_lookup_table) => address_lookup_table,
        Err(e) => {
            eprintln!("Error deserializing LUT account: {}", e);
            return Err(e.into());
        }
    };
    let address_lookup_table_account = AddressLookupTableAccount {
        key: lut_key,
        addresses: address_lookup_table.addresses.to_vec(),
    };

    let wallets_chunks = wallets.chunks(6).collect::<Vec<_>>();
    let mut bundle_txn = vec![];

    let recent_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

    let mut is_first_iteration = true;

    let mut token_balance = Vec::new();

    let pool_info = pool_keys_fetcher(pool_id).await?;

    for (chunk_index, wallet_chunk) in wallets_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();
        let mut current_wallets = Vec::new();

        for (i, wallet) in wallet_chunk.iter().enumerate() {
            let token_account = get_associated_token_address(
                &wallet.pubkey(),
                &Pubkey::from_str(&data.token_mint).unwrap(),
            );

            let balance = rpc_client
                .get_token_account_balance(&token_account)
                .await
                .unwrap()
                .amount
                .parse::<u64>()
                .unwrap();

            token_balance.push(balance);

            println!("Balance: {} SOL", balance);

            let source_sol_ata = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);

            let create_sol_ata = create_associated_token_account_idempotent(
                &buyer_wallet.pubkey(),
                &buyer_wallet.pubkey(),
                &SOLC_MINT,
                &spl_token::id(),
            );

            if is_first_iteration {
                current_instructions.push(create_sol_ata);
                is_first_iteration = false;
            }

            let swap_ixs = swap_ixs(
                data.clone(),
                amm_keys,
                market_keys.clone(),
                &wallet,
                sol_to_lamports(
                    lamports_to_sol(token_balance.iter().sum::<u64>()) * percentage_tokens,
                ),
                true,
                source_sol_ata,
            )
            .unwrap();

            if chunk_index == wallets_chunks.len() - 1 && i == wallet_chunk.len() - 1 {
                let mut tip = vec![tip_txn(
                    buyer_wallet.pubkey(),
                    tip_account(),
                    sol_to_lamports(0.001),
                )];
                let source_ata = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);

                let tax_destination = get_associated_token_address(&TAX_ACCOUNT, &SOLC_MINT);

                let token_to_sol = match token_price_data(
                    rpc_client.clone(),
                    pool_info.clone(),
                    Arc::new(Keypair::from_base58_string("3J9DCjfeVVsa46hzA3uLuaAQefw2UPwD5UqbSQRBkT7UsUvtJtxaG3rLduMndKNYqbq9shuRGX2A7GUBiWwD3Mms")),
                    balance,
                    SwapDirection::Coin2PC,
                ).await{
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error getting token price data: {}", e);
                        return Err(e.into());
                    }
                };
                tip.push(system_instruction::transfer(
                    &source_ata,
                    &tax_destination,
                    sol_to_lamports(lamports_to_sol(token_to_sol as u64) * 0.05),
                ));

                info!("Pushing tax instruction");
                current_instructions.extend(tip);
            }
            current_wallets.push(wallet);
            current_instructions.push(swap_ixs);
        }
        current_wallets.push(&buyer_wallet);

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
                eprintln!("Error creating Swap transaction: {}", e);
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

    bundle_txn.iter().for_each(|tx| {
        println!("Transaction: {:?}", tx.signatures);
    });

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
