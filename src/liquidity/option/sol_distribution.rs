use std::{str::FromStr, sync::Arc};

use bincode::serialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::instruction::sync_native;

use crate::{
    env::{
        load_settings,
        minter::{load_minter_settings, PoolDataSettings},
    },
    instruction::instruction::SOL_MINT,
    liquidity::{
        option::wallet_gen::load_wallets,
        utils::{tip_account, tip_txn},
    },
    raydium::swap::{instructions::SOLC_MINT, swapper::auth_keypair},
    rpc::HTTP_CLIENT,
    user_inputs::amounts::{bundle_priority_tip, sol_amount},
    utils::rand::distribute_randomly,
};

pub async fn sol_distribution(
    server_data: PoolDataSettings,
) -> eyre::Result<(
    Vec<u64>,
    Vec<VersionedTransaction>,
    Vec<VersionedTransaction>,
)> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let buyer_wallet = Arc::new(Keypair::from_base58_string(&server_data.buyer_key));

    let wallets: Vec<Keypair> = match load_wallets().await {
        Ok(wallets) => wallets,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let buy_amount = sol_amount().await;
    let bundle_tip = bundle_priority_tip().await;

    let rand_amount = distribute_randomly(buy_amount, wallets.len());

    let lut_creation = match Pubkey::from_str(&server_data.lut_key) {
        Ok(lut) => lut,
        Err(e) => {
            panic!("LUT key not Found in Settings: {}", e);
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

    let mut distribution_ix: Vec<Instruction> = vec![];

    //-----------------------------------------------------------------------------------------------

    for (index, wallet) in wallets.iter().enumerate() {
        let transfer_instruction = system_instruction::transfer(
            &buyer_wallet.pubkey(),
            &wallet.pubkey(),
            rand_amount[index],
        );

        distribution_ix.push(transfer_instruction);
    }

    let tip_ix = tip_txn(buyer_wallet.pubkey(), tip_account(), bundle_tip);

    distribution_ix.push(tip_ix);

    let distribution_ix_chunks: Vec<_> = distribution_ix.chunks(21).collect();

    //----------------------------------------------------------------------------------

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let mut transactions: Vec<VersionedTransaction> = vec![];

    for (i, chunk) in distribution_ix_chunks.iter().enumerate() {
        let versioned_msg = VersionedMessage::V0(
            match solana_sdk::message::v0::Message::try_compile(
                &buyer_wallet.pubkey(),
                chunk,
                &[address_lookup_table_account.clone()],
                recent_blockhash,
            ) {
                Ok(message) => message,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    panic!("Error: {}", e);
                }
            },
        );

        let transaction = VersionedTransaction::try_new(versioned_msg, &[&buyer_wallet])?;

        println!("Chunk {}: {} instructions", i, chunk.len());

        transactions.push(transaction);
    }

    let mut sum = 0;
    let txn_size: Vec<_> = transactions
        .iter()
        .map(|x| {
            let serialized_x = serialize(x).unwrap();
            //sum all of them
            sum += serialized_x.len();
            serialized_x.len()
        })
        .collect();

    println!("Sum: {:?}", sum);
    println!("txn_size: {:?}", txn_size);

    println!("Generated transactions: {}", transactions.len());

    let wrap = match wsol(
        server_data,
        Arc::new(wallets),
        address_lookup_table_account.clone(),
    )
    .await
    {
        Ok(wrap) => wrap,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    Ok((rand_amount, transactions, wrap))
}

pub async fn wsol(
    pool_data: PoolDataSettings,
    wallets: Arc<Vec<Keypair>>,
    address_lookup_table_account: AddressLookupTableAccount,
) -> Result<Vec<VersionedTransaction>, Box<dyn std::error::Error + Send>> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let buyer_wallet = Keypair::from_base58_string(&pool_data.buyer_key);

    let mint = Pubkey::from_str(&pool_data.token_mint).unwrap();

    let wallets = Arc::clone(&wallets);

    let recent_blockhash = match connection.get_latest_blockhash().await {
        Ok(recent_blockhash) => recent_blockhash,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let wallet_chunks: Vec<_> = wallets.chunks(6).collect();
    let mut txns_chunk = Vec::new();

    for (chunk_index, wallet_chunk) in wallet_chunks.iter().enumerate() {
        let mut current_instructions = Vec::new();
        let mut current_wallets = Vec::new();

        for (i, wallet) in wallet_chunk.iter().enumerate() {
            let user_token_source = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

            let balance = connection.get_balance(&wallet.pubkey()).await.unwrap();
            println!("Balance: {}", balance);
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
                    &mint,
                    &spl_token::id(),
                ),
            );
            current_instructions.push(system_instruction::transfer(
                &wallet.pubkey(),
                &user_token_source,
                balance,
            ));

            let sync_native = match sync_native(&spl_token::id(), &user_token_source) {
                Ok(sync_native) => sync_native,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    panic!("Error: {}", e);
                }
            };
            current_instructions.push(sync_native);

            current_wallets.push(wallet);
        }

        println!(
            "Chunk {}: {} instructions",
            chunk_index,
            current_instructions.len()
        );
        println!("Chunk {}: {} wallets", chunk_index, current_wallets.len());

        current_wallets.push(&buyer_wallet);

        if current_instructions.len() < 13 {
            let tip = tip_txn(buyer_wallet.pubkey(), tip_account(), sol_to_lamports(0.001));
            current_instructions.push(tip);
        }

        let versioned_msg = VersionedMessage::V0(
            Message::try_compile(
                &buyer_wallet.pubkey(),
                &current_instructions,
                &[address_lookup_table_account.clone()],
                recent_blockhash,
            )
            .unwrap(),
        );

        let versioned_tx = match VersionedTransaction::try_new(versioned_msg, &current_wallets) {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("Error creating pool transaction: {}", e);
                panic!("Error: {}", e);
            }
        };

        txns_chunk.push(versioned_tx);
        // Now you can use chunk_index, current_wallets, and current_instructions
    }

    println!("txn: {:?}", txns_chunk.len());

    let txn_size: Vec<_> = txns_chunk
        .iter()
        .map(|x| {
            let serialized_x = serialize(x).unwrap();
            serialized_x.len()
        })
        .collect();

    println!("txn_size: {:?}", txn_size);

    Ok(txns_chunk)
}

