use {
    super::raydium_swap_in::raydium_in,
    crate::{
        app::MevApe,
        env::{load_settings, EngineSettings},
        plugins::yellowstone_plugin::lib::GeyserGrpcClient,
        raydium::{
            sniper::utils::{market_authority, MARKET_STATE_LAYOUT_V3, SPL_MINT_LAYOUT},
            subscribe::PoolKeysSniper,
            swap::metadata::decode_metadata,
        },
    },
    chrono::{LocalResult, TimeZone, Utc},
    colorize::AnsiColor,
    crossterm::style::Stylize,
    futures::{sink::SinkExt, stream::StreamExt},
    log::{error, info, warn},
    maplit::hashmap,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::Keypair,
    std::{
        collections::HashMap,
        str::FromStr,
        sync::Arc,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::{sync::mpsc::channel, time::sleep},
    yellowstone_grpc_proto::{
        prelude::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
            SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterTransactions,
        },
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

pub async fn grpc_pair_sub(
    mev_ape: MevApe,
    args: EngineSettings,
    manual_snipe: bool,
    base_mint: Pubkey,
) -> anyhow::Result<()> {
    info!("Calling Events..");

    let endpoint = args.grpc_url.clone();
    let rpc_client = Arc::new(RpcClient::new(args.rpc_url));

    let x_token = Some("00000000-0000-0000-0000-000000000000");

    let program_id = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap();

    let mut client = GeyserGrpcClient::connect(endpoint, x_token, None)?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;
    let private_key = &mev_ape.wallet;
    let secret_key = bs58::decode(private_key.clone()).into_vec()?;

    info!("Successfully Subscribed to the stream...!");

    let wallet = Keypair::from_bytes(&secret_key)?;
    let commitment = 0;
    subscribe_tx
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: Default::default(),
                failed: Default::default(),
                signature: Default::default(),
                account_include: ["7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5".to_string()].into(),
                account_exclude: Default::default(),
                account_required: Default::default(),
            } },
            entry: HashMap::new(),
            blocks: HashMap::new(),
            blocks_meta: hashmap! { "".to_owned() => SubscribeRequestFilterBlocksMeta {} },
            commitment: Some(commitment as i32),
            accounts_data_slice: vec![],
            ping: None,
        })
        .await?;
    let mev_ape = Arc::new(mev_ape);

    while let Some(message) = stream.next().await {
        let rpc_client = rpc_client.clone();
        let mev_ape = Arc::clone(&mev_ape);
        let private_key = &mev_ape.wallet;
        let secret_key = bs58::decode(private_key.clone()).into_vec()?;

        let wallet = Keypair::from_bytes(&secret_key)?;
        tokio::spawn(async move {
            match message {
                Ok(msg) => {
                    match msg.update_oneof {
                        Some(UpdateOneof::Transaction(tx)) => {
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
                            let open_time_match =
                                log_messages.iter().find(|m| m.contains("open_time"));

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

                            match Utc.timestamp_opt(open_time_i64, 0) {
                                LocalResult::Single(datetime) => {
                                    warn!("Pool Open Time: {}", datetime);
                                }
                                _ => {
                                    warn!("Open time is out of range");
                                }
                            }

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
                                if accounts[item.program_id_index as usize] != program_id {
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
                                let account_infos =
                                    match rpc_client.get_multiple_accounts(&account_keys).await {
                                        Ok(a) => a,
                                        Err(e) => {
                                            error!("Error: {:?}", e);
                                            return Ok::<(), ()>(());
                                        }
                                    };
                                let base_mint = &account_infos[0];
                                let quote_mint = &account_infos[1];
                                let market_info = &account_infos[2];

                                let base_mint_info = match base_mint {
                                    Some(basemintinfo) => {
                                        match SPL_MINT_LAYOUT::decode(&mut &basemintinfo.data[..]) {
                                            Ok(basemintinfo) => basemintinfo,
                                            Err(e) => {
                                                error!("Error: {:?}", e);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    None => {
                                        error!("Error: {:?}", "No Base Mint Info");
                                        return Ok(());
                                    }
                                };

                                let quote_mint_info = match quote_mint {
                                    Some(quoteinfo) => {
                                        match SPL_MINT_LAYOUT::decode(&mut &quoteinfo.data[..]) {
                                            Ok(quoteinfo) => quoteinfo,
                                            Err(e) => {
                                                error!("Error: {:?}", e);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    None => {
                                        error!("Error: {:?}", "No Quote Mint Info");
                                        return Ok(());
                                    }
                                };

                                let (market_account, market_info) = match market_info {
                                    Some(marketinfo) => {
                                        let decoded_marketinfo =
                                            match MARKET_STATE_LAYOUT_V3::decode(
                                                &mut &marketinfo.data[..],
                                            ) {
                                                Ok(marketinfo) => marketinfo,
                                                Err(e) => {
                                                    error!("Error: {:?}", e);
                                                    return Ok(());
                                                }
                                            };
                                        (marketinfo, decoded_marketinfo)
                                    }
                                    None => {
                                        error!("Error: {:?}", "No Market Info");
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
                                    program_id: program_id,
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
                                    market_authority: market_authority(
                                        rpc_client.clone(),
                                        market_info.quoteVault,
                                    )
                                    .await,
                                    market_base_vault: market_info.baseVault,
                                    market_quote_vault: market_info.quoteVault,
                                    market_bids: market_info.bids,
                                    market_asks: market_info.asks,
                                    market_event_queue: market_info.eventQueue,
                                    lookup_table_account: Pubkey::default(),
                                });

                                break;
                            }

                            let _ = sniper_txn_in_2(
                                pool_keys[0].clone(),
                                open_time,
                                mev_ape,
                                manual_snipe,
                                base_mint,
                            )
                            .await;
                        }

                        _ => {}
                    }
                }
                Err(error) => {
                    error!("stream error: {error:?}");
                }
            }
            Ok(())
        });
    }

    Ok(())
}

pub async fn sniper_txn_in_2(
    pool_keys: PoolKeysSniper,
    sleep_duration: u64,
    mev_ape: Arc<MevApe>,
    manual_snipe: bool,
    base_mint: Pubkey,
) -> eyre::Result<()> {
    if manual_snipe && pool_keys.base_mint != base_mint {
        return Ok(());
    }
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

    println!(
        "---------------------------------------------------------------\n\
        Name: {}\nSymb: {}\nBase Mint: {}\nPool ID: {}\n\
        ----------------------------------------------------------------",
        colorize::AnsiColor::bold(token_name.to_string()).white(),
        colorize::AnsiColor::bold(token_name.to_string()).b_cyan(),
        pool_keys.base_mint.to_string(),
        pool_keys.id.to_string(),
    );

    let args = match load_settings().await {
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
