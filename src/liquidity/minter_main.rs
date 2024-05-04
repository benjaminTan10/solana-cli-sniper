use std::{error::Error, pin::Pin};

use demand::{DemandOption, Select};
use futures::Future;

use crate::{
    app::theme,
    env::minter::load_minter_settings,
    liquidity::{
        option::sol_distribution::distributor, pool_27::pool_main,
        remove_liq::remover::remove_liquidity,
    },
};

use super::option::wallet_gen::{gen_wallet_save, list_folders};

use async_recursion::async_recursion;

#[async_recursion]
pub async fn raydium_creator() -> Result<(), Box<dyn Error>> {
    let theme = theme();
    let ms = Select::new("Minter Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Generate Wallets").label("[1] Generate New Wallets"))
        .option(DemandOption::new("Distribute SOL & ATAs").label("[2] Distribute SOL & ATAs"))
        .option(DemandOption::new("Add Liquidity").label("[3] Add Liquidity"))
        .option(DemandOption::new("Remove Liquidity").label("[4] Remove Liquidity"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Generate Wallets" => {
            let _ = gen_wallet_save().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Distribute SOL & ATAs" => {
            let _ = distributor().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Add Liquidity" => {
            let _ = pooler_mode().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Remove Liquidity" => {
            let _ = remove_liquidity().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

#[async_recursion]
pub async fn pooler_mode() -> Result<(), Box<dyn Error>> {
    let theme = theme();
    let ms = Select::new("Pool Type")
        .description("Select the Pool Type")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Single Wallet").label("[1] 1-Wallet (Only Buyer)"))
        .option(DemandOption::new("Multiple Wallets").label("[2] Multiple Wallets"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Single Wallet" => {
            let _ = raydium_creator().await;
            println!("-------------------Returning to Main Menu-------------------");
            pooler_mode().await?;
        }
        "Multiple Wallets" => {
            let _ = pool_main().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}
