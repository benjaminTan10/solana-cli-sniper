use std::{error::Error, sync::Arc};

use demand::{DemandOption, Select};
use solana_sdk::signature::Keypair;

use crate::{
    app::theme,
    env::{load_settings, utils::read_keys},
    rpc::HTTP_CLIENT,
};

use super::pump_swap_in::pump_in;

pub async fn pump_main() -> Result<(), Box<dyn Error + Send>> {
    let theme = theme();
    let ms = Select::new("Pump Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("PumpBuy").label("▪ Pump In"))
        .option(DemandOption::new("Pump Sell").label("▪ Pump Out"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "PumpBuy" => {
            let _ = pump_swap_in().await;
        }
        // "Pump Out" => {
        //     let _ = pump_out().await;
        // }
        // "Main Menu" => {
        //     let _ = app(false).await;
        // }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub async fn pump_swap_in() -> eyre::Result<()> {
    let settings = match load_settings().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let wallet = Keypair::from_base58_string(&settings.payer_keypair);

    let swap = pump_in(&Arc::new(wallet), settings).await;

    println!("Press any key to exit...");
    let _ = read_keys().await;

    Ok(())
}
