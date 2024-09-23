use std::error::Error;

use crate::{
    app::{embeds::embed, theme},
    env::vanity::vanity_main,
    liquidity::option::{
        sol_distribution::distributor, wallet_gen::gen_wallet_save, withdraw_sol::withdraw_sol,
        withdraw_wrapped::withdraw_wrapped_sol, wrap_sol::sol_wrap,
    },
    pumpfun::bundler::{
        multi_deployer::multi_wallet_token, pump_lut::pump_lut_main,
        pumpfun_deploy::one_pumpfun_deploy, sell_mode::pump_seller::pump_bundle_seller,
    },
    utils::terminal::clear_screen,
};
use colored::Colorize;
use demand::{DemandOption, Select};

use async_recursion::async_recursion;

#[async_recursion]
pub async fn pump_bundler() -> Result<(), Box<dyn Error>> {
    // let _auth = match auth_verification().await {
    //     Ok(_) => {}
    //     Err(e) => {
    //         error!("Error: {}", e);
    //         return Ok(());
    //     }
    // };

    let theme = theme();
    let ms = Select::new("Minter Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(
            DemandOption::new("Generate Wallets")
                .label(&format!("▪ Generate {}", "New Wallets".bold().white())),
        )
        .option(
            DemandOption::new("CreateTokenVanity")
                .label(&format!("▪ Vanity {}", "Generation".bold().yellow())),
        )
        .option(
            DemandOption::new("CreateLUT")
                .label(&format!("▪ Create {}", "LUT".bold().bright_cyan())),
        )
        .option(
            DemandOption::new("Distribute SOL")
                .label(&format!("▪ Distribute {}", "SOL".bold().bright_magenta())),
        )
        .option(DemandOption::new("Wrap SOL & ATAs").label("▪ Wrap SOL & ATAs"))
        .option(DemandOption::new("1-Liquidity").label(&format!(
            "💠 Create token - {}",
            "Single-Wallet".bold().green()
        )))
        .option(DemandOption::new("multi-Liquidity").label(&format!(
            "💊  Create token - {}",
            "Multi-Wallet".bold().green()
        )))
        .option(DemandOption::new("Sell%").label(&format!("▪ Percentage {}", "Sell".bold().red())))
        .option(
            DemandOption::new("SellAll").label(&format!("▪ Entire {}", "Sell".bold().bright_red())),
        )
        .option(DemandOption::new("WithdrawSol").label("▪ Withdraw SOL"))
        .option(DemandOption::new("WithdrawWSol&ATAS").label("▪ Withdraw WSOL & Close Accounts"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Generate Wallets" => {
            let _ = gen_wallet_save().await;
        }
        "CreateTokenVanity" => {
            let _ = vanity_main().await;
        }
        "CreateLUT" => {
            let _ = pump_lut_main().await;
        }
        "Distribute SOL" => {
            let _ = distributor().await;
        }
        "Wrap SOL & ATAs" => {
            let _ = sol_wrap().await;
        }
        "1-Liquidity" => {
            let _ = one_pumpfun_deploy().await;
        }
        "multi-Liquidity" => {
            let _ = multi_wallet_token().await;
        }
        "Sell%" => {
            let _ = match pump_bundle_seller(true).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            };
        }
        "SellAll" => {
            let _ = match pump_bundle_seller(false).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            };
        }
        "WithdrawSol" => {
            let _ = withdraw_sol().await;
        }
        "WithdrawWSol&ATAS" => {
            let _ = withdraw_wrapped_sol().await;
        }
        "Main Menu" => {
            //clear terminal
            clear_screen();
            //clear the previous line
            println!("{}", embed());
            let _ = main_menu(false).await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    // clear_screen();
    //clear the previous line
    println!("{}", embed());
    let _ = pump_bundler().await;

    Ok(())
}
