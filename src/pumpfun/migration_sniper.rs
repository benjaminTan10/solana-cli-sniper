use {
    crate::{
        app::MevApe,
        raydium_amm::{
            sniper::utils::{market_authority, MARKET_STATE_LAYOUT_V3, SPL_MINT_LAYOUT},
            subscribe::PoolKeysSniper,
            swap::raydium_amm_sniper::{sniper_txn_in_2, RAYDIUM_AMM_V4_PROGRAM_ID},
        },
    },
    chrono::{LocalResult, TimeZone, Utc},
    futures::{channel::mpsc::SendError, Sink},
    log::{error, info, warn},
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    solana_sdk::pubkey,
    std::sync::Arc,
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

    if accounts[0] != PUMPFUN_MIGRATION_SIGNER {
        return Ok(());
    }

    let meta = info.meta.unwrap_or_default();

    let log_messages = meta.clone().log_messages;
    let open_time_match = log_messages.iter().find(|m| m.contains("open_time"));

    let open_time_split = open_time_match
        .clone()
        .and_then(|s| s.split("open_time: ").nth(1))
        .and_then(|s| s.split(',').next());

    info!("Account Keys: {:#?}", accounts);

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
            accounts[key_index[9] as usize],
            accounts[key_index[8] as usize],
            accounts[accounts.len() - 1],
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
        let mut market_info: Option<solana_sdk::account::Account> = None;

        let base_mint_info = match base_mint {
            Some(basemintinfo) => match SPL_MINT_LAYOUT::decode(&mut &basemintinfo.data[..]) {
                Ok(basemintinfo) => basemintinfo,
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            },
            None => {
                error!("Error: {:?}", "No Base Mint Info");
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

        if market_info.is_none() {
            let info = match rpc_client.get_account(&accounts[accounts.len() - 1]).await {
                Ok(marketinfo) => Some(marketinfo),
                Err(e) => {
                    error!("Error: {:?}", e);
                    return Ok(());
                }
            };

            market_info = info;
        }

        let (market_account, market_info) = match market_info {
            Some(marketinfo) => {
                let decoded_marketinfo =
                    match MARKET_STATE_LAYOUT_V3::decode(&mut &marketinfo.data[..]) {
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

        println!("Pool Keys: {:#?}", pool_keys[0].base_mint.to_string());
        break;
    }

    if base_mint.is_some() {
        if pool_keys[0].base_mint != base_mint.unwrap() {
            return Ok(());
        }
    }

    let signature = bs58::encode(&info.signature).into_string();
    println!(
        "Transaction: {}\nPool: {:?}\nBaseMint: {}\nMaker: {}",
        signature.to_string(),
        pool_keys[0].id,
        pool_keys[0].base_mint,
        accounts[0]
    );

    let _ = sniper_txn_in_2(pool_keys[0].clone(), open_time, mev_ape, datetime).await;

    Ok(())
}
