pub mod embeds;
pub mod wallets;
use csv;
use demand::{DemandOption, Input, Select, Theme};
use log::{error, info};
use serde::Deserialize;
use solana_program::native_token::sol_to_lamports;
use solana_program::pubkey::Pubkey;
use solana_sdk::bs58;
use solana_sdk::signature::Keypair;
use std::fs;
use std::thread::sleep;
use std::{error::Error, str::FromStr};
use termcolor::{Color, ColorSpec, WriteColor};

use crate::raydium::volume_pinger::volume::generate_volume;
use crate::raydium::{
    self,
    manual_sniper::raydium_stream,
    subscribe::auto_sniper_stream,
    swap::{
        instructions::{SOLC_MINT, USDC_MINT},
        swapper::raydium_in,
    },
};
use crate::yellowstoneplugin::tx_blocktime::txn_blocktime;

use self::{
    embeds::{embed, license_checker},
    wallets::{private_key, wallet_logger},
};

#[warn(non_snake_case)]
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

pub async fn app() -> Result<(), Box<dyn std::error::Error>> {
    info!("{}", embed());

    let theme = Theme {
        title: ColorSpec::new().set_fg(Some(Color::Blue)).clone(),
        ..Theme::default()
    };
    let ms = Select::new("Main Menu")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("[1] Wrap SOL"))
        .option(DemandOption::new("[2] Generate Volume"))
        .option(DemandOption::new("[3] Mev new Pairs"))
        .option(DemandOption::new("[4] View Wallets"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "[1] Wrap SOL" => {}
        "[2] Generate Volume" => {
            let _ = generate_volume().await;
        }
        "[3] Mev new Pairs" => {
            let _ = new_pair_mev().await;
        }
        "[4] View Wallets" => {
            let _ = wallet_logger().await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub async fn tasks_list() -> Result<(), Box<dyn Error>> {
    let mut select = Select::new(" ")
        .description("Welcome, please select a CSV file for Task")
        .filterable(true);

    let entries = fs::read_dir("./tasks/")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("csv")) {
            select = select.option(DemandOption::new(path.to_string_lossy().to_string()));
        }
    }

    let selected_option = select.run().expect("error running select");

    // Open the selected CSV file
    let mut rdr = csv::Reader::from_path(&selected_option)?;

    // Read the CSV records
    for result in rdr.deserialize() {
        let record: UserData = result?;
        // Handle the record here
        let _ = tasks_handler(record).await;
    }

    Ok(())
}

pub async fn tasks_handler(record: UserData) -> Result<(), Box<dyn Error>> {
    if record.mode == "manual".to_string() {
        let _ = match raydium_stream(record).await {
            Ok(_) => info!("Manual Sniper Started"),
            Err(e) => error!("{}", e),
        };
    } else {
        let _ = match auto_sniper_stream(record).await {
            Ok(_) => info!("Auto Sniper Started"),
            Err(e) => error!("{}", e),
        };
    }

    Ok(())
}

pub async fn token_env() -> Result<Pubkey, Box<dyn Error>> {
    let t = Input::new("Pool Address:")
        .placeholder("5eSB1...vYF49")
        .prompt("Input: ");

    let mint_address = t.run().expect("error running input");

    let token_pubkey = Pubkey::from_str(&mint_address)?;

    Ok(token_pubkey)
}
pub async fn sol_amount() -> Result<u64, Box<dyn Error>> {
    let t = Input::new("Sol Amount:")
        .placeholder("0.01")
        .prompt("Input: ");

    let string = t.run().expect("error running input");

    let amount = sol_to_lamports(string.parse::<f64>()?);

    Ok(amount)
}
pub async fn priority_fee() -> Result<u64, Box<dyn Error>> {
    let t = Input::new("Priority Fee:")
        .placeholder("0.0001")
        .prompt("Input: ");

    let string = t.run().expect("error running input");

    let amount = sol_to_lamports(string.parse::<f64>()?);

    Ok(amount)
}
pub async fn bundle_priority_tip() -> Result<u64, Box<dyn Error>> {
    let t = Input::new("Bundle Tip:")
        .placeholder("0.0001")
        .prompt("Input: ");

    let string = t.run().expect("error running input");

    let amount = sol_to_lamports(string.parse::<f64>()?);

    Ok(amount)
}
pub async fn private_key_env() -> Result<String, Box<dyn Error>> {
    let t = Input::new("Private Key: ")
        .placeholder("5eSB1...vYF49")
        .prompt("Input: ");

    let private_key = t.run().expect("error running input");

    Ok(private_key)
}

pub async fn new_pair_mev() -> Result<(), Box<dyn Error>> {
    let sol_amount = sol_amount().await?;
    let priority_fee = priority_fee().await?;
    let bundle_tip = bundle_priority_tip().await?;
    let wallet = private_key_env().await?;

    let mev_ape = MevApe {
        sol_amount,
        priority_fee,
        bundle_tip,
        wallet,
    };

    let mev = match txn_blocktime(mev_ape).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}

#[derive(Debug)]
pub struct MevApe {
    pub sol_amount: u64,
    pub priority_fee: u64,
    pub bundle_tip: u64,
    pub wallet: String,
}
