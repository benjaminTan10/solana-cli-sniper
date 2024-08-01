use std::{error::Error, str::FromStr};

use solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_program, sysvar};
use spl_associated_token_account::get_associated_token_address;

use crate::pumpfun::instructions::{
    instructions::{get_bonding_curve, EVENT_AUTH, GLOBAL_STATE, PUMP_PROGRAM},
    pumpfun_program::instructions::{create_ix_with_program_id, CreateIxArgs, CreateKeys},
};

use super::pump_lut::{METAPLEX_METADATA, MINT_AUTH};

pub fn create_ix_accounts(
    mint: Pubkey,
    wallet: Pubkey,
) -> Result<CreateKeys, Box<dyn Error + Send>> {
    let bonding_curve_pda = get_bonding_curve(mint, &PUMP_PROGRAM);
    let bonding_curve_ata = get_associated_token_address(&bonding_curve_pda, &mint);
    let signer_ata = get_associated_token_address(&wallet, &mint);

    let keys = CreateKeys {
        mint,
        mint_authority: MINT_AUTH,
        bonding_curve: bonding_curve_pda,
        associated_bonding_curve: bonding_curve_ata,
        global: GLOBAL_STATE,
        mpl_token_metadata: METAPLEX_METADATA,
        metadata: metadata_pdas(mint).0,
        user: wallet,
        system_program: system_program::id(),
        token_program: spl_token::id(),
        associated_token_program: spl_associated_token_account::id(),
        rent: sysvar::rent::id(),
        event_authority: EVENT_AUTH,
        program: PUMP_PROGRAM,
    };

    Ok(keys)
}

pub fn metadata_pdas(mint: Pubkey) -> (Pubkey, Pubkey) {
    let metadata = Pubkey::find_program_address(
        &[b"metadata", &METAPLEX_METADATA.to_bytes(), &mint.to_bytes()],
        &Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
    )
    .0;
    let metadata_ata = get_associated_token_address(&metadata, &mint);

    (metadata, metadata_ata)
}

pub fn token_create_ix(mint: Pubkey, wallet: Pubkey, metadata: CreateIxArgs) -> Vec<Instruction> {
    let keys = create_ix_accounts(mint, wallet).unwrap();
    let ix = create_ix_with_program_id(PUMP_PROGRAM, keys, metadata).unwrap();

    vec![ix]
}
