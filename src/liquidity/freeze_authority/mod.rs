use std::{collections::HashMap, str::FromStr, sync::Arc};

use demand::Confirm;
use futures::{SinkExt, StreamExt};
use jito_protos::searcher::searcher_service_client::SearcherServiceClient;
use jito_searcher_client::{send_bundle_no_wait, token_authenticator::ClientInterceptor};
use log::info;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::freeze_account;
use tonic::{service::interceptor::InterceptedService, transport::Channel};
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
        SubscribeRequestFilterTransactions,
    },
    solana::storage::confirmed_block::CompiledInstruction,
};

use crate::{
    env::{
        load_settings,
        minter::{load_minter_settings, PoolDataSettings},
        EngineSettings,
    },
    plugins::yellowstone_plugin::lib::GeyserGrpcClient,
};

use super::utils::{tip_account, tip_txn};

pub async fn freeze_sells(
    search_client: SearcherServiceClient<InterceptedService<Channel, ClientInterceptor>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let settings = Arc::new(load_minter_settings().await.unwrap());
    let engine = Arc::new(load_settings().await.unwrap());

    let endpoint = engine.grpc_url.clone();
    let x_token = Some("00000000-0000-0000-0000-000000000000");
    let mut client = GeyserGrpcClient::connect(endpoint, x_token, None)?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    info!("Successfully connected to geyser!");

    let token_mint = Pubkey::from_str(&settings.token_mint).unwrap();

    let commitment = 0;
    subscribe_tx
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: Default::default(),
                failed: Some(false),
                signature: Default::default(),
                account_include: [settings.token_mint.clone()].into(),
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

    while let Some(message) = stream.next().await {
        let settings = settings.clone();
        let engine = engine.clone();
        let client = search_client.clone();
        tokio::spawn(async move {
            match message {
                Ok(msg) => match msg.update_oneof {
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
                        let signature_base58 = bs58::encode(info.signature).into_string();

                        info!("Signature: {:?}", signature_base58);
                        let meta = info.meta.unwrap_or_default();

                        let outer_instructions = {
                            let transaction = info.transaction.unwrap_or_default();
                            let message = transaction.message.unwrap_or_default();
                            let instructions = message.instructions.iter();
                            instructions.cloned().collect::<Vec<_>>()
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
                        let mut account_keys: Vec<String> = Vec::new();

                        for item in outer_instructions
                            .iter()
                            .cloned()
                            .chain(inner_instructions.iter().cloned())
                        {
                            let key_index: Vec<u8> = item.accounts.iter().map(|b| *b).collect();

                            let keys = key_index
                                .iter()
                                .filter_map(|i| {
                                    let index = *i as usize;
                                    if index < accounts.len() {
                                        Some(accounts[index].to_string()) // Convert Pubkey to string
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<String>>();

                            account_keys.extend(keys);
                        }
                        // info!(
                        //     "Keys: {}",
                        //     serde_json::to_string_pretty(&account_keys).unwrap()
                        // );

                        let signer = match Pubkey::from_str(&account_keys[0]) {
                            Ok(signer) => signer,
                            Err(e) => {
                                info!("Error parsing signer: {:?}", e);
                                return;
                            }
                        };

                        let signer_mint = get_associated_token_address(&signer, &token_mint);

                        let _ = freeze_incoming(settings, engine, signer_mint, client).await;
                    }
                    _ => {}
                },

                Err(e) => {
                    info!("Error: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

pub async fn freeze_authority() -> Result<bool, Box<dyn std::error::Error>> {
    let confirm = Confirm::new("Freeze Accounts")
        .description(
            "All the incoming txns account will be Frozen. Are you sure you want to proceed?",
        )
        .affirmative("No")
        .negative("Yes")
        .selected(false)
        .run()
        .unwrap();

    Ok(confirm)
}

pub async fn freeze_incoming(
    settings: Arc<PoolDataSettings>,
    engine: Arc<EngineSettings>,
    signer: Pubkey,
    mut search_client: SearcherServiceClient<InterceptedService<Channel, ClientInterceptor>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_client = Arc::new(RpcClient::new(engine.rpc_url.clone()));
    let deployer_key = Keypair::from_base58_string(&settings.deployer_key);
    let mint = match Pubkey::from_str(&settings.token_mint) {
        Ok(mint) => mint,
        Err(e) => {
            eprintln!("Error parsing token mint: {}", e);
            return Ok(());
        }
    };

    // let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(80000);
    // let compute_price = ComputeBudgetInstruction::set_compute_unit_price(sol_to_lamports(0.00001));

    let tip_txn = tip_txn(
        deployer_key.pubkey(),
        tip_account(),
        sol_to_lamports(0.0001),
    );

    let authority = match freeze_account(
        &spl_token::id(),
        &signer,
        &mint,
        &deployer_key.pubkey(),
        &[],
    ) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error freezing account: {}", e);
            return Ok(());
        }
    };

    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    let versioned_msg = VersionedMessage::V0(Message::try_compile(
        &deployer_key.pubkey(),
        &[authority, tip_txn],
        &[],
        recent_blockhash,
    )?);

    let txn = VersionedTransaction::try_new(versioned_msg, &[&deployer_key])?;

    let bundle = send_bundle_no_wait(&[txn], &mut search_client).await;

    match bundle {
        Ok(response) => {
            let bundle = response.into_inner();
            // Assuming `bundle` now has a `result` field of type `bool` where `true` indicates success.
            println!("{}", bundle.uuid);
        }
        Err(e) => {
            info!("Error freezing account: {:?}", e);
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct AccountData {
    address: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Pubkey,
    isInitialized: bool,
    isFrozen: bool,
    isNative: bool,
    rentExemptReserve: u64,
    closeAuthority: Pubkey,
}

// let pubsub_client = match pubsub_client::PubsubClient::new(&engine.pubsub_url).await {
//     Ok(client) => client,
//     Err(e) => {
//         eprintln!("Error creating pubsub client: {}", e);
//         return;
//     }
// };

// let mint = match Pubkey::from_str(&settings.token_mint) {
//     Ok(mint) => mint,
//     Err(e) => {
//         eprintln!("Error parsing token mint: {}", e);
//         return;
//     }
// };

// let config = RpcAccountInfoConfig {
//     commitment: Some(CommitmentConfig::processed()),
//     encoding: Some(UiAccountEncoding::Base58),
//     ..Default::default()
// };

// let (mut subscription, _cancel) =
//     match pubsub_client.account_subscribe(&mint, Some(config)).await {
//         Ok((subscription, cancel)) => (subscription, cancel),
//         Err(e) => {
//             eprintln!("Error subscribing to account: {}", e);
//             return;
//         }
//     };

// while let Some(logs_response) = subscription.next().await {
//     info!(
//         "Signature {}",
//         serde_json::to_string_pretty(&logs_response).unwrap()
//     );
// }
