// use crate::app::{sol_amount, MevApe};
// use crate::env::env_loader::{amount_in, priority_fee, private_key};
// use crate::env::load_settings;
// use crate::raydium::sniper::utils::{market_authority, MARKET_STATE_LAYOUT_V3, SPL_MINT_LAYOUT};
// use crate::raydium::subscribe::PoolKeysSniper;
// use crate::raydium::swap::swapper::raydium_in;
// use crate::raydium::utils::parser::parse_signatures;
// use log::error;
// use rand::thread_rng;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use solana_client::nonblocking::pubsub_client::PubsubClient;
// use solana_client::nonblocking::rpc_client::{self, RpcClient};
// use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};

// use solana_program::native_token::sol_to_lamports;
// use solana_program::pubkey::Pubkey;
// use solana_sdk::commitment_config::CommitmentConfig;
// use solana_sdk::signature::Keypair;
// use solana_sdk::signer::Signer;
// use std::fs;
// use std::str::FromStr;
// use std::sync::Arc;
// use tokio::time::sleep;

// use futures::{pin_mut, StreamExt};
// use solana_transaction_status::option_serializer::OptionSerializer;
// use solana_transaction_status::{
//     EncodedTransaction, UiInstruction, UiMessage, UiParsedInstruction,
// };
// use std::time::{Duration, SystemTime, UNIX_EPOCH};

// #[derive(Debug, Deserialize)]
// #[allow(non_snake_case, dead_code)]
// pub struct ParsedInfo {
//     pub account: String,
//     pub mint: String,
//     pub source: String,
//     pub systemProgram: String,
//     pub tokenProgram: String,
//     pub wallet: String,
// }

// #[derive(Debug, Deserialize)]
// #[allow(non_snake_case, dead_code)]
// pub struct ParsedObject {
//     pub info: ParsedInfo,
//     #[serde(rename = "type")]
//     pub type_: String,
// }

// #[derive(Debug, Deserialize)]
// #[allow(non_snake_case, dead_code)]
// pub struct Info {
//     pub amount: String,
//     pub authority: String,
//     pub destination: String,
//     pub source: String,
// }

// #[derive(Debug, Deserialize)]
// #[allow(non_snake_case, dead_code)]
// pub struct ParsedObject2 {
//     pub info: Info,
//     #[serde(rename = "type")]
//     pub type_: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct Parsed19info {
//     pub account: String,
//     pub owner: String,
// }

// #[derive(Debug, Deserialize)]
// #[allow(non_snake_case, dead_code)]
// pub struct Parsed19Object {
//     pub info: Parsed19info,
//     #[serde(rename = "type")]
//     pub type_: String,
// }
// #[derive(Debug, Deserialize)]
// #[allow(non_snake_case, dead_code)]
// pub struct Parsed2Object {
//     pub info: Parsed2info,
//     #[serde(rename = "type")]
//     pub type_: String,
// }

// #[derive(Debug, Deserialize)]
// pub struct Parsed2info {
//     pub destination: String,
//     pub lamports: u64,
//     pub source: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct ParsedData {
//     pub id: String,
//     pub base_mint: String,
//     pub lp_mint: String,
//     pub decimals: u8,
//     pub authority: String,
//     pub openorders: String,
//     pub targetorders: String,
//     pub basevault: String,
//     pub quotevault: String,
//     pub message_id: String,
// }
// #[derive(Serialize)]
// pub struct MarketData {
//     pub market: String,
//     pub request_queue: String,
//     pub event_queue: String,
//     pub bids: String,
//     pub asks: String,
//     pub base_vault: String,
//     pub quote_vault: String,
//     pub base_mint: String,
//     pub quote_mint: String,
//     pub serum_signer: String,
// }
// use std::sync::atomic::{AtomicBool, Ordering};

// async fn main() -> eyre::Result<()> {
//     let rpc_client_url = rpc_key();
//     let pubsub_client = PubsubClient::new(&rpc_client_url.clone()).await?;
//     let rpc_client_url = rpc_client_url.replace("wss", "https");
//     let rpc_client = Arc::new(RpcClient::new(rpc_client_url));

//     let raydium_liquidity = vec!["7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5".to_string()];

//     // Use RpcTransactionLogsFilter and RpcTransactionLogsConfig as expected by the method
//     let filter = RpcTransactionLogsFilter::Mentions(raydium_liquidity);
//     let config = RpcTransactionLogsConfig {
//         commitment: Some(CommitmentConfig::processed()),
//     };

//     let (subscription, _cancel) = pubsub_client.logs_subscribe(filter, config).await?;

//     pin_mut!(subscription);

//     /*---------------------------Subscription ------------------------------------------ */
//     while let Some(logs_response) = subscription.next().await {
//         let current_time = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .expect("SystemTime before UNIX EPOCH!")
//             .as_millis();

