use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::Read;

use super::VIRTUAL_DAOS;
#[derive(Clone, Debug, PartialEq)]
pub enum VirtualXykProgramIx {
    BuyToken(BuyTokenIxArgs),
    Initialize(InitializeIxArgs),
    RedeemFees(RedeemFeesIxArgs),
    SellToken(SellTokenIxArgs),
}
impl VirtualXykProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            BUY_TOKEN_IX_DISCM => Ok(Self::BuyToken(BuyTokenIxArgs::deserialize(&mut reader)?)),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(
                &mut reader,
            )?)),
            REDEEM_FEES_IX_DISCM => Ok(Self::RedeemFees(RedeemFeesIxArgs::deserialize(
                &mut reader,
            )?)),
            SELL_TOKEN_IX_DISCM => Ok(Self::SellToken(SellTokenIxArgs::deserialize(&mut reader)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::BuyToken(args) => {
                writer.write_all(&BUY_TOKEN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RedeemFees(args) => {
                writer.write_all(&REDEEM_FEES_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SellToken(args) => {
                writer.write_all(&SELL_TOKEN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const BUY_TOKEN_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct BuyTokenAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub curve: &'me AccountInfo<'info>,
    pub signer_token_ata: &'me AccountInfo<'info>,
    pub signer_funding_ata: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub funding_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub funding_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyTokenKeys {
    pub signer: Pubkey,
    pub depositor: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub curve: Pubkey,
    pub signer_token_ata: Pubkey,
    pub signer_funding_ata: Pubkey,
    pub token_vault: Pubkey,
    pub funding_vault: Pubkey,
    pub token_program: Pubkey,
    pub funding_token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<BuyTokenAccounts<'_, '_>> for BuyTokenKeys {
    fn from(accounts: BuyTokenAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor: *accounts.depositor.key,
            token_mint: *accounts.token_mint.key,
            funding_mint: *accounts.funding_mint.key,
            curve: *accounts.curve.key,
            signer_token_ata: *accounts.signer_token_ata.key,
            signer_funding_ata: *accounts.signer_funding_ata.key,
            token_vault: *accounts.token_vault.key,
            funding_vault: *accounts.funding_vault.key,
            token_program: *accounts.token_program.key,
            funding_token_program: *accounts.funding_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<BuyTokenKeys> for [AccountMeta; BUY_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BUY_TOKEN_IX_ACCOUNTS_LEN]> for BuyTokenKeys {
    fn from(pubkeys: [Pubkey; BUY_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor: pubkeys[1],
            token_mint: pubkeys[2],
            funding_mint: pubkeys[3],
            curve: pubkeys[4],
            signer_token_ata: pubkeys[5],
            signer_funding_ata: pubkeys[6],
            token_vault: pubkeys[7],
            funding_vault: pubkeys[8],
            token_program: pubkeys[9],
            funding_token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
        }
    }
}
impl<'info> From<BuyTokenAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor.clone(),
            accounts.token_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.curve.clone(),
            accounts.signer_token_ata.clone(),
            accounts.signer_funding_ata.clone(),
            accounts.token_vault.clone(),
            accounts.funding_vault.clone(),
            accounts.token_program.clone(),
            accounts.funding_token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_TOKEN_IX_ACCOUNTS_LEN]>
    for BuyTokenAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; BUY_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            depositor: &arr[1],
            token_mint: &arr[2],
            funding_mint: &arr[3],
            curve: &arr[4],
            signer_token_ata: &arr[5],
            signer_funding_ata: &arr[6],
            token_vault: &arr[7],
            funding_vault: &arr[8],
            token_program: &arr[9],
            funding_token_program: &arr[10],
            associated_token_program: &arr[11],
        }
    }
}
pub const BUY_TOKEN_IX_DISCM: [u8; 8] = [138, 127, 14, 91, 38, 87, 115, 105];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyTokenIxArgs {
    pub funding_amount: u64,
    pub min_token_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyTokenIxData(pub BuyTokenIxArgs);
impl From<BuyTokenIxArgs> for BuyTokenIxData {
    fn from(args: BuyTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BuyTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BUY_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BuyTokenIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_TOKEN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_token_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyTokenKeys,
    args: BuyTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_token_ix(keys: BuyTokenKeys, args: BuyTokenIxArgs) -> std::io::Result<Instruction> {
    buy_token_ix_with_program_id(VIRTUAL_DAOS, keys, args)
}
pub fn buy_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyTokenAccounts<'_, '_>,
    args: BuyTokenIxArgs,
) -> ProgramResult {
    let keys: BuyTokenKeys = accounts.into();
    let ix = buy_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_token_invoke(accounts: BuyTokenAccounts<'_, '_>, args: BuyTokenIxArgs) -> ProgramResult {
    buy_token_invoke_with_program_id(VIRTUAL_DAOS, accounts, args)
}
pub fn buy_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyTokenAccounts<'_, '_>,
    args: BuyTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyTokenKeys = accounts.into();
    let ix = buy_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_token_invoke_signed(
    accounts: BuyTokenAccounts<'_, '_>,
    args: BuyTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_token_invoke_signed_with_program_id(VIRTUAL_DAOS, accounts, args, seeds)
}
pub fn buy_token_verify_account_keys(
    accounts: BuyTokenAccounts<'_, '_>,
    keys: BuyTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor.key, keys.depositor),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.curve.key, keys.curve),
        (*accounts.signer_token_ata.key, keys.signer_token_ata),
        (*accounts.signer_funding_ata.key, keys.signer_funding_ata),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.funding_vault.key, keys.funding_vault),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.funding_token_program.key,
            keys.funding_token_program,
        ),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_token_verify_writable_privileges<'me, 'info>(
    accounts: BuyTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.curve,
        accounts.signer_token_ata,
        accounts.signer_funding_ata,
        accounts.token_vault,
        accounts.funding_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_token_verify_signer_privileges<'me, 'info>(
    accounts: BuyTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_token_verify_account_privileges<'me, 'info>(
    accounts: BuyTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_token_verify_writable_privileges(accounts)?;
    buy_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub depositor: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub curve: &'me AccountInfo<'info>,
    pub depositor_token_ata: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub depositor: Pubkey,
    pub payer: Pubkey,
    pub fee_authority: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub curve: Pubkey,
    pub depositor_token_ata: Pubkey,
    pub token_vault: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            depositor: *accounts.depositor.key,
            payer: *accounts.payer.key,
            fee_authority: *accounts.fee_authority.key,
            token_mint: *accounts.token_mint.key,
            funding_mint: *accounts.funding_mint.key,
            curve: *accounts.curve.key,
            depositor_token_ata: *accounts.depositor_token_ata.key,
            token_vault: *accounts.token_vault.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            depositor: pubkeys[0],
            payer: pubkeys[1],
            fee_authority: pubkeys[2],
            token_mint: pubkeys[3],
            funding_mint: pubkeys[4],
            curve: pubkeys[5],
            depositor_token_ata: pubkeys[6],
            token_vault: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.depositor.clone(),
            accounts.payer.clone(),
            accounts.fee_authority.clone(),
            accounts.token_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.curve.clone(),
            accounts.depositor_token_ata.clone(),
            accounts.token_vault.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            depositor: &arr[0],
            payer: &arr[1],
            fee_authority: &arr[2],
            token_mint: &arr[3],
            funding_mint: &arr[4],
            curve: &arr[5],
            depositor_token_ata: &arr[6],
            token_vault: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub virtual_funding_amount: u64,
    pub deposit: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys, args: InitializeIxArgs) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(VIRTUAL_DAOS, keys, args)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(VIRTUAL_DAOS, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(VIRTUAL_DAOS, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.depositor.key, keys.depositor),
        (*accounts.payer.key, keys.payer),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.curve.key, keys.curve),
        (*accounts.depositor_token_ata.key, keys.depositor_token_ata),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.depositor,
        accounts.payer,
        accounts.curve,
        accounts.depositor_token_ata,
        accounts.token_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.depositor, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_FEES_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RedeemFeesAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub curve: &'me AccountInfo<'info>,
    pub signer_funding_ata: &'me AccountInfo<'info>,
    pub funding_vault: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub funding_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemFeesKeys {
    pub signer: Pubkey,
    pub depositor: Pubkey,
    pub funding_mint: Pubkey,
    pub curve: Pubkey,
    pub signer_funding_ata: Pubkey,
    pub funding_vault: Pubkey,
    pub associated_token_program: Pubkey,
    pub funding_token_program: Pubkey,
}
impl From<RedeemFeesAccounts<'_, '_>> for RedeemFeesKeys {
    fn from(accounts: RedeemFeesAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor: *accounts.depositor.key,
            funding_mint: *accounts.funding_mint.key,
            curve: *accounts.curve.key,
            signer_funding_ata: *accounts.signer_funding_ata.key,
            funding_vault: *accounts.funding_vault.key,
            associated_token_program: *accounts.associated_token_program.key,
            funding_token_program: *accounts.funding_token_program.key,
        }
    }
}
impl From<RedeemFeesKeys> for [AccountMeta; REDEEM_FEES_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemFeesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDEEM_FEES_IX_ACCOUNTS_LEN]> for RedeemFeesKeys {
    fn from(pubkeys: [Pubkey; REDEEM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor: pubkeys[1],
            funding_mint: pubkeys[2],
            curve: pubkeys[3],
            signer_funding_ata: pubkeys[4],
            funding_vault: pubkeys[5],
            associated_token_program: pubkeys[6],
            funding_token_program: pubkeys[7],
        }
    }
}
impl<'info> From<RedeemFeesAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDEEM_FEES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RedeemFeesAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor.clone(),
            accounts.funding_mint.clone(),
            accounts.curve.clone(),
            accounts.signer_funding_ata.clone(),
            accounts.funding_vault.clone(),
            accounts.associated_token_program.clone(),
            accounts.funding_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_FEES_IX_ACCOUNTS_LEN]>
    for RedeemFeesAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_FEES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            depositor: &arr[1],
            funding_mint: &arr[2],
            curve: &arr[3],
            signer_funding_ata: &arr[4],
            funding_vault: &arr[5],
            associated_token_program: &arr[6],
            funding_token_program: &arr[7],
        }
    }
}
pub const REDEEM_FEES_IX_DISCM: [u8; 8] = [215, 39, 180, 41, 173, 46, 248, 220];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RedeemFeesIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemFeesIxData(pub RedeemFeesIxArgs);
impl From<RedeemFeesIxArgs> for RedeemFeesIxData {
    fn from(args: RedeemFeesIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemFeesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_FEES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDEEM_FEES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RedeemFeesIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_FEES_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_fees_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemFeesKeys,
    args: RedeemFeesIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_FEES_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemFeesIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_fees_ix(
    keys: RedeemFeesKeys,
    args: RedeemFeesIxArgs,
) -> std::io::Result<Instruction> {
    redeem_fees_ix_with_program_id(VIRTUAL_DAOS, keys, args)
}
pub fn redeem_fees_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemFeesAccounts<'_, '_>,
    args: RedeemFeesIxArgs,
) -> ProgramResult {
    let keys: RedeemFeesKeys = accounts.into();
    let ix = redeem_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_fees_invoke(
    accounts: RedeemFeesAccounts<'_, '_>,
    args: RedeemFeesIxArgs,
) -> ProgramResult {
    redeem_fees_invoke_with_program_id(VIRTUAL_DAOS, accounts, args)
}
pub fn redeem_fees_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemFeesAccounts<'_, '_>,
    args: RedeemFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemFeesKeys = accounts.into();
    let ix = redeem_fees_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_fees_invoke_signed(
    accounts: RedeemFeesAccounts<'_, '_>,
    args: RedeemFeesIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_fees_invoke_signed_with_program_id(VIRTUAL_DAOS, accounts, args, seeds)
}
pub fn redeem_fees_verify_account_keys(
    accounts: RedeemFeesAccounts<'_, '_>,
    keys: RedeemFeesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor.key, keys.depositor),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.curve.key, keys.curve),
        (*accounts.signer_funding_ata.key, keys.signer_funding_ata),
        (*accounts.funding_vault.key, keys.funding_vault),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (
            *accounts.funding_token_program.key,
            keys.funding_token_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_fees_verify_writable_privileges<'me, 'info>(
    accounts: RedeemFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.curve,
        accounts.signer_funding_ata,
        accounts.funding_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_fees_verify_signer_privileges<'me, 'info>(
    accounts: RedeemFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_fees_verify_account_privileges<'me, 'info>(
    accounts: RedeemFeesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_fees_verify_writable_privileges(accounts)?;
    redeem_fees_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_TOKEN_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SellTokenAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub curve: &'me AccountInfo<'info>,
    pub signer_token_ata: &'me AccountInfo<'info>,
    pub signer_funding_ata: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub funding_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub funding_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellTokenKeys {
    pub signer: Pubkey,
    pub depositor: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub curve: Pubkey,
    pub signer_token_ata: Pubkey,
    pub signer_funding_ata: Pubkey,
    pub token_vault: Pubkey,
    pub funding_vault: Pubkey,
    pub token_program: Pubkey,
    pub funding_token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<SellTokenAccounts<'_, '_>> for SellTokenKeys {
    fn from(accounts: SellTokenAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            depositor: *accounts.depositor.key,
            token_mint: *accounts.token_mint.key,
            funding_mint: *accounts.funding_mint.key,
            curve: *accounts.curve.key,
            signer_token_ata: *accounts.signer_token_ata.key,
            signer_funding_ata: *accounts.signer_funding_ata.key,
            token_vault: *accounts.token_vault.key,
            funding_vault: *accounts.funding_vault.key,
            token_program: *accounts.token_program.key,
            funding_token_program: *accounts.funding_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<SellTokenKeys> for [AccountMeta; SELL_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: SellTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SELL_TOKEN_IX_ACCOUNTS_LEN]> for SellTokenKeys {
    fn from(pubkeys: [Pubkey; SELL_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            depositor: pubkeys[1],
            token_mint: pubkeys[2],
            funding_mint: pubkeys[3],
            curve: pubkeys[4],
            signer_token_ata: pubkeys[5],
            signer_funding_ata: pubkeys[6],
            token_vault: pubkeys[7],
            funding_vault: pubkeys[8],
            token_program: pubkeys[9],
            funding_token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
        }
    }
}
impl<'info> From<SellTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; SELL_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SellTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.depositor.clone(),
            accounts.token_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.curve.clone(),
            accounts.signer_token_ata.clone(),
            accounts.signer_funding_ata.clone(),
            accounts.token_vault.clone(),
            accounts.funding_vault.clone(),
            accounts.token_program.clone(),
            accounts.funding_token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_TOKEN_IX_ACCOUNTS_LEN]>
    for SellTokenAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SELL_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            depositor: &arr[1],
            token_mint: &arr[2],
            funding_mint: &arr[3],
            curve: &arr[4],
            signer_token_ata: &arr[5],
            signer_funding_ata: &arr[6],
            token_vault: &arr[7],
            funding_vault: &arr[8],
            token_program: &arr[9],
            funding_token_program: &arr[10],
            associated_token_program: &arr[11],
        }
    }
}
pub const SELL_TOKEN_IX_DISCM: [u8; 8] = [109, 61, 40, 187, 230, 176, 135, 174];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellTokenIxArgs {
    pub amount: u64,
    pub min_funding_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellTokenIxData(pub SellTokenIxArgs);
impl From<SellTokenIxArgs> for SellTokenIxData {
    fn from(args: SellTokenIxArgs) -> Self {
        Self(args)
    }
}
impl SellTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SELL_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SellTokenIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_TOKEN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_token_ix_with_program_id(
    program_id: Pubkey,
    keys: SellTokenKeys,
    args: SellTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_token_ix(keys: SellTokenKeys, args: SellTokenIxArgs) -> std::io::Result<Instruction> {
    sell_token_ix_with_program_id(VIRTUAL_DAOS, keys, args)
}
pub fn sell_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
) -> ProgramResult {
    let keys: SellTokenKeys = accounts.into();
    let ix = sell_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_token_invoke(
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
) -> ProgramResult {
    sell_token_invoke_with_program_id(VIRTUAL_DAOS, accounts, args)
}
pub fn sell_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellTokenKeys = accounts.into();
    let ix = sell_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_token_invoke_signed(
    accounts: SellTokenAccounts<'_, '_>,
    args: SellTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_token_invoke_signed_with_program_id(VIRTUAL_DAOS, accounts, args, seeds)
}
pub fn sell_token_verify_account_keys(
    accounts: SellTokenAccounts<'_, '_>,
    keys: SellTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.depositor.key, keys.depositor),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.curve.key, keys.curve),
        (*accounts.signer_token_ata.key, keys.signer_token_ata),
        (*accounts.signer_funding_ata.key, keys.signer_funding_ata),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.funding_vault.key, keys.funding_vault),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.funding_token_program.key,
            keys.funding_token_program,
        ),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_token_verify_writable_privileges<'me, 'info>(
    accounts: SellTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.curve,
        accounts.signer_token_ata,
        accounts.signer_funding_ata,
        accounts.token_vault,
        accounts.funding_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_token_verify_signer_privileges<'me, 'info>(
    accounts: SellTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_token_verify_account_privileges<'me, 'info>(
    accounts: SellTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_token_verify_writable_privileges(accounts)?;
    sell_token_verify_signer_privileges(accounts)?;
    Ok(())
}
