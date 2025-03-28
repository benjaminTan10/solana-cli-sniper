// use futures::stream::StreamExt;
// use histogram::Histogram;
// use jito_protos::{
//     bundle::BundleResult,
//     convert::versioned_tx_from_packet,
//     searcher::{
//         searcher_service_client::SearcherServiceClient, ConnectedLeadersRequest,
//         NextScheduledLeaderRequest, PendingTxNotification, SendBundleResponse,
//     },
// };
// use jito_searcher_client::{
//     get_searcher_client, send_bundle_no_wait, token_authenticator::ClientInterceptor,
//     BlockEngineConnectionError,
// };
// use log::*;
// use rand::{rngs::ThreadRng, thread_rng, Rng};
// use solana_client::{
//     client_error::ClientError,
//     nonblocking::{pubsub_client::PubsubClientError, rpc_client::RpcClient},
//     rpc_response,
//     rpc_response::RpcBlockUpdate,
// };
// use solana_metrics::datapoint_info;
// use solana_sdk::{
//     clock::Slot,
//     commitment_config::{CommitmentConfig, CommitmentLevel},
//     hash::Hash,
//     message::VersionedMessage,
//     pubkey::Pubkey,
//     signature::{Keypair, Signature},
//     transaction::VersionedTransaction,
// };
// use std::{
//     collections::{hash_map::Entry, HashMap, HashSet},
//     result,
//     str::FromStr,
//     sync::{Arc, Mutex},
//     time::{Duration, Instant},
// };
// use thiserror::Error;
// use tokio::{join, sync::mpsc::Receiver, time::interval};
// use tonic::{codegen::InterceptedService, transport::Channel, Response, Status};

// use crate::{
//     env::SettingsConfig,
//     raydium_amm::{
//         bundles::{
//             mev_trades::{MEVBotSettings, POOL_KEYS},
//             swap_direction::unpack,
//             swap_instructions::{swap_base_out_bundler, swap_in_builder},
//         },
//         swap::instructions::AmmInstruction,
//     },
// };

// #[derive(Debug, Error)]
// pub enum BackrunError {
//     #[error("TonicError {0}")]
//     TonicError(#[from] tonic::transport::Error),
//     #[error("GrpcError {0}")]
//     GrpcError(#[from] Status),
//     #[error("RpcError {0}")]
//     RpcError(#[from] ClientError),
//     #[error("PubSubError {0}")]
//     PubSubError(#[from] PubsubClientError),
//     #[error("BlockEngineConnectionError {0}")]
//     BlockEngineConnectionError(#[from] BlockEngineConnectionError),
//     #[error("Shutdown")]
//     Shutdown,
// }

// #[derive(Clone)]
// pub struct BundledTransactions {
//     pub mempool_txs: Vec<VersionedTransaction>,
//     pub middle_txs: Vec<VersionedTransaction>,
//     pub backrun_txs: Vec<VersionedTransaction>,
// }

// #[derive(Default)]
// pub struct BlockStats {
//     pub bundles_sent: Vec<(
//         BundledTransactions,
//         tonic::Result<Response<SendBundleResponse>>,
//     )>,
//     pub send_elapsed: u64,
//     pub send_rt_per_packet: Histogram,
// }

// type Result<T> = result::Result<T, BackrunError>;

// async fn build_bundles(
//     rpc_client: Arc<RpcClient>,
//     pending_tx_notification: PendingTxNotification,
//     keypair: &Keypair,
//     blockhash: &Hash,
//     tip_accounts: &[Pubkey],
//     rng: Arc<Mutex<ThreadRng>>,
//     message: &str,
//     preference: Arc<MEVBotSettings>,
// ) -> Vec<BundledTransactions> {
//     futures::stream::iter(pending_tx_notification.transactions.into_iter())
//         .map(|packet| {
//             let rng = Arc::clone(&rng);
//             let rpc_client = Arc::clone(&rpc_client);
//             let preference = Arc::clone(&preference);
//             let keypair = Arc::new(keypair);
//             async move {
//                 let mut rng = rng.lock().unwrap();
//                 let buy_amount = rng.gen_range(preference.min_amount..=preference.max_amount);