//         println!("------------------------------------------------------------------------------------------");
//         println!("Signature {:?}", logs_response.value.signature);

//         let rpc_client = Arc::clone(&rpc_client);
//         tokio::spawn(async move {
//             let signature = &logs_response.value.signature;

//             let parsed_sigs = parse_signatures(&signature).await;

//             if let Some((transaction_meta, transaction)) = parsed_sigs {
//                 if let OptionSerializer::Some(inner_instructions) =
//                     transaction_meta.clone().inner_instructions.clone()
//                 {
//                     if let Some(first_instr) = inner_instructions.get(0) {
//                         if let Some(first_instr_first_acc) = first_instr.instructions.get(24) {
//                             if let UiInstruction::Parsed(UiParsedInstruction::Parsed(
//                                 parsed_instruction,
//                             )) = first_instr_first_acc
//                             {
//                                 let mut pool_infos = PoolKeysSniper::new();
//                                 if let Value::Object(parsed_object) = &parsed_instruction.parsed {
//                                     let parsed: ParsedObject = serde_json::from_value(
//                                         serde_json::Value::Object(parsed_object.clone()),
//                                     )
//                                     .unwrap();

//                                     match transaction {
//                                         EncodedTransaction::Json(ui_accounts_list) => {
//                                             let account_keys = match &ui_accounts_list.message {
//                                                 UiMessage::Parsed(parsed_message) => {
//                                                     parsed_message.account_keys.clone()
//                                                 }
//                                                 _ => {
//                                                     println!("Error: {:?}", "No Parsed Message");
//                                                     return;
//                                                 }
//                                             };

//                                             // Now you can use account_keys
//                                             let account_infos = match rpc_client
//                                                 .get_multiple_accounts(&[
//                                                     Pubkey::from_str(&account_keys[17].pubkey)
//                                                         .unwrap(),
//                                                     Pubkey::from_str(&account_keys[14].pubkey)
//                                                         .unwrap(),
//                                                     Pubkey::from_str(&account_keys[19].pubkey)
//                                                         .unwrap(),
//                                                 ])
//                                                 .await
//                                             {
//                                                 Ok(account_infos) => account_infos,
//                                                 Err(e) => {
//                                                     println!("Error: {:?}", e);
//                                                     return;
//                                                 }
//                                             };

//                                             let base_mint = &account_infos[0];
//                                             let quote = &account_infos[1];
//                                             let market = &account_infos[2];

//                                             let base_mint_info = match base_mint {
//                                                 Some(basemintinfo) => {
//                                                     match SPL_MINT_LAYOUT::decode(
//                                                         &mut &basemintinfo.data[..],
//                                                     ) {
//                                                         Ok(basemintinfo) => basemintinfo,
//                                                         Err(e) => {
//                                                             println!("Error: {:?}", e);
//                                                             return;
//                                                         }
//                                                     }
//                                                 }
//                                                 None => {
//                                                     println!("Error: {:?}", "No Base Mint Info");
//                                                     return;
//                                                 }
//                                             };

//                                             let quote_mint_info = match quote {
//                                                 Some(quoteinfo) => {
//                                                     match SPL_MINT_LAYOUT::decode(
//                                                         &mut &quoteinfo.data[..],
//                                                     ) {
//                                                         Ok(quoteinfo) => quoteinfo,
//                                                         Err(e) => {
//                                                             println!("Error: {:?}", e);
//                                                             return;
//                                                         }
//                                                     }
//                                                 }
//                                                 None => {
//                                                     println!("Error: {:?}", "No Quote Mint Info");
//                                                     return;
//                                                 }
//                                             };
//                                             let (market_account, market_info) = match market {
//                                                 Some(marketinfo) => {
//                                                     let decoded_marketinfo =
//                                                         match MARKET_STATE_LAYOUT_V3::decode(
//                                                             &mut &marketinfo.data[..],
//                                                         ) {
//                                                             Ok(marketinfo) => marketinfo,
//                                                             Err(e) => {
//                                                                 println!("Error: {:?}", e);
//                                                                 return;
//                                                             }
//                                                         };
//                                                     (marketinfo, decoded_marketinfo)
//                                                 }
//                                                 None => {
//                                                     println!("Error: {:?}", "No Market Info");
//                                                     return;
//                                                 }
//                                             };

