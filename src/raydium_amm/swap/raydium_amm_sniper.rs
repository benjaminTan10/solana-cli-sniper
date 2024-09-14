use {
    super::{
        instructions::swap_base_in,
        raydium_swap_in::{price_logger, raydium_in},
        swap_in::PriorityTip,
        swapper::auth_keypair,
    },
    crate::{
        app::{config_init::get_config, MevApe},
        env::load_config,
        instruction::instruction::{
            get_keys_for_market, AmmInstruction, InitializePoolAccounts,
            INITIALIZE_POOL_ACCOUNTS_LEN,
        },
        liquidity::utils::tip_account,
        raydium_amm::{
            pool_searcher::amm_keys::{get_market_accounts, pool_keys_fetcher},
            sniper::utils::{market_authority, SPL_MINT_LAYOUT},
            subscribe::PoolKeysSniper,
            swap::{instructions::SOLC_MINT, metadata::decode_metadata},
            utils::utils::MARKET_STATE_LAYOUT_V3,
        },
        router::SniperRoute,
        rpc::HTTP_CLIENT,
        utils::read_single_key_impl,
    },
    chrono::{DateTime, LocalResult, TimeZone, Utc},
    colorize::AnsiColor,
    crossterm::style::Stylize,
    eyre::Context,
    futures::{channel::mpsc::SendError, Sink},
    jito_protos::searcher::SubscribeBundleResultsRequest,
    jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation},
    log::{error, info, warn},
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig},
    solana_program::pubkey::Pubkey,
    solana_sdk::{
        native_token::sol_to_lamports,
        program_pack::Pack,
        signature::Keypair,
        signer::Signer,
        system_instruction::transfer,
        system_program,
        transaction::{Transaction, VersionedTransaction},
    },
    spl_token::state::Mint,
    std::{
        io::{self, Write},
        str::FromStr,
        sync::Arc,
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::{time::sleep, try_join},
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

    let _ = sniper_txn_in_2(accounts.clone(), open_time, datetime).await;

    Ok(())
}

pub async fn sniper_txn_in_2(
    pool_keys: InitializePoolAccounts,
    sleep_duration: u64,
    datetime: chrono::DateTime<Utc>,
) -> eyre::Result<()> {
    let accounts = pool_keys.clone();
    tokio::spawn(async move {
        if let Err(e) = process_pool_metadata(&accounts.clone(), &datetime).await {
            error!("Failed to process pool metadata: {}", e);
        }
    });

    let sleep_duration = calculate_sleep_duration(sleep_duration).await;
    sleep(sleep_duration).await;

    let _ = match raydium_snipe_launch(pool_keys, None, 1).await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(eyre::eyre!("Error: {:?}", e));
        }
    };

    Ok(())
}

pub fn clear_previous_line() {
    let clear_line = "\x1b[1A\x1b[2K";
    let _ = io::stdout().write_all(clear_line.as_bytes());
    let _ = io::stdout().flush();
}

async fn process_pool_metadata(
    pool_keys: &InitializePoolAccounts,
    datetime: &DateTime<Utc>,
) -> eyre::Result<()> {
    let (token, _) = mpl_token_metadata::accounts::Metadata::find_pda(&pool_keys.amm_pc_mint);

    let metadata = decode_metadata(&token)
        .await
        .context("Failed to decode metadata")?;

    let token_name = metadata.name;
    let token_symbol = metadata.symbol;

    info!("Pool Metadata:");
    info!("Name: {}", token_name.white().bold());
    info!("Symbol: {}", token_symbol.b_cyan());
    info!("Base Mint: {}", pool_keys.amm_pc_mint);
    info!("Pool ID: {}", pool_keys.amm_pool);
    info!("Open Time: {}", datetime);

    Ok(())
}

async fn calculate_sleep_duration(sleep_duration: u64) -> Duration {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs();

    if sleep_duration > current_time {
        let duration = sleep_duration - current_time;
        info!("Sleep Duration: {:?} mins", duration / 60);
        Duration::from_secs(duration)
    } else {
        warn!("Sleep duration is in the past, proceeding immediately");
        Duration::from_secs(0)
    }
}