//                 let mempool_tx = versioned_tx_from_packet(&packet)?;
//                 info!("mempool_tx: {:?}", mempool_tx.signatures[0].to_string());

//                 let transaction_route = mempool_tx.message.instructions();

//                 let filtered_instructions: Vec<_> = transaction_route
//                     .iter()
//                     .filter(|instruction| {
//                         instruction.data.get(0) == Some(&9) && instruction.data.len() > 1
//                     })
//                     .collect();
//                 if filtered_instructions.is_empty() {
//                     warn!("No instructions with starting data 9 found");
//                     return None;
//                 }

//                 info!("Filtered Instructions: {:?}", filtered_instructions);

//                 let amount_input = match unpack(
//                     filtered_instructions
//                         .first()
//                         .map(|instruction| instruction.data.clone())
//                         .unwrap_or_else(|| vec![]),
//                 )
//                 .ok()
//                 {
//                     Some(AmmInstruction::SwapBaseIn(swap_instruction)) => {
//                         swap_instruction.amount_in
//                     }
//                     _ => {
//                         error!("Error unpacking instruction or instruction is not SwapBaseIn");
//                         return None;
//                     }
//                 };

//                 if amount_input < 500000000 {
//                     return None;
//                 }

//                 let tip_account = tip_accounts[rng.gen_range(0..tip_accounts.len())];
//                 let account_keys = mempool_tx.message.static_account_keys();
//                 info!("account_keys: {:?}", account_keys);

//                 let map = {
//                     let lock = POOL_KEYS.lock().unwrap();
//                     lock.clone()
//                 };

//                 let mut pool_keys_data = None;
//                 for key in account_keys {
//                     // Skip the iteration if the key is the one you want to ignore
//                     if key.to_string().to_lowercase()
//                         == "jup6lkbzbjs1jkkwapdhny74zcz3tluzoi5qnyvtaV4".to_lowercase()
//                     {
//                         return None;
//                     }

//                     for (pubkey, pool_keys_sniper) in map.iter() {
//                         if pool_keys_sniper.base_mint.to_string().to_lowercase()
//                             == key.to_string().to_lowercase()
//                         {
//                             pool_keys_data = Some(pool_keys_sniper.clone());
//                             break;
//                         }
//                     }
//                 }
//                 let transaction_route = match mempool_tx.message {
//                     VersionedMessage::V0(ref message) => {
//                         let wallet = &message.account_keys;
//                         info!("Raw Account Keys: {:?}", wallet);
//                     }
//                     VersionedMessage::Legacy(ref message) => {
//                         let wallet = &message.account_keys;
//                         info!("Raw Account Keys: {:?}", wallet);
//                     }

//                     _ => {
//                         return None;
//                     }
//                 };

//                 let pool_keys_data = match pool_keys_data {
//                     Some(pool_keys_data) => pool_keys_data,
//                     None => {
//                         error!("No matching pool_keys_sniper found");
//                         return None;
//                     }
//                 };

//                 // let pubkeys: Vec<Pubkey> = transaction_route
//                 //     .iter()
//                 //     .map(|instruction| {
//                 //         instruction
//                 //             .accounts
//                 //             .iter()
//                 //             .filter_map(|account_idx| account_keys.get(*account_idx as usize))
//                 //             .cloned()
//                 //             .collect::<Vec<_>>()
//                 //     })
//                 //     .flatten()
//                 //     .collect();
//                 // info!("pubkeys: {:?}", pubkeys);
//                 // let target_key =
//                 //     Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

//                 // // Find the last occurrence of the target_key
//                 // let reversed_position = pubkeys.iter().rev().position(|key| *key == target_key);

//                 // if let Some(position) = reversed_position {
//                 //     // Calculate the position in the original array
//                 //     let original_position = pubkeys.len() - position - 1;

//                 //     // Slice the array from the found position
//                 //     let swap_keys = &pubkeys[original_position..];
//                 //     warn!("swap_keys: {:?}", swap_keys);

//                 //     //Swap keys len less than 15 then return
//                 //     if swap_keys.len() < 15 {
//                 //         return None;
//                 //     }

