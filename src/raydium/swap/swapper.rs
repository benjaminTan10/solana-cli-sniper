use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::{signature::Keypair, signer::Signer};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::env::EngineSettings;
use crate::raydium::subscribe::PoolKeysSniper;
use crate::raydium::swap::instructions::{swap_base_in, swap_base_out, SOLC_MINT, TAX_ACCOUNT};

pub async fn raydium_in(
    rpc_client: Arc<RpcClient>,
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    amount_in: u64,
    amount_out: u64,
    priority_fee: u64,
    args: EngineSettings,
) -> eyre::Result<()> {
    let user_source_owner = wallet.pubkey();

    let token_address = if pool_keys.base_mint == SOLC_MINT.to_string() {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };
    let mut swap_instructions = swap_base_in(
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

    //2% of amount_in
    let tax_amount = amount_in * 2 / 100;

    let tax_instruction = transfer(&user_source_owner, &TAX_ACCOUNT, tax_amount);

    swap_instructions.push(tax_instruction);

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

    let transaction = match VersionedTransaction::try_new(
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
        preflight_commitment: Some(CommitmentLevel::Finalized),
        max_retries: Some(20),
        ..Default::default()
    };

    let result = match rpc_client
        .send_transaction_with_config(&transaction, config)
        .await
    {
        Ok(x) => x,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };

    info!("Transaction Signature: {:?}", result.to_string());

    let rpc_client_1 = rpc_client.clone();
    tokio::spawn(async move {
        let _ = match rpc_client_1
            .confirm_transaction_with_spinner(
                &result,
                &rpc_client_1.get_latest_blockhash().await.unwrap(),
                solana_sdk::commitment_config::CommitmentConfig::confirmed(),
            )
            .await
        {
            Ok(x) => x,
            Err(e) => {
                error!("Error: {:?}", e);
            }
        };
    });

    let raydium_txn =
        match raydium_txn_backrun(rpc_client, wallet, pool_keys, &args, priority_fee).await {
            Ok(x) => x,
            Err(e) => {
                error!("Error: {:?}", e);
            }
        };

    Ok(())
}

pub async fn raydium_txn_backrun(
    rpc_client: Arc<RpcClient>,
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    args: &EngineSettings,
    priority_fee: u64,
) -> eyre::Result<()> {
    info!("Searching for Token Balance...!");
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
            info!("Token Balance: {:?}", balance.amount);
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
        }

        if token_balance != 0 {
            break;
        }
    }

    if token_balance == 0 {
        return Ok(());
    }

    let _ = match raydium_out(
        wallet,
        pool_keys.clone(),
        token_balance,
        1,
        priority_fee,
        args.clone(),
        rpc_client.clone(),
    )
    .await
    {
        Ok(x) => x,
        Err(e) => {
            error!("Error: {:?}", e);
        }
    };

    Ok(())
}

pub async fn raydium_out(
    wallet: &Arc<Keypair>,
    pool_keys: PoolKeysSniper,
    amount_in: u64,
    amount_out: u64,
    priority_fee: u64,
    args: EngineSettings,
    rpc_client: Arc<RpcClient>,
) -> eyre::Result<()> {
    info!("Swapping Out...");
    let user_source_owner = wallet.pubkey();

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

    let transaction = match VersionedTransaction::try_new(
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
        preflight_commitment: Some(CommitmentLevel::Finalized),
        max_retries: Some(20),
        ..Default::default()
    };

    let result = match rpc_client
        .send_transaction_with_config(&transaction, config)
        .await
    {
        Ok(x) => {
            info!("Transaction Signature: {:?}", x.to_string());
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    };

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