//server_data.BlockEngineSelections
pub async fn atas_creation(
    wallets: Vec<Pubkey>,
    buyer_key: Arc<Keypair>,
    mint: Pubkey,
) -> eyre::Result<(Vec<Instruction>, Pubkey, Pubkey)> {
    let mut mint_ata = Pubkey::default();
    let mut sol_ata = Pubkey::default();
    let mut instructions = vec![];
    for wallet in wallets {
        mint_ata = get_associated_token_address(&wallet, &mint);
        sol_ata = get_associated_token_address(&wallet, &SOL_MINT);

        let create_mint_ata =
            create_associated_token_account(&buyer_key.pubkey(), &wallet, &mint, &spl_token::id());
        let create_sol_ata = create_associated_token_account(
            &buyer_key.pubkey(),
            &wallet,
            &SOL_MINT,
            &spl_token::id(),
        );

        instructions.push(create_mint_ata);
        instructions.push(create_sol_ata);
    }

    Ok((instructions, mint_ata, sol_ata))
}

pub async fn distributor() -> eyre::Result<()> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let data = match load_minter_settings().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let settings = load_settings().await?;

    let mut client =
        get_searcher_client(&settings.block_engine_url, &Arc::new(auth_keypair())).await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let (amounts, transactions_1, transactions_2) = match sol_distribution(data).await {
        Ok((amounts, transactions_1, transactions_2)) => (amounts, transactions_1, transactions_2),
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    for amount in amounts {
        let amount = format!("{}", lamports_to_sol(amount));
        info!("{}", amount);
    }

    info!("Sending Bundle 1 ");

    info!("Len: {}", transactions_2.len());

    let bundle = match send_bundle_with_confirmation(
        &transactions_1,
        &connection,
        &mut client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    info!("Sending Bundle 2");

    let bundle = match send_bundle_with_confirmation(
        &transactions_2,
        &connection,
        &mut client,
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
