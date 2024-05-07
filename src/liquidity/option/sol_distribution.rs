use std::{str::FromStr, sync::Arc};

use bincode::serialize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use solana_sdk::{
    instruction::Instruction,
    message::VersionedMessage,
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
    env::minter::{load_minter_settings, PoolDataSettings},
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
) -> eyre::Result<(Vec<Keypair>, Vec<u64>, Vec<VersionedTransaction>)> {
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

    let mut distribution_ix: Vec<Instruction> = vec![];

    //------------------------------------Wrapped-SOL Distribution------------------------------------
    let user_token_destination = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);
    distribution_ix.push(
        spl_associated_token_account::instruction::create_associated_token_account(
            &buyer_wallet.pubkey(),
            &buyer_wallet.pubkey(),
            &SOLC_MINT,
            &spl_token::id(),
        ),
    );

    distribution_ix.push(system_instruction::transfer(
        &buyer_wallet.pubkey(),
        &user_token_destination,
        buy_amount,
    ));

    let sync_native = sync_native(&spl_token::id(), &user_token_destination)?;
    distribution_ix.push(sync_native);

    //-----------------------------------------------------------------------------------------------

    let source_ata = get_associated_token_address(&buyer_wallet.pubkey(), &SOLC_MINT);

    for (index, wallet) in wallets.iter().enumerate() {
        // distribution_ix.push(
        //     solana_sdk::system_instruction::transfer(
        //     &buyer_wallet.pubkey(),
        //     &wallet.pubkey(),
        //     rand_amount[index],
        // ));
        let tax_destination = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);
        let transfer_instruction = spl_token::instruction::transfer(
            &spl_token::id(),
            &source_ata,
            &tax_destination,
            &buyer_wallet.pubkey(),
            &[&buyer_wallet.pubkey()],
            rand_amount[index],
        )?;

        distribution_ix.push(transfer_instruction);
    }

    let tip_ix = tip_txn(buyer_wallet.pubkey(), tip_account(), sol_to_lamports(0.005));
    let (atas_creation, _, _) = atas_creation(
        wallets.iter().map(|x| x.pubkey()).collect::<Vec<_>>(),
        buyer_wallet.clone(),
        Pubkey::from_str(&server_data.token_mint).unwrap(),
    )
    .await
    .unwrap();

    distribution_ix.push(tip_ix);

    let mut atas_ix: Vec<Instruction> = vec![];
    atas_ix.extend(atas_creation);

    let distribution_ix_chunks: Vec<_> = distribution_ix.chunks(20).collect();
    let atas_ix_chunks: Vec<_> = atas_ix.chunks(16).collect();

    let all_chunks = distribution_ix_chunks
        .into_iter()
        .chain(atas_ix_chunks.into_iter())
        .collect::<Vec<_>>();

    let recent_blockhash = connection.get_latest_blockhash().await?;

    let mut transactions: Vec<VersionedTransaction> = vec![];

    for chunk in all_chunks {
        let versioned_msg = VersionedMessage::V0(
            match solana_sdk::message::v0::Message::try_compile(
                &buyer_wallet.pubkey(),
                chunk,
                &[],
                recent_blockhash,
            ) {
                Ok(message) => message,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    panic!("Error: {}", e);
                }
            },
        );

        let transaction =
            VersionedTransaction::try_new(versioned_msg, &[&buyer_wallet as &dyn Signer])?;

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

    Ok((wallets, rand_amount, transactions))
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

    let data = load_minter_settings().await?;

    let token_address = match Pubkey::from_str(&data.token_mint) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let total_amount = sol_amount().await;
    let bundle_tip = bundle_priority_tip().await;

    let (wallets, amounts, transactions) = sol_distribution(data, total_amount).await?;

    for amount in amounts {
        let amount = format!("{}", lamports_to_sol(amount));
        info!("{}", amount);
    }

    let mut client = get_searcher_client(
        "https://ny.mainnet.block-engine.jito.wtf",
        &Arc::new(auth_keypair()),
    )
    .await?;

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    info!("Subscribed to bundle results");

    let bundle = match send_bundle_with_confirmation(
        &transactions,
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