//                 //     // Now swap_keys is a slice of Pubkey values starting from the last occurrence of target_key
//                 //     // You can use swap_keys here
//                 //     let direction = match process_swap_base_in(swap_keys, pool_keys_data.clone()) {
//                 //         Ok(r) => r,
//                 //         Err(e) => {
//                 //             error!("{}", e);
//                 //             return None;
//                 //         }
//                 //     };

//                 //     info!("direction: {:?}", direction);

//                 //     if direction == SwapDirection::Coin2PC {
//                 //         return None;
//                 //     }
//                 // } else {
//                 //     warn!("Target key not found in pubkeys");
//                 // }

//                 // info!("pubkeys: {:?}", pubkeys);

//                 let tokens_amount = 526000000;
//                 // match token_price_data(
//                 //     rpc_client.clone(),
//                 //     pool_keys_data.clone(),
//                 //     keypair,
//                 //     buy_amount,
//                 // )
//                 // .await
//                 // {
//                 //     Ok(r) => r,
//                 //     Err(e) => {
//                 //         error!("{}", e);
//                 //         0
//                 //     }
//                 // };
//                 let swap_in_future = swap_in_builder(
//                     rpc_client.clone(),
//                     keypair.clone(),
//                     pool_keys_data.clone(),
//                     preference.clone(),
//                     buy_amount,
//                     tokens_amount as u64,
//                     blockhash,
//                 );

//                 let swap_out_future = swap_base_out_bundler(
//                     rpc_client,
//                     keypair.clone(),
//                     pool_keys_data,
//                     preference.clone(),
//                     tokens_amount as u64,
//                     tip_account,
//                     blockhash,
//                 );

//                 let (swap_in, swap_out) = join!(swap_in_future, swap_out_future);

//                 // let tip_tx = VersionedTransaction::from(Transaction::new_signed_with_payer(
//                 //     &[
//                 //         build_memo(
//                 //             format!(
//                 //                 "Jeet Molesting Tip: {:?}",
//                 //                 mempool_tx.signatures[0].to_string()
//                 //             )
//                 //             .as_bytes(),
//                 //             &[],
//                 //         ),
//                 //         transfer(&keypair.pubkey(), &tip_account, preference.bundle_tip),
//                 //     ],
//                 //     Some(&keypair.pubkey()),
//                 //     &[&*keypair],
//                 //     *blockhash,
//                 // ));

//                 Some(BundledTransactions {
//                     mempool_txs: vec![swap_in],
//                     middle_txs: vec![mempool_tx],
//                     backrun_txs: vec![swap_out],
//                 })
//             }
//         })
//         .filter_map(|future| async move {
//             match future.await {
//                 Some(value) => Some(value),
//                 None => None,
//             }
//         })
//         .collect::<Vec<_>>()
//         .await
// }

// pub async fn send_bundles(
//     searcher_client: &mut SearcherServiceClient<InterceptedService<Channel, ClientInterceptor>>,
//     bundles: &[BundledTransactions],
// ) -> Result<Vec<result::Result<Response<SendBundleResponse>, Status>>> {
//     let mut futs = Vec::with_capacity(bundles.len());
//     for b in bundles {
//         let mut searcher_client = searcher_client.clone();
//         let txs = b
//             .mempool_txs
//             .clone()
//             .into_iter()
//             .chain(b.middle_txs.clone().into_iter())
//             .chain(b.backrun_txs.clone().into_iter())
//             .collect::<Vec<VersionedTransaction>>();
//         let task =
//             tokio::spawn(async move { send_bundle_no_wait(&txs, &mut searcher_client).await });
//         futs.push(task);
//     }

//     let responses = futures_util::future::join_all(futs).await;
//     let send_bundle_responses = responses.into_iter().map(|r| r.unwrap()).collect();
//     Ok(send_bundle_responses)
// }

use solana_sdk::pubkey::Pubkey;

pub fn generate_tip_accounts(tip_program_pubkey: &Pubkey) -> Vec<Pubkey> {
    let tip_pda_0 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_0"], tip_program_pubkey).0;
    let tip_pda_1 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_1"], tip_program_pubkey).0;
    let tip_pda_2 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_2"], tip_program_pubkey).0;
    let tip_pda_3 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_3"], tip_program_pubkey).0;
    let tip_pda_4 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_4"], tip_program_pubkey).0;
    let tip_pda_5 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_5"], tip_program_pubkey).0;
    let tip_pda_6 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_6"], tip_program_pubkey).0;
    let tip_pda_7 = Pubkey::find_program_address(&[b"TIP_ACCOUNT_7"], tip_program_pubkey).0;

    vec![
        tip_pda_0, tip_pda_1, tip_pda_2, tip_pda_3, tip_pda_4, tip_pda_5, tip_pda_6, tip_pda_7,
    ]
}

