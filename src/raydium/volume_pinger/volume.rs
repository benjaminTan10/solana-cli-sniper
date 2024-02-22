use std::{
    error::Error,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use jito_searcher_client::get_searcher_client;
use log::{error, info};
use rand::{rngs::StdRng, Rng, SeedableRng};
use solana_client::{
    nonblocking::rpc_client::{self, RpcClient},
    rpc_config::RpcSendTransactionConfig,
    rpc_request::TokenAccountsFilter,
};
use solana_program::pubkey;
use solana_sdk::{
    commitment_config::CommitmentLevel,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction::transfer,
    transaction::{Transaction, VersionedTransaction},
};
use spl_memo::build_memo;

use crate::{
    app::{bundle_priority_tip, priority_fee, private_key_env, sol_amount, token_env, MevApe},
    env::{load_settings, EngineSettings},
    jito_plugin::lib::{generate_tip_accounts, send_bundles, BundledTransactions},
    raydium::{
        pool_searcher::amm_keys::pool_keys_fetcher,
        subscribe::PoolKeysSniper,
        swap::{
            instructions::{swap_base_in, swap_base_out, token_price_data, SOLC_MINT},
            swapper::auth_keypair,
        },
    },
};

pub async fn generate_volume() -> Result<(), Box<dyn Error>> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };
    let sol_amount = sol_amount().await?;
    let priority_fee = priority_fee().await?;
    // let bundle_tip = bundle_priority_tip().await?;
    let pool_address = token_env().await?;
    let wallet = private_key_env().await?;
    let secret_key = bs58::decode(wallet.clone()).into_vec()?;
    let mev_ape = MevApe {
        sol_amount,
        priority_fee,
        bundle_tip: 0,
        wallet,
    };
    let wallet = Keypair::from_bytes(&secret_key)?;

    let pool_keys = pool_keys_fetcher(pool_address.to_string()).await?;
    let rpc_client = RpcClient::new(args.rpc_url.to_string());

    let generate_volume = volume_round(
        Arc::new(rpc_client),
        pool_keys,
        Arc::new(wallet),
        args,
        mev_ape,
    )
    .await?;

    Ok(())
}

pub async fn volume_round(
    rpc_client: Arc<RpcClient>,
    pool_keys: PoolKeysSniper,
    wallet: Arc<Keypair>,
    args: EngineSettings,
    mev_ape: MevApe,
) -> Result<(), Box<dyn Error>> {
    let user_source_owner = wallet.pubkey();
    let mut searcher_client =
        get_searcher_client(&args.block_engine_url, &Arc::new(auth_keypair())).await?;
    let tip_accounts =
        generate_tip_accounts(&pubkey!("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt"));
    let mut rng = StdRng::from_entropy();
    let tip_account = tip_accounts[rng.gen_range(0..tip_accounts.len())];
    let token_address = if pool_keys.base_mint == SOLC_MINT.to_string() {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };

    let transaction_main_instructions = swap_base_in(
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
        mev_ape.sol_amount,
        0,
        mev_ape.priority_fee,
    )
    .await?;

    // let tokens_amount = token_price_data(
    //     rpc_client.clone(),
    //     pool_keys.clone(),
    //     &wallet.clone(),
    //     mev_ape.sol_amount,
    // )
    // .await?;

    // transaction_main_instructions.extend(swap_out_instructions);

    let config = CommitmentLevel::Finalized;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &transaction_main_instructions,
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

    let config = RpcSendTransactionConfig {
        skip_preflight: true,
        ..Default::default()
    };
    let txn = rpc_client
        .send_transaction_with_config(&frontrun_tx, config)
        .await;

    match txn {
        Ok(x) => {
            info!("Swap in Transaction: {:?}", x);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }

    let mut token_balance = 0;
    let start = Instant::now();
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
            println!("balance: {:?}", balance);
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
    let swap_out_instructions = swap_base_out(
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
        token_balance as u64,
        0,
        mev_ape.priority_fee,
    )
    .await?;

    let config = CommitmentLevel::Finalized;
    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: config,
        })
        .await?;

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_out_instructions,
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

    let config = RpcSendTransactionConfig {
        skip_preflight: true,
        ..Default::default()
    };
    let txn = rpc_client
        .send_transaction_with_config(&frontrun_tx, config)
        .await;

    match txn {
        Ok(x) => {
            info!("Swap out Transaction: {:?}", x);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
    Ok(())
}
