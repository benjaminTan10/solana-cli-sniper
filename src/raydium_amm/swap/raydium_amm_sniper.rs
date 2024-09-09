use {
    super::raydium_swap_in::raydium_in,
    crate::{
        app::MevApe,
        env::load_config,
        raydium_amm::{
            sniper::utils::{market_authority, MARKET_STATE_LAYOUT_V3, SPL_MINT_LAYOUT},
            subscribe::PoolKeysSniper,
            swap::metadata::decode_metadata,
        },
    },
    chrono::{LocalResult, TimeZone, Utc},
    colorize::AnsiColor,
    crossterm::style::Stylize,
    futures::{channel::mpsc::SendError, sink::SinkExt, Sink},
    log::{error, info, warn},
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    solana_sdk::{program_pack::Pack, signature::Keypair},
    spl_token::state::Mint,
    std::{
        io::{self, Write},
        sync::Arc,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::time::sleep,
    yellowstone_grpc_proto::{
        geyser::SubscribeUpdateTransaction,
        prelude::{CommitmentLevel, SubscribeRequest},
        solana::storage::confirmed_block::CompiledInstruction,
    },
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Args {
    /// Service endpoint
    endpoint: String,

    x_token: Option<String>,

    /// Commitment level: processed, confirmed or finalized
    commitment: Option<ArgsCommitment>,

    /// Filter vote transactions
    vote: Option<bool>,

    /// Filter failed transactions
    failed: Option<bool>,

    /// Filter by transaction signature
    signature: Option<String>,

    /// Filter included account in transactions
    account_include: Vec<String>,

    /// Filter excluded account in transactions
    account_exclude: Vec<String>,

    /// Filter required account in transactions
    account_required: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
enum ArgsCommitment {
    #[default]
    Processed,
    Confirmed,
    Finalized,
}

impl From<ArgsCommitment> for CommitmentLevel {
    fn from(commitment: ArgsCommitment) -> Self {
        match commitment {
            ArgsCommitment::Processed => CommitmentLevel::Processed,
            ArgsCommitment::Confirmed => CommitmentLevel::Confirmed,
            ArgsCommitment::Finalized => CommitmentLevel::Finalized,
        }
    }
}

pub const RAYDIUM_AMM_V4_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

pub const RAYDIUM_AMM_FEE_COLLECTOR: &str = "7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5";

pub async fn raydium_sniper_parser(
    rpc_client: Arc<RpcClient>,
    tx: SubscribeUpdateTransaction,
    manual_snipe: bool,
    base_mint: Option<Pubkey>,
    mev_ape: Arc<MevApe>,
    mut subscribe_tx: tokio::sync::MutexGuard<
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

    let mut pool_keys = Vec::new();

    for item in outer_instructions
        .iter()
        .cloned()
        .chain(inner_instructions.iter().cloned())
    {
        if accounts[item.program_id_index as usize] != RAYDIUM_AMM_V4_PROGRAM_ID {
            continue;
        }

        if item.data[0] != 1 {
            continue;
        }

        let key_index: Vec<u8> = item.accounts.iter().map(|b| *b).collect();

        let account_keys = vec![
            accounts[key_index[8] as usize],
            accounts[key_index[9] as usize],
            accounts[key_index[16] as usize],
        ];

        println!("Account Keys: {:#?}", account_keys);

        let account_infos = match rpc_client.get_multiple_accounts(&account_keys).await {
            Ok(a) => a,
            Err(e) => {
                error!("Error: {:?}", e);
                return Ok(());
            }
        };
        let base_mint = &account_infos[0];
        let quote_mint = &account_infos[1];
        let market_info = &account_infos[2];

        let base_mint_info = match base_mint {
            Some(basemintinfo) => match SPL_MINT_LAYOUT::decode(&mut &basemintinfo.data[..]) {
                Ok(basemintinfo) => basemintinfo,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            },
            None => {
                return Ok(());
            }
        };

        let quote_mint_info = match quote_mint {
            Some(quoteinfo) => match SPL_MINT_LAYOUT::decode(&mut &quoteinfo.data[..]) {
                Ok(quoteinfo) => quoteinfo,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            },
            None => {
                error!("Error: {:?}", "No Quote Mint Info");
                return Ok(());
            }
        };

        let (market_account, market_info) = match market_info {
            Some(marketinfo) => {
                let decoded_marketinfo =
                    match MARKET_STATE_LAYOUT_V3::decode(&mut &marketinfo.data[..]) {
                        Ok(marketinfo) => marketinfo,
                        Err(e) => {
                            return Ok(());
                        }
                    };
                (marketinfo, decoded_marketinfo)
            }
            None => {
                return Ok(());
            }
        };

        pool_keys.push(PoolKeysSniper {
            id: accounts[key_index[4] as usize],
            base_mint: accounts[key_index[8] as usize],
            quote_mint: accounts[key_index[9] as usize],
            lp_mint: accounts[key_index[7] as usize],
            base_decimals: base_mint_info.decimals,
            quote_decimals: quote_mint_info.decimals,
            lp_decimals: base_mint_info.decimals,
            version: 4,
            program_id: RAYDIUM_AMM_V4_PROGRAM_ID,
            authority: accounts[key_index[5] as usize],
            open_orders: accounts[key_index[6] as usize],
            target_orders: accounts[key_index[12] as usize],
            base_vault: accounts[key_index[10] as usize],
            quote_vault: accounts[key_index[11] as usize],
            withdraw_queue: Pubkey::default(),
            lp_vault: Pubkey::default(),
            market_version: 3,
            market_program_id: market_account.owner,
            market_id: accounts[key_index[16] as usize],
            market_authority: market_authority(rpc_client.clone(), market_info.quoteVault).await,
            market_base_vault: market_info.baseVault,
            market_quote_vault: market_info.quoteVault,
            market_bids: market_info.bids,
            market_asks: market_info.asks,
            market_event_queue: market_info.eventQueue,
            lookup_table_account: Pubkey::default(),
        });

        break;
    }

    // if manual_snipe && pool_keys[0].base_mint != base_mint {
    //     return Ok(());
    // } else if manual_snipe && pool_keys[0].base_mint == base_mint {
    //     let _ = subscribe_tx.close().await;
    // }

    if base_mint.is_some() {
        if pool_keys[0].base_mint != base_mint.unwrap() {
            return Ok(());
        }
    }

    let freeze_check = rpc_client.get_account_data(&pool_keys[0].base_mint).await?;

    let freeze_check = Mint::unpack(&freeze_check).unwrap();

    println!("Freeze Check: {:#?}", freeze_check);

    if freeze_check.freeze_authority.is_some() {
        info!("Freeze Authority set, skipping transaction");
        return Ok(());
    }

    // let _ = sniper_txn_in_2(pool_keys[0].clone(), open_time, mev_ape, datetime).await;

    Ok(())
}

pub async fn sniper_txn_in_2(
    pool_keys: PoolKeysSniper,
    sleep_duration: u64,
    mev_ape: Arc<MevApe>,
    datetime: chrono::DateTime<Utc>,
) -> eyre::Result<()> {
    tokio::spawn(async move {
        let (token, data) = mpl_token_metadata::accounts::Metadata::find_pda(&pool_keys.base_mint);
        let metadata = match decode_metadata(&token).await {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                error!("Error: {:?}", e);
                None
            }
        };

        let token_name = metadata
            .clone()
            .and_then(|m| Some(m.name))
            .unwrap_or_else(|| "Unknown".to_string());
        let token_symbol = metadata
            .clone()
            .and_then(|m| Some(m.symbol))
            .unwrap_or_else(|| "Unknown".to_string());

        clear_previous_line().unwrap();
        println!(
            "Name: {}\nSymbol: {}\nBase Mint: {}\nPool ID: {}",
            colorize::AnsiColor::bold(token_name.to_string()).white(),
            colorize::AnsiColor::bold(token_symbol.to_string()).b_cyan(),
            pool_keys.base_mint.to_string(),
            pool_keys.id.to_string(),
        );
        println!("Open Time: {}", datetime.to_string());
    });

    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(eyre::eyre!("Error: {:?}", e));
        }
    };

    let private_key = &mev_ape.wallet;
    let secret_key = bs58::decode(private_key.clone()).into_vec()?;

    let wallet = Keypair::from_bytes(&secret_key)?;
    let amount_in = &mev_ape.sol_amount;
    let fees = &mev_ape.fee;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs();

    let sleep_duration = if sleep_duration > current_time {
        info!(
            "Sleep Duration: {:?} Mins",
            (sleep_duration - current_time) / 60
        );
        Duration::from_secs(sleep_duration - current_time)
    } else {
        Duration::from_secs(0)
    };

    sleep(sleep_duration).await;

    let _ = match raydium_in(
        &Arc::new(wallet),
        pool_keys.clone(),
        *amount_in,
        1,
        fees.clone(),
        args,
    )
    .await
    {
        Ok(tx) => tx,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(eyre::eyre!("Error: {:?}", e));
        }
    };

    Ok(())
}

pub fn clear_previous_line() -> io::Result<()> {
    let clear_line = "\x1b[1A\x1b[2K";
    io::stdout().write_all(clear_line.as_bytes())?;
    io::stdout().flush()
}