// async fn maintenance_tick(
//     searcher_client: &mut SearcherServiceClient<InterceptedService<Channel, ClientInterceptor>>,
//     rpc_client: &RpcClient,
//     leader_schedule: &mut HashMap<Pubkey, HashSet<Slot>>,
//     blockhash: &mut Hash,
//     regions: Vec<String>,
// ) -> Result<()> {
//     *blockhash = rpc_client
//         .get_latest_blockhash_with_commitment(CommitmentConfig {
//             commitment: CommitmentLevel::Confirmed,
//         })
//         .await?
//         .0;
//     let new_leader_schedule = searcher_client
//         .get_connected_leaders(ConnectedLeadersRequest {})
//         .await?
//         .into_inner()
//         .connected_validators
//         .iter()
//         .fold(HashMap::new(), |mut hmap, (pubkey, slot_list)| {
//             hmap.insert(
//                 Pubkey::from_str(pubkey).unwrap(),
//                 slot_list.slots.iter().cloned().collect(),
//             );
//             hmap
//         });
//     if new_leader_schedule != *leader_schedule {
//         info!("connected_validators: {:?}", new_leader_schedule.keys());
//         *leader_schedule = new_leader_schedule;
//     }

//     let next_scheduled_leader = searcher_client
//         .get_next_scheduled_leader(NextScheduledLeaderRequest { regions })
//         .await?
//         .into_inner();
//     info!(
//         "next_scheduled_leader: {} in {} slots from {}",
//         next_scheduled_leader.next_leader_identity,
//         next_scheduled_leader.next_leader_slot - next_scheduled_leader.current_slot,
//         next_scheduled_leader.next_leader_region
//     );

//     Ok(())
// }

// fn print_block_stats(
//     block_stats: &mut HashMap<Slot, BlockStats>,
//     block: rpc_response::Response<RpcBlockUpdate>,
//     leader_schedule: &HashMap<Pubkey, HashSet<Slot>>,
//     block_signatures: &mut HashMap<Slot, HashSet<Signature>>,
// ) {
//     const KEEP_SIGS_SLOTS: u64 = 20;

//     if let Some(stats) = block_stats.get(&block.context.slot) {
//         datapoint_info!(
//             "bundles-sent",
//             ("slot", block.context.slot, i64),
//             ("bundles", stats.bundles_sent.len(), i64),
//             ("total_send_elapsed_us", stats.send_elapsed, i64),
//             (
//                 "sent_rt_pp_min",
//                 stats.send_rt_per_packet.minimum().unwrap_or_default(),
//                 i64
//             ),
//             (
//                 "sent_rt_pp_max",
//                 stats.send_rt_per_packet.maximum().unwrap_or_default(),
//                 i64
//             ),
//             (
//                 "sent_rt_pp_avg",
//                 stats.send_rt_per_packet.mean().unwrap_or_default(),
//                 i64
//             ),
//             (
//                 "sent_rt_pp_p50",
//                 stats
//                     .send_rt_per_packet
//                     .percentile(50.0)
//                     .unwrap_or_default(),
//                 i64
//             ),
//             (
//                 "sent_rt_pp_p90",
//                 stats
//                     .send_rt_per_packet
//                     .percentile(90.0)
//                     .unwrap_or_default(),
//                 i64
//             ),
//             (
//                 "sent_rt_pp_p95",
//                 stats
//                     .send_rt_per_packet
//                     .percentile(95.0)
//                     .unwrap_or_default(),
//                 i64
//             ),
//         );
//     }

//     let maybe_leader = leader_schedule
//         .iter()
//         .find(|(_, slots)| slots.contains(&block.context.slot))
//         .map(|(leader, _)| leader);

