use crate::app::UserData;
use crate::raydium::manual_sniper::sniper_txn_in;
use crate::raydium::utils::parser::parse_signatures;
use crate::raydium::utils::utils::{market_authority, MARKET_STATE_LAYOUT_V3, SPL_MINT_LAYOUT};
use crate::rpc::{wss_key, HTTP_CLIENT};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};

use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signer::Signer;
use std::str::FromStr;
use std::sync::Arc;

use futures::{pin_mut, StreamExt};
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedTransaction, UiInstruction, UiMessage, UiParsedInstruction,
};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct ParsedInfo {
    pub account: String,
    pub mint: String,
    pub source: String,
    pub systemProgram: String,
    pub tokenProgram: String,
    pub wallet: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct ParsedObject {
    pub info: ParsedInfo,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct Info {
    pub amount: String,
    pub authority: String,
    pub destination: String,
    pub source: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct ParsedObject2 {
    pub info: Info,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct Parsed19info {
    pub account: String,
    pub owner: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct Parsed19Object {
    pub info: Parsed19info,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case, dead_code)]
pub struct Parsed2Object {
    pub info: Parsed2info,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct Parsed2info {
    pub destination: String,
    pub lamports: u64,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedData {
    pub id: String,
    pub base_mint: String,
    pub lp_mint: String,
    pub decimals: u8,
    pub authority: String,
    pub openorders: String,
    pub targetorders: String,
    pub basevault: String,
    pub quotevault: String,
    pub message_id: String,
}
#[derive(Serialize)]
pub struct MarketData {
    pub market: String,
    pub request_queue: String,
    pub event_queue: String,
    pub bids: String,
    pub asks: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub serum_signer: String,
}

pub async fn auto_sniper_stream(manual_snipe: bool) -> eyre::Result<()> {
    let rpc_client_url =
        "wss://mainnet.helius-rpc.com/?api-key=d9e80c44-bc75-4139-8cc7-084cefe506c7";
    let pubsub_client = PubsubClient::new(&rpc_client_url).await?;
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let raydium_liquidity = vec!["7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5".to_string()];

    // Use RpcTransactionLogsFilter and RpcTransactionLogsConfig as expected by the method
    let filter = RpcTransactionLogsFilter::Mentions(raydium_liquidity);
    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::processed()),
    };

    let (subscription, _cancel) = pubsub_client.logs_subscribe(filter, config).await?;

    info!("Subscribed to logs...");

    pin_mut!(subscription);

    while let Some(logs_response) = subscription.next().await {
        info!("Signature {:?}", logs_response);

        let rpc_client = Arc::clone(&rpc_client);
        tokio::spawn(async move {
            let signature = &logs_response.value.signature;
            let pubkey = &logs_response.value.logs;
            info!("{}", serde_json::to_string_pretty(pubkey).unwrap());
            let parsed_sigs = parse_signatures(&signature).await;

            if let Some((transaction_meta, transaction)) = parsed_sigs {
                if let OptionSerializer::Some(inner_instructions) =
                    transaction_meta.clone().inner_instructions.clone()
                {
                    if let Some(first_instr) = inner_instructions.get(0) {
                        if let Some(first_instr_first_acc) = first_instr.instructions.get(24) {
                            if let UiInstruction::Parsed(UiParsedInstruction::Parsed(
                                parsed_instruction,
                            )) = first_instr_first_acc
                            {
                                let mut pool_infos = PoolKeysSniper::new();
                                if let Value::Object(parsed_object) = &parsed_instruction.parsed {
                                    let parsed: ParsedObject = serde_json::from_value(
                                        serde_json::Value::Object(parsed_object.clone()),
                                    )
                                    .unwrap();

                                    match transaction {
                                        EncodedTransaction::Json(ui_accounts_list) => {
                                            let account_keys = match &ui_accounts_list.message {
                                                UiMessage::Parsed(parsed_message) => {
                                                    parsed_message.account_keys.clone()
                                                }
                                                _ => {
                                                    error!("Error: {:?}", "No Parsed Message");
                                                    return;
                                                }
                                            };

                                            // Now you can use account_keys
                                            let account_infos = match rpc_client
                                                .get_multiple_accounts(&[
                                                    Pubkey::from_str(&account_keys[17].pubkey)
                                                        .unwrap(),
                                                    Pubkey::from_str(&account_keys[14].pubkey)
                                                        .unwrap(),
                                                    Pubkey::from_str(&account_keys[19].pubkey)
                                                        .unwrap(),
                                                ])
                                                .await
                                            {
                                                Ok(account_infos) => account_infos,
                                                Err(e) => {
                                                    error!("Error: {:?}", e);
                                                    return;
                                                }
                                            };

                                            let base_mint = &account_infos[0];
                                            let quote = &account_infos[1];
                                            let market = &account_infos[2];

                                            let base_mint_info = match base_mint {
                                                Some(basemintinfo) => {
                                                    match SPL_MINT_LAYOUT::decode(
                                                        &mut &basemintinfo.data[..],
                                                    ) {
                                                        Ok(basemintinfo) => basemintinfo,
                                                        Err(e) => {
                                                            error!("Error: {:?}", e);
                                                            return;
                                                        }
                                                    }
                                                }
                                                None => {
                                                    error!("Error: {:?}", "No Base Mint Info");
                                                    return;
                                                }
                                            };

                                            let quote_mint_info = match quote {
                                                Some(quoteinfo) => {
                                                    match SPL_MINT_LAYOUT::decode(
                                                        &mut &quoteinfo.data[..],
                                                    ) {
                                                        Ok(quoteinfo) => quoteinfo,
                                                        Err(e) => {
                                                            error!("Error: {:?}", e);
                                                            return;
                                                        }
                                                    }
                                                }
                                                None => {
                                                    error!("Error: {:?}", "No Quote Mint Info");
                                                    return;
                                                }
                                            };
                                            let (market_account, market_info) = match market {
                                                Some(marketinfo) => {
                                                    let decoded_marketinfo =
                                                        match MARKET_STATE_LAYOUT_V3::decode(
                                                            &mut &marketinfo.data[..],
                                                        ) {
                                                            Ok(marketinfo) => marketinfo,
                                                            Err(e) => {
                                                                error!("Error: {:?}", e);
                                                                return;
                                                            }
                                                        };
                                                    (marketinfo, decoded_marketinfo)
                                                }
                                                None => {
                                                    error!("Error: {:?}", "No Market Info");
                                                    return;
                                                }
                                            };

                                            pool_infos = PoolKeysSniper {
                                                id: Pubkey::from_str(
                                                    &account_keys[2].clone().pubkey,
                                                )
                                                .unwrap(),
                                                base_mint: Pubkey::from_str(
                                                    &account_keys[17].clone().pubkey,
                                                )
                                                .unwrap(),
                                                quote_mint: Pubkey::from_str(
                                                    &account_keys[14].clone().pubkey,
                                                )
                                                .unwrap(),
                                                lp_mint: Pubkey::from_str(
                                                    &account_keys[4].clone().pubkey,
                                                )
                                                .unwrap(),
                                                base_decimals: base_mint_info.decimals,
                                                quote_decimals: quote_mint_info.decimals,
                                                lp_decimals: base_mint_info.decimals,
                                                version: 4,
                                                program_id: Pubkey::from_str(
                                                    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
                                                )
                                                .unwrap(),
                                                authority: Pubkey::from_str(
                                                    &account_keys[16].clone().pubkey,
                                                )
                                                .unwrap(),
                                                open_orders: Pubkey::from_str(
                                                    &account_keys[3].clone().pubkey,
                                                )
                                                .unwrap(),
                                                target_orders: Pubkey::from_str(
                                                    &account_keys[7].clone().pubkey,
                                                )
                                                .unwrap(),
                                                base_vault: Pubkey::from_str(
                                                    &account_keys[5].clone().pubkey,
                                                )
                                                .unwrap(),
                                                quote_vault: Pubkey::from_str(
                                                    &account_keys[6].clone().pubkey,
                                                )
                                                .unwrap(),
                                                withdraw_queue: Pubkey::default(),
                                                lp_vault: Pubkey::default(),
                                                market_version: 3,
                                                market_program_id: market_account.owner,
                                                market_id: Pubkey::from_str(
                                                    &account_keys[19].clone().pubkey,
                                                )
                                                .unwrap(),
                                                market_authority: market_authority(
                                                    &Arc::clone(&rpc_client),
                                                    market_info.quoteVault,
                                                )
                                                .await,
                                                market_base_vault: market_info.baseVault,
                                                market_quote_vault: market_info.quoteVault,
                                                market_bids: market_info.bids,
                                                market_asks: market_info.asks,
                                                market_event_queue: market_info.eventQueue,
                                                lookup_table_account: Pubkey::default(),
                                            };
                                        }
                                        _ => println!("Transaction is not of type Accounts"),
                                    }
                                }
                                let mut open_time = 0;

                                if let OptionSerializer::Some(log_messages) =
                                    transaction_meta.log_messages.clone()
                                {
                                    // Define a regex pattern to match the open_time value
                                    let re = regex::Regex::new(r"open_time: (\d+)").unwrap();

                                    for message in log_messages.iter() {
                                        // Check if the message contains the open_time information
                                        if let Some(captures) = re.captures(message) {
                                            // Extract the open_time value from the captures
                                            if let Some(open_time_match) = captures.get(1) {
                                                // Convert the captured value to a numeric type (e.g., u64)
                                                if let Ok(open_time_value) =
                                                    open_time_match.as_str().parse::<u64>()
                                                {
                                                    open_time = open_time_value;
                                                    // Calculate the time to sleep
                                                }
                                            }
                                        }
                                    }
                                }

                                // info!(
                                //     "Pool Infos: {}",
                                //     serde_json::to_string_pretty(&pool_infos).unwrap()
                                // );

                                // let data = sniper_txn_in(pool_infos, rpc_client, open_time);
                                // match data.await {
                                //     Ok(_) => {}
                                //     Err(e) => {
                                //         error!("Error: {:?}", e);
                                //     }
                                // }
                            }
                        }
                    }
                }
            }
        });
    }

    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoolKeysSniper {
    pub id: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub lp_decimals: u8,
    pub version: u8,
    pub program_id: Pubkey,
    pub authority: Pubkey,
    pub open_orders: Pubkey,
    pub target_orders: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub withdraw_queue: Pubkey,
    pub lp_vault: Pubkey,
    pub market_version: u8,
    pub market_program_id: Pubkey,
    pub market_id: Pubkey,
    pub market_authority: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub market_bids: Pubkey,
    pub market_asks: Pubkey,
    pub market_event_queue: Pubkey,
    pub lookup_table_account: Pubkey,
}

impl PoolKeysSniper {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
