use jito_searcher_client::get_searcher_client;
use log::{error, info};
use rand::rngs::{StdRng, ThreadRng};
use rand::{thread_rng, Rng, SeedableRng};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use solana_program::slot_history::Slot;
use solana_program::system_instruction::transfer;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::pubkey;
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_memo::build_memo;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::env::EngineSettings;
use crate::jito_plugin::lib::{
    generate_tip_accounts, send_bundles, BlockStats, BundledTransactions,
};
use crate::raydium;
use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::swap::instructions::{swap_base_in, swap_base_out, SOLC_MINT};
use crate::rpc::rpc_key;

pub async fn raydium_in(
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    amount_in: u64,
    amount_out: u64,
    priority_fee: u64,
    args: EngineSettings,
) -> eyre::Result<()> {
    let user_source_owner = wallet.pubkey();
    let url = "http://64.176.215.55:8899".to_string();
    let config = CommitmentLevel::Confirmed;
    let rpc_client = Arc::new(RpcClient::new_with_commitment(
        url,
        solana_sdk::commitment_config::CommitmentConfig { commitment: config },
    ));

    let token_address = if pool_keys.base_mint == SOLC_MINT.to_string() {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };
    let swap_instructions = swap_base_in(
        &Pubkey::from_str(&pool_keys.program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.id).unwrap(),
        &Pubkey::from_str(&pool_keys.authority).unwrap(),
        &Pubkey::from_str(&pool_keys.open_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.target_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_bids).unwrap(),
        &Pubkey::from_str(&pool_keys.market_asks).unwrap(),
        &Pubkey::from_str(&pool_keys.market_event_queue).unwrap(),
        &Pubkey::from_str(&pool_keys.market_base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_authority).unwrap(),
        &user_source_owner,
        &user_source_owner,
        &user_source_owner,
        &Pubkey::from_str(&token_address).unwrap(),
        amount_in,
        amount_out,
        priority_fee,
    )
    .await?;

    let config = CommitmentLevel::Confirmed;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let frontrun_tx = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };
    let tip_accounts =
        generate_tip_accounts(&pubkey!("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt"));
    let mut rng = StdRng::from_entropy();
    let tip_account = tip_accounts[rng.gen_range(0..tip_accounts.len())];

    let message = "Front Transaction to Molest Jeets".to_string();

    //Tip Transaction to Jito
    let backrun_tx = VersionedTransaction::from(Transaction::new_signed_with_payer(
        &[
            build_memo(
                format!("{}: {:?}", message, frontrun_tx.signatures[0].to_string()).as_bytes(),
                &[],
            ),
            transfer(&wallet.pubkey(), &tip_account, 10_000),
        ],
        Some(&wallet.pubkey()),
        &[wallet],
        rpc_client.get_latest_blockhash().await.unwrap(),
    ));
    let mut searcher_client =
        get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;
    let config = RpcSendTransactionConfig {
        skip_preflight: true,
        ..Default::default()
    };

    let bundle_txn = BundledTransactions {
        mempool_txs: vec![frontrun_tx],
        backrun_txs: vec![backrun_tx],
    };
    let mut block_stats: HashMap<Slot, BlockStats> = HashMap::new();

    let mut results = send_bundles(&mut searcher_client, &[bundle_txn]).await?;

    if let Ok(response) = results.remove(0) {
        let message = response.into_inner();
        let uuid = &message.uuid;
        info!("UUID: {:?}", uuid);
    }
    // let result = match rpc_client
    //     .send_transaction_with_config(&transaction, config)
    //     .await
    // {
    //     Ok(x) => x,
    //     Err(e) => {
    //         error!("Error: {:?}", e);
    //         return Ok(());
    //     }
    // };

    // info!("Transaction Signature: {:?}", result.to_string());

    // let rpc_client_1 = rpc_client.clone();
    // tokio::spawn(async move {
    //     let _ = match rpc_client_1
    //         .confirm_transaction_with_spinner(
    //             &result,
    //             &rpc_client_1.get_latest_blockhash().await.unwrap(),
    //             solana_sdk::commitment_config::CommitmentConfig::processed(),
    //         )
    //         .await
    //     {
    //         Ok(x) => x,
    //         Err(e) => {
    //             error!("Error: {:?}", e);
    //         }
    //     };
    // });

    let raydium_txn = raydium_txn_backrun(rpc_client, wallet, pool_keys, &args).await?;

    Ok(())
}