//     if let Some(b) = &block.value.block {
//         if let Some(sigs) = &b.signatures {
//             let block_signatures: HashSet<Signature> = sigs
//                 .iter()
//                 .map(|s| Signature::from_str(s).unwrap())
//                 .collect();

//             // bundles that were sent before or during this slot
//             #[allow(clippy::type_complexity)]
//             let bundles_sent_before_slot: HashMap<
//                 Slot,
//                 &[(
//                     BundledTransactions,
//                     tonic::Result<Response<SendBundleResponse>>,
//                 )],
//             > = block_stats
//                 .iter()
//                 .filter(|(slot, _)| **slot <= block.context.slot)
//                 .map(|(slot, stats)| (*slot, stats.bundles_sent.as_ref()))
//                 .collect();

//             if let Some(leader) = maybe_leader {
//                 // number of bundles sent before or during this slot
//                 let num_bundles_sent: usize = bundles_sent_before_slot
//                     .values()
//                     .map(|bundles_sent| bundles_sent.len())
//                     .sum();

//                 // number of bundles where sending returned ok
//                 let num_bundles_sent_ok: usize = bundles_sent_before_slot
//                     .values()
//                     .map(|bundles_sent| {
//                         bundles_sent
//                             .iter()
//                             .filter(|(_, send_response)| send_response.is_ok())
//                             .count()
//                     })
//                     .sum();

//                 // a list of all bundles landed this slot that were sent before or during this slot
//                 let bundles_landed: Vec<(Slot, &BundledTransactions)> = bundles_sent_before_slot
//                     .iter()
//                     .flat_map(|(slot, bundles_sent_slot)| {
//                         bundles_sent_slot
//                             .iter()
//                             .filter(|(_, send_response)| send_response.is_ok())
//                             .filter_map(|(bundle_sent, _)| {
//                                 if bundle_sent
//                                     .backrun_txs
//                                     .iter()
//                                     .chain(bundle_sent.mempool_txs.iter())
//                                     .all(|tx| block_signatures.contains(&tx.signatures[0]))
//                                 {
//                                     Some((*slot, bundle_sent))
//                                 } else {
//                                     None
//                                 }
//                             })
//                     })
//                     .collect();

//                 let mempool_txs_landed_no_bundle: Vec<(Slot, &BundledTransactions)> =
//                     bundles_sent_before_slot
//                         .iter()
//                         .flat_map(|(slot, bundles_sent_slot)| {
//                             bundles_sent_slot
//                                 .iter()
//                                 .filter(|(_, send_response)| send_response.is_ok())
//                                 .filter_map(|(bundle_sent, _)| {
//                                     if bundle_sent
//                                         .mempool_txs
//                                         .iter()
//                                         .any(|tx| block_signatures.contains(&tx.signatures[0]))
//                                         && !bundle_sent
//                                             .backrun_txs
//                                             .iter()
//                                             .any(|tx| block_signatures.contains(&tx.signatures[0]))
//                                     {
//                                         Some((*slot, bundle_sent))
//                                     } else {
//                                         None
//                                     }
//                                 })
//                         })
//                         .collect();

//                 // find the min and max distance from when the bundle was sent to what block it landed in
//                 let min_bundle_send_slot = bundles_landed
//                     .iter()
//                     .map(|(slot, _)| *slot)
//                     .min()
//                     .unwrap_or(0);
//                 let max_bundle_send_slot = bundles_landed
//                     .iter()
//                     .map(|(slot, _)| *slot)
//                     .max()
//                     .unwrap_or(0);

//                 datapoint_info!(
//                     "leader-bundle-stats",
//                     ("slot", block.context.slot, i64),
//                     ("leader", leader.to_string(), String),
//                     ("block_txs", block_signatures.len(), i64),
//                     ("num_bundles_sent", num_bundles_sent, i64),
//                     ("num_bundles_sent_ok", num_bundles_sent_ok, i64),
//                     (
//                         "num_bundles_sent_err",
//                         num_bundles_sent - num_bundles_sent_ok,
//                         i64
//                     ),
//                     ("num_bundles_landed", bundles_landed.len(), i64),
//                     (
//                         "num_bundles_dropped",
//                         num_bundles_sent - bundles_landed.len(),
//                         i64
//                     ),
//                     ("min_bundle_send_slot", min_bundle_send_slot, i64),
//                     ("max_bundle_send_slot", max_bundle_send_slot, i64),
//                     (
//                         "mempool_txs_landed_no_bundle",
//                         mempool_txs_landed_no_bundle.len(),
//                         i64
//                     ),
//                 );

