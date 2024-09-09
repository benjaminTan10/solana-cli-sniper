use std::error::Error;

use demand::{DemandOption, Select};

use crate::{
    app::{app, embeds::embed, theme},
    liquidity::{
        lut::extend_lut::lut_main,
        option::{
            sol_distribution::distributor, withdraw_sol::withdraw_sol,
            withdraw_wrapped::withdraw_wrapped_sol, wrap_sol::sol_wrap,
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
        .option(DemandOption::new("Generate Wallets").label("▪ Generate New Wallets"))
        .option(DemandOption::new("CreateLUT").label("▪ Create LUT"))
        .option(DemandOption::new("Distribute SOL").label("▪ Distribute SOL"))
        .option(DemandOption::new("Wrap SOL & ATAs").label("▪ Wrap SOL & ATAs"))
        .option(DemandOption::new("1-Liquidity").label("💠 Add Liquidity - Single-Wallet"))
        .option(DemandOption::new("multi-Liquidity").label("♾️  Add Liquidity - Multi-Wallet"))
        .option(DemandOption::new("Remove Liquidity").label("▪ Remove Liquidity"))
        .option(DemandOption::new("Sell%").label("▪ Percentage Sell"))
        .option(DemandOption::new("SellAll").label("▪ All Sell"))
        .option(DemandOption::new("WithdrawSol").label("▪ Withdraw SOL"))
        .option(DemandOption::new("WithdrawWSol&ATAS").label("▪ Withdraw WSOL & Close Accounts"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "Generate Wallets" => {
            let _ = gen_wallet_save().await;
        }
        "CreateLUT" => {
            let _ = lut_main().await;
        }
        "Distribute SOL" => {
            let _ = distributor().await;
        }
        "Wrap SOL & ATAs" => {
            let _ = sol_wrap().await;
        }
        "1-Liquidity" => {
            let _ = single_pool().await;
        }
        "multi-Liquidity" => {
            let _ = pool_main().await;
        }
        "Remove Liquidity" => {
            let _ = match remove_liquidity().await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            };
        }
        "Sell%" => {
            let _ = match sell_specific(true).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            };
        }
        "SellAll" => {
            let _ = match sell_specific(false).await {
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
            println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            //clear the previous line
            println!("{}", embed());
            let _ = app(false).await;
        }
        _ => {
            // Handle unexpected option here
        }
    }

    // println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //clear the previous line
    println!("{}", embed());
    let _ = raydium_creator().await;

    Ok(())
}

// #[async_recursion]
// pub async fn pooler_mode() -> Result<(), Box<dyn Error>> {
//     let theme = theme();
//     let ms = Select::new("Pool Type")
//         .description("Select the Pool Type")
//         .theme(&theme)
//         .filterable(true)
//         .option(DemandOption::new("Single Wallet").label("[1] 1-Wallet (Only Buyer)"))
//         .option(DemandOption::new("Multiple Wallets").label("[2] Multiple Wallets"))
//         .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

//     let selected_option = ms.run().expect("error running select");

//     match selected_option {
//         "Single Wallet" => {
//             let _ = single_pool().await;
//             println!("-------------------Returning to Main Menu-------------------");
//             pooler_mode().await?;
//         }
//         "Multiple Wallets" => {
//             let _ = pool_main().await;
//             println!("-------------------Returning to Main Menu-------------------");
//             pooler_mode().await?;
//         }
//         _ => {
//             // Handle unexpected option here
//         }
//     }

//     Ok(())
// }
