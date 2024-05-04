use std::{str::FromStr, sync::Arc, thread::sleep};

use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use serde::Serialize;
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    env::minter::load_minter_settings,
    instruction::instruction::SOL_MINT,
    liquidity::{
        option::{sol_distribution::sol_distribution, wallet_gen::list_folders},
        utils::{tip_account, tip_txn},
    },
    raydium::swap::{instructions::TAX_ACCOUNT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
    user_inputs::amounts::bundle_priority_tip,
};

use super::{
    lut::extend_lut::lut_main,
    pool_ixs::pool_ixs,
    swap_ixs::{load_pool_keys, swap_ixs},
    utils::JitoPoolData,
};

#[derive(Debug, serde::Serialize)]
pub struct PoolDeployResponse {
    pub wallets: Vec<String>,
    pub amm_pool: Pubkey,
}

pub async fn pool_main() -> eyre::Result<()> {
    let (folders, wallets) = match list_folders().await {
        Ok(folders) => folders,
        Err(e) => {
            eprintln!("Error listing folders: {}", e);
            return Ok(());
        }
    };

    let bundle_tip = bundle_priority_tip().await;

    let data = load_minter_settings().await?;

    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let deployer_key = Keypair::from_base58_string(&data.deployer_key);
    let buyer_key = Keypair::from_base58_string(&data.buyer_key);

    // -------------------Wallet Generation & Distribution-------------------
    // let (wallets, rand_amount, sol_distribution) = match sol_distribution(data.clone()).await {
    //     Ok(wallets) => wallets,
    //     Err(e) => {
    //         eprintln!("Error distributing SOL: {}", e);
    //         return Err(e);
    //     }
    // };

    // let rand_amount: Vec<u64> = rand_amount
    //     .iter()
    //     .map(|x| x - sol_to_lamports(0.0001))
    //     .collect();

    let mut wallet_response = vec![];

    wallets.iter().for_each(|wallet| {
        let keypair = wallet.to_base58_string();
        wallet_response.push(keypair);
    });

    println!("Wallets: {:?}", wallet_response);

    // -------------------Pool Creation Instructions--------------------------

    let (create_pool_ixs, amm_pool, amm_keys) = match pool_ixs(data.clone()).await {
        Ok(ixs) => ixs,
        Err(e) => {
            eprintln!("Error creating pool IXs: {}", e);
            return Err(e);
        }
    };

    let tax_txn = tip_txn(deployer_key.pubkey(), TAX_ACCOUNT, sol_to_lamports(0.3));

    // -------------------Pool Keys------------------------------------------
    let market_keys = load_pool_keys(amm_pool, amm_keys).await?;

    // -------------------LUT Creation & Extending---------------------------------------
    let lut_creation = //Pubkey::from_str("5T6TfZ7g8xEgoY47AS2bQDfZ6vJmAZFsBibVRiKRjj8V").unwrap();
    match lut_main(
        data.clone(),
        amm_keys,
        market_keys.clone(),
        wallets.iter().map(|x| x.pubkey()).collect::<Vec<_>>(),
    )
    .await
    {
        Ok(lut) => lut,
        Err(e) => {
            eprintln!("Error creating LUT: {}", e);
            return Err(e);
        }
    };

    let mut raw_account = None;

    while raw_account.is_none() {
        match connection.get_account(&lut_creation).await {
            Ok(account) => raw_account = Some(account),
            Err(e) => {
                eprintln!("Error getting LUT account: {}, retrying...", e);
            }
        }
    }

    let raw_account = raw_account.unwrap();

    let address_lookup_table = AddressLookupTable::deserialize(&raw_account.data)?;
    let address_lookup_table_account = AddressLookupTableAccount {
        key: lut_creation,
        addresses: address_lookup_table.addresses.to_vec(),
    };
    let market_keys = market_keys.clone();
    let server_data = data.clone();

    //-------------------Pool Transaction---------------------------------------
    let versioned_msg = VersionedMessage::V0(
        Message::try_compile(
            &deployer_key.pubkey(),
            &[create_pool_ixs, tax_txn],
            &[address_lookup_table_account.clone()],
            connection.get_latest_blockhash().await?,
        )
        .unwrap(),
    );

    let versioned_tx = match VersionedTransaction::try_new(versioned_msg, &[&deployer_key]) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error creating pool transaction: {}", e);
            return Err(e.into());
        }
    };

    // -------------------Swap Instructions---------------------------------------

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let mut wallet_chunks = Vec::new();
    let mut txns_chunk = Vec::new();

    txns_chunk.push(versioned_tx);

    let mut balance_amounts = vec![];

    for wallet in &wallets {
        let wsol_ata = get_associated_token_address(&wallet.pubkey(), &SOL_MINT);
        let amount_in = connection
            .get_token_account_balance(&wsol_ata)
            .await
            .unwrap();
        balance_amounts.push(amount_in.amount.parse::<u64>().unwrap() - sol_to_lamports(0.0001));
    }

    let address_lookup_table_account = address_lookup_table_account.clone();
    wallets
        .chunks(7)
        .enumerate()
        .for_each(|(index, wallet_chunk)| {
            let mut current_wallets = Vec::new();
            let mut current_instructions = Vec::new();

            wallet_chunk.iter().enumerate().for_each(|(j, wallet)| {
                let swap_ixs = swap_ixs(
                    server_data.clone(),
                    amm_keys,
                    market_keys.clone(),
                    wallet,
                    balance_amounts[index * 7 + j],
                )
                .unwrap();

                current_wallets.push(wallet);
                current_instructions.push(swap_ixs);
            });

            // If current chunk has less than 7 instructions, add jito_txn
            if current_instructions.len() < 7 {
                let jito_txn = tip_txn(buyer_key.pubkey(), tip_account(), bundle_tip);
                current_instructions.push(jito_txn);
                current_wallets.push(&buyer_key); // Assuming buyer_key is the corresponding wallet
            }

            let versioned_msg = VersionedMessage::V0(
                Message::try_compile(
                    &current_wallets[0].pubkey(),
                    &current_instructions,
                    &[address_lookup_table_account.clone()],
                    recent_blockhash,
                )
                .unwrap(),
            );

            let versioned_txn =
                VersionedTransaction::try_new(versioned_msg, &current_wallets).unwrap();

            println!(
                "Pushing {} wallets and {} instructions to transaction",
                current_wallets.len(),
                current_instructions.len()
            );

            wallet_chunks.push(current_wallets);
            txns_chunk.push(versioned_txn);
        });
    // check the size of each txn in instructions
    use bincode::serialize;

    let txn_size: Vec<_> = txns_chunk
        .iter()
        .map(|x| {
            let serialized_x = serialize(x).unwrap();
            serialized_x.len()
        })
        .collect();

    println!("txn_size: {:?}", txn_size);

    // -------------------Subscribe to Bundle Results---------------------------------------

    let mut client = get_searcher_client(
        &"https://ny.mainnet.block-engine.jito.wtf",
        &Arc::new(auth_keypair()),
    )
    .await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    if txns_chunk.len() > 5 {
        eprintln!("Too many transactions to send in one bundle");
        return Err(eyre::eyre!("Too many transactions to send in one bundle"));
    }

    let connection = &Arc::new(connection);

    let bundle_results = match send_bundle_with_confirmation(
        &txns_chunk,
        connection,
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
