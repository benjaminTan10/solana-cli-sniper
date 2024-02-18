use std::fs::File;

use yellowstone_grpc_proto::geyser::SubscribeRequestFilterTransactions;

use crate::yellowstoneplugin::lib::GeyserGrpcClient;
use {
    clap::Parser,
    futures::{sink::SinkExt, stream::StreamExt},
    log::info,
    std::env,
    tokio::time::{interval, Duration},
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterSlots, SubscribeRequestPing, SubscribeUpdatePong,
        SubscribeUpdateSlot,
    },
};
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct Args {
    /// Service endpoint
    endpoint: String,

    x_token: Option<String>,
}

pub async fn transaction_subscribe() -> anyhow::Result<()> {
    info!("Calling Events..");

    // Open the JSON file
    let file = File::open("settings.json")?;

    // Deserialize the JSON file into an Args object
    let args: Args = match serde_json::from_reader(file) {
        Ok(args) => args,
        Err(e) => return Err(e.into()),
    };

    let mut client = GeyserGrpcClient::connect(args.endpoint, args.x_token, None)?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;
    let raydium_liquidity = vec!["7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5".to_string()];

    futures::try_join!(
        async move {
            subscribe_tx
            .send(SubscribeRequest {
                slots: maplit::hashmap! { "".to_owned() => SubscribeRequestFilterSlots { filter_by_commitment: Some(true) } },
                commitment: Some(CommitmentLevel::Processed as i32),
                transactions: maplit::hashmap! { "".to_owned() => SubscribeRequestFilterTransactions{
                    account_include: raydium_liquidity,
                    ..Default::default()
                }},
                ..Default::default()
            })
            .await?;

            let mut timer = interval(Duration::from_secs(3));
            let mut id = 0;
            loop {
                timer.tick().await;
                id += 1;
                subscribe_tx
                    .send(SubscribeRequest {
                        ping: Some(SubscribeRequestPing { id }),
                        ..Default::default()
                    })
                    .await?;
            }
            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(())
        },
        async move {
            while let Some(message) = stream.next().await {
                info!("Response: {:?}", message);
                match message?.update_oneof.expect("valid message") {
                    // UpdateOneof::Slot(SubscribeUpdateSlot { slot, .. }) => {
                    //     info!("slot received: {slot}");
                    // }
                    // UpdateOneof::Ping(_msg) => {
                    //     info!("ping received");
                    // }
                    // UpdateOneof::Pong(SubscribeUpdatePong { id }) => {
                    //     info!("pong received: id#{id}");
                    // }
                    UpdateOneof::Transaction(tx) => {
                        println!("Txn:{:?}", tx);
                        // let entry = messages.entry(tx.slot).or_default();
                        // let sig = Signature::try_from(tx.transaction.unwrap().signature.as_slice())
                        //     .expect("valid signature from transaction")
                        //     .to_string();
                        // if let Some(timestamp) = entry.0 {
                        //     info!("received txn {} at {}", sig, timestamp);
                        // } else {
                        //     entry.1.push(sig);
                        // }
                    }
                    msg => anyhow::bail!("received unexpected message: {msg:?}"),
                }
            }
            Ok::<(), anyhow::Error>(())
        }
    )?;

    Ok(())
}