//                                             pool_infos = PoolKeysSniper {
//                                                 id: account_keys[2].clone().pubkey,
//                                                 base_mint: account_keys[17].clone().pubkey,
//                                                 quote_mint: account_keys[14].clone().pubkey,
//                                                 lp_mint: account_keys[4].clone().pubkey,
//                                                 base_decimals: base_mint_info.decimals,
//                                                 quote_decimals: quote_mint_info.decimals,
//                                                 lp_decimals: base_mint_info.decimals,
//                                                 version: 4,
//                                                 program_id:
//                                                     "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
//                                                         .to_string(),
//                                                 authority: account_keys[16].clone().pubkey,
//                                                 open_orders: account_keys[3].clone().pubkey,
//                                                 target_orders: account_keys[7].clone().pubkey,
//                                                 base_vault: account_keys[5].clone().pubkey,
//                                                 quote_vault: account_keys[6].clone().pubkey,
//                                                 withdraw_queue: Pubkey::default().to_string(),
//                                                 lp_vault: Pubkey::default().to_string(),
//                                                 market_version: 3,
//                                                 market_program_id: market_account.owner.to_string(),
//                                                 market_id: account_keys[19].clone().pubkey,
//                                                 market_authority: market_authority(
//                                                     Arc::clone(&rpc_client),
//                                                     &market_info.quoteVault.to_string(),
//                                                 )
//                                                 .await,
//                                                 market_base_vault: market_info
//                                                     .baseVault
//                                                     .to_string(),
//                                                 market_quote_vault: market_info
//                                                     .quoteVault
//                                                     .to_string(),
//                                                 market_bids: market_info.bids.to_string(),
//                                                 market_asks: market_info.asks.to_string(),
//                                                 market_event_queue: market_info
//                                                     .eventQueue
//                                                     .to_string(),
//                                                 lookup_table_account: Pubkey::default().to_string(),
//                                             };
//                                         }
//                                         _ => println!("Transaction is not of type Accounts"),
//                                     }
//                                 }
//                                 let mut open_time = 0;

//                                 if let OptionSerializer::Some(log_messages) =
//                                     transaction_meta.log_messages.clone()
//                                 {
//                                     // Define a regex pattern to match the open_time value
//                                     let re = regex::Regex::new(r"open_time: (\d+)").unwrap();

//                                     for message in log_messages.iter() {
//                                         // Check if the message contains the open_time information
//                                         if let Some(captures) = re.captures(message) {
//                                             // Extract the open_time value from the captures
//                                             if let Some(open_time_match) = captures.get(1) {
//                                                 // Convert the captured value to a numeric type (e.g., u64)
//                                                 if let Ok(open_time_value) =
//                                                     open_time_match.as_str().parse::<u64>()
//                                                 {
//                                                     open_time = open_time_value;
//                                                     // Calculate the time to sleep
//                                                     let current_time = SystemTime::now()
//                                                         .duration_since(UNIX_EPOCH)
//                                                         .expect("SystemTime before UNIX EPOCH!")
//                                                         .as_secs();

//                                                     let sleep_duration =
//                                                         if open_time_value > current_time {
//                                                             Duration::from_secs(
//                                                                 open_time_value - current_time - 2,
//                                                             )
//                                                         } else {
//                                                             Duration::from_secs(0)
//                                                         };

//                                                     // Only send the message if there is a delay
//                                                 }
//                                             }
//                                         }
//                                     }
//                                 }

//                                 //let chat_id = msg.from().unwrap().id.to_string();
//                                 let data =
//                                     sniper_txn_in(pool_infos, rpc_client, open_time, current_time);
//                                 match data.await {
//                                     Ok(Result) => {}
//                                     Err(e) => {
//                                         println!("Error: {:?}", e);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             // }
//         });
//     }

//     Ok(())
// }

// #[derive(Deserialize)]
// pub struct Response {
//     pub message: String,
//     pub signature: Option<Vec<String>>,
//     pub error: Option<String>,
// }

// pub async fn sniper_txn_in(
//     pool_keys: PoolKeysSniper,
//     rpc_client: Arc<RpcClient>,
//     sleep_duration: u64,
//     current_time_1: u128,
// ) -> eyre::Result<()> {
//     let wallet = Arc::new(private_key());
//     let args = match load_settings().await {
//         Ok(args) => args,
//         Err(e) => {
//             error!("Error: {:?}", e);
//             return Err(eyre::eyre!("Error: {:?}", e));
//         }
//     };
//     let amount_in = sol_amount().await.unwrap();
//     let priority_fee = priority_fee();

//     let current_time = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("SystemTime before UNIX EPOCH!")
//         .as_secs();

//     let sleep_duration = if sleep_duration > current_time {
//         println!(
//             "Sleep Duration: {:?} Mins",
//             (sleep_duration - current_time) / 60
//         );
//         Duration::from_secs(sleep_duration - current_time)
//     } else {
//         Duration::from_secs(0)
//     };

//     sleep(sleep_duration).await;

//     // let swap_transaction =
//     //     raydium_in(&wallet, pool_keys.clone(), amount_in, 1, priority_fee, args).await;

//     Ok(())
// }
