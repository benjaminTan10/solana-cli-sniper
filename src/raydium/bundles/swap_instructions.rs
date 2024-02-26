use std::{str::FromStr, sync::Arc};

use jito_protos::block;
use log::error;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::{
    commitment_config::CommitmentLevel,
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction::transfer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;

use crate::raydium::{
    subscribe::PoolKeysSniper,
    swap::instructions::{
        swap_base_out, token_price_data, AmmInstruction, SwapInstructionBaseIn, SOLC_MINT,
    },
    volume_pinger::volume::{self, VolumeBotSettings},
};

use super::mev_trades::MEVBotSettings;

pub async fn swap_in_builder(
    rpc_client: Arc<RpcClient>,
    wallet: Arc<&Keypair>,
    pool_keys: PoolKeysSniper,
    settings: Arc<MEVBotSettings>,
    buy_amount: u64,
    block_hash: &Hash,
) -> (VersionedTransaction, u64) {
    let user_source_owner = wallet.pubkey();
    let tokens_amount = match token_price_data(
        rpc_client.clone(),
        pool_keys.clone(),
        wallet.clone(),
        buy_amount,
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            0
        }
    };

    let tokens_amount = tokens_amount * 999 / 1000;

    let swap_in = match volume_swap_base_in(
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
        &Pubkey::from_str(&pool_keys.base_mint).unwrap(),
        buy_amount,
        tokens_amount as u64,
        settings.priority_fee,
        rpc_client.clone(),
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            Vec::new()
        }
    };

    // let config = CommitmentLevel::Confirmed;
    // let (latest_blockhash, _) = match rpc_client
    //     .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
    //         commitment: config,
    //     })
    //     .await
    // {
    //     Ok(r) => r,
    //     Err(e) => {
    //         error!("{}", e);
    //         return (VersionedTransaction::default(), 0);
    //     }
    // };

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_in,
        &[],
        *block_hash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return (VersionedTransaction::default(), 0);
        }
    };

    let frontrun_tx = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return (VersionedTransaction::default(), 0);
        }
    };

    (frontrun_tx, tokens_amount as u64)
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

    // instructions.push(unit_limit);
    // instructions.push(compute_price);

    let token_account = match rpc_client
        .get_token_accounts_by_owner(&user_source_owner, TokenAccountsFilter::Mint(*base_mint))
        .await
    {
        Ok(x) => x,
        Err(_) => {
            error!("Error: {:?}", "No Token Account");
            return Ok(instructions);
        }
    };

    if token_account.is_empty() {
        instructions.push(
            spl_associated_token_account::instruction::create_associated_token_account(
                &wallet_address,
                &wallet_address,
                base_mint,
                &spl_token::id(),
            ),
        );
    }

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
use solana_program::hash::Hash;
pub async fn swap_base_out_bundler(
    rpc_client: Arc<RpcClient>,
    wallet: Arc<&Keypair>,
    pool_keys: PoolKeysSniper,
    settings: Arc<MEVBotSettings>,
    sell_amount: u64,
    tip_account: Pubkey,
    block_hash: &Hash,
) -> VersionedTransaction {
    let user_source_owner = wallet.pubkey();
    let mut swap_out_instructions = match swap_base_out(
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
        &Pubkey::from_str(&pool_keys.base_mint).unwrap(),
        sell_amount as u64,
        0,
        settings.priority_fee,
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            Vec::new()
        }
    };

    let tip_instruction = transfer(&user_source_owner, &tip_account, settings.bundle_tip);

    swap_out_instructions.push(tip_instruction);
    // let config = CommitmentLevel::Confirmed;
    // let (latest_blockhash, _) = match rpc_client
    //     .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
    //         commitment: config,
    //     })
    //     .await
    // {
    //     Ok(r) => r,
    //     Err(e) => {
    //         error!("{}", e);
    //         return VersionedTransaction::default();
    //     }
    // };

    let message = match solana_program::message::v0::Message::try_compile(
        &user_source_owner,
        &swap_out_instructions,
        &[],
        *block_hash,
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return VersionedTransaction::default();
        }
    };

    let backrun_tx = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(x) => x,
        Err(e) => {
            println!("Error: {:?}", e);
            return VersionedTransaction::default();
        }
    };

    backrun_tx
}