pub async fn raydium_txn_backrun(
    rpc_client: Arc<RpcClient>,
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    args: &EngineSettings,
) -> eyre::Result<()> {
    let start = Instant::now();
    let mut token_balance = 0;

    while start.elapsed() < Duration::from_secs(15) {
        let token_accounts = rpc_client
            .get_token_accounts_by_owner(
                &wallet.pubkey(),
                TokenAccountsFilter::Mint(Pubkey::from_str(&pool_keys.base_mint).unwrap()),
            )
            .await?;

        for rpc_keyed_account in &token_accounts {
            let pubkey = &rpc_keyed_account.pubkey;
            //convert to pubkey
            let pubkey = Pubkey::from_str(&pubkey)?;

            let balance = rpc_client.get_token_account_balance(&pubkey).await?;
            info!("balance: {:?}", balance);
            let lamports = match balance.amount.parse::<u64>() {
                Ok(lamports) => lamports,
                Err(e) => {
                    eprintln!("Failed to parse balance: {}", e);
                    break;
                }
            };

            token_balance = lamports;

            if lamports != 0 {
                break;
            }

            std::thread::sleep(Duration::from_secs(1));
        }

        if token_balance != 0 {
            break;
        }
    }

    if token_balance == 0 {
        return Ok(());
    }

    info!("Tokens: {:?}", token_balance);

    let _ = raydium_out(wallet, pool_keys.clone(), token_balance, 1, 0, args.clone()).await?;

    Ok(())
}

pub async fn raydium_out(
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    amount_in: u64,
    amount_out: u64,
    priority_fee: u64,
    args: EngineSettings,
) -> eyre::Result<()> {
    let user_source_owner = wallet.pubkey();
    let url = "http://64.176.215.55:8899".to_string();
    let config = CommitmentLevel::Confirmed;
    let rpc_client = Arc::new(RpcClient::new_with_commitment(
        url,
        solana_sdk::commitment_config::CommitmentConfig { commitment: config },
    ));

    let token_address = if pool_keys.base_mint == SOLC_MINT.to_string() {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };

    let swap_instructions = swap_base_out(
        &Pubkey::from_str(&pool_keys.program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.id).unwrap(),
        &Pubkey::from_str(&pool_keys.authority).unwrap(),
        &Pubkey::from_str(&pool_keys.open_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.target_orders).unwrap(),
        &Pubkey::from_str(&pool_keys.base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_program_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_id).unwrap(),
        &Pubkey::from_str(&pool_keys.market_bids).unwrap(),
        &Pubkey::from_str(&pool_keys.market_asks).unwrap(),
        &Pubkey::from_str(&pool_keys.market_event_queue).unwrap(),
        &Pubkey::from_str(&pool_keys.market_base_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_quote_vault).unwrap(),
        &Pubkey::from_str(&pool_keys.market_authority).unwrap(),
        &user_source_owner,
        &user_source_owner,
        &Pubkey::from_str(&token_address).unwrap(),
        amount_in,
        amount_out,
        priority_fee,
    )
    .await?;

    let config = CommitmentLevel::Confirmed;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_instructions,
        &[],
        latest_blockhash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let frontrun_tx = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let tip_accounts =
        generate_tip_accounts(&pubkey!("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt"));
    let mut rng = StdRng::from_entropy();
    let tip_account = tip_accounts[rng.gen_range(0..tip_accounts.len())];

    //Tip Transaction to Jito
    let backrun_tx = VersionedTransaction::from(Transaction::new_signed_with_payer(
        &[
            build_memo(
                format!(
                    "{}: {:?}",
                    args.message,
                    frontrun_tx.signatures[0].to_string()
                )
                .as_bytes(),
                &[],
            ),
            transfer(&wallet.pubkey(), &tip_account, 10_000),
        ],
        Some(&wallet.pubkey()),
        &[wallet],
        rpc_client.get_latest_blockhash().await.unwrap(),
    ));

    let mut searcher_client =
        get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;

    let bundle_txn = BundledTransactions {
        mempool_txs: vec![frontrun_tx],
        backrun_txs: vec![backrun_tx],
    };

    let results = send_bundles(&mut searcher_client, &[bundle_txn]).await?;

    info!("Results: {:?}", &results[0]);

    Ok(())
}

pub fn auth_keypair() -> Keypair {
    let bytes_auth_vec = vec![
        198, 214, 173, 4, 113, 67, 147, 103, 75, 216, 80, 150, 174, 158, 63, 61, 10, 228, 165, 151,
        189, 0, 34, 29, 24, 166, 40, 136, 166, 58, 116, 242, 35, 218, 175, 128, 50, 244, 240, 13,
        176, 112, 152, 243, 132, 142, 93, 20, 112, 225, 9, 103, 175, 8, 161, 234, 247, 176, 242,
        78, 131, 96, 57, 100,
    ];
    let bytes_auth = bytes_auth_vec.as_slice();
    let auth_keypair = Keypair::from_bytes(bytes_auth).unwrap();
    auth_keypair
}
