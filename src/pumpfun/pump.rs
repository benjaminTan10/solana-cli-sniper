use std::{error::Error, sync::Arc};

use demand::{DemandOption, Select};
use solana_sdk::signature::Keypair;

use crate::{
    app::{app, theme},
    env::load_config,
};

use super::{
    instructions::instructions::PumpFunDirection, pump_swap_in::pump_swap, sniper::pumpfun_sniper,
};

pub async fn pump_main() -> Result<(), Box<dyn Error + Send>> {
    let theme = theme();
    let ms = Select::new("Pump Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("MigrationManual").label("💠 PumpFun Migration Manual Sniper"))
        .option(DemandOption::new("MigrationAuto").label("💠 PumpFun Migration Auto Sniper"))
        .option(DemandOption::new("PumpSniperAuto").label("▪ PumpFun Coin Auto Sniper"))
        .option(DemandOption::new("PumpSniperManual").label("▪ PumpFun Coin Manual Sniper"))
        .option(DemandOption::new("PumpBuy").label("▪ Pump Swap-In"))
        .option(DemandOption::new("PumpSell").label("▪ Pump Swap-Out"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "MigrationManual" => {
            let _ = pumpfun_sniper(true, crate::router::SniperRoute::PumpFunMigration).await;
        }
        "MigrationAuto" => {
            let _ = pumpfun_sniper(false, crate::router::SniperRoute::PumpFunMigration).await;
        }
        "PumpSniperAuto" => {
            let _ = pumpfun_sniper(false, crate::router::SniperRoute::PumpFun).await;
        }
        "PumpSniperManual" => {
            let _ = pumpfun_sniper(true, crate::router::SniperRoute::PumpFun).await;
        }
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
    let settings = match load_config().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let wallet = Keypair::from_base58_string(&settings.engine.payer_keypair);

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
    let settings = match load_config().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let wallet = Keypair::from_base58_string(&settings.engine.payer_keypair);

    match pump_swap(&Arc::new(wallet), settings, PumpFunDirection::Sell).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    Ok(())
}
