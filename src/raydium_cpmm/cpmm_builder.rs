#![allow(dead_code)]
use anyhow::{format_err, Result};
use arrayref::array_ref;
use clap::Parser;
use serde::{Deserialize, Serialize};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
};
use solana_transaction_status::UiTransactionEncoding;
use spl_associated_token_account::{
    get_associated_token_address, get_associated_token_address_with_program_id,
};
use spl_token_client::spl_token_2022::{
    self,
    extension::StateWithExtensionsMut,
    state::{Account, Mint},
};
use std::str::FromStr;
use std::sync::Arc;

use crate::{env::load_settings, raydium_cpmm::cpmm_instructions::PoolStateAccount};

use super::instructions::events_instructions_parse::*;
use super::instructions::rpc::*;
use super::instructions::token_instructions::*;
use super::instructions::utils::*;
use super::{instructions::amm_instructions::*, menu::RaydiumCPMMDirection};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConfig {
    pub http_url: String,
    pub ws_url: String,
    pub payer_path: String,
    pub admin_path: String,
    pub raydium_cp_program: Pubkey,
    pub slippage: f64,
}

fn read_keypair_file(s: &str) -> Result<Keypair> {
    solana_sdk::signature::read_keypair_file(s)
        .map_err(|_| format_err!("failed to read keypair from {}", s))
}

#[derive(Debug)]
pub struct Opts {
    pub command: RaydiumCpCommands,
}

#[derive(Debug)]
pub enum RaydiumCpCommands {
    SwapBaseIn {
        pool_id: Pubkey,
        user_input_amount: u64,
    },
    DecodeInstruction {
        instr_hex_data: String,
    },
    DecodeEvent {
        log_event: String,
    },
    DecodeTxLog {
        tx_id: String,
    },
}

