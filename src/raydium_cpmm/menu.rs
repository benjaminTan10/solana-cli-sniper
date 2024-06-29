use std::{error::Error, sync::Arc};

use demand::{DemandOption, Select};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    app::{app, theme},
    env::load_settings,
    raydium_amm::volume_pinger::volume::buy_amount,
    rpc::HTTP_CLIENT,
    user_inputs::tokens::token_env,
};

use super::cpmm_builder::{Opts, RaydiumCpCommands};

#[derive(PartialEq)]
pub enum RaydiumCPMMDirection {
    BuyTokens,
    SellTokens,
}

pub async fn raydium_cpmm() -> Result<(), Box<dyn Error + Send>> {
    let theme = theme();
    let ms = Select::new("Raydium CPMM")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("BuyTokens").label("▪ Swap Tokens to SOL"))
        .option(DemandOption::new("SellTokens").label("▪ Swap SOL to Tokens"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "BuyTokens" => {
            let _ = cpmm_swap_builder(RaydiumCPMMDirection::BuyTokens).await;
        }
        "SellTokens" => {
            let _ = cpmm_swap_builder(RaydiumCPMMDirection::SellTokens).await;
        }
        "Main Menu" => {
            let _ = app(false).await;
        }

        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}

pub async fn cpmm_swap_builder(direction: RaydiumCPMMDirection) -> eyre::Result<()> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    let connection = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    let wallet = Keypair::from_base58_string(&args.payer_keypair);

    let pool_address = token_env("Pool Address").await;

    let swap_amount = match buy_amount("Swap Amount").await {
        Ok(amount) => amount,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    match direction {
        RaydiumCPMMDirection::BuyTokens => {
            match super::cpmm_builder::cpmm_transaction(
                Opts {
                    command: RaydiumCpCommands::SwapBaseIn {
                        pool_id: (pool_address),
                        user_input_amount: (swap_amount),
                    },
                },
                Arc::new(wallet),
                connection,
                0,
                RaydiumCPMMDirection::BuyTokens,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Ok(());
                }
            };
        }
        RaydiumCPMMDirection::SellTokens => {
            // Implement sell tokens logic here
        }
    }

    Ok(())
}
