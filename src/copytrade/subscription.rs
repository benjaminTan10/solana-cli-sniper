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
use solana_sdk::signature::Keypair;
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterTransactions,
};

use crate::{
    app::MevApe,
    copytrade::copytrading_decoder::copy_trade_sub,
    env::SettingsConfig,
    plugins::yellowstone_plugin::lib::GeyserGrpcClient,
    raydium_amm::swap::raydium_amm_sniper::clear_previous_line,
};

#[derive(PartialEq, Debug, Clone)]
pub enum SniperRoute {
    RaydiumAMM,
    RaydiumCPMM,
    PumpFun,
    PumpFunMigration,
    MoonShot,
}

pub async fn copytrading_grpc(
    mev_ape: MevApe,
    args: SettingsConfig,
    address: Vec<String>,
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
            clear_previous_line().unwrap();
            info!("Connecting to the Port{}", dots.clone().red());
            thread::sleep(Duration::from_millis(500));
        }
    });

    let endpoint = args.network.grpc_url.clone();
    let rpc_client = Arc::new(RpcClient::new(args.clone().network.rpc_url));

    let x_token = Some("00000000-0000-0000-0000-000000000000");

    let mut client = GeyserGrpcClient::connect(endpoint, x_token, None)?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;
    let private_key = &mev_ape.wallet;
    let secret_key = bs58::decode(private_key.clone()).into_vec()?;

    clear_previous_line()?;
    info!("Successfully connected to Geyser");

    // Stop the animation
    stop_animation.store(true, Ordering::Relaxed);

    // Wait for the animation thread to finish
    handle.join().unwrap();

    let wallet = Keypair::from_bytes(&secret_key)?;
    let commitment = 0;
    subscribe_tx
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: Default::default(),
                failed: Some(false),
                signature: Default::default(),
                account_include: address.into(),
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

    let subscribe_tx = Arc::new(tokio::sync::Mutex::new(subscribe_tx));

    while let Some(message) = stream.next().await {
        let rpc_client = rpc_client.clone();
        let mev_ape = Arc::clone(&mev_ape);
        let subscribe_tx = Arc::clone(&subscribe_tx);
        let args = args.clone();
        tokio::spawn(async move {
            let subscribe_tx = subscribe_tx.lock().await;
            match message {
                Ok(msg) => match msg.update_oneof {
                    Some(UpdateOneof::Transaction(tx)) => {
                        let _ = match copy_trade_sub(
                            rpc_client.clone(),
                            tx,
                            mev_ape.clone(),
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