pub async fn cpmm_transaction(
    opts: Opts,
    payer: Arc<Keypair>,
    rpc_client: Arc<RpcClient>,
    slippage: u64,
    direction: RaydiumCPMMDirection,
) -> Result<()> {
    let args = match load_settings().await {
        Ok(a) => a,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };
    let payer_pubkey = payer.pubkey();

    // let program = client.program(RAYDIUM_CPMM)?;

    match opts.command {
        RaydiumCpCommands::SwapBaseIn {
            pool_id,
            user_input_amount,
        } => {
            let pool_account = rpc_client.get_account_data(&pool_id).await?;
            let sliced: &mut &[u8] = &mut pool_account.as_slice();
            let pool_state = PoolStateAccount::deserialize(sliced)?.0;

            let user_input_token = if direction == RaydiumCPMMDirection::BuyTokens {
                get_associated_token_address(&payer_pubkey, &pool_state.token_1_mint)
            } else if direction == RaydiumCPMMDirection::SellTokens {
                get_associated_token_address(&payer_pubkey, &pool_state.token_0_mint)
            } else {
                // Handle the case where direction is neither BuyTokens nor SellTokens
                return Err(format_err!("Invalid direction"));
            };

            println!("User input token: {:?}", user_input_token);

            let load_pubkeys = vec![
                pool_state.amm_config,
                pool_state.token_0_vault,
                pool_state.token_1_vault,
                pool_state.token_0_mint,
                pool_state.token_1_mint,
                user_input_token,
            ];

            let rsps = rpc_client.get_multiple_accounts(&load_pubkeys).await?;
            let epoch = rpc_client.get_epoch_info().await.unwrap().epoch;
            let [amm_config_account, token_0_vault_account, token_1_vault_account, token_0_mint_account, token_1_mint_account, user_input_token_account] =
                array_ref![rsps, 0, 6];

            // docode account
            let mut token_0_vault_data = token_0_vault_account.clone().unwrap().data;
            let mut token_1_vault_data = token_1_vault_account.clone().unwrap().data;
            let mut token_0_mint_data = token_0_mint_account.clone().unwrap().data;
            let mut token_1_mint_data = token_1_mint_account.clone().unwrap().data;
            // let mut user_input_token_data = user_input_token_account.clone().unwrap().data;
            let amm_config_state = deserialize_anchor_account::<raydium_cp_swap::states::AmmConfig>(
                amm_config_account.as_ref().unwrap(),
            )?;
            let token_0_vault_info =
                StateWithExtensionsMut::<Account>::unpack(&mut token_0_vault_data)?;
            let token_1_vault_info =
                StateWithExtensionsMut::<Account>::unpack(&mut token_1_vault_data)?;
            let token_0_mint_info = StateWithExtensionsMut::<Mint>::unpack(&mut token_0_mint_data)?;
            let token_1_mint_info = StateWithExtensionsMut::<Mint>::unpack(&mut token_1_mint_data)?;

            let (total_token_0_amount, total_token_1_amount) = pool_state.vault_amount_without_fee(
                token_0_vault_info.base.amount,
                token_1_vault_info.base.amount,
            );

            // Check: If token is 2022, then create associated token account
            // token 0 mint = SOLC Address
            // token 1 mint = TOKEN Address

            let token_in = if direction == RaydiumCPMMDirection::BuyTokens {
                pool_state.token_1_mint
            } else {
                pool_state.token_0_mint
            };

            let token_out = if direction == RaydiumCPMMDirection::BuyTokens {
                pool_state.token_0_mint
            } else {
                pool_state.token_1_mint
            };

            let mint_info = rpc_client.get_account(&token_in).await?;

            let user_input_token;
            let create_user_input_token_instr;
            let create_user_output_token_instr;

            if mint_info.owner != spl_token_2022::id() {
                user_input_token = spl_associated_token_account::get_associated_token_address(
                    &payer_pubkey,
                    &token_in,
                );
                create_user_input_token_instr = create_ata_token_account_instr(
                    payer.clone(),
                    spl_token::id(),
                    &token_in,
                    &payer_pubkey,
                )
                .await?;
            } else {
                user_input_token = get_associated_token_address_with_program_id(
                    &payer_pubkey,
                    &token_in,
                    &spl_token_2022::id(),
                );
                create_user_input_token_instr = create_ata_token_account_instr(
                    payer.clone(),
                    spl_token_2022::id(),
                    &token_in,
                    &payer_pubkey,
                )
                .await?;
            };

            let mint_info = rpc_client.get_account(&token_out).await?;

            let output_token_account;

            if mint_info.owner != spl_token_2022::id() {
                output_token_account = spl_associated_token_account::get_associated_token_address(
                    &payer_pubkey,
                    &token_out,
                );
                create_user_output_token_instr = create_ata_token_account_instr(
                    payer.clone(),
                    spl_token::id(),
                    &token_out,
                    &payer_pubkey,
                )
                .await?;
            } else {
                output_token_account = get_associated_token_address_with_program_id(
                    &payer_pubkey,
                    &token_out,
                    &spl_token_2022::id(),
                );
                create_user_output_token_instr = create_ata_token_account_instr(
                    payer.clone(),
                    spl_token_2022::id(),
                    &token_out,
                    &payer_pubkey,
                )
                .await?;
            };

            let (
                trade_direction,
                total_input_token_amount,
                total_output_token_amount,
                user_input_token,
                user_output_token,
                input_vault,
                output_vault,
                input_token_mint,
                output_token_mint,
                input_token_program,
                output_token_program,
                transfer_fee,
            ) = if direction == RaydiumCPMMDirection::SellTokens {
                (
                    raydium_cp_swap::curve::TradeDirection::ZeroForOne,
                    total_token_0_amount,
                    total_token_1_amount,
                    user_input_token,
                    output_token_account,
                    pool_state.token_0_vault,
                    pool_state.token_1_vault,
                    pool_state.token_0_mint,
                    pool_state.token_1_mint,
                    pool_state.token_0_program,
                    pool_state.token_1_program,
                    get_transfer_fee(&token_0_mint_info, epoch, user_input_amount),
                )
            } else {
                (
                    raydium_cp_swap::curve::TradeDirection::OneForZero,
                    total_token_1_amount,
                    total_token_0_amount,
                    user_input_token,
                    output_token_account,
                    pool_state.token_1_vault,
                    pool_state.token_0_vault,
                    pool_state.token_1_mint,
                    pool_state.token_0_mint,
                    pool_state.token_1_program,
                    pool_state.token_0_program,
                    get_transfer_fee(&token_1_mint_info, epoch, user_input_amount),
                )
            };

            // Take transfer fees into account for actual amount transferred in
            let actual_amount_in = user_input_amount.saturating_sub(transfer_fee);
            let result = raydium_cp_swap::curve::CurveCalculator::swap_base_input(
                u128::from(actual_amount_in),
                u128::from(total_input_token_amount),
                u128::from(total_output_token_amount),
                amm_config_state.trade_fee_rate,
                amm_config_state.protocol_fee_rate,
                amm_config_state.fund_fee_rate,
            )
            .ok_or(raydium_cp_swap::error::ErrorCode::ZeroTradingTokens)
            .unwrap();
            let amount_out = u64::try_from(result.destination_amount_swapped).unwrap();
            let transfer_fee = match trade_direction {
                raydium_cp_swap::curve::TradeDirection::ZeroForOne => {
                    get_transfer_fee(&token_1_mint_info, epoch, amount_out)
                }
                raydium_cp_swap::curve::TradeDirection::OneForZero => {
                    get_transfer_fee(&token_0_mint_info, epoch, amount_out)
                }
            };
            let amount_received = amount_out.checked_sub(transfer_fee).unwrap();
            // calc mint out amount with slippage
            let minimum_amount_out =
                amount_with_slippage(amount_received, slippage as f64 / 100.0, false);

            let mut instructions = Vec::new();

            instructions.extend(create_user_input_token_instr);
            instructions.extend(create_user_output_token_instr);
            let swap_base_in_instr = swap_base_input_instr(
                payer_pubkey,
                pool_id,
                pool_state.amm_config,
                pool_state.observation_key,
                user_input_token,
                user_output_token,
                input_vault,
                output_vault,
                input_token_mint,
                output_token_mint,
                input_token_program,
                output_token_program,
                user_input_amount,
                minimum_amount_out,
            )?;
            instructions.extend(swap_base_in_instr);

            match transaction_handler(&rpc_client, payer, instructions, 10000, &args).await {
                Ok(()) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Ok(());
                }
            };
        }

        RaydiumCpCommands::DecodeInstruction { instr_hex_data } => {
            handle_program_instruction(&instr_hex_data, InstructionDecodeType::BaseHex)?;
        }
        RaydiumCpCommands::DecodeEvent { log_event } => {
            handle_program_log(&RAYDIUM_CPMM.to_string(), &log_event, false)?;
        }
        RaydiumCpCommands::DecodeTxLog { tx_id } => {
            let signature = Signature::from_str(&tx_id)?;
            let tx = rpc_client
                .get_transaction_with_config(
                    &signature,
                    RpcTransactionConfig {
                        encoding: Some(UiTransactionEncoding::Json),
                        commitment: Some(CommitmentConfig::confirmed()),
                        max_supported_transaction_version: Some(0),
                    },
                )
                .await?;
            let transaction = tx.transaction;
            // get meta
            let meta = if transaction.meta.is_some() {
                transaction.meta
            } else {
                None
            };
            // get encoded_transaction
            let encoded_transaction = transaction.transaction;
            // decode instruction data
            parse_program_instruction(
                &RAYDIUM_CPMM.to_string(),
                encoded_transaction,
                meta.clone(),
            )?;
            // decode logs
            parse_program_event(&RAYDIUM_CPMM.to_string(), meta.clone())?;
        }
    }
    Ok(())
}
