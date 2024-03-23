pub mod embeds;
pub mod wallets;
use std::error::Error;

use demand::{DemandOption, Input, Select, Theme};
use log::{error, info};
use serde::Deserialize;
use termcolor::{Color, ColorSpec};

use crate::raydium::bundles::mev_trades::mev_trades;
use crate::raydium::swap::swap_in::{swap_in, swap_out};
use crate::raydium::volume_pinger::volume::generate_volume;
use crate::user_inputs::mode::{automatic_snipe, wrap_sol_call};

use self::{embeds::embed, wallets::wallet_logger};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct UserData {
    pub module: String,
    pub platform: String,
    pub mode: String,
    pub wallet: String,
    #[serde(rename = "in")]
    pub tokenIn: String,
    #[serde(rename = "out")]
    pub tokenOut: String,
    pub amount_sol: f64,
    pub max_tx: f64,
    pub tx_delay: f64,
    pub priority_fee: f64,
    pub ms_before_drop: f64,
    pub autosell_take_profit: f64,
    pub autosell_stop_loss: f64,
    pub autosell_percent: f64,
    pub autosell_ms: f64,
}

pub fn theme() -> Theme {
    Theme {
        title: ColorSpec::new()
            .set_fg(Some(Color::Rgb(181, 228, 140)))
            .clone(),
        cursor: ColorSpec::new().set_fg(Some(Color::Yellow)).clone(),
        selected_option: ColorSpec::new()
            .set_fg(Some(Color::Rgb(38, 70, 83)))
            .clone(),
        input_cursor: ColorSpec::new()
            .set_fg(Some(Color::Rgb(22, 138, 173)))
            .clone(),
        input_prompt: ColorSpec::new().set_fg(Some(Color::Blue)).clone(),

        ..Theme::default()
    }
}

pub async fn app() -> Result<(), Box<dyn std::error::Error>> {
    info!("{}", embed());
    let theme = theme();
    let ms = Select::new("Main Menu")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Wrap Sol Mode").label("📦 Wrap SOL"))
        .option(DemandOption::new("MEV Trades").label("[1] Sandwich Mode (Depricated)"))
        .option(DemandOption::new("Swap Tokens").label("[2] Swap Mode"))
        .option(DemandOption::new("Generate Volume").label("[3] Spam Volume"))
        .option(DemandOption::new("Snipe Pools").label("[4] Snipe Mode"))
        .option(DemandOption::new("Wallet Details").label("[5] Wallet Details"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Wrap Sol Mode" => {
            let _ = wrap_sol_call().await;
        }
        "Snipe Pools" => {
            let _ = sniper_mode().await;
        }
        "Wallet Details" => {
            let _ = wallet_logger().await;
        }
        "Generate Volume" => {
            let _ = match generate_volume().await {
                Ok(_) => info!("Volume Sent"),
                Err(e) => error!("{}", e),
            };
        }
        "Swap Tokens" => {
            let _ = swap_mode().await;
        }
        "MEV Trades" => {
            let _ = mev_trades().await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub async fn swap_mode() -> Result<(), Box<dyn Error>> {
    let theme = theme();
    let ms = Select::new("Swap Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Buy Tokens").label("[1] Buy Tokens"))
        .option(DemandOption::new("Sell Tokens").label("[2] Sell Tokens"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Buy Tokens" => {
            let _ = swap_in().await;
        }
        "Sell Tokens" => {
            let _ = swap_out().await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub async fn private_key_env() -> Result<String, Box<dyn Error>> {
    let t = Input::new("Private Key: ")
        .placeholder("5eSB1...vYF49")
        .prompt("Input: ");

    let private_key = t.run().expect("error running input");

    Ok(private_key)
}

pub async fn sniper_mode() -> Result<(), Box<dyn Error>> {
    let theme = theme();
    let ms = Select::new("Sniper Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Automatic Sniper").label("[1] Set Automatic Snipe"))
        .option(DemandOption::new("Manual Sniper").label("[2] Set Manual Snipe"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Manual Sniper" => {
            let _ = automatic_snipe(false).await;
        }
        "Automatic Sniper" => {
            let _ = automatic_snipe(true).await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct MevApe {
    pub sol_amount: u64,
    pub priority_fee: u64,
    // pub bundle_tip: u64,
    pub wallet: String,
}