pub async fn raydium_snipe_launch(
    pool_keys: InitializePoolAccounts,
    init_amount_in: Option<u64>,
    amount_out: u64,
) -> eyre::Result<()> {
    let config = get_config().await?;

    let wallet = Keypair::from_base58_string(&config.engine.payer_keypair);

    let amount_in = init_amount_in.unwrap_or_else(|| sol_to_lamports(config.trading.buy_amount));

    let user_source_owner = wallet.pubkey();
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };
    let mut searcher_client =
        get_searcher_client(&config.network.block_engine_url, &Arc::new(auth_keypair())).await?;

    let tip_account = tip_account();

    let rpc_client = rpc_client;

    let token_address = if pool_keys.amm_coin_mint == SOLC_MINT {
        pool_keys.clone().amm_pc_mint
    } else {
        pool_keys.clone().amm_coin_mint
    };

    let (market, market_vault_signer) = fetch_market_data(rpc_client.clone(), &pool_keys).await?;

    let swap_instructions = swap_base_in(
        &RAYDIUM_AMM_V4_PROGRAM_ID,
        &pool_keys.amm_pool,
        &pool_keys.amm_authority,
        &pool_keys.amm_open_orders,
        &pool_keys.amm_target_orders,
        &pool_keys.amm_coin_vault,
        &pool_keys.amm_pc_vault,
        &pool_keys.market_program,
        &pool_keys.market,
        &market.bids,
        &market.asks,
        &market.eventQueue,
        &market.baseVault,
        &market.quoteVault,
        &market_vault_signer,
        &user_source_owner,
        &user_source_owner,
        &user_source_owner,
        &token_address,
        amount_in.clone(),
        amount_out,
        sol_to_lamports(config.trading.priority_fee),
        config.clone(),
    )
    .await?;

    let (latest_blockhash, _) = rpc_client
        .get_latest_blockhash_with_commitment(solana_sdk::commitment_config::CommitmentConfig {
            commitment: solana_sdk::commitment_config::CommitmentLevel::Finalized,
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

    if config.engine.use_bundles {
        info!("Building Bundle");

        let tip_txn = VersionedTransaction::from(Transaction::new_signed_with_payer(
            &[transfer(
                &wallet.pubkey(),
                &tip_account,
                sol_to_lamports(config.trading.bundle_tip),
            )],
            Some(&wallet.pubkey()),
            &[&wallet],
            rpc_client.get_latest_blockhash().await.unwrap(),
        ));

        let bundle_txn = vec![transaction, tip_txn];

        let mut bundle_results_subscription = searcher_client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        let bundle = match send_bundle_with_confirmation(
            &bundle_txn,
            &rpc_client,
            &mut searcher_client,
            &mut bundle_results_subscription,
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        std::mem::drop(bundle_results_subscription);
    } else {
        info!("Sending Transaction");
        let transaction_flight = RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        };

        if config.trading.spam {
            let mut counter = 0;
            while counter < config.trading.spam_count {
                let result = match rpc_client
                    .send_transaction_with_config(&transaction, transaction_flight)
                    .await
                {
                    Ok(x) => x,
                    Err(e) => {
                        error!("Error: {:?}", e);
                        return Ok(());
                    }
                };

                info!("Transaction Sent {:?}", result);
                counter += 1;
            }
        } else {
            let result = match rpc_client
                .send_transaction_with_config(&transaction, transaction_flight)
                .await
            {
                Ok(x) => x,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            };

            info!("Transaction Sent {:?}", result);
        }
    }

    let pool_keys = match fetch_pool_keys_with_retry(
        pool_keys.amm_pool,
        Arc::clone(&rpc_client),
        10,
        Duration::from_secs(1),
    )
    .await
    {
        Ok(keys) => keys,
        Err(e) => {
            return Err(e.into());
        }
    };

    let (mut stop_tx, mut stop_rx) = tokio::sync::mpsc::channel::<()>(100);

    let pool_keys_clone = pool_keys.clone();

    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            read_single_key_impl(&mut stop_tx, pool_keys_clone)
                .await
                .unwrap();
        });
    });

    price_logger(
        &mut stop_rx,
        amount_in,
        Some(pool_keys),
        None,
        SniperRoute::RaydiumAMM,
    )
    .await;

    handle.join().unwrap();
    Ok(())
}

async fn fetch_market_data(
    rpc_client: Arc<RpcClient>,
    pool_keys: &InitializePoolAccounts,
) -> eyre::Result<(MARKET_STATE_LAYOUT_V3, Pubkey)> {
    let market_future = get_market_accounts(rpc_client.clone(), pool_keys.market);

    let market_authority_future = async {
        let market = get_market_accounts(rpc_client.clone(), pool_keys.market).await?;
        Ok(market_authority(rpc_client.clone(), market.quoteVault).await)
    };

    let (market, market_vault_signer) = try_join!(market_future, market_authority_future)?;

    Ok((market, market_vault_signer))
}

async fn fetch_pool_keys_with_retry(
    amm_pool: Pubkey,
    rpc_client: Arc<RpcClient>,
    max_retries: u32,
    retry_delay: Duration,
) -> eyre::Result<PoolKeysSniper> {
    let mut attempts = 0;
    loop {
        match pool_keys_fetcher(amm_pool, rpc_client.clone()).await {
            Ok(keys) => return Ok(keys),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Ok(PoolKeysSniper::default());
                }
                println!("Error fetching pool keys (attempt {}): {:?}", attempts, e);
                sleep(retry_delay).await;
            }
        }
    }
}