//                 // leaders last slot, clear everything out
//                 // might mess up metrics if leader doesn't produce a last slot or there's lots of slots
//                 // close to each other
//                 if block.context.slot % 4 == 3 {
//                     block_stats.clear();
//                 }
//             } else {
//                 // figure out how many transactions in bundles landed in slots other than our leader
//                 let num_mempool_txs_landed: usize = bundles_sent_before_slot
//                     .values()
//                     .map(|bundles| {
//                         bundles
//                             .iter()
//                             .filter(|(bundle, _)| {
//                                 bundle
//                                     .mempool_txs
//                                     .iter()
//                                     .any(|tx| block_signatures.contains(&tx.signatures[0]))
//                             })
//                             .count()
//                     })
//                     .sum();
//                 if num_mempool_txs_landed > 0 {
//                     datapoint_info!(
//                         "non-leader-bundle-stats",
//                         ("slot", block.context.slot, i64),
//                         ("mempool_txs_landed", num_mempool_txs_landed, i64),
//                     );
//                 }
//             }
//         }
//     }

//     if let Some(b) = &block.value.block {
//         if let Some(sigs) = &b.signatures {
//             block_signatures.insert(
//                 block.context.slot,
//                 sigs.iter()
//                     .map(|s| Signature::from_str(s).unwrap())
//                     .collect(),
//             );
//         }
//     }

//     // throw away signatures for slots > KEEP_SIGS_SLOTS old
//     block_signatures.retain(|slot, _| *slot > block.context.slot - KEEP_SIGS_SLOTS);
// }

// #[allow(clippy::too_many_arguments)]
// async fn run_searcher_loop(
//     block_engine_url: String,
//     auth_keypair: Arc<Keypair>,
//     keypair: &Keypair,
//     rpc_url: String,
//     regions: Vec<String>,
//     message: String,
//     tip_program_pubkey: Pubkey,
//     preferences: Arc<MEVBotSettings>,
//     mut slot_receiver: Receiver<Slot>,
//     mut block_receiver: Receiver<rpc_response::Response<RpcBlockUpdate>>,
//     mut bundle_results_receiver: Receiver<BundleResult>,
//     mut pending_tx_receiver: Receiver<PendingTxNotification>,
// ) -> Result<()> {
//     let mut leader_schedule: HashMap<Pubkey, HashSet<Slot>> = HashMap::new();
//     let mut block_stats: HashMap<Slot, BlockStats> = HashMap::new();
//     let mut block_signatures: HashMap<Slot, HashSet<Signature>> = HashMap::new();

//     let mut searcher_client = get_searcher_client(&block_engine_url, &auth_keypair).await?;

//     let rng = Arc::new(Mutex::new(thread_rng()));

//     let tip_accounts = generate_tip_accounts(&tip_program_pubkey);
//     info!("tip accounts: {:?}", tip_accounts);

//     let rpc_client = Arc::new(RpcClient::new(rpc_url));
//     let mut blockhash = rpc_client
//         .get_latest_blockhash_with_commitment(CommitmentConfig {
//             commitment: CommitmentLevel::Confirmed,
//         })
//         .await?
//         .0;

//     let mut highest_slot = 0;
//     let mut is_leader_slot = false;

