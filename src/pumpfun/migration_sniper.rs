use {
    crate::{
        instruction::instruction::{
            AmmInstruction, InitializePoolAccounts, INITIALIZE_POOL_ACCOUNTS_LEN,
        },
        raydium_amm::swap::{
                instructions::SOLC_MINT,
                raydium_amm_sniper::sniper_txn_in_2,
            },
        router::SniperRoute,
        utils::transaction_history::add_transaction_to_history,
    },
    chrono::{LocalResult, TimeZone, Utc},
    futures::{channel::mpsc::SendError, Sink},
    log::{info, warn},
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    solana_sdk::{program_pack::Pack, pubkey, system_program},
    spl_token::state::Mint,
    std::{str::FromStr, sync::Arc},
    yellowstone_grpc_proto::{
        geyser::SubscribeUpdateTransaction, prelude::SubscribeRequest,
        solana::storage::confirmed_block::CompiledInstruction,
    },
};

pub const PUMPFUN_MIGRATION_SIGNER: Pubkey =
    pubkey!("39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg");

pub async fn pumpfun_migration_snipe_parser(
    rpc_client: Arc<RpcClient>,
    tx: SubscribeUpdateTransaction,
    manual_snipe: bool,
    base_mint: Option<Pubkey>,
    route: SniperRoute,
    subscribe_tx: tokio::sync::MutexGuard<
        '_,
        impl Sink<SubscribeRequest, Error = SendError> + std::marker::Unpin,
    >,
) -> eyre::Result<()> {
    let info = tx.clone().transaction.unwrap_or_default();
    let accounts = info
        .transaction
        .clone()
        .unwrap_or_default()
        .message
        .unwrap_or_default()
        .account_keys
        .iter()
        .map(|i| {
            let mut array = [0; 32];
            let bytes = &i[..array.len()]; // panics if not enough data
            array.copy_from_slice(bytes);
            Pubkey::new_from_array(array)
        })
        .collect::<Vec<Pubkey>>();
    let outer_instructions = {
        let transaction = info.transaction.unwrap_or_default();
        let message = transaction.message.unwrap_or_default();
        let instructions = message.instructions.iter();
        instructions.cloned().collect::<Vec<_>>()
    };

    add_transaction_to_history(tx);

    let meta = info.meta.unwrap_or_default();

    let log_messages = meta.clone().log_messages;
    let open_time_match = log_messages.iter().find(|m| m.contains("open_time"));

    let open_time_split = open_time_match
        .clone()
        .and_then(|s| s.split("open_time: ").nth(1))
        .and_then(|s| s.split(',').next());

    let open_time = match open_time_split {
        Some(time_str) => match time_str.parse::<u64>() {
            Ok(time) => time,
            Err(_) => {
                info!("No open time found");
                0
            }
        },
        None => {
            info!("No open time found");
            0
        }
    };

    let open_time_i64: i64 = match open_time.try_into() {
        Ok(time) => time,
        Err(_) => {
            warn!("Open time is out of range");
            0
        }
    };

    info!("Account Keys: {:#?}", accounts);

    let datetime = match Utc.timestamp_opt(open_time_i64, 0) {
        LocalResult::Single(datetime) => datetime,
        LocalResult::None => {
            warn!("Open time is not available");
            Utc::now()
        }
        LocalResult::Ambiguous(_, _) => {
            warn!("Open time is out of range");
            Utc::now()
        }
    };

    let inner_instructions: Vec<CompiledInstruction> = meta
        .clone()
        .inner_instructions
        .iter()
        .flat_map(|inner| {
            inner.instructions.iter().map(|instr| CompiledInstruction {
                program_id_index: instr.program_id_index,
                accounts: instr.accounts.clone(),
                data: instr.data.clone(),
            })
        })
        .collect();

    let mut coin_args_amm: Option<crate::instruction::instruction::InitializeInstruction2> = None;
    let mut raydium_accounts: Option<InitializePoolAccounts> = None;
    let mut trade_route: Option<SniperRoute> = None;

    for (index, instructions) in outer_instructions.iter().enumerate() {
        match AmmInstruction::unpack(&instructions.data) {
            Ok(AmmInstruction::Initialize2(decode_new_coin)) => {
                coin_args_amm = Some(decode_new_coin);
                trade_route = Some(SniperRoute::RaydiumAMM);

                if instructions.accounts.len() >= INITIALIZE_POOL_ACCOUNTS_LEN {
                    raydium_accounts = Some(InitializePoolAccounts {
                        spl_token: spl_token::id(),
                        spl_associated_token_account: spl_associated_token_account::id(),
                        system_program: system_program::id(),
                        rent: Pubkey::from_str("SysvarRent111111111111111111111111111111111")?,
                        amm_pool: accounts[instructions.accounts[4] as usize],
                        amm_authority: accounts[instructions.accounts[5] as usize],
                        amm_open_orders: accounts[instructions.accounts[6] as usize],
                        amm_lp_mint: accounts[instructions.accounts[7] as usize],
                        amm_coin_mint: accounts[instructions.accounts[8] as usize],
                        amm_pc_mint: accounts[instructions.accounts[9] as usize],
                        amm_coin_vault: accounts[instructions.accounts[10] as usize],
                        amm_pc_vault: accounts[instructions.accounts[11] as usize],
                        amm_target_orders: accounts[instructions.accounts[12] as usize],
                        amm_config: accounts[instructions.accounts[13] as usize],
                        create_fee_destination: accounts[instructions.accounts[14] as usize],
                        market_program: Pubkey::from_str(
                            "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
                        )?,
                        market: accounts[instructions.accounts[16] as usize],
                        user_wallet: accounts[instructions.accounts[17] as usize],
                        user_token_coin: accounts[instructions.accounts[18] as usize],
                        user_token_pc: accounts[instructions.accounts[19] as usize],
                        user_token_lp: accounts[instructions.accounts[20] as usize],
                        // Additional fields from the previous RaydiumAmmAccounts
                    });
                }
                break;
            }
            Ok(_) => continue, // Handle other variants if needed
            Err(e) => {
                println!("{e:#?}");
                continue;
            }
        }
    }

    println!("{coin_args_amm:#?}");
    println!("{raydium_accounts:#?}");

    let mut accounts = match raydium_accounts {
        Some(accounts) => accounts,
        None => return Ok(()),
    };

    if accounts.amm_coin_mint != SOLC_MINT {
        std::mem::swap(&mut accounts.amm_coin_mint, &mut accounts.amm_pc_mint);
    }

    if let Some(base) = base_mint {
        println!("base_mint: {:?}", base);
        println!("accounts.amm_coin_mint: {:?}", accounts.amm_coin_mint);
        println!("accounts.amm_pc_mint: {:?}", accounts.amm_pc_mint);

        if accounts.amm_coin_mint == base || accounts.amm_pc_mint == base {
            println!("Base mint matches one of the AMM mints, continuing execution");
            // Function continues...
        } else {
            println!("Base mint doesn't match either AMM mint, returning early");
            return Ok(());
        }
    }

    // Rest of the function remains the same...

    let freeze_check = rpc_client.get_account_data(&accounts.amm_pc_mint).await?;

    let freeze_check = Mint::unpack(&freeze_check)
        .map_err(|e| format!("Failed to unpack Mint: {}", e))
        .unwrap();

    if freeze_check.freeze_authority.is_some() {
        info!("Freeze Authority set, skipping transaction");
        return Ok(());
    }

    let _ = sniper_txn_in_2(accounts.clone(), open_time, datetime, route).await;

    Ok(())
}
