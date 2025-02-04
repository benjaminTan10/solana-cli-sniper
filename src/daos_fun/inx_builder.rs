use anchor_lang::prelude::ProgramError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use spl_associated_token_account::{
    get_associated_token_address, get_associated_token_address_with_program_id,
};
use spl_token::instruction::sync_native;

use super::virtual_xyk_interface::{buy_token_ix_with_program_id, BuyTokenIxArgs, BuyTokenKeys};
use crate::app::config_init::get_config;
use crate::daos_fun::virtual_xyk_interface::{
    sell_token_ix_with_program_id, SellTokenIxArgs, SellTokenKeys,
};
use crate::raydium_amm::swap::instructions::SOLC_MINT;
use solana_sdk::system_instruction::transfer;

pub const DAOS_PROGRAM: Pubkey = pubkey!("5jnapfrAN47UYkLkEf7HnprPPBCQLvkYWGZDeKkaP5hv");
pub const DAOS_BURNED_PROGRAM: Pubkey = pubkey!("4FqThZWv3QKWkSyXCDmATpWkpEiCHq5yhkdGWpSEDAZM");
pub const FUND_RAISE_PROGRAM: Pubkey = pubkey!("ETK5PUmiqVDRsd1TPFqCu84bsrLNG4YySZND96PEjW97");

pub fn create_buy_instruction(
    program_id: &Pubkey,
    signer: &Pubkey,
    token_mint: &Pubkey,
    token_mint_program: &Pubkey,
    amount: u64,
    min_token_amount: u64,
) -> Result<Vec<Instruction>, ProgramError> {
    let depositor = Pubkey::find_program_address(
        &[b"state".as_ref(), token_mint.as_ref()],
        &DAOS_BURNED_PROGRAM,
    )
    .0;
    let (curve_pda, _) = Pubkey::find_program_address(&[b"curve", depositor.as_ref()], program_id);
    // These match the JS y() function which creates ATAs
    let signer_token_ata =
        get_associated_token_address_with_program_id(signer, token_mint, token_mint_program);
    let signer_funding_ata = get_associated_token_address(signer, &SOLC_MINT);
    let token_vault =
        get_associated_token_address_with_program_id(&curve_pda, token_mint, token_mint_program);
    let funding_vault = get_associated_token_address(&curve_pda, &SOLC_MINT);

    let account =
        create_associated_token_account_idempotent(&signer, &signer, &SOLC_MINT, &spl_token::id());
    let sol_amount = transfer(signer, &signer_funding_ata, amount);

    let sol_native = sync_native(&spl_token::id(), &signer_funding_ata)?;

    let accounts = BuyTokenKeys {
        signer: *signer,
        depositor: depositor,
        token_mint: *token_mint,
        funding_mint: SOLC_MINT,
        curve: curve_pda,
        signer_funding_ata,
        signer_token_ata,
        token_vault,
        funding_vault,
        token_program: *token_mint_program,
        funding_token_program: spl_token::id(),
        associated_token_program: spl_associated_token_account::id(),
    };

    println!("{accounts:#?}");

    let args = BuyTokenIxArgs {
        funding_amount: amount,
        min_token_amount: min_token_amount,
    };

    let mut inxs = vec![];
    let inx = buy_token_ix_with_program_id(DAOS_PROGRAM, accounts, args)?;

    inxs.push(account);
    inxs.push(sol_amount);
    inxs.push(sol_native);
    inxs.push(inx);

    Ok(inxs)
}

pub async fn create_sell_instruction(
    program_id: &Pubkey,
    signer: &Pubkey,
    token_mint: &Pubkey,
    token_mint_program: &Pubkey,
    amount: u64,
    min_token_amount: u64,
) -> Result<Instruction, ProgramError> {
    let config = get_config().await.unwrap();
    let rpc_client = RpcClient::new(config.network.rpc_url);
    let depositor = Pubkey::find_program_address(
        &[b"state".as_ref(), token_mint.as_ref()],
        &DAOS_BURNED_PROGRAM,
    )
    .0;
    let (curve_pda, _) = Pubkey::find_program_address(&[b"curve", depositor.as_ref()], program_id);
    // These match the JS y() function which creates ATAs
    let signer_token_ata =
        get_associated_token_address_with_program_id(signer, token_mint, token_mint_program);
    let signer_funding_ata = get_associated_token_address(signer, &SOLC_MINT);
    let token_vault =
        get_associated_token_address_with_program_id(&curve_pda, token_mint, token_mint_program);
    let funding_vault = get_associated_token_address(&curve_pda, &SOLC_MINT);

    let tokens_amount = rpc_client
        .get_token_account_balance(&signer_token_ata)
        .await
        .unwrap();

    let amount = tokens_amount.amount.parse::<u64>().unwrap() * amount / 100;

    let accounts = SellTokenKeys {
        signer: *signer,
        depositor: depositor,
        token_mint: *token_mint,
        funding_mint: SOLC_MINT,
        curve: curve_pda,
        signer_funding_ata,
        signer_token_ata,
        token_vault,
        funding_vault,
        token_program: *token_mint_program,
        funding_token_program: spl_token::id(),
        associated_token_program: spl_associated_token_account::id(),
    };

    println!("Sell: {accounts:#?}");

    let args = SellTokenIxArgs {
        amount,
        min_funding_amount: min_token_amount,
    };

    let inx = sell_token_ix_with_program_id(DAOS_PROGRAM, accounts, args)?;

    Ok(inx)
}

// Example Instruction enum
#[derive(Debug)]
pub enum CurveInstruction {
    BuyToken {
        amount: u64,     // t in JS
        min_amount: u64, // r in JS
    },
}

impl CurveInstruction {
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(32);
        match self {
            Self::BuyToken { amount, min_amount } => {
                // Anchor discriminator for "buy_token" - first 8 bytes
                buf.extend_from_slice(
                    &anchor_lang::solana_program::hash::hash(b"global:buy_token").to_bytes()[..8],
                );
                // Amount
                buf.extend_from_slice(&amount.to_le_bytes());
                // Min amount
                buf.extend_from_slice(&min_amount.to_le_bytes());
            }
        }
        Ok(buf)
    }
}
