pub mod embeds;
pub mod wallets;
use colored::*;
use futures::Future;
use solana_sdk::signature::Keypair;
use std::error::Error;
use std::pin::Pin;

use demand::{DemandOption, Input, Select, Theme};
use log::{error, info};
use serde::Deserialize;
use termcolor::{Color, ColorSpec};

use crate::env::load_settings;
use crate::liquidity::minter_main::raydium_creator;
use crate::raydium::bundles::mev_trades::mev_trades;
use crate::raydium::swap::swap_in::{swap_in, swap_out, PriorityTip};
use crate::raydium::swap::trades::track_trades;
use crate::rpc::rpc_key;
use crate::user_inputs::mode::{automatic_snipe, wrap_sol_call};
use crate::volume_bot::volume_menu;

use self::wallets::wallet_logger;

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

pub async fn app(mainmenu: bool) -> Result<(), Box<dyn std::error::Error>> {
    if mainmenu {
        let args = match load_settings().await {
            Ok(args) => {
                info!("{}", "Settings Loaded".bold().bright_white());
                args
            }
            Err(e) => {
                error!("Error: {:?}", e);
                return Ok(());
            }
        };

        let _http_loader = rpc_key(args.rpc_url.clone()).await;
    }

    let theme = theme();
    let ms = Select::new("Main Menu")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Wrap Sol Mode").label("📦 Wrap SOL"))
        .option(DemandOption::new("Swap Tokens").label("[1] Swap Mode"))
        .option(DemandOption::new("Snipe Pools").label("[2] Snipe Mode"))
        .option(DemandOption::new("Minter Mode").label("[4] Minter Mode"))
        // .option(DemandOption::new("Generate Volume").label("[5] Spam Volume"))
        // .option(DemandOption::new("MEV Trades").label("[4] Sandwich Mode (Depricated)"))
        .option(DemandOption::new("Wallet Details").label("[.] Wallet Details"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Wrap Sol Mode" => {
            let _ = wrap_sol_call().await;
        }
        "Swap Tokens" => {
            let _ = swap_mode().await;
        }
        "Snipe Pools" => {
            let _ = sniper_mode().await;
        }
        "Minter Mode" => {
            let _ = raydium_creator().await;
        }
        "Generate Volume" => {
            let _ = volume_menu().await;
        }
        "MEV Trades" => {
            let _ = mev_trades().await;
        }
        "Wallet Details" => {
            let _ = wallet_logger().await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub fn swap_mode() -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>> {
    Box::pin(async {
        let theme = theme();
        let ms = Select::new("Swap Mode")
            .description("Select the Mode")
            .theme(&theme)
            .filterable(true)
            .option(DemandOption::new("Buy Tokens").label("[1] Buy Tokens"))
            .option(DemandOption::new("Sell Tokens").label("[2] Sell Tokens"))
            .option(DemandOption::new("Track Trade").label("[3] Track Trade"))
            .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

        let selected_option = ms.run().expect("error running select");

        match selected_option {
            "Buy Tokens" => {
                let _ = swap_in().await;
            }
            "Sell Tokens" => {
                let _ = swap_out().await;
            }
            "Track Trade" => {
                let _ = track_trades().await;
            }
            "Main Menu" => {
                let _ = app(false).await;
            }

            _ => {
                // Handle unexpected option here
            }
        }

        Ok(())
    })
}

pub async fn private_key_env(key: &str) -> Result<String, Box<dyn Error>> {
    loop {
        let t = Input::new(key)
            .placeholder("5eSB1...vYF49")
            .prompt("Input: ");

        let private_key = t.run().expect("error running input");

        // Check if the private key is valid
        if is_valid_private_key(&private_key) {
            return Ok(private_key);
        } else {
            println!("Invalid private key. Please enter a valid private key.");
        }
    }
}

// This is a placeholder function. You should replace this with your own validation logic.
fn string_to_bytes(s: &str) -> Vec<u8> {
    s.split(',').map(|b| b.parse::<u8>().unwrap()).collect()
}

use bs58;

fn is_valid_private_key(private_key: &str) -> bool {
    let decoded = bs58::decode(private_key)
        .into_vec()
        .unwrap_or_else(|_| vec![]);
    Keypair::from_bytes(&decoded).is_ok()
}

pub fn sniper_mode() -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>> {
    Box::pin(async {
        let theme = theme();
        let ms = Select::new("Sniper Mode")
            .description("Select the Mode")
            .theme(&theme)
            .filterable(true)
            .option(DemandOption::new("Manual Sniper").label("[1] Set Manual Snipe"))
            .option(DemandOption::new("Automatic Sniper").label("[2] Set Automatic Snipe"))
            .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

        let selected_option = ms.run().expect("error running select");

        match selected_option {
            "Manual Sniper" => {
                let _ = automatic_snipe(true).await;
            }
            "Automatic Sniper" => {
                let _ = automatic_snipe(false).await;
            }
            "Main Menu" => {
                let _ = app(false).await;
            }
            _ => {
                // Handle unexpected option here
            }
        }

        Ok(())
    })
}

#[derive(Debug)]
pub struct MevApe {
    pub sol_amount: u64,
    pub fee: PriorityTip,
    // pub bundle_tip: u64,
    pub wallet: String,
}
