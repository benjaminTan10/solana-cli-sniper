use std::{
    error::Error,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use log::{error, info};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig,
    rpc_request::TokenAccountsFilter,
};
use solana_sdk::{
    commitment_config::CommitmentLevel,
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    app::{priority_fee, private_key_env, sol_amount, token_env, MevApe},
    env::{load_settings, EngineSettings},
    raydium::{
        pool_searcher::amm_keys::pool_keys_fetcher,
        subscribe::PoolKeysSniper,
        swap::instructions::{swap_base_out, AmmInstruction, SwapInstructionBaseIn, SOLC_MINT},
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

    let _ = volume_round(Arc::new(rpc_client), pool_keys, Arc::new(wallet), mev_ape).await?;

    Ok(())
}

pub async fn volume_round(
    rpc_client: Arc<RpcClient>,
    pool_keys: PoolKeysSniper,
    wallet: Arc<Keypair>,
    mev_ape: MevApe,
) -> Result<(), Box<dyn Error>> {
    let user_source_owner = wallet.pubkey();

    let token_address = if pool_keys.base_mint == SOLC_MINT.to_string() {
        pool_keys.clone().quote_mint
    } else {
        pool_keys.clone().base_mint
    };

    let transaction_main_instructions = volume_swap_base_in(
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
        mev_ape.sol_amount,
        0,
        mev_ape.priority_fee,
        rpc_client.clone(),
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
    while start.elapsed() < Duration::from_secs(60) {
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
            info!("Token Balance: {:?}", token_balance);
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

pub async fn volume_swap_base_in(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,
    market_event_queue: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
    user_source_owner: &Pubkey,
    wallet_address: &Pubkey,
    base_mint: &Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
    priority_fee: u64,
    rpc_client: Arc<RpcClient>,
) -> Result<Vec<Instruction>, ProgramError> {
    let data = AmmInstruction::SwapBaseIn(SwapInstructionBaseIn {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(100000000);
    let compute_price = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);

    let source_token_account = get_associated_token_address(wallet_address, &SOLC_MINT);
    let destination_token_account = get_associated_token_address(wallet_address, base_mint);

    let mut instructions = Vec::new();

    instructions.push(unit_limit);
    instructions.push(compute_price);

    let _ = match rpc_client
        .get_token_accounts_by_owner(&user_source_owner, TokenAccountsFilter::Mint(*base_mint))
        .await
    {
        Ok(x) => x,
        Err(_) => {
            instructions.push(
                spl_associated_token_account::instruction::create_associated_token_account(
                    &wallet_address,
                    &wallet_address,
                    base_mint,
                    &spl_token::id(),
                ),
            );
            vec![]
        }
    };

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        // market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new(*market_bids, false),
        AccountMeta::new(*market_asks, false),
        AccountMeta::new(*market_event_queue, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
        // user
        AccountMeta::new(source_token_account, false),
        AccountMeta::new(destination_token_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    let account_swap_instructions = Instruction {
        program_id: *amm_program,
        data,
        accounts,
    };

    instructions.push(account_swap_instructions);

    Ok(instructions)
}
