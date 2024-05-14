use std::error::Error;

use demand::{DemandOption, Select};

use crate::{
    app::{app, theme},
    liquidity::{
        lut::extend_lut::lut_main,
        option::{
            sol_distribution::distributor,
            withdraw_sol::{deployer_details, withdraw_sol},
            withdraw_wrapped::withdraw_wrapped_sol,
            wrap_sol::sol_wrap,
        },
        pool_1::single_pool,
        pool_27::pool_main,
        remove_liq::remover::remove_liquidity,
        sell_mode::sell_percentage::sell_specific,
    },
};

use super::option::wallet_gen::gen_wallet_save;

use async_recursion::async_recursion;

#[async_recursion]
pub async fn raydium_creator() -> Result<(), Box<dyn Error>> {
    let theme = theme();
    let ms = Select::new("Minter Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("Generate Wallets").label("[1] Generate New Wallets"))
        .option(DemandOption::new("CreateLUT").label("[2] Create LUT"))
        .option(DemandOption::new("Distribute SOL").label("[3] Distribute SOL"))
        .option(DemandOption::new("Wrap SOL & ATAs").label("[4] Wrap SOL & ATAs"))
        .option(DemandOption::new("Add Liquidity").label("[5] Add Liquidity"))
        .option(DemandOption::new("Remove Liquidity").label("[6] Remove Liquidity"))
        .option(DemandOption::new("Sell%").label("[7] Percentage Sell"))
        .option(DemandOption::new("SellAll").label("[8] All Sell"))
        .option(DemandOption::new("WithdrawSol").label("[9] Withdraw SOL"))
        .option(DemandOption::new("WithdrawWSol&ATAS").label("[10] Withdraw WSOL & Close Accounts"))
        .option(DemandOption::new("deployerdetails").label("🍄 Deployer Wallet Details"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Generate Wallets" => {
            let _ = gen_wallet_save().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "CreateLUT" => {
            let _ = lut_main().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Distribute SOL" => {
            let _ = distributor().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Wrap SOL & ATAs" => {
            let _ = sol_wrap().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Add Liquidity" => {
            let _ = pooler_mode().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Remove Liquidity" => {
            let _ = match remove_liquidity().await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            };
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Sell%" => {
            let _ = sell_specific(true).await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "SellAll" => {
            let _ = sell_specific(false).await;

            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "WithdrawSol" => {
            let _ = withdraw_sol().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "WithdrawWSol&ATAS" => {
            let _ = withdraw_wrapped_sol().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "deployerdetails" => {
            let _ = deployer_details().await;
            println!("-------------------Returning to Main Menu-------------------");
            raydium_creator().await?;
        }
        "Main Menu" => {
            //clear terminal
            println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            //clear the previous line
            // println!("{esc}[2K", esc = 27 as char);
            let _ = app(false).await;
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
            let _ = single_pool().await;
            println!("-------------------Returning to Main Menu-------------------");
            pooler_mode().await?;
        }
        "Multiple Wallets" => {
            let _ = pool_main().await;
            println!("-------------------Returning to Main Menu-------------------");
            pooler_mode().await?;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}