//     let mut tick = interval(Duration::from_secs(5));
//     loop {
//         tokio::select! {
//             _ = tick.tick() => {
//                 maintenance_tick(&mut searcher_client, &rpc_client, &mut leader_schedule, &mut blockhash, regions.clone()).await?;
//             }
//             maybe_bundle_result = bundle_results_receiver.recv() => {
//                 let bundle_result: BundleResult = maybe_bundle_result.ok_or(BackrunError::Shutdown)?;
//                 info!("received bundle_result: [bundle_id={:?}, result={:?}]", bundle_result.bundle_id, bundle_result.result);
//             }
//             maybe_pending_tx_notification = pending_tx_receiver.recv() => {
//                 // block engine starts forwarding a few slots early, for super high activity accounts
//                 // it might be ideal to wait until the leader slot is up
//                 if is_leader_slot {
//                     let pending_tx_notification = maybe_pending_tx_notification.ok_or(BackrunError::Shutdown)?;
//                     let bundles = build_bundles(rpc_client.clone(),pending_tx_notification, keypair, &blockhash, &tip_accounts, Arc::clone(&rng), &message, preferences.clone()).await;
//                     if !bundles.is_empty() {
//                         let now = Instant::now();
//                         let results = send_bundles(&mut searcher_client, &bundles,).await?;
//                         let send_elapsed = now.elapsed().as_micros() as u64;
//                         let send_rt_pp_us = send_elapsed / bundles.len() as u64;

//                         match block_stats.entry(highest_slot) {
//                             Entry::Occupied(mut entry) => {
//                                 let stats = entry.get_mut();
//                                 stats.bundles_sent.extend(bundles.into_iter().zip(results.into_iter()));
//                                 stats.send_elapsed += send_elapsed;
//                                 let _ = stats.send_rt_per_packet.increment(send_rt_pp_us);
//                             }
//                             Entry::Vacant(entry) => {
//                                 let mut send_rt_per_packet = Histogram::new();
//                                 let _ = send_rt_per_packet.increment(send_rt_pp_us);
//                                 entry.insert(BlockStats {
//                                     bundles_sent: bundles.into_iter().zip(results.into_iter()).collect(),
//                                     send_elapsed,
//                                     send_rt_per_packet
//                                 });
//                             }
//                         }
//                     }
//                 }
//             }
//             maybe_slot = slot_receiver.recv() => {
//                 highest_slot = maybe_slot.ok_or(BackrunError::Shutdown)?;
//                 is_leader_slot = leader_schedule.iter().any(|(_, slots)| slots.contains(&highest_slot));
//             }
//             maybe_block = block_receiver.recv() => {
//                 let block = maybe_block.ok_or(BackrunError::Shutdown)?;
//                 print_block_stats(&mut block_stats, block, &leader_schedule, &mut block_signatures);
//             }
//         }
//     }
// }

// pub async fn backrun_jito(args: SettingsConfig, preference: Arc<MEVBotSettings>) -> Result<()> {
//     // let payer_keypair = Arc::new(Keypair::from_base58_string(&preference.wallet));
//     // let auth_keypair = Arc::new(Keypair::from_bytes(&args.auth_keypair).unwrap());
//     // // info!(
//     // //     "Accounts: {:?}",
//     // //     args.backrun_accounts.iter().map(|a| a).collect::<Vec<_>>()
//     // // );
//     // set_host_id(auth_keypair.pubkey().to_string());

//     // let (slot_sender, slot_receiver) = channel(100);
//     // let (block_sender, block_receiver) = channel(100);
//     // let (bundle_results_sender, bundle_results_receiver) = channel(100);
//     // let (pending_tx_sender, pending_tx_receiver) = channel(100);

//     // tokio::spawn(slot_subscribe_loop(args.pubsub_url.clone(), slot_sender));
//     // tokio::spawn(block_subscribe_loop(args.pubsub_url.clone(), block_sender));
//     // tokio::spawn(pending_tx_loop(
//     //     args.network.block_engine_url.clone(),
//     //     auth_keypair.clone(),
//     //     pending_tx_sender,
//     //     args.backrun_accounts.clone(),
//     // ));

//     // if args.subscribe_bundle_results {
//     //     tokio::spawn(bundle_results_loop(
//     //         args.network.block_engine_url.clone(),
//     //         auth_keypair.clone(),
//     //         bundle_results_sender,
//     //     ));
//     // }

//     // let result = run_searcher_loop(
//     //     args.network.block_engine_url,
//     //     auth_keypair,
//     //     &payer_keypair,
//     //     args.network.rpc_url,
//     //     args.regions,
//     //     args.message,
//     //     args.tip_program_id,
//     //     preference,
//     //     slot_receiver,
//     //     block_receiver,
//     //     bundle_results_receiver,
//     //     pending_tx_receiver,
//     // )
//     // .await;
//     // error!("searcher loop exited result: {result:?}");

//     Ok(())
// }
