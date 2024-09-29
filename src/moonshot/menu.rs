use std::{error::Error, sync::Arc};

use solana_sdk::signature::Keypair;

use crate::{env::load_config, user_inputs::tokens::token_env};

use super::generate_moonshot_buy_ix;

#[derive(PartialEq)]
pub enum MoonShotDirection {
    Buy,
    Sell,
}

// pub async fn moonshot_menu() -> Result<(), Box<dyn Error + Send>> {
//     let theme = theme();
//     let ms = Select::new("Moonshot Menu")
//         .description("Select an Option")
//         .theme(&theme)
//         .filterable(true)
//         .option(DemandOption::new("MoonShotSniper").label("▪ Snipe Incoming Coins"))
//         .option(DemandOption::new("BuyTokens").label("▪ Buy Tokens"))
//         .option(DemandOption::new("SellTokens").label("▪ Sell Tokens"))
//         .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

//     let selected_option = ms.run().expect("error running select");

//     match selected_option {
//         "MoonShotSniper" => {
//             let _ = moonshot_sniper(false).await;
//         }
//         "BuyTokens" => {
//             let _ = moonshot_swap_handler(MoonShotDirection::Buy).await;
//         }
//         "SellTokens" => {
//             let _ = moonshot_swap_handler(MoonShotDirection::Sell).await;
//         }
//         "Main Menu" => {
//             let _ = main_menu(false).await;
//         }
//         _ => {}
//     }

//     Ok(())
// }

pub async fn moonshot_swap_handler(dir: MoonShotDirection) -> Result<(), Box<dyn Error + Send>> {
    let args = match load_config().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    let wallet = Keypair::from_base58_string(&args.engine.payer_keypair);

    let token_address = token_env("Token Address: ").await;

    let ix = generate_moonshot_buy_ix(token_address, 10000, Arc::new(wallet)).await;

    // let _ = match dir {
    //     MoonShotDirection::Buy => {
    //         let _ = crate::moonshot::buy_tokens(&args, &token_address, &amount).await;
    //     }
    //     MoonShotDirection::Sell => {
    //         let _ = crate::moonshot::sell_tokens(&args, &token_address, &amount).await;
    //     }
    // };

    Ok(())
}
