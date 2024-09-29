pub mod config_init;
pub mod embeds;
pub mod wallets;

use async_recursion::async_recursion;
use solana_sdk::signature::Keypair;
use std::error::Error;
use tokio::time::sleep;

use demand::{DemandOption, Input, Select, Theme};
use log::error;
use serde::Deserialize;
use termcolor::{Color, ColorSpec};

use crate::app::embeds::embed;
use crate::copytrade::copytrade;
use crate::env::load_config;
use crate::pumpfun::pump::pump_main;
use crate::raydium_amm::swap::swap_in::{swap_in, swap_out, PriorityTip};
use crate::raydium_amm::swap::trades::track_trades;
use crate::rpc::rpc_key;
use crate::user_inputs::mode::{automatic_snipe, unwrap_sol_call, wrap_sol_call};
use crate::utils::terminal::clear_screen;

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

#[derive(Debug)]
pub struct MevApe {
    pub sol_amount: u64,
    pub fee: PriorityTip,
    // pub bundle_tip: u64,
    pub wallet: String,
}

pub fn theme() -> Theme {
    Theme {
        title: ColorSpec::new()
            .set_fg(Some(Color::Rgb(181, 228, 140)))
            .clone(),
        cursor: ColorSpec::new()
            .set_fg(Some(Color::Green))
            .set_bold(true)
            .clone(),

        selected_option: ColorSpec::new()
            .set_fg(Some(Color::Rgb(38, 70, 83)))
            .set_bold(true) // make the selected option bold
            .clone(),
        selected_prefix_fg: ColorSpec::new()
            .set_fg(Some(Color::Rgb(181, 228, 140)))
            .clone(),
        input_cursor: ColorSpec::new()
            .set_fg(Some(Color::Rgb(22, 138, 173)))
            .clone(),
        input_prompt: ColorSpec::new().set_fg(Some(Color::Blue)).clone(),
        ..Theme::default()
    }
}

#[async_recursion]
pub async fn main_menu(mainmenu: bool) -> Result<(), Box<dyn std::error::Error + Send>> {
    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(());
        }
    };

    if mainmenu {
        let _http_loader = rpc_key(args.network.rpc_url.clone()).await;
    }

    let theme = theme();
    let ms = Select::new("Main Menu")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("RaydiumAMM").label("▪ Raydium AMM Mode"))
        // .option(DemandOption::new("RaydiumCPMM").label("▪ Raydium CPMM Mode"))
        .option(DemandOption::new("PumpFun").label("▪ PumpFun Mode"))
        .option(DemandOption::new("CopyTrade").label("▪ CopyTrade Mode"))
        // .option(DemandOption::new("MoonShot").label("▪ MoonShot Mode"))
        .option(DemandOption::new("Wrap Sol Mode").label("📦 Wrap SOL"))
        .option(DemandOption::new("Unwrap Sol Mode").label("🪤  Unwrap SOL"))
        .option(DemandOption::new("Wallet Details").label("🍄 Wallet Details"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "RaydiumAMM" => {
            let _ = raydium_amm_mode().await;
        }

        "PumpFun" => {
            let _ = pump_main().await;
        }
        "CopyTrade" => {
            let _ = copytrade().await;
        }
        "Wrap Sol Mode" => {
            let _ = wrap_sol_call().await;
        }
        "Unwrap Sol Mode" => {
            let _ = unwrap_sol_call().await;
        }
        "Wallet Details" => {
            let _ = wallet_logger().await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    sleep(tokio::time::Duration::from_secs(3)).await;
    //clear the terminal
    clear_screen();
    embed();
    let _ = main_menu(false).await;

    Ok(())
}

#[async_recursion]
pub async fn raydium_amm_mode() -> Result<(), Box<dyn Error + Send>> {
    let theme = theme();
    let ms = Select::new("Swap Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Automatic Sniper").label("▪ Snipe Incoming Pools"))
        .option(DemandOption::new("Manual Sniper").label("▪ Manual Sniper"))
        .option(DemandOption::new("Buy Tokens").label("▪ Swap SOL to Tokens"))
        .option(DemandOption::new("Sell Tokens").label("▪ Swap Tokens to SOL"))
        .option(DemandOption::new("Track Trade").label("🎯 Track Token Gains"))
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
        "Manual Sniper" => {
            let _ = automatic_snipe(true).await;
        }
        "Automatic Sniper" => {
            let _ = automatic_snipe(false).await;
        }
        "Main Menu" => {
            let _ = main_menu(false).await;
        }

        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
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

fn is_valid_private_key(private_key: &str) -> bool {
    let decoded = bs58::decode(private_key)
        .into_vec()
        .unwrap_or_else(|_| vec![]);
    Keypair::from_bytes(&decoded).is_ok()
}
