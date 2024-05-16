use std::{collections::HashMap, str::FromStr, sync::Arc};

use demand::Confirm;
use futures::{SinkExt, StreamExt};
use log::info;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcAccountInfoConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction,
    native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::freeze_account;
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

pub async fn freeze_sells() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_minter_settings().await.unwrap();
    let engine = load_settings().await.unwrap();

    let rpc_client = Arc::new(RpcClient::new(engine.rpc_url.clone()));

    let endpoint = engine.grpc_url.clone();
    let x_token = Some("00000000-0000-0000-0000-000000000000");
    let mut client = GeyserGrpcClient::connect(endpoint, x_token, None)?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    info!("Successfully Subscribed to the stream...!");

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
                account_include: [settings.token_mint].into(),
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
        let rpc_client = rpc_client.clone();
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

                        let config = RpcAccountInfoConfig {
                            commitment: Some(CommitmentConfig::processed()),
                            encoding: Some(UiAccountEncoding::Base64),
                            ..Default::default()
                        };

                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        let account_data = match rpc_client
                            .get_account_with_config(&signer_mint, config)
                            .await
                        {
                            Ok(response) => match response.value {
                                Some(account) => account,
                                None => {
                                    info!("No account data found");
                                    return;
                                }
                            },
                            Err(e) => {
                                info!("Error getting account: {:?}", e);
                                return;
                            }
                        };

                        println!("Account Data: {:?}", (account_data.data));

                        let account_data: Result<AccountData, _> =
                            bincode::deserialize(&account_data.data);

                        match account_data {
                            Ok(data) => println!("Account Data: {:?}", data),
                            Err(e) => {
                                info!("Error deserializing account data: {:?}", e);
                                return;
                            }
                        }
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
    settings: PoolDataSettings,
    engine: EngineSettings,
    signer: Pubkey,
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

    let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(80000);
    let compute_price = ComputeBudgetInstruction::set_compute_unit_price(sol_to_lamports(0.00001));

    let authority = match freeze_account(
        &spl_token::id(),
        &deployer_key.pubkey(),
        &mint,
        &deployer_key.pubkey(),
        &[&deployer_key.pubkey()],
    ) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error freezing account: {}", e);
            return Ok(());
        }
    };

    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    let txn = Transaction::new_signed_with_payer(
        &[unit_limit, compute_price, authority],
        Some(&deployer_key.pubkey()),
        &[&deployer_key],
        recent_blockhash,
    );

    loop {
        match rpc_client
            .send_and_confirm_transaction_with_spinner(&txn)
            .await
        {
            Ok(txn) => {
                println!("Transaction successful: {:?}", txn);
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                // continue;
            }
        };
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
