use std::{
    mem::size_of,
    ops::{Add, Mul},
    rc::Rc,
    str::FromStr,
    sync::Arc,
};

use anchor_client::Program;
use borsh::BorshDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::sol_to_lamports,
    program_error::ProgramError,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
};
use spl_associated_token_account::get_associated_token_address;

use crate::pumpfun::instructions::pumpfun_program::accounts::{Global, GlobalAccount};

use super::pumpfun_program::accounts::BondingCurve;

pub const GLOBAL_STATE: Pubkey = pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
pub const FEE_RECEPIENT: Pubkey = pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");
pub const EVENT_AUTH: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
pub const PUMP_PROGRAM: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BuyArgs {
    pub amount: u64,
    pub max_sol_amount: u64,
}

pub enum PumpFunInstructions {
    BuyArgs { amount: u64, max_sol_amount: u64 },
}

impl PumpFunInstructions {
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::BuyArgs {
                amount,
                max_sol_amount,
            } => {
                buf.push(9);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(&max_sol_amount.to_le_bytes());
            }
        }
        Ok(buf)
    }
}

pub async fn generate_pump_buy_ix(
    rpc_client: Arc<RpcClient>,
    token: Pubkey,
    amount: u64,
    main_signer: Arc<Keypair>,
) -> eyre::Result<Vec<Instruction>> {
    let bonding_curve_pda = get_bonding_curve(token, &PUMP_PROGRAM);
    let bonding_curve_ata = get_associated_token_address(&bonding_curve_pda, &token);
    let signer_ata = get_associated_token_address(&main_signer.pubkey(), &token);

    let account_data = rpc_client.get_account_data(&bonding_curve_pda).await?;
    let global_data = rpc_client.get_account_data(&GLOBAL_STATE).await?;

    let sliced_data: &mut &[u8] = &mut account_data.as_slice();
    let global_sliced: &mut &[u8] = &mut global_data.as_slice();

    let reserves = BondingCurve::deserialize_reader(sliced_data)?;
    let global = GlobalAccount::deserialize(global_sliced)?;

    let reserves = (
        reserves.real_token_reserves as u128,
        reserves.virtual_sol_reserves as u128,
        reserves.real_sol_reserves as u128,
    );

    let price: (u128, (u128, u128, u128)) = calculate_buy_price(amount as u128, reserves);

    let max_sol_amount = price.0 as u64;

    let arguments = PumpFunInstructions::BuyArgs {
        amount,
        max_sol_amount,
    }
    .pack()?;

    let accounts = vec![
        AccountMeta::new(GLOBAL_STATE, false),
        AccountMeta::new(FEE_RECEPIENT, false),
        AccountMeta::new(token, false),
        AccountMeta::new(bonding_curve_pda, false),
        AccountMeta::new(bonding_curve_ata, false),
        AccountMeta::new(signer_ata, false),
        AccountMeta::new(main_signer.pubkey(), true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::id(), false),
        AccountMeta::new_readonly(EVENT_AUTH, false),
        AccountMeta::new_readonly(PUMP_PROGRAM, false),
    ];

    let buy_ix = Instruction {
        program_id: PUMP_PROGRAM,
        accounts,
        data: arguments,
    };

    Ok([buy_ix].to_vec())
}

fn get_bonding_curve(mint: Pubkey, program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"bonding-curve", &mint.to_bytes()], program_id).0
}

pub fn generate_pump_sell_ix(
    token: Pubkey,
    main_signer: Keypair,
    amount: u64,
    min_sol_amount: u64,
    pump_program: Program<Rc<Keypair>>,
) -> eyre::Result<Vec<Instruction>> {
    let bonding_curve_pda = get_bonding_curve(token, &pump_program.id());
    let bonding_curve_ata = get_associated_token_address(&bonding_curve_pda, &token);
    let signer_ata = get_associated_token_address(&main_signer.pubkey(), &token);

    let arguments = PumpFunInstructions::BuyArgs {
        amount,
        max_sol_amount: min_sol_amount,
    }
    .pack()?;

    let accounts = vec![
        AccountMeta::new(GLOBAL_STATE, false),
        AccountMeta::new(FEE_RECEPIENT, false),
        AccountMeta::new(token, false),
        AccountMeta::new(bonding_curve_pda, false),
        AccountMeta::new(bonding_curve_ata, false),
        AccountMeta::new(signer_ata, false),
        AccountMeta::new(main_signer.pubkey(), true),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(EVENT_AUTH, false),
        AccountMeta::new_readonly(pump_program.id(), false),
    ];

    let sell_ix = Instruction {
        program_id: pump_program.id(),
        accounts,
        data: arguments,
    };

    Ok([sell_ix].to_vec())
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
