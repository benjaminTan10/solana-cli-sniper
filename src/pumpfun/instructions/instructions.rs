use std::{
    ops::{Add, Mul, Sub},
    sync::Arc,
};

use borsh::BorshDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program,
};
use spl_associated_token_account::get_associated_token_address;

use crate::pumpfun::instructions::pumpfun_program::instructions::{
    buy_ix_with_program_id, BuyIxArgs, BuyKeys,
};

use super::pumpfun_program::{
    accounts::BondingCurve,
    instructions::{sell_ix_with_program_id, SellIxArgs, SellKeys},
};

pub const GLOBAL_STATE: Pubkey = pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
pub const FEE_RECEPIENT: Pubkey = pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");
pub const EVENT_AUTH: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
pub const PUMP_PROGRAM: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[derive(PartialEq)]
pub enum PumpFunDirection {
    Buy,
    Sell,
}

pub async fn generate_pump_buy_ix(
    rpc_client: Arc<RpcClient>,
    token: Pubkey,
    sol_amount: u64,
    main_signer: Arc<Keypair>,
) -> eyre::Result<Vec<Instruction>> {
    let bonding_curve_pda = get_bonding_curve(token, &PUMP_PROGRAM);
    let bonding_curve_ata = get_associated_token_address(&bonding_curve_pda, &token);
    let signer_ata = get_associated_token_address(&main_signer.pubkey(), &token);

    let account_data = rpc_client.get_account_data(&bonding_curve_pda).await?;

    let sliced_data: &mut &[u8] = &mut account_data.as_slice();

    let reserves = BondingCurve::deserialize_reader(sliced_data)?;

    let reserves = (
        reserves.real_token_reserves as u128,
        reserves.virtual_sol_reserves as u128,
        reserves.real_sol_reserves as u128,
    );

    let price: (u128, (u128, u128, u128)) = calculate_buy_price(sol_amount as u128, reserves);

    println!("Price: {:?}", price);
    let amount = (price.0) as u64;
    let fee_bps = sol_amount * 1 / 100;

    let buy_ix = buy_ix_with_program_id(
        PUMP_PROGRAM,
        BuyKeys {
            global: GLOBAL_STATE,
            fee_recipient: FEE_RECEPIENT,
            mint: token,
            bonding_curve: bonding_curve_pda,
            associated_bonding_curve: bonding_curve_ata,
            associated_user: signer_ata,
            user: main_signer.pubkey(),
            system_program: system_program::id(),
            token_program: spl_token::id(),
            rent: solana_program::sysvar::rent::id(),
            event_authority: EVENT_AUTH,
            program: PUMP_PROGRAM,
        },
        BuyIxArgs {
            amount,
            max_sol_cost: sol_amount + fee_bps,
        },
    )?;

    Ok([buy_ix].to_vec())
}

pub async fn generate_pump_multi_buy_ix(
    rpc_client: Arc<RpcClient>,
    token: Pubkey,
    sol_amount: u64,
    main_signer: Arc<Keypair>,
    reserves: (u128, u128, u128),
) -> eyre::Result<Vec<Instruction>> {
    let bonding_curve_pda = get_bonding_curve(token, &PUMP_PROGRAM);
    let bonding_curve_ata = get_associated_token_address(&bonding_curve_pda, &token);
    let signer_ata = get_associated_token_address(&main_signer.pubkey(), &token);

    let price: (u128, (u128, u128, u128)) = calculate_buy_price(sol_amount as u128, reserves);

    println!("Price: {:?}", price);
    let amount = (price.0) as u64;
    let fee_bps = sol_amount * 1 / 100;

    let buy_ix = buy_ix_with_program_id(
        PUMP_PROGRAM,
        BuyKeys {
            global: GLOBAL_STATE,
            fee_recipient: FEE_RECEPIENT,
            mint: token,
            bonding_curve: bonding_curve_pda,
            associated_bonding_curve: bonding_curve_ata,
            associated_user: signer_ata,
            user: main_signer.pubkey(),
            system_program: system_program::id(),
            token_program: spl_token::id(),
            rent: solana_program::sysvar::rent::id(),
            event_authority: EVENT_AUTH,
            program: PUMP_PROGRAM,
        },
        BuyIxArgs {
            amount,
            max_sol_cost: sol_amount + fee_bps,
        },
    )?;

    Ok([buy_ix].to_vec())
}

