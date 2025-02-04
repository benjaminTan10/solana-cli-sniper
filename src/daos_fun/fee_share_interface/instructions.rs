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

use super::FEE_SHARED;
#[derive(Clone, Debug, PartialEq)]
pub enum FeeShareProgramIx {
    CurveWithdraw(CurveWithdrawIxArgs),
    Initialize(InitializeIxArgs),
    Redeem(RedeemIxArgs),
}
impl FeeShareProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            CURVE_WITHDRAW_IX_DISCM => Ok(Self::CurveWithdraw(CurveWithdrawIxArgs::deserialize(
                &mut reader,
            )?)),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(
                &mut reader,
            )?)),
            REDEEM_IX_DISCM => Ok(Self::Redeem(RedeemIxArgs::deserialize(&mut reader)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CurveWithdraw(args) => {
                writer.write_all(&CURVE_WITHDRAW_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Redeem(args) => {
                writer.write_all(&REDEEM_IX_DISCM)?;
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
pub const CURVE_WITHDRAW_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct CurveWithdrawAccounts<'me, 'info> {
    pub curve: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub depositor: &'me AccountInfo<'info>,
    pub fee_share_state: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub funding_ata: &'me AccountInfo<'info>,
    pub curve_funding_ata: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub funding_token_program: &'me AccountInfo<'info>,
    pub virtual_xyk_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CurveWithdrawKeys {
    pub curve: Pubkey,
    pub token_mint: Pubkey,
    pub depositor: Pubkey,
    pub fee_share_state: Pubkey,
    pub funding_mint: Pubkey,
    pub funding_ata: Pubkey,
    pub curve_funding_ata: Pubkey,
    pub associated_token_program: Pubkey,
    pub funding_token_program: Pubkey,
    pub virtual_xyk_program: Pubkey,
}
impl From<CurveWithdrawAccounts<'_, '_>> for CurveWithdrawKeys {
    fn from(accounts: CurveWithdrawAccounts) -> Self {
        Self {
            curve: *accounts.curve.key,
            token_mint: *accounts.token_mint.key,
            depositor: *accounts.depositor.key,
            fee_share_state: *accounts.fee_share_state.key,
            funding_mint: *accounts.funding_mint.key,
            funding_ata: *accounts.funding_ata.key,
            curve_funding_ata: *accounts.curve_funding_ata.key,
            associated_token_program: *accounts.associated_token_program.key,
            funding_token_program: *accounts.funding_token_program.key,
            virtual_xyk_program: *accounts.virtual_xyk_program.key,
        }
    }
}
impl From<CurveWithdrawKeys> for [AccountMeta; CURVE_WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: CurveWithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.depositor,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_share_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_funding_ata,
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
            AccountMeta {
                pubkey: keys.virtual_xyk_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CURVE_WITHDRAW_IX_ACCOUNTS_LEN]> for CurveWithdrawKeys {
    fn from(pubkeys: [Pubkey; CURVE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curve: pubkeys[0],
            token_mint: pubkeys[1],
            depositor: pubkeys[2],
            fee_share_state: pubkeys[3],
            funding_mint: pubkeys[4],
            funding_ata: pubkeys[5],
            curve_funding_ata: pubkeys[6],
            associated_token_program: pubkeys[7],
            funding_token_program: pubkeys[8],
            virtual_xyk_program: pubkeys[9],
        }
    }
}
impl<'info> From<CurveWithdrawAccounts<'_, 'info>>
    for [AccountInfo<'info>; CURVE_WITHDRAW_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CurveWithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.curve.clone(),
            accounts.token_mint.clone(),
            accounts.depositor.clone(),
            accounts.fee_share_state.clone(),
            accounts.funding_mint.clone(),
            accounts.funding_ata.clone(),
            accounts.curve_funding_ata.clone(),
            accounts.associated_token_program.clone(),
            accounts.funding_token_program.clone(),
            accounts.virtual_xyk_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CURVE_WITHDRAW_IX_ACCOUNTS_LEN]>
    for CurveWithdrawAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CURVE_WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            curve: &arr[0],
            token_mint: &arr[1],
            depositor: &arr[2],
            fee_share_state: &arr[3],
            funding_mint: &arr[4],
            funding_ata: &arr[5],
            curve_funding_ata: &arr[6],
            associated_token_program: &arr[7],
            funding_token_program: &arr[8],
            virtual_xyk_program: &arr[9],
        }
    }
}
pub const CURVE_WITHDRAW_IX_DISCM: [u8; 8] = [172, 159, 49, 255, 19, 177, 246, 44];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CurveWithdrawIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CurveWithdrawIxData(pub CurveWithdrawIxArgs);
impl From<CurveWithdrawIxArgs> for CurveWithdrawIxData {
    fn from(args: CurveWithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl CurveWithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CURVE_WITHDRAW_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CURVE_WITHDRAW_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CurveWithdrawIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CURVE_WITHDRAW_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn curve_withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: CurveWithdrawKeys,
    args: CurveWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CURVE_WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    let data: CurveWithdrawIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn curve_withdraw_ix(
    keys: CurveWithdrawKeys,
    args: CurveWithdrawIxArgs,
) -> std::io::Result<Instruction> {
    curve_withdraw_ix_with_program_id(FEE_SHARED, keys, args)
}
pub fn curve_withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CurveWithdrawAccounts<'_, '_>,
    args: CurveWithdrawIxArgs,
) -> ProgramResult {
    let keys: CurveWithdrawKeys = accounts.into();
    let ix = curve_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn curve_withdraw_invoke(
    accounts: CurveWithdrawAccounts<'_, '_>,
    args: CurveWithdrawIxArgs,
) -> ProgramResult {
    curve_withdraw_invoke_with_program_id(FEE_SHARED, accounts, args)
}
pub fn curve_withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CurveWithdrawAccounts<'_, '_>,
    args: CurveWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CurveWithdrawKeys = accounts.into();
    let ix = curve_withdraw_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn curve_withdraw_invoke_signed(
    accounts: CurveWithdrawAccounts<'_, '_>,
    args: CurveWithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    curve_withdraw_invoke_signed_with_program_id(FEE_SHARED, accounts, args, seeds)
}
pub fn curve_withdraw_verify_account_keys(
    accounts: CurveWithdrawAccounts<'_, '_>,
    keys: CurveWithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.curve.key, keys.curve),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.depositor.key, keys.depositor),
        (*accounts.fee_share_state.key, keys.fee_share_state),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.funding_ata.key, keys.funding_ata),
        (*accounts.curve_funding_ata.key, keys.curve_funding_ata),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (
            *accounts.funding_token_program.key,
            keys.funding_token_program,
        ),
        (*accounts.virtual_xyk_program.key, keys.virtual_xyk_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn curve_withdraw_verify_writable_privileges<'me, 'info>(
    accounts: CurveWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.curve,
        accounts.fee_share_state,
        accounts.funding_ata,
        accounts.curve_funding_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn curve_withdraw_verify_account_privileges<'me, 'info>(
    accounts: CurveWithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    curve_withdraw_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub fee_share_state: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub payer: Pubkey,
    pub dao_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub fee_share_state: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            dao_mint: *accounts.dao_mint.key,
            funding_mint: *accounts.funding_mint.key,
            fee_share_state: *accounts.fee_share_state.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_share_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            dao_mint: pubkeys[1],
            funding_mint: pubkeys[2],
            fee_share_state: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.dao_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.fee_share_state.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            dao_mint: &arr[1],
            funding_mint: &arr[2],
            fee_share_state: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub creator: Pubkey,
    pub referrer: Pubkey,
    pub platform: Pubkey,
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
    initialize_ix_with_program_id(FEE_SHARED, keys, args)
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
    initialize_invoke_with_program_id(FEE_SHARED, accounts, args)
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
    initialize_invoke_signed_with_program_id(FEE_SHARED, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.fee_share_state.key, keys.fee_share_state),
        (*accounts.system_program.key, keys.system_program),
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
    for should_be_writable in [accounts.payer, accounts.fee_share_state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer, accounts.dao_mint] {
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
pub const REDEEM_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RedeemAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub fee_share_state: &'me AccountInfo<'info>,
    pub signer_funding_ata: &'me AccountInfo<'info>,
    pub vault_funding_ata: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemKeys {
    pub signer: Pubkey,
    pub funding_mint: Pubkey,
    pub token_mint: Pubkey,
    pub fee_share_state: Pubkey,
    pub signer_funding_ata: Pubkey,
    pub vault_funding_ata: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<RedeemAccounts<'_, '_>> for RedeemKeys {
    fn from(accounts: RedeemAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            funding_mint: *accounts.funding_mint.key,
            token_mint: *accounts.token_mint.key,
            fee_share_state: *accounts.fee_share_state.key,
            signer_funding_ata: *accounts.signer_funding_ata.key,
            vault_funding_ata: *accounts.vault_funding_ata.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<RedeemKeys> for [AccountMeta; REDEEM_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_share_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer_funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_funding_ata,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; REDEEM_IX_ACCOUNTS_LEN]> for RedeemKeys {
    fn from(pubkeys: [Pubkey; REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            funding_mint: pubkeys[1],
            token_mint: pubkeys[2],
            fee_share_state: pubkeys[3],
            signer_funding_ata: pubkeys[4],
            vault_funding_ata: pubkeys[5],
            token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
        }
    }
}
impl<'info> From<RedeemAccounts<'_, 'info>> for [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN] {
    fn from(accounts: RedeemAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.funding_mint.clone(),
            accounts.token_mint.clone(),
            accounts.fee_share_state.clone(),
            accounts.signer_funding_ata.clone(),
            accounts.vault_funding_ata.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN]>
    for RedeemAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            funding_mint: &arr[1],
            token_mint: &arr[2],
            fee_share_state: &arr[3],
            signer_funding_ata: &arr[4],
            vault_funding_ata: &arr[5],
            token_program: &arr[6],
            associated_token_program: &arr[7],
        }
    }
}
pub const REDEEM_IX_DISCM: [u8; 8] = [184, 12, 86, 149, 70, 196, 97, 225];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RedeemIxArgs {
    pub entity: u8,
    pub amount: Option<u64>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemIxData(pub RedeemIxArgs);
impl From<RedeemIxArgs> for RedeemIxData {
    fn from(args: RedeemIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDEEM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RedeemIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemKeys,
    args: RedeemIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_ix(keys: RedeemKeys, args: RedeemIxArgs) -> std::io::Result<Instruction> {
    redeem_ix_with_program_id(FEE_SHARED, keys, args)
}
pub fn redeem_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemAccounts<'_, '_>,
    args: RedeemIxArgs,
) -> ProgramResult {
    let keys: RedeemKeys = accounts.into();
    let ix = redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_invoke(accounts: RedeemAccounts<'_, '_>, args: RedeemIxArgs) -> ProgramResult {
    redeem_invoke_with_program_id(FEE_SHARED, accounts, args)
}
pub fn redeem_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemAccounts<'_, '_>,
    args: RedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemKeys = accounts.into();
    let ix = redeem_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_invoke_signed(
    accounts: RedeemAccounts<'_, '_>,
    args: RedeemIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_invoke_signed_with_program_id(FEE_SHARED, accounts, args, seeds)
}
pub fn redeem_verify_account_keys(
    accounts: RedeemAccounts<'_, '_>,
    keys: RedeemKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.fee_share_state.key, keys.fee_share_state),
        (*accounts.signer_funding_ata.key, keys.signer_funding_ata),
        (*accounts.vault_funding_ata.key, keys.vault_funding_ata),
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
pub fn redeem_verify_writable_privileges<'me, 'info>(
    accounts: RedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.fee_share_state,
        accounts.signer_funding_ata,
        accounts.vault_funding_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_verify_signer_privileges<'me, 'info>(
    accounts: RedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_verify_account_privileges<'me, 'info>(
    accounts: RedeemAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_verify_writable_privileges(accounts)?;
    redeem_verify_signer_privileges(accounts)?;
    Ok(())
}
