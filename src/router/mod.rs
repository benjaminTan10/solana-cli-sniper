use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use colorize::AnsiColor;
use futures::{SinkExt, StreamExt};
use log::{error, info};
use maplit::hashmap;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterTransactions,
};

use crate::{
    app::MevApe,
    env::SettingsConfig,
    moonshot::sniper::moonshot_parser,
    plugins::yellowstone_plugin::lib::GeyserGrpcClient,
    pumpfun::{migration_sniper::pumpfun_migration_snipe_parser, sniper::pumpfun_parser},
    raydium_amm::swap::raydium_amm_sniper::{clear_previous_line, raydium_sniper_parser},
};

#[derive(PartialEq, Debug, Clone)]
pub enum SniperRoute {
    RaydiumAMM,
    RaydiumCPMM,
    PumpFun,
    PumpFunMigration,
    MoonShot,
    Jupiter,
}

pub async fn grpc_pair_sub(
    args: SettingsConfig,
    manual_snipe: bool,
    base_mint: Option<Pubkey>,
    contract: String,
    route: SniperRoute,
) -> anyhow::Result<()> {
    // Shared atomic boolean flag to stop the animation
    let stop_animation = Arc::new(AtomicBool::new(false));
    let stop_animation_clone = Arc::clone(&stop_animation);

    // Start the dot animation in a separate thread
    let handle = thread::spawn(move || {
        let mut dots = String::new();
        let mut count = 0;
        while !stop_animation_clone.load(Ordering::Relaxed) {
            if count == 3 {
                dots.clear();
                count = 0;
            }
            dots.push('.');
            count += 1;
            clear_previous_line();

            info!("Connecting to the Port{}", dots.clone().red());
            thread::sleep(Duration::from_millis(500));
        }
    });

    let endpoint = args.network.grpc_url.clone();
    let rpc_client = Arc::new(RpcClient::new(args.clone().network.rpc_url));

    let x_token = Some("00000000-0000-0000-0000-000000000000");

    let mut client = GeyserGrpcClient::connect(endpoint, x_token, None)?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    clear_previous_line();
    info!("Successfully connected to Geyser");

    // Stop the animation
    stop_animation.store(true, Ordering::Relaxed);

    // Wait for the animation thread to finish
    handle.join().unwrap();

    let commitment = 0;
    subscribe_tx
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: Default::default(),
                failed: Default::default(),
                signature: Default::default(),
                account_include: [contract].into(),
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

    let subscribe_tx = Arc::new(tokio::sync::Mutex::new(subscribe_tx));

    while let Some(message) = stream.next().await {
        let rpc_client = rpc_client.clone();
        let subscribe_tx = Arc::clone(&subscribe_tx);
        let args = args.clone();
        let route = route.clone();
        tokio::spawn(async move {
            let subscribe_tx = subscribe_tx.lock().await;
            match message {
                Ok(msg) => match msg.update_oneof {
                    Some(UpdateOneof::Transaction(tx)) => {
                        if route == SniperRoute::RaydiumAMM {
                            let tx = tx.clone();
                            let _ = match raydium_sniper_parser(
                                rpc_client.clone(),
                                tx,
                                manual_snipe,
                                base_mint,
                                route,
                                subscribe_tx,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error: {:?}", e);
                                }
                            };
                        } else if route == SniperRoute::PumpFunMigration {
                            let tx = tx.clone();
                            let _ = match pumpfun_migration_snipe_parser(
                                rpc_client.clone(),
                                tx,
                                manual_snipe,
                                base_mint,
                                route,
                                subscribe_tx,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error: {:?}", e);
                                }
                            };
                        } else if route == SniperRoute::PumpFun {
                            let _ = match pumpfun_parser(
                                rpc_client.clone(),
                                args,
                                tx,
                                manual_snipe,
                                base_mint,
                                subscribe_tx,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error: {:?}", e);
                                }
                            };
                        } else if route == SniperRoute::MoonShot {
                            let _ = match moonshot_parser(
                                rpc_client.clone(),
                                tx,
                                manual_snipe,
                                subscribe_tx,
                            )
                            .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error: {:?}", e);
                                }
                            };
                        }
                    }
                    _ => {}
                },
                Err(error) => {
                    error!("stream error: {error:?}");
                }
            }
            Ok::<(), ()>(())
        });
    }

    Ok(())
}
