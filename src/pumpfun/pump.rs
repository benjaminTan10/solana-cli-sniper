use std::{error::Error, sync::Arc};

use demand::{DemandOption, Select};
use solana_sdk::signature::Keypair;

use crate::{
    app::{app, theme},
    env::load_settings,
};

use super::{instructions::instructions::PumpFunDirection, pump_swap_in::pump_swap};

pub async fn pump_main() -> Result<(), Box<dyn Error + Send>> {
    let theme = theme();
    let ms = Select::new("Pump Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("PumpBuy").label("▪ Pump In"))
        .option(DemandOption::new("PumpSell").label("▪ Pump Out"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "PumpBuy" => {
            let _ = pump_swap_in().await;
        }
        "PumpSell" => {
            let _ = pump_swap_out().await;
        }
        "Main Menu" => {
            let _ = app(false).await;
        }
        _ => {}
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

    let wallet = Keypair::from_base58_string(&settings.payer_keypair);

    match pump_swap(&Arc::new(wallet), settings, PumpFunDirection::Buy).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    Ok(())
}

pub async fn pump_swap_out() -> eyre::Result<()> {
    let settings = match load_settings().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let wallet = Keypair::from_base58_string(&settings.payer_keypair);

    match pump_swap(&Arc::new(wallet), settings, PumpFunDirection::Sell).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    Ok(())
}
