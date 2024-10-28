use std::sync::Arc;

use colorize::AnsiColor;
use crossterm::style::Stylize;
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    signature::Keypair,
    system_program,
};
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction;

use crate::{
    app::config_init::{get_config, update_config_field},
    daos_fun::{
        dao_burned_interface::{InitializeIxData, InitializeKeys, INITIALIZE_IX_ACCOUNTS_LEN},
        daos_transaction::daosfun_sender,
    },
    env::{load_config, SettingsConfig},
    pumpfun::{
        executor::pump_swap,
        pump_interface::{builder::PumpFunDirection, instructions::CreateIxData},
    },
    raydium_amm::{
        subscribe::auto_sniper_stream,
        swap::{metadata::decode_metadata, raydium_amm_sniper::RAYDIUM_AMM_FEE_COLLECTOR},
    },
    router::{grpc_pair_sub, SniperRoute},
    user_inputs::{amounts::sol_amount, tokens::token_env},
};

use super::daos_transaction::DAOS_BURNED_PROGRAM;

pub async fn daosfun_sniper(manual_snipe: bool, route: SniperRoute) -> eyre::Result<()> {
    let args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    if args.network.grpc_url.is_empty() {
        let _ = auto_sniper_stream(manual_snipe).await?;
        return Ok(());
    }
    let sol_amount = sol_amount("Snipe Amount:").await;

    update_config_field(|c| &mut c.trading.buy_amount, lamports_to_sol(sol_amount)).await?;

    let token;

    if manual_snipe {
        token = Some(token_env("Base Mint").await);

        let (token, data) = mpl_token_metadata::accounts::Metadata::find_pda(&token.unwrap());
        let metadata = match decode_metadata(&token).await {
            Ok(metadata) => Some(metadata),
            Err(e) => {
                error!("Error: {:?}", e);
                None
            }
        };

        let token_name = metadata
            .clone()
            .and_then(|m| Some(m.name))
            .unwrap_or_else(|| "Unknown".to_string());
        let token_symbol = metadata
            .clone()
            .and_then(|m| Some(m.symbol))
            .unwrap_or_else(|| "Unknown".to_string());

        println!(
            "Name: {}\nSymb: {}",
            colorize::AnsiColor::bold(token_name.to_string()).white(),
            colorize::AnsiColor::bold(token_name.to_string()).b_cyan(),
        );

        info!("Listening for the Launch...")
    } else {
        token = None;
    }

    let contract = if route == SniperRoute::DaosFun {
        DAOS_BURNED_PROGRAM
    } else {
        DAOS_BURNED_PROGRAM
    };

    let _ = match grpc_pair_sub(args, manual_snipe, token, contract.to_string(), route).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}

pub async fn daosfun_parser(
    args: SettingsConfig,
    tx: SubscribeUpdateTransaction,
    base_mint: Option<Pubkey>,
) -> eyre::Result<()> {
    let config = get_config().await?;
    let wallet = Keypair::from_base58_string(&config.engine.payer_keypair);

    let info = tx.clone().transaction.unwrap_or_default();
    let accounts = info
        .transaction
        .clone()
        .unwrap_or_default()
        .message
        .unwrap_or_default()
        .account_keys
        .iter()
        .map(|i| {
            let mut array = [0; 32];
            let bytes = &i[..array.len()]; // panics if not enough data
            array.copy_from_slice(bytes);
            Pubkey::new_from_array(array)
        })
        .collect::<Vec<Pubkey>>();
    let outer_instructions = {
        let transaction = info.transaction.clone().unwrap_or_default();
        let message = transaction.message.unwrap_or_default();
        let instructions = message.instructions.iter();
        instructions.cloned().collect::<Vec<_>>()
    };

    let mut daosfun_accounts: Option<InitializeKeys> = None;
    for (index, instructions) in outer_instructions.iter().enumerate() {
        for (index, instructions) in outer_instructions.iter().enumerate() {
            if instructions.accounts.len() >= INITIALIZE_IX_ACCOUNTS_LEN {
                daosfun_accounts = Some(InitializeKeys {
                    admin: accounts[instructions.accounts[0] as usize],
                    state: accounts[instructions.accounts[1] as usize],
                    wallet: accounts[instructions.accounts[2] as usize],
                    dao_mint: accounts[instructions.accounts[3] as usize],
                    funding_mint: accounts[instructions.accounts[4] as usize],
                    dao_mint_vault: accounts[instructions.accounts[5] as usize],
                    token_program: spl_token::id(),
                    system_program: system_program::id(),
                    associated_token_program: spl_associated_token_account::id(),
                    fundraise_state: accounts[instructions.accounts[6] as usize],
                    fundraise_token_vault: accounts[instructions.accounts[7] as usize],
                    fundraise_program: accounts[instructions.accounts[8] as usize],
                });
            }
            break;
        }
    }

    let mut coin_found = false;

    let mut coin_args: Option<InitializeIxData> = None;
    for instructions in outer_instructions.iter() {
        match InitializeIxData::deserialize(&instructions.data) {
            Ok(decode_new_coin) => {
                coin_found = true;
                coin_args = Some(decode_new_coin);
                break;
            }
            Err(_) => {
                continue;
            }
        };
    }

    if !coin_found {
        return Ok(());
    }

    let signature = bs58::encode(&info.signature).into_string();

    if base_mint.is_some() {
        if accounts[1] != base_mint.unwrap() {
            return Ok(());
        }
    }

    if daosfun_accounts.is_none() {
        return Ok(()); // or handle the error case appropriately
    }

    let dao_mint = daosfun_accounts.unwrap().dao_mint;

    println!(
        "Transaction: {}\nCoin: {:?}\nMaker: {}\nMint: {}",
        &signature.to_string(),
        coin_args.as_ref().unwrap().0,
        accounts[0],
        accounts[1]
    );

    let config = get_config().await?;

    let amount = sol_to_lamports(config.trading.buy_amount);
    let rpc_client = RpcClient::new(args.clone().network.rpc_url);

    let token = rpc_client.get_account(&dao_mint).await?;
    // let mint = Mint::unpack(&token).unwrap();
    let dao_mint_program = token.owner;

    match daosfun_sender(
        rpc_client,
        args,
        wallet,
        dao_mint,
        dao_mint_program,
        amount,
        crate::daos_fun::daos_transaction::DAOSTrade::Buy,
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return Ok(());
        }
    };

    Ok(())
}
