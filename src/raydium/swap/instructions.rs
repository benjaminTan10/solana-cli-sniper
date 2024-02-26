use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSimulateTransactionConfig};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    message::v0::Message,
    pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::{Transaction, VersionedTransaction},
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::sync_native;
use std::{convert::TryInto, sync::Arc};
use std::{mem::size_of, str::FromStr};

use crate::raydium::{subscribe::PoolKeysSniper, utils::utils::LIQUIDITY_STATE_LAYOUT_V4};

/// Instructions supported by the AmmInfo program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum AmmInstruction {
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` (optional)AMM target orders Account, no longer used in the contract, recommended no need to add this Account.
    ///   5. `[writable]` AMM coin vault Account to swap FROM or To.
    ///   6. `[writable]` AMM pc vault Account to swap FROM or To.
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market bids Account
    ///   10. `[writable]` Market asks Account
    ///   11. `[writable]` Market event queue Account
    ///   12. `[writable]` Market coin vault Account
    ///   13. `[writable]` Market pc vault Account
    ///   14. '[]` Market vault signer Account
    ///   15. `[writable]` User source token Account.
    ///   16. `[writable]` User destination token Account.
    ///   17. `[singer]` User wallet Account
    SwapBaseIn(SwapInstructionBaseIn),
}

impl AmmInstruction {
    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            9 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::SwapBaseIn(SwapInstructionBaseIn {
                    amount_in,
                    minimum_amount_out,
                })
            }

            _ => return Err(ProgramError::InvalidInstructionData.into()),
        })
    }
    /// Packs a [AmmInstruction](enum.AmmInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::SwapBaseIn(SwapInstructionBaseIn {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(9);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
        }
        Ok(buf)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseIn {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

pub const SOLC_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const TAX_ACCOUNT: Pubkey = pubkey!("D5bBVBQDNDzroQpduEJasYL5HkvARD6TcNu3yJaeVK5W");
/// Creates a 'swap base in' instruction.
pub async fn swap_base_in(
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
    user_token_source: &Pubkey,
    user_source_owner: &Pubkey,
    wallet_address: &Pubkey,
    base_mint: &Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
    priority_fee: u64,
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

    instructions.push(
        spl_associated_token_account::instruction::create_associated_token_account(
            &wallet_address,
            &wallet_address,
            base_mint,
            &spl_token::id(),
        ),
    );

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

pub async fn swap_base_out(
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
) -> Result<Vec<Instruction>, ProgramError> {
    let data = AmmInstruction::SwapBaseIn(SwapInstructionBaseIn {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(100000000);
    let compute_price = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);

    let source_token_account = get_associated_token_address(wallet_address, base_mint);
    let destination_token_account = get_associated_token_address(wallet_address, &SOLC_MINT);

    let mut instructions = Vec::new();

    // instructions.push(unit_limit);
    // instructions.push(compute_price);

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolInfo {
    status: u64,
    coin_decimals: u32,
    pc_decimals: u32,
    lp_decimals: u32,
    pool_pc_amount: u64,
    pool_coin_amount: u64,
    pnl_pc_amount: u64,
    pnl_coin_amount: u64,
    pool_lp_supply: u64,
    pool_open_time: u64,
    amm_id: String,
}

pub async fn fetch_muliple_info(
    rpc_client: Arc<RpcClient>,
    pool_keys: PoolKeysSniper,
    wallet: Arc<&Keypair>,
) -> eyre::Result<PoolInfo> {
    let instructions = vec![make_simulate_pool_info_instruction(pool_keys.clone()).await?];

    let log =
        simulate_multiple_instruction(&rpc_client, instructions, pool_keys.clone(), wallet).await?;

    let pool_info: PoolInfo = serde_json::from_str(&log.to_string())?;

    Ok(pool_info)
}

pub async fn make_simulate_pool_info_instruction(
    pool_keys: PoolKeysSniper,
) -> Result<Instruction, ProgramError> {
    let instruction_data: [u8; 2] = [12, 0]; // 12 for instruction, 0 for simulateType

    warn!(
        "Pool Keys: {}",
        serde_json::to_string_pretty(&pool_keys).unwrap()
    );
    let keys = vec![
        AccountMeta::new_readonly(Pubkey::from_str(&pool_keys.id).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(&pool_keys.authority).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(&pool_keys.open_orders).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(&pool_keys.base_vault).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(&pool_keys.quote_vault).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(&pool_keys.lp_mint).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(&pool_keys.market_id).unwrap(), false),
        AccountMeta::new_readonly(
            Pubkey::from_str(&pool_keys.market_event_queue).unwrap(),
            false,
        ),
    ];
    let instruction = Instruction {
        program_id: Pubkey::from_str(&pool_keys.program_id).unwrap(),
        accounts: keys,
        data: instruction_data.to_vec(),
    };

    Ok(instruction)
}
pub async fn simulate_multiple_instruction(
    rpc_client: &RpcClient,
    instructions: Vec<Instruction>,
    pool_keys: PoolKeysSniper,
    wallet: Arc<&Keypair>,
) -> eyre::Result<Value> {
    let lookup =
        address_deserailizer([Pubkey::from_str(&pool_keys.lookup_table_account)?].to_vec());
    let message = Message::try_compile(
        &wallet.pubkey(),
        &instructions,
        &[lookup],
        rpc_client.get_latest_blockhash().await?,
    )?;
    let transaction = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(transaction) => transaction,
        Err(e) => {
            println!("Error: {:?}", e);
            return Err(eyre::eyre!("Error: {:?}", e));
        }
    };

    let result = rpc_client
        .simulate_transaction_with_config(
            &transaction,
            RpcSimulateTransactionConfig {
                commitment: Some(CommitmentConfig::confirmed()),
                ..RpcSimulateTransactionConfig::default()
            },
        )
        .await?;
    if let Some(logs) = result.value.logs {
        for log in logs {
            if log.starts_with("Program log: GetPoolData:") {
                let json_part = &log["Program log: GetPoolData:".len()..];
                let pool_data: Value = serde_json::from_str(json_part)?;
                return Ok(pool_data);
            }
        }
    }

    Err(eyre::eyre!("No pool data found"))
}

pub fn address_deserailizer(address_lookup: Vec<Pubkey>) -> AddressLookupTableAccount {
    let mut addresses = Vec::new();

    for address in address_lookup {
        addresses.push(address);
    }
    let address_lookup_table_account = AddressLookupTableAccount {
        key: Pubkey::new_from_array([0; 32]),
        addresses,
    };
    address_lookup_table_account
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum SwapDirection {
    /// Input token pc, output token coin
    PC2Coin = 1u64,
    /// Input token coin, output token pc
    Coin2PC = 2u64,
}
pub fn swap_token_amount_base_in(
    amount_in: u128,
    total_pc_without_take_pnl: u128,
    total_coin_without_take_pnl: u128,
    swap_direction: SwapDirection,
) -> u128 {
    let amount_out;
    match swap_direction {
        SwapDirection::Coin2PC => {
            // (x + delta_x) * (y + delta_y) = x * y
            // (coin + amount_in) * (pc - amount_out) = coin * pc
            // => amount_out = pc - coin * pc / (coin + amount_in)
            // => amount_out = ((pc * coin + pc * amount_in) - coin * pc) / (coin + amount_in)
            // => amount_out =  pc * amount_in / (coin + amount_in)
            let denominator = total_coin_without_take_pnl.checked_add(amount_in).unwrap();
            amount_out = total_pc_without_take_pnl
                .checked_mul(amount_in)
                .unwrap()
                .checked_div(denominator)
                .unwrap();
        }
        SwapDirection::PC2Coin => {
            // (x + delta_x) * (y + delta_y) = x * y
            // (pc + amount_in) * (coin - amount_out) = coin * pc
            // => amount_out = coin - coin * pc / (pc + amount_in)
            // => amount_out = (coin * pc + coin * amount_in - coin * pc) / (pc + amount_in)
            // => amount_out = coin * amount_in / (pc + amount_in)
            let denominator = total_pc_without_take_pnl.checked_add(amount_in).unwrap();
            amount_out = total_coin_without_take_pnl
                .checked_mul(amount_in)
                .unwrap()
                .checked_div(denominator)
                .unwrap();
        }
    }
    return amount_out;
}

pub async fn swap_amount_out(pool_info: PoolInfo, amount_in: u64) -> u128 {
    let swap_fee_numerator = 25 as u128;
    let swap_fee_denominator = 10000 as u128;
    let swap_fee = u128::from(amount_in)
        .checked_mul(swap_fee_numerator)
        .unwrap()
        .checked_div(swap_fee_denominator)
        .unwrap();

    let swap_in_after_deduct_fee = u128::from(amount_in).checked_sub(swap_fee).unwrap();
    let swap_amount_out = swap_token_amount_base_in(
        swap_in_after_deduct_fee,
        pool_info.pool_coin_amount.into(),
        pool_info.pool_pc_amount.into(),
        SwapDirection::Coin2PC,
    );
    return swap_amount_out;
}

pub async fn token_price_data(
    rpc_client: Arc<RpcClient>,
    pool_keys: PoolKeysSniper,
    wallet: Arc<&Keypair>,
    amount_in: u64,
) -> eyre::Result<u128> {
    let mut pool_ids = pool_keys.clone();
    if pool_keys.base_mint == SOLC_MINT.to_string() {
        pool_ids.base_mint = pool_keys.quote_mint.clone();
        pool_ids.quote_mint = pool_keys.base_mint.clone();
    }
    let pool_info = fetch_muliple_info(rpc_client, pool_ids.clone(), wallet).await?;
    // info!("Pool Info: {}", serde_json::to_string_pretty(&pool_info)?);
    let swap_amount_out = swap_amount_out(pool_info, amount_in).await;

    info!("Swap amount in: {}", amount_in);
    Ok(swap_amount_out)
}

/* ---------------------------------------------------------------- */

pub async fn wrap_sol(
    rpc_client: Arc<RpcClient>,
    wallet: &Keypair,
    amount_in: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_token_destination = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

    info!("Wrapping Sol...");
    let mut instructions = Vec::new();

    // Check if the account already exists and is owned by the SPL Token program
    if let Ok(account) = rpc_client.get_account(&user_token_destination).await {
        if account.owner != spl_token::id() {
            return Err(
                eyre::eyre!("Error: Account already exists: {}", user_token_destination).into(),
            );
        }
    } else {
        // If the account does not exist or is not owned by the SPL Token program,
        // create the account.
        instructions.push(
            spl_associated_token_account::instruction::create_associated_token_account(
                &wallet.pubkey(),
                &wallet.pubkey(),
                &SOLC_MINT,
                &spl_token::id(),
            ),
        );
    }

    instructions.push(system_instruction::transfer(
        &wallet.pubkey(),
        &user_token_destination,
        amount_in,
    ));

    let sync_native = sync_native(&spl_token::id(), &user_token_destination)?;
    instructions.push(sync_native);

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&wallet.pubkey()),
        &[wallet],
        rpc_client.get_latest_blockhash().await?,
    );

    info!("Transaction Sent: {:?}", transaction.signatures[0]);

    rpc_client
        .send_and_confirm_transaction_with_spinner(&transaction)
        .await
        .unwrap();

    Ok(())
}

/*------------------------------------------------ */