pub async fn generate_pump_sell_ix(
    token: Pubkey,
    token_amount: u64,
    main_signer: Arc<Keypair>,
) -> eyre::Result<Vec<Instruction>> {
    let bonding_curve_pda = get_bonding_curve(token, &PUMP_PROGRAM);
    let bonding_curve_ata = get_associated_token_address(&bonding_curve_pda, &token);
    let signer_ata = get_associated_token_address(&main_signer.pubkey(), &token);

    let sell_ix = sell_ix_with_program_id(
        PUMP_PROGRAM,
        SellKeys {
            global: GLOBAL_STATE,
            fee_recipient: FEE_RECEPIENT,
            mint: token,
            bonding_curve: bonding_curve_pda,
            associated_bonding_curve: bonding_curve_ata,
            associated_user: signer_ata,
            user: main_signer.pubkey(),
            system_program: system_program::id(),
            associated_token_program: spl_associated_token_account::id(),
            token_program: spl_token::id(),
            event_authority: EVENT_AUTH,
            program: PUMP_PROGRAM,
        },
        SellIxArgs {
            amount: token_amount,
            min_sol_output: 0,
        },
    )?;

    Ok([sell_ix].to_vec())
}

pub fn get_bonding_curve(mint: Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"bonding-curve", &mint.to_bytes()], program_id).0
}

pub fn calculate_buy_price(
    sol_amount: u128,
    bonding_curve: (u128, u128, u128),
) -> (u128, (u128, u128, u128)) {
    if sol_amount == 0 {
        return (0, bonding_curve);
    }

    let (virtual_sol_reserves, virtual_token_reserves, real_token_reserves) = bonding_curve;

    // Calculate the product of virtual reserves
    let product = virtual_sol_reserves.mul(virtual_token_reserves);
    // Update the virtual SOL reserves with the new amount of SOL added
    let new_sol_reserves = virtual_sol_reserves.add(sol_amount);
    // Calculate the new token amount based on the new virtual SOL reserves
    let new_token_amount = product.checked_div(new_sol_reserves).unwrap() + 1;
    // Determine the number of tokens to be given out
    let token_amount = virtual_token_reserves - new_token_amount;
    // Ensure the token amount does not exceed the real token reserves
    let token_amount = std::cmp::min(token_amount, real_token_reserves);

    // Update the real token reserves by subtracting the token amount to be given out
    let new_reserves = (
        new_sol_reserves,                   // Updated virtual SOL reserves
        virtual_token_reserves,             // Virtual token reserves remain the same
        real_token_reserves - token_amount, // Updated real token reserves
    );

    // Return the token amount to be given out and the new reserves
    (token_amount, new_reserves)
}

pub fn calculate_sell_price(
    token_amount: u128,
    bonding_curve: (u128, u128, u128),
) -> (u128, (u128, u128, u128)) {
    if token_amount == 0 {
        return (0, bonding_curve);
    }

    let (virtual_sol_reserves, virtual_token_reserves, real_sol_reserves) = bonding_curve;

    // Calculate the product of virtual reserves
    let product = virtual_sol_reserves.mul(virtual_token_reserves);
    // Update the virtual token reserves with the new amount of tokens removed
    let new_token_reserves = virtual_token_reserves.sub(token_amount);
    // Calculate the new SOL amount based on the new virtual token reserves
    let new_sol_amount = product.checked_div(new_token_reserves).unwrap();
    // Determine the amount of SOL to be given out
    let sol_amount = new_sol_amount - virtual_sol_reserves;

    // Update the reserves
    let new_reserves = (
        virtual_sol_reserves,           // Virtual SOL reserves remain the same
        new_token_reserves,             // Updated virtual token reserves
        real_sol_reserves - sol_amount, // Updated real SOL reserves
    );

    // Return the SOL amount to be given out and the new reserves
    (sol_amount, new_reserves)
}
