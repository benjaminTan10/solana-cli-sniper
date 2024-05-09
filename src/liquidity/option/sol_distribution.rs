use std::{str::FromStr, sync::Arc};

use bincode::serialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    compute_budget::ComputeBudgetInstruction,
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
    buy_amount: u64,
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

    //------------------------------------Wrapped-SOL Distribution------------------------------------
    // let user_token_destination = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);
    // distribution_ix.push(
    //     spl_associated_token_account::instruction::create_associated_token_account(
    //         &buyer_wallet.pubkey(),
    //         &buyer_wallet.pubkey(),
    //         &SOLC_MINT,
    //         &spl_token::id(),
    //     ),
    // );

    // distribution_ix.push(system_instruction::transfer(
    //     &buyer_wallet.pubkey(),
    //     &user_token_destination,
    //     buy_amount,
    // ));

    // let sync_native = sync_native(&spl_token::id(), &user_token_destination)?;
    // distribution_ix.push(sync_native);

    // let wrap = match wrap_sol(connection.clone(), &buyer_wallet, buy_amount).await {
    //     Ok(wrap) => wrap,
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         panic!("Error: {}", e);
    //     }
    // };

    //-----------------------------------------------------------------------------------------------

    let source_ata = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);

    for (index, wallet) in wallets.iter().enumerate() {
        let tax_destination = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);
        // let transfer_instruction = spl_token::instruction::transfer(
        //     &spl_token::id(),
        //     &source_ata,
        //     &tax_destination,
        //     &buyer_wallet.pubkey(),
        //     &[&buyer_wallet.pubkey()],
        //     rand_amount[index],
        // )?;
        let transfer_instruction = system_instruction::transfer(
            &buyer_wallet.pubkey(),
            &tax_destination,
            rand_amount[index],
        );

        distribution_ix.push(transfer_instruction);
    }
    let bundle_tip = bundle_priority_tip().await;

    let tip_ix = tip_txn(buyer_wallet.pubkey(), tip_account(), bundle_tip);
    let (atas_creation, _, _) = atas_creation(
        wallets.iter().map(|x| x.pubkey()).collect::<Vec<_>>(),
        buyer_wallet.clone(),
        Pubkey::from_str(&server_data.token_mint).unwrap(),
    )
    .await
    .unwrap();

    // distribution_ix.push(tip_ix);
    let mut atas_ix: Vec<Instruction> = vec![];
    atas_ix.extend(atas_creation);
    atas_ix.push(tip_ix);

    // Split atas_ix into two vectors at index 8
    let mut rest_atas_ix = atas_ix.split_off(10);

    // Push the first chunk of atas_ix to distribution_ix
    distribution_ix.extend(atas_ix);

    let distribution_ix_chunks: Vec<_> = distribution_ix.chunks(21).collect();

    // Push the rest of atas_ix to atas_ix_chunks
    let atas_ix_chunks: Vec<_> = rest_atas_ix.chunks(16).collect();

    let all_chunks = atas_ix_chunks
        .into_iter()
        .chain(distribution_ix_chunks.into_iter())
        .collect::<Vec<_>>();

    let wrap = match wsol(
        server_data,
        Arc::new(wallets),
        rand_amount.clone(),
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

    //----------------------------------------------------------------------------------

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let mut transactions: Vec<VersionedTransaction> = vec![];

    for chunk in all_chunks {
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

    Ok((rand_amount, transactions, wrap))
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

    let token_address = match Pubkey::from_str(&data.token_mint) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let mut client =
        get_searcher_client(&settings.block_engine_url, &Arc::new(auth_keypair())).await?;

    let total_amount = sol_amount().await;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let (amounts, transactions_1, transactions_2) = match sol_distribution(data, total_amount).await
    {
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

    // info!("Subscribed to bundle results");

    // transactions.iter().for_each(|txn| {
    //     info!("Transaction: {:?}", txn.signatures[0]);
    // });

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
            eprintln!("Distribution Error: {}", e);
            panic!("Error: {}", e);
        }
    };
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

pub async fn wsol(
    pool_data: PoolDataSettings,
    wallets: Arc<Vec<Keypair>>,
    amount_in: Vec<u64>,
    address_lookup_table_account: AddressLookupTableAccount,
) -> Result<Vec<VersionedTransaction>, Box<dyn std::error::Error + Send>> {
    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let buyer_wallet = Keypair::from_base58_string(&pool_data.buyer_key);

    let wallets = Arc::clone(&wallets);
    let mut txns_chunk = Vec::new();
    let mut wallet_chunks = Vec::new();

    let recent_blockhash = match connection.get_latest_blockhash().await {
        Ok(recent_blockhash) => recent_blockhash,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    wallets
        .chunks(6)
        .enumerate()
        .for_each(|(index, wallet_chunk)| {
            let mut current_wallets = Vec::new();
            let mut current_instructions = Vec::new();

            wallet_chunk.iter().enumerate().for_each(|(j, wallet)| {
                let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(80000);
                let compute_price = ComputeBudgetInstruction::set_compute_unit_price(sol_to_lamports(0.0001));
                let user_token_destination = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

                current_instructions.push(unit_limit);
                current_instructions.push(compute_price);
                current_instructions.push(
                    spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                        &wallet.pubkey(),
                        &wallet.pubkey(),
                        &SOLC_MINT,
                        &spl_token::id(),
                    ),
                );
                current_instructions.push(system_instruction::transfer(
                    &wallet.pubkey(),
                    &user_token_destination,
                    amount_in[index * 6 + j], // Updated here
                ));

                let sync_native = match sync_native(&spl_token::id(), &user_token_destination) {
                    Ok(sync_native) => sync_native,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        panic!("Error: {}", e);
                    }
                };
                current_instructions.push(sync_native);

                current_wallets.push(wallet);
            });

            // If current chunk has less than 7 instructions, add jito_txn
            if current_instructions.len() < 7 {
                let jito_txn = tip_txn(buyer_wallet.pubkey(), tip_account(), sol_to_lamports(0.001));
                current_instructions.push(jito_txn);
                current_wallets.push(&buyer_wallet); // Assuming buyer_wallet is the corresponding wallet
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

            let versioned_txn = VersionedTransaction::try_new(versioned_msg, &current_wallets).unwrap();

          

            wallet_chunks.push(current_wallets);
            txns_chunk.push(versioned_txn);
        });
    use bincode::serialize;

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
