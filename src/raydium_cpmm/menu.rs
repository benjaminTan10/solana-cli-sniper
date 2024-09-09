use std::{error::Error, sync::Arc};

use demand::{DemandOption, Select};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token_client::spl_token_2022;

use crate::{
    app::{app, theme},
    env::load_config,
    raydium_amm::volume_pinger::volume::buy_amount,
    rpc::HTTP_CLIENT,
    user_inputs::{amounts::amount_percentage, tokens::token_env},
};

use super::{
    cpmm_builder::{Opts, RaydiumCpCommands},
    cpmm_instructions::PoolStateAccount,
};

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
        .option(DemandOption::new("BuyTokens").label("▪ Swap SOL to Tokens"))
        .option(DemandOption::new("SellTokens").label("▪ Swap Tokens to SOL"))
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
    let args = match load_config().await {
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

    let wallet = Keypair::from_base58_string(&args.engine.payer_keypair);

    let pool_address = token_env("Pool Address").await;

    match direction {
        RaydiumCPMMDirection::BuyTokens => {
            let swap_amount = match buy_amount("Swap Amount").await {
                Ok(amount) => amount,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Ok(());
                }
            };
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
            let percentage_amount = amount_percentage().await;

            let pool_account = connection.get_account_data(&pool_address).await?;
            let sliced: &mut &[u8] = &mut pool_account.as_slice();
            let pool_state = PoolStateAccount::deserialize(sliced)?.0;

            let mint_info = connection.get_account(&pool_state.token_0_mint).await?;

            let user_input_token;
            if mint_info.owner != spl_token_2022::id() {
                user_input_token = spl_associated_token_account::get_associated_token_address(
                    &wallet.pubkey(),
                    &pool_state.token_0_mint,
                );
            } else {
                user_input_token = get_associated_token_address_with_program_id(
                    &wallet.pubkey(),
                    &pool_state.token_0_mint,
                    &spl_token_2022::id(),
                );
            }

            let get_tokens = connection
                .get_token_account_balance(&user_input_token)
                .await?;

            let swap_amount = get_tokens.amount.parse::<u64>()? * percentage_amount / 100;

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
                RaydiumCPMMDirection::SellTokens,
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
    }

    Ok(())
}
