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

use crate::daos_fun::inx_builder::DAOS_BURNED_PROGRAM;
#[derive(Clone, Debug, PartialEq)]
pub enum DaoBurnedProgramIx {
    AddDelegateAuthority(AddDelegateAuthorityIxArgs),
    BurnDaoTokens(BurnDaoTokensIxArgs),
    CloseFund,
    ExecuteInvoke(ExecuteInvokeIxArgs),
    InitCurve,
    InitRedemption,
    Initialize(InitializeIxArgs),
    RedeemSol(RedeemSolIxArgs),
    RedeemTokens(RedeemTokensIxArgs),
    RevokeDelegateAuthority(RevokeDelegateAuthorityIxArgs),
}
impl DaoBurnedProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            ADD_DELEGATE_AUTHORITY_IX_DISCM => Ok(Self::AddDelegateAuthority(
                AddDelegateAuthorityIxArgs::deserialize(&mut reader)?,
            )),
            BURN_DAO_TOKENS_IX_DISCM => Ok(Self::BurnDaoTokens(BurnDaoTokensIxArgs::deserialize(
                &mut reader,
            )?)),
            CLOSE_FUND_IX_DISCM => Ok(Self::CloseFund),
            EXECUTE_INVOKE_IX_DISCM => Ok(Self::ExecuteInvoke(ExecuteInvokeIxArgs::deserialize(
                &mut reader,
            )?)),
            INIT_CURVE_IX_DISCM => Ok(Self::InitCurve),
            INIT_REDEMPTION_IX_DISCM => Ok(Self::InitRedemption),
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(
                &mut reader,
            )?)),
            REDEEM_SOL_IX_DISCM => Ok(Self::RedeemSol(RedeemSolIxArgs::deserialize(&mut reader)?)),
            REDEEM_TOKENS_IX_DISCM => Ok(Self::RedeemTokens(RedeemTokensIxArgs::deserialize(
                &mut reader,
            )?)),
            REVOKE_DELEGATE_AUTHORITY_IX_DISCM => Ok(Self::RevokeDelegateAuthority(
                RevokeDelegateAuthorityIxArgs::deserialize(&mut reader)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::AddDelegateAuthority(args) => {
                writer.write_all(&ADD_DELEGATE_AUTHORITY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::BurnDaoTokens(args) => {
                writer.write_all(&BURN_DAO_TOKENS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CloseFund => writer.write_all(&CLOSE_FUND_IX_DISCM),
            Self::ExecuteInvoke(args) => {
                writer.write_all(&EXECUTE_INVOKE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitCurve => writer.write_all(&INIT_CURVE_IX_DISCM),
            Self::InitRedemption => writer.write_all(&INIT_REDEMPTION_IX_DISCM),
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RedeemSol(args) => {
                writer.write_all(&REDEEM_SOL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RedeemTokens(args) => {
                writer.write_all(&REDEEM_TOKENS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RevokeDelegateAuthority(args) => {
                writer.write_all(&REVOKE_DELEGATE_AUTHORITY_IX_DISCM)?;
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
pub const ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AddDelegateAuthorityAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AddDelegateAuthorityKeys {
    pub signer: Pubkey,
    pub dao_mint: Pubkey,
    pub state: Pubkey,
}
impl From<AddDelegateAuthorityAccounts<'_, '_>> for AddDelegateAuthorityKeys {
    fn from(accounts: AddDelegateAuthorityAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dao_mint: *accounts.dao_mint.key,
            state: *accounts.state.key,
        }
    }
}
impl From<AddDelegateAuthorityKeys> for [AccountMeta; ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: AddDelegateAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]> for AddDelegateAuthorityKeys {
    fn from(pubkeys: [Pubkey; ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dao_mint: pubkeys[1],
            state: pubkeys[2],
        }
    }
}
impl<'info> From<AddDelegateAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AddDelegateAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dao_mint.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]>
    for AddDelegateAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dao_mint: &arr[1],
            state: &arr[2],
        }
    }
}
pub const ADD_DELEGATE_AUTHORITY_IX_DISCM: [u8; 8] = [176, 85, 248, 72, 172, 47, 89, 175];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddDelegateAuthorityIxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddDelegateAuthorityIxData(pub AddDelegateAuthorityIxArgs);
impl From<AddDelegateAuthorityIxArgs> for AddDelegateAuthorityIxData {
    fn from(args: AddDelegateAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl AddDelegateAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ADD_DELEGATE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_DELEGATE_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddDelegateAuthorityIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ADD_DELEGATE_AUTHORITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_delegate_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: AddDelegateAuthorityKeys,
    args: AddDelegateAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddDelegateAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_delegate_authority_ix(
    keys: AddDelegateAuthorityKeys,
    args: AddDelegateAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    add_delegate_authority_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, args)
}
pub fn add_delegate_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddDelegateAuthorityAccounts<'_, '_>,
    args: AddDelegateAuthorityIxArgs,
) -> ProgramResult {
    let keys: AddDelegateAuthorityKeys = accounts.into();
    let ix = add_delegate_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_delegate_authority_invoke(
    accounts: AddDelegateAuthorityAccounts<'_, '_>,
    args: AddDelegateAuthorityIxArgs,
) -> ProgramResult {
    add_delegate_authority_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts, args)
}
pub fn add_delegate_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddDelegateAuthorityAccounts<'_, '_>,
    args: AddDelegateAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddDelegateAuthorityKeys = accounts.into();
    let ix = add_delegate_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_delegate_authority_invoke_signed(
    accounts: AddDelegateAuthorityAccounts<'_, '_>,
    args: AddDelegateAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_delegate_authority_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, args, seeds)
}
pub fn add_delegate_authority_verify_account_keys(
    accounts: AddDelegateAuthorityAccounts<'_, '_>,
    keys: AddDelegateAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.state.key, keys.state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn add_delegate_authority_verify_writable_privileges<'me, 'info>(
    accounts: AddDelegateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_delegate_authority_verify_signer_privileges<'me, 'info>(
    accounts: AddDelegateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_delegate_authority_verify_account_privileges<'me, 'info>(
    accounts: AddDelegateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_delegate_authority_verify_writable_privileges(accounts)?;
    add_delegate_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BURN_DAO_TOKENS_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct BurnDaoTokensAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub dao_mint_signer_ata: &'me AccountInfo<'info>,
    pub dao_mint_state_ata: &'me AccountInfo<'info>,
    pub user_dao_burned: &'me AccountInfo<'info>,
    pub dao_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BurnDaoTokensKeys {
    pub signer: Pubkey,
    pub dao_mint: Pubkey,
    pub state: Pubkey,
    pub dao_mint_signer_ata: Pubkey,
    pub dao_mint_state_ata: Pubkey,
    pub user_dao_burned: Pubkey,
    pub dao_token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<BurnDaoTokensAccounts<'_, '_>> for BurnDaoTokensKeys {
    fn from(accounts: BurnDaoTokensAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dao_mint: *accounts.dao_mint.key,
            state: *accounts.state.key,
            dao_mint_signer_ata: *accounts.dao_mint_signer_ata.key,
            dao_mint_state_ata: *accounts.dao_mint_state_ata.key,
            user_dao_burned: *accounts.user_dao_burned.key,
            dao_token_program: *accounts.dao_token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<BurnDaoTokensKeys> for [AccountMeta; BURN_DAO_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(keys: BurnDaoTokensKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint_signer_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint_state_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_dao_burned,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; BURN_DAO_TOKENS_IX_ACCOUNTS_LEN]> for BurnDaoTokensKeys {
    fn from(pubkeys: [Pubkey; BURN_DAO_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dao_mint: pubkeys[1],
            state: pubkeys[2],
            dao_mint_signer_ata: pubkeys[3],
            dao_mint_state_ata: pubkeys[4],
            user_dao_burned: pubkeys[5],
            dao_token_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
        }
    }
}
impl<'info> From<BurnDaoTokensAccounts<'_, 'info>>
    for [AccountInfo<'info>; BURN_DAO_TOKENS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: BurnDaoTokensAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dao_mint.clone(),
            accounts.state.clone(),
            accounts.dao_mint_signer_ata.clone(),
            accounts.dao_mint_state_ata.clone(),
            accounts.user_dao_burned.clone(),
            accounts.dao_token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BURN_DAO_TOKENS_IX_ACCOUNTS_LEN]>
    for BurnDaoTokensAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; BURN_DAO_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dao_mint: &arr[1],
            state: &arr[2],
            dao_mint_signer_ata: &arr[3],
            dao_mint_state_ata: &arr[4],
            user_dao_burned: &arr[5],
            dao_token_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
        }
    }
}
pub const BURN_DAO_TOKENS_IX_DISCM: [u8; 8] = [184, 248, 164, 209, 110, 230, 127, 180];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BurnDaoTokensIxArgs {
    pub dao_mint_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BurnDaoTokensIxData(pub BurnDaoTokensIxArgs);
impl From<BurnDaoTokensIxArgs> for BurnDaoTokensIxData {
    fn from(args: BurnDaoTokensIxArgs) -> Self {
        Self(args)
    }
}
impl BurnDaoTokensIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BURN_DAO_TOKENS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BURN_DAO_TOKENS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BurnDaoTokensIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BURN_DAO_TOKENS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn burn_dao_tokens_ix_with_program_id(
    program_id: Pubkey,
    keys: BurnDaoTokensKeys,
    args: BurnDaoTokensIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BURN_DAO_TOKENS_IX_ACCOUNTS_LEN] = keys.into();
    let data: BurnDaoTokensIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn burn_dao_tokens_ix(
    keys: BurnDaoTokensKeys,
    args: BurnDaoTokensIxArgs,
) -> std::io::Result<Instruction> {
    burn_dao_tokens_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, args)
}
pub fn burn_dao_tokens_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BurnDaoTokensAccounts<'_, '_>,
    args: BurnDaoTokensIxArgs,
) -> ProgramResult {
    let keys: BurnDaoTokensKeys = accounts.into();
    let ix = burn_dao_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn burn_dao_tokens_invoke(
    accounts: BurnDaoTokensAccounts<'_, '_>,
    args: BurnDaoTokensIxArgs,
) -> ProgramResult {
    burn_dao_tokens_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts, args)
}
pub fn burn_dao_tokens_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BurnDaoTokensAccounts<'_, '_>,
    args: BurnDaoTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BurnDaoTokensKeys = accounts.into();
    let ix = burn_dao_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn burn_dao_tokens_invoke_signed(
    accounts: BurnDaoTokensAccounts<'_, '_>,
    args: BurnDaoTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    burn_dao_tokens_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, args, seeds)
}
pub fn burn_dao_tokens_verify_account_keys(
    accounts: BurnDaoTokensAccounts<'_, '_>,
    keys: BurnDaoTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.state.key, keys.state),
        (*accounts.dao_mint_signer_ata.key, keys.dao_mint_signer_ata),
        (*accounts.dao_mint_state_ata.key, keys.dao_mint_state_ata),
        (*accounts.user_dao_burned.key, keys.user_dao_burned),
        (*accounts.dao_token_program.key, keys.dao_token_program),
        (*accounts.system_program.key, keys.system_program),
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
pub fn burn_dao_tokens_verify_writable_privileges<'me, 'info>(
    accounts: BurnDaoTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.state,
        accounts.dao_mint_signer_ata,
        accounts.dao_mint_state_ata,
        accounts.user_dao_burned,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn burn_dao_tokens_verify_signer_privileges<'me, 'info>(
    accounts: BurnDaoTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn burn_dao_tokens_verify_account_privileges<'me, 'info>(
    accounts: BurnDaoTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    burn_dao_tokens_verify_writable_privileges(accounts)?;
    burn_dao_tokens_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLOSE_FUND_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CloseFundAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloseFundKeys {
    pub admin: Pubkey,
    pub dao_mint: Pubkey,
    pub state: Pubkey,
}
impl From<CloseFundAccounts<'_, '_>> for CloseFundKeys {
    fn from(accounts: CloseFundAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            dao_mint: *accounts.dao_mint.key,
            state: *accounts.state.key,
        }
    }
}
impl From<CloseFundKeys> for [AccountMeta; CLOSE_FUND_IX_ACCOUNTS_LEN] {
    fn from(keys: CloseFundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLOSE_FUND_IX_ACCOUNTS_LEN]> for CloseFundKeys {
    fn from(pubkeys: [Pubkey; CLOSE_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            dao_mint: pubkeys[1],
            state: pubkeys[2],
        }
    }
}
impl<'info> From<CloseFundAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLOSE_FUND_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CloseFundAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.dao_mint.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLOSE_FUND_IX_ACCOUNTS_LEN]>
    for CloseFundAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLOSE_FUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            dao_mint: &arr[1],
            state: &arr[2],
        }
    }
}
pub const CLOSE_FUND_IX_DISCM: [u8; 8] = [230, 183, 3, 112, 236, 252, 5, 185];
#[derive(Clone, Debug, PartialEq)]
pub struct CloseFundIxData;
impl CloseFundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLOSE_FUND_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLOSE_FUND_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLOSE_FUND_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn close_fund_ix_with_program_id(
    program_id: Pubkey,
    keys: CloseFundKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLOSE_FUND_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloseFundIxData.try_to_vec()?,
    })
}
pub fn close_fund_ix(keys: CloseFundKeys) -> std::io::Result<Instruction> {
    close_fund_ix_with_program_id(DAOS_BURNED_PROGRAM, keys)
}
pub fn close_fund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloseFundAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloseFundKeys = accounts.into();
    let ix = close_fund_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn close_fund_invoke(accounts: CloseFundAccounts<'_, '_>) -> ProgramResult {
    close_fund_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts)
}
pub fn close_fund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloseFundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloseFundKeys = accounts.into();
    let ix = close_fund_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn close_fund_invoke_signed(
    accounts: CloseFundAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    close_fund_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, seeds)
}
pub fn close_fund_verify_account_keys(
    accounts: CloseFundAccounts<'_, '_>,
    keys: CloseFundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.state.key, keys.state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn close_fund_verify_writable_privileges<'me, 'info>(
    accounts: CloseFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.admin, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn close_fund_verify_signer_privileges<'me, 'info>(
    accounts: CloseFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn close_fund_verify_account_privileges<'me, 'info>(
    accounts: CloseFundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    close_fund_verify_writable_privileges(accounts)?;
    close_fund_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXECUTE_INVOKE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ExecuteInvokeAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub instruction_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExecuteInvokeKeys {
    pub authority: Pubkey,
    pub dao_mint: Pubkey,
    pub state: Pubkey,
    pub wallet: Pubkey,
    pub instruction_program: Pubkey,
}
impl From<ExecuteInvokeAccounts<'_, '_>> for ExecuteInvokeKeys {
    fn from(accounts: ExecuteInvokeAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            dao_mint: *accounts.dao_mint.key,
            state: *accounts.state.key,
            wallet: *accounts.wallet.key,
            instruction_program: *accounts.instruction_program.key,
        }
    }
}
impl From<ExecuteInvokeKeys> for [AccountMeta; EXECUTE_INVOKE_IX_ACCOUNTS_LEN] {
    fn from(keys: ExecuteInvokeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instruction_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXECUTE_INVOKE_IX_ACCOUNTS_LEN]> for ExecuteInvokeKeys {
    fn from(pubkeys: [Pubkey; EXECUTE_INVOKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            dao_mint: pubkeys[1],
            state: pubkeys[2],
            wallet: pubkeys[3],
            instruction_program: pubkeys[4],
        }
    }
}
impl<'info> From<ExecuteInvokeAccounts<'_, 'info>>
    for [AccountInfo<'info>; EXECUTE_INVOKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ExecuteInvokeAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.dao_mint.clone(),
            accounts.state.clone(),
            accounts.wallet.clone(),
            accounts.instruction_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXECUTE_INVOKE_IX_ACCOUNTS_LEN]>
    for ExecuteInvokeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; EXECUTE_INVOKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            dao_mint: &arr[1],
            state: &arr[2],
            wallet: &arr[3],
            instruction_program: &arr[4],
        }
    }
}
pub const EXECUTE_INVOKE_IX_DISCM: [u8; 8] = [25, 143, 207, 190, 174, 228, 130, 107];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExecuteInvokeIxArgs {
    pub instruction_data: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExecuteInvokeIxData(pub ExecuteInvokeIxArgs);
impl From<ExecuteInvokeIxArgs> for ExecuteInvokeIxData {
    fn from(args: ExecuteInvokeIxArgs) -> Self {
        Self(args)
    }
}
impl ExecuteInvokeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXECUTE_INVOKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EXECUTE_INVOKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ExecuteInvokeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXECUTE_INVOKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn execute_invoke_ix_with_program_id(
    program_id: Pubkey,
    keys: ExecuteInvokeKeys,
    args: ExecuteInvokeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXECUTE_INVOKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExecuteInvokeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn execute_invoke_ix(
    keys: ExecuteInvokeKeys,
    args: ExecuteInvokeIxArgs,
) -> std::io::Result<Instruction> {
    execute_invoke_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, args)
}
pub fn execute_invoke_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteInvokeAccounts<'_, '_>,
    args: ExecuteInvokeIxArgs,
) -> ProgramResult {
    let keys: ExecuteInvokeKeys = accounts.into();
    let ix = execute_invoke_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn execute_invoke_invoke(
    accounts: ExecuteInvokeAccounts<'_, '_>,
    args: ExecuteInvokeIxArgs,
) -> ProgramResult {
    execute_invoke_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts, args)
}
pub fn execute_invoke_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExecuteInvokeAccounts<'_, '_>,
    args: ExecuteInvokeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExecuteInvokeKeys = accounts.into();
    let ix = execute_invoke_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn execute_invoke_invoke_signed(
    accounts: ExecuteInvokeAccounts<'_, '_>,
    args: ExecuteInvokeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    execute_invoke_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, args, seeds)
}
pub fn execute_invoke_verify_account_keys(
    accounts: ExecuteInvokeAccounts<'_, '_>,
    keys: ExecuteInvokeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.authority.key, keys.authority),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.state.key, keys.state),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.instruction_program.key, keys.instruction_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn execute_invoke_verify_writable_privileges<'me, 'info>(
    accounts: ExecuteInvokeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.authority, accounts.state, accounts.wallet] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn execute_invoke_verify_signer_privileges<'me, 'info>(
    accounts: ExecuteInvokeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn execute_invoke_verify_account_privileges<'me, 'info>(
    accounts: ExecuteInvokeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    execute_invoke_verify_writable_privileges(accounts)?;
    execute_invoke_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_CURVE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct InitCurveAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub fundraise_state: &'me AccountInfo<'info>,
    pub dao_mint_vault: &'me AccountInfo<'info>,
    pub curve: &'me AccountInfo<'info>,
    pub curve_dao_mint_ata: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub virtual_xyk_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitCurveKeys {
    pub payer: Pubkey,
    pub dao_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub state: Pubkey,
    pub fundraise_state: Pubkey,
    pub dao_mint_vault: Pubkey,
    pub curve: Pubkey,
    pub curve_dao_mint_ata: Pubkey,
    pub fee_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub virtual_xyk_program: Pubkey,
}
impl From<InitCurveAccounts<'_, '_>> for InitCurveKeys {
    fn from(accounts: InitCurveAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            dao_mint: *accounts.dao_mint.key,
            funding_mint: *accounts.funding_mint.key,
            state: *accounts.state.key,
            fundraise_state: *accounts.fundraise_state.key,
            dao_mint_vault: *accounts.dao_mint_vault.key,
            curve: *accounts.curve.key,
            curve_dao_mint_ata: *accounts.curve_dao_mint_ata.key,
            fee_authority: *accounts.fee_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            virtual_xyk_program: *accounts.virtual_xyk_program.key,
        }
    }
}
impl From<InitCurveKeys> for [AccountMeta; INIT_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitCurveKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fundraise_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_dao_mint_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_authority,
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
            AccountMeta {
                pubkey: keys.virtual_xyk_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INIT_CURVE_IX_ACCOUNTS_LEN]> for InitCurveKeys {
    fn from(pubkeys: [Pubkey; INIT_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            dao_mint: pubkeys[1],
            funding_mint: pubkeys[2],
            state: pubkeys[3],
            fundraise_state: pubkeys[4],
            dao_mint_vault: pubkeys[5],
            curve: pubkeys[6],
            curve_dao_mint_ata: pubkeys[7],
            fee_authority: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
            associated_token_program: pubkeys[11],
            virtual_xyk_program: pubkeys[12],
        }
    }
}
impl<'info> From<InitCurveAccounts<'_, 'info>>
    for [AccountInfo<'info>; INIT_CURVE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitCurveAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.dao_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.state.clone(),
            accounts.fundraise_state.clone(),
            accounts.dao_mint_vault.clone(),
            accounts.curve.clone(),
            accounts.curve_dao_mint_ata.clone(),
            accounts.fee_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.virtual_xyk_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_CURVE_IX_ACCOUNTS_LEN]>
    for InitCurveAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INIT_CURVE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            dao_mint: &arr[1],
            funding_mint: &arr[2],
            state: &arr[3],
            fundraise_state: &arr[4],
            dao_mint_vault: &arr[5],
            curve: &arr[6],
            curve_dao_mint_ata: &arr[7],
            fee_authority: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
            associated_token_program: &arr[11],
            virtual_xyk_program: &arr[12],
        }
    }
}
pub const INIT_CURVE_IX_DISCM: [u8; 8] = [38, 93, 1, 214, 59, 185, 76, 89];
#[derive(Clone, Debug, PartialEq)]
pub struct InitCurveIxData;
impl InitCurveIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_CURVE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INIT_CURVE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_CURVE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_curve_ix_with_program_id(
    program_id: Pubkey,
    keys: InitCurveKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_CURVE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitCurveIxData.try_to_vec()?,
    })
}
pub fn init_curve_ix(keys: InitCurveKeys) -> std::io::Result<Instruction> {
    init_curve_ix_with_program_id(DAOS_BURNED_PROGRAM, keys)
}
pub fn init_curve_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitCurveAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitCurveKeys = accounts.into();
    let ix = init_curve_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_curve_invoke(accounts: InitCurveAccounts<'_, '_>) -> ProgramResult {
    init_curve_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts)
}
pub fn init_curve_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitCurveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitCurveKeys = accounts.into();
    let ix = init_curve_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_curve_invoke_signed(
    accounts: InitCurveAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_curve_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, seeds)
}
pub fn init_curve_verify_account_keys(
    accounts: InitCurveAccounts<'_, '_>,
    keys: InitCurveKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.state.key, keys.state),
        (*accounts.fundraise_state.key, keys.fundraise_state),
        (*accounts.dao_mint_vault.key, keys.dao_mint_vault),
        (*accounts.curve.key, keys.curve),
        (*accounts.curve_dao_mint_ata.key, keys.curve_dao_mint_ata),
        (*accounts.fee_authority.key, keys.fee_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.virtual_xyk_program.key, keys.virtual_xyk_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn init_curve_verify_writable_privileges<'me, 'info>(
    accounts: InitCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.dao_mint,
        accounts.state,
        accounts.fundraise_state,
        accounts.dao_mint_vault,
        accounts.curve,
        accounts.curve_dao_mint_ata,
        accounts.fee_authority,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_curve_verify_signer_privileges<'me, 'info>(
    accounts: InitCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_curve_verify_account_privileges<'me, 'info>(
    accounts: InitCurveAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_curve_verify_writable_privileges(accounts)?;
    init_curve_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INIT_REDEMPTION_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitRedemptionAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub wallet_funding_ata: &'me AccountInfo<'info>,
    pub admin_funding_ata: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitRedemptionKeys {
    pub signer: Pubkey,
    pub dao_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub wallet: Pubkey,
    pub wallet_funding_ata: Pubkey,
    pub admin_funding_ata: Pubkey,
    pub state: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<InitRedemptionAccounts<'_, '_>> for InitRedemptionKeys {
    fn from(accounts: InitRedemptionAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dao_mint: *accounts.dao_mint.key,
            funding_mint: *accounts.funding_mint.key,
            wallet: *accounts.wallet.key,
            wallet_funding_ata: *accounts.wallet_funding_ata.key,
            admin_funding_ata: *accounts.admin_funding_ata.key,
            state: *accounts.state.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<InitRedemptionKeys> for [AccountMeta; INIT_REDEMPTION_IX_ACCOUNTS_LEN] {
    fn from(keys: InitRedemptionKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet_funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
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
impl From<[Pubkey; INIT_REDEMPTION_IX_ACCOUNTS_LEN]> for InitRedemptionKeys {
    fn from(pubkeys: [Pubkey; INIT_REDEMPTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dao_mint: pubkeys[1],
            funding_mint: pubkeys[2],
            wallet: pubkeys[3],
            wallet_funding_ata: pubkeys[4],
            admin_funding_ata: pubkeys[5],
            state: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitRedemptionAccounts<'_, 'info>>
    for [AccountInfo<'info>; INIT_REDEMPTION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitRedemptionAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dao_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.wallet.clone(),
            accounts.wallet_funding_ata.clone(),
            accounts.admin_funding_ata.clone(),
            accounts.state.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_REDEMPTION_IX_ACCOUNTS_LEN]>
    for InitRedemptionAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INIT_REDEMPTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dao_mint: &arr[1],
            funding_mint: &arr[2],
            wallet: &arr[3],
            wallet_funding_ata: &arr[4],
            admin_funding_ata: &arr[5],
            state: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
        }
    }
}
pub const INIT_REDEMPTION_IX_DISCM: [u8; 8] = [3, 176, 47, 90, 46, 52, 86, 46];
#[derive(Clone, Debug, PartialEq)]
pub struct InitRedemptionIxData;
impl InitRedemptionIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INIT_REDEMPTION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INIT_REDEMPTION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INIT_REDEMPTION_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn init_redemption_ix_with_program_id(
    program_id: Pubkey,
    keys: InitRedemptionKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INIT_REDEMPTION_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitRedemptionIxData.try_to_vec()?,
    })
}
pub fn init_redemption_ix(keys: InitRedemptionKeys) -> std::io::Result<Instruction> {
    init_redemption_ix_with_program_id(DAOS_BURNED_PROGRAM, keys)
}
pub fn init_redemption_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitRedemptionAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitRedemptionKeys = accounts.into();
    let ix = init_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn init_redemption_invoke(accounts: InitRedemptionAccounts<'_, '_>) -> ProgramResult {
    init_redemption_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts)
}
pub fn init_redemption_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitRedemptionKeys = accounts.into();
    let ix = init_redemption_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn init_redemption_invoke_signed(
    accounts: InitRedemptionAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    init_redemption_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, seeds)
}
pub fn init_redemption_verify_account_keys(
    accounts: InitRedemptionAccounts<'_, '_>,
    keys: InitRedemptionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.wallet_funding_ata.key, keys.wallet_funding_ata),
        (*accounts.admin_funding_ata.key, keys.admin_funding_ata),
        (*accounts.state.key, keys.state),
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
pub fn init_redemption_verify_writable_privileges<'me, 'info>(
    accounts: InitRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.wallet,
        accounts.wallet_funding_ata,
        accounts.admin_funding_ata,
        accounts.state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn init_redemption_verify_signer_privileges<'me, 'info>(
    accounts: InitRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn init_redemption_verify_account_privileges<'me, 'info>(
    accounts: InitRedemptionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    init_redemption_verify_writable_privileges(accounts)?;
    init_redemption_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub admin: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub dao_mint_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub fundraise_state: &'me AccountInfo<'info>,
    pub fundraise_token_vault: &'me AccountInfo<'info>,
    pub fundraise_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub wallet: Pubkey,
    pub dao_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub dao_mint_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub fundraise_state: Pubkey,
    pub fundraise_token_vault: Pubkey,
    pub fundraise_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            wallet: *accounts.wallet.key,
            dao_mint: *accounts.dao_mint.key,
            funding_mint: *accounts.funding_mint.key,
            dao_mint_vault: *accounts.dao_mint_vault.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            fundraise_state: *accounts.fundraise_state.key,
            fundraise_token_vault: *accounts.fundraise_token_vault.key,
            fundraise_program: *accounts.fundraise_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dao_mint_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fundraise_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fundraise_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fundraise_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: pubkeys[0],
            state: pubkeys[1],
            wallet: pubkeys[2],
            dao_mint: pubkeys[3],
            funding_mint: pubkeys[4],
            dao_mint_vault: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            fundraise_state: pubkeys[9],
            fundraise_token_vault: pubkeys[10],
            fundraise_program: pubkeys[11],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.wallet.clone(),
            accounts.dao_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.dao_mint_vault.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.fundraise_state.clone(),
            accounts.fundraise_token_vault.clone(),
            accounts.fundraise_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            admin: &arr[0],
            state: &arr[1],
            wallet: &arr[2],
            dao_mint: &arr[3],
            funding_mint: &arr[4],
            dao_mint_vault: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            associated_token_program: &arr[8],
            fundraise_state: &arr[9],
            fundraise_token_vault: &arr[10],
            fundraise_program: &arr[11],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub dao_duration_seconds: u32,
    pub funding_goal: u64,
    pub funding_duration_seconds: u32,
    pub carry_basis: Option<u16>,
    pub fee_authority: Pubkey,
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
    initialize_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, args)
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
    initialize_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts, args)
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
    initialize_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.admin.key, keys.admin),
        (*accounts.state.key, keys.state),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.dao_mint_vault.key, keys.dao_mint_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.fundraise_state.key, keys.fundraise_state),
        (
            *accounts.fundraise_token_vault.key,
            keys.fundraise_token_vault,
        ),
        (*accounts.fundraise_program.key, keys.fundraise_program),
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
        accounts.admin,
        accounts.state,
        accounts.wallet,
        accounts.dao_mint,
        accounts.dao_mint_vault,
        accounts.fundraise_state,
        accounts.fundraise_token_vault,
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
    for should_be_signer in [accounts.admin, accounts.dao_mint] {
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
pub const REDEEM_SOL_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct RedeemSolAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub user_dao_burned: &'me AccountInfo<'info>,
    pub sol_redeem: &'me AccountInfo<'info>,
    pub user_sol_redeem: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemSolKeys {
    pub signer: Pubkey,
    pub dao_mint: Pubkey,
    pub wallet: Pubkey,
    pub user_dao_burned: Pubkey,
    pub sol_redeem: Pubkey,
    pub user_sol_redeem: Pubkey,
    pub system_program: Pubkey,
}
impl From<RedeemSolAccounts<'_, '_>> for RedeemSolKeys {
    fn from(accounts: RedeemSolAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dao_mint: *accounts.dao_mint.key,
            wallet: *accounts.wallet.key,
            user_dao_burned: *accounts.user_dao_burned.key,
            sol_redeem: *accounts.sol_redeem.key,
            user_sol_redeem: *accounts.user_sol_redeem.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<RedeemSolKeys> for [AccountMeta; REDEEM_SOL_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemSolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_dao_burned,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.sol_redeem,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_sol_redeem,
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
impl From<[Pubkey; REDEEM_SOL_IX_ACCOUNTS_LEN]> for RedeemSolKeys {
    fn from(pubkeys: [Pubkey; REDEEM_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dao_mint: pubkeys[1],
            wallet: pubkeys[2],
            user_dao_burned: pubkeys[3],
            sol_redeem: pubkeys[4],
            user_sol_redeem: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<RedeemSolAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDEEM_SOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RedeemSolAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dao_mint.clone(),
            accounts.wallet.clone(),
            accounts.user_dao_burned.clone(),
            accounts.sol_redeem.clone(),
            accounts.user_sol_redeem.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_SOL_IX_ACCOUNTS_LEN]>
    for RedeemSolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_SOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dao_mint: &arr[1],
            wallet: &arr[2],
            user_dao_burned: &arr[3],
            sol_redeem: &arr[4],
            user_sol_redeem: &arr[5],
            system_program: &arr[6],
        }
    }
}
pub const REDEEM_SOL_IX_DISCM: [u8; 8] = [66, 41, 191, 194, 105, 10, 68, 248];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RedeemSolIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemSolIxData(pub RedeemSolIxArgs);
impl From<RedeemSolIxArgs> for RedeemSolIxData {
    fn from(args: RedeemSolIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemSolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_SOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDEEM_SOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RedeemSolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_SOL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_sol_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemSolKeys,
    args: RedeemSolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_SOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemSolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_sol_ix(keys: RedeemSolKeys, args: RedeemSolIxArgs) -> std::io::Result<Instruction> {
    redeem_sol_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, args)
}
pub fn redeem_sol_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemSolAccounts<'_, '_>,
    args: RedeemSolIxArgs,
) -> ProgramResult {
    let keys: RedeemSolKeys = accounts.into();
    let ix = redeem_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_sol_invoke(
    accounts: RedeemSolAccounts<'_, '_>,
    args: RedeemSolIxArgs,
) -> ProgramResult {
    redeem_sol_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts, args)
}
pub fn redeem_sol_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemSolAccounts<'_, '_>,
    args: RedeemSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemSolKeys = accounts.into();
    let ix = redeem_sol_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_sol_invoke_signed(
    accounts: RedeemSolAccounts<'_, '_>,
    args: RedeemSolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_sol_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, args, seeds)
}
pub fn redeem_sol_verify_account_keys(
    accounts: RedeemSolAccounts<'_, '_>,
    keys: RedeemSolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.user_dao_burned.key, keys.user_dao_burned),
        (*accounts.sol_redeem.key, keys.sol_redeem),
        (*accounts.user_sol_redeem.key, keys.user_sol_redeem),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn redeem_sol_verify_writable_privileges<'me, 'info>(
    accounts: RedeemSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.wallet,
        accounts.user_dao_burned,
        accounts.sol_redeem,
        accounts.user_sol_redeem,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_sol_verify_signer_privileges<'me, 'info>(
    accounts: RedeemSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_sol_verify_account_privileges<'me, 'info>(
    accounts: RedeemSolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_sol_verify_writable_privileges(accounts)?;
    redeem_sol_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REDEEM_TOKENS_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct RedeemTokensAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub redeem_mint: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub redeem_wallet_token_acc: &'me AccountInfo<'info>,
    pub redeem_signer_ata: &'me AccountInfo<'info>,
    pub user_dao_burned: &'me AccountInfo<'info>,
    pub token_account_redemption: &'me AccountInfo<'info>,
    pub user_dao_burn_redeemed: &'me AccountInfo<'info>,
    pub redeem_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RedeemTokensKeys {
    pub signer: Pubkey,
    pub dao_mint: Pubkey,
    pub redeem_mint: Pubkey,
    pub wallet: Pubkey,
    pub redeem_wallet_token_acc: Pubkey,
    pub redeem_signer_ata: Pubkey,
    pub user_dao_burned: Pubkey,
    pub token_account_redemption: Pubkey,
    pub user_dao_burn_redeemed: Pubkey,
    pub redeem_token_program: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<RedeemTokensAccounts<'_, '_>> for RedeemTokensKeys {
    fn from(accounts: RedeemTokensAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dao_mint: *accounts.dao_mint.key,
            redeem_mint: *accounts.redeem_mint.key,
            wallet: *accounts.wallet.key,
            redeem_wallet_token_acc: *accounts.redeem_wallet_token_acc.key,
            redeem_signer_ata: *accounts.redeem_signer_ata.key,
            user_dao_burned: *accounts.user_dao_burned.key,
            token_account_redemption: *accounts.token_account_redemption.key,
            user_dao_burn_redeemed: *accounts.user_dao_burn_redeemed.key,
            redeem_token_program: *accounts.redeem_token_program.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<RedeemTokensKeys> for [AccountMeta; REDEEM_TOKENS_IX_ACCOUNTS_LEN] {
    fn from(keys: RedeemTokensKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.redeem_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redeem_wallet_token_acc,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redeem_signer_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_dao_burned,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account_redemption,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_dao_burn_redeemed,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.redeem_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
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
impl From<[Pubkey; REDEEM_TOKENS_IX_ACCOUNTS_LEN]> for RedeemTokensKeys {
    fn from(pubkeys: [Pubkey; REDEEM_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dao_mint: pubkeys[1],
            redeem_mint: pubkeys[2],
            wallet: pubkeys[3],
            redeem_wallet_token_acc: pubkeys[4],
            redeem_signer_ata: pubkeys[5],
            user_dao_burned: pubkeys[6],
            token_account_redemption: pubkeys[7],
            user_dao_burn_redeemed: pubkeys[8],
            redeem_token_program: pubkeys[9],
            system_program: pubkeys[10],
            associated_token_program: pubkeys[11],
        }
    }
}
impl<'info> From<RedeemTokensAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDEEM_TOKENS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RedeemTokensAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dao_mint.clone(),
            accounts.redeem_mint.clone(),
            accounts.wallet.clone(),
            accounts.redeem_wallet_token_acc.clone(),
            accounts.redeem_signer_ata.clone(),
            accounts.user_dao_burned.clone(),
            accounts.token_account_redemption.clone(),
            accounts.user_dao_burn_redeemed.clone(),
            accounts.redeem_token_program.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDEEM_TOKENS_IX_ACCOUNTS_LEN]>
    for RedeemTokensAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REDEEM_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dao_mint: &arr[1],
            redeem_mint: &arr[2],
            wallet: &arr[3],
            redeem_wallet_token_acc: &arr[4],
            redeem_signer_ata: &arr[5],
            user_dao_burned: &arr[6],
            token_account_redemption: &arr[7],
            user_dao_burn_redeemed: &arr[8],
            redeem_token_program: &arr[9],
            system_program: &arr[10],
            associated_token_program: &arr[11],
        }
    }
}
pub const REDEEM_TOKENS_IX_DISCM: [u8; 8] = [246, 98, 134, 41, 152, 33, 120, 69];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RedeemTokensIxArgs {
    pub dao_mint_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedeemTokensIxData(pub RedeemTokensIxArgs);
impl From<RedeemTokensIxArgs> for RedeemTokensIxData {
    fn from(args: RedeemTokensIxArgs) -> Self {
        Self(args)
    }
}
impl RedeemTokensIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REDEEM_TOKENS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDEEM_TOKENS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RedeemTokensIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REDEEM_TOKENS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn redeem_tokens_ix_with_program_id(
    program_id: Pubkey,
    keys: RedeemTokensKeys,
    args: RedeemTokensIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REDEEM_TOKENS_IX_ACCOUNTS_LEN] = keys.into();
    let data: RedeemTokensIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn redeem_tokens_ix(
    keys: RedeemTokensKeys,
    args: RedeemTokensIxArgs,
) -> std::io::Result<Instruction> {
    redeem_tokens_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, args)
}
pub fn redeem_tokens_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedeemTokensAccounts<'_, '_>,
    args: RedeemTokensIxArgs,
) -> ProgramResult {
    let keys: RedeemTokensKeys = accounts.into();
    let ix = redeem_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn redeem_tokens_invoke(
    accounts: RedeemTokensAccounts<'_, '_>,
    args: RedeemTokensIxArgs,
) -> ProgramResult {
    redeem_tokens_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts, args)
}
pub fn redeem_tokens_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedeemTokensAccounts<'_, '_>,
    args: RedeemTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedeemTokensKeys = accounts.into();
    let ix = redeem_tokens_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redeem_tokens_invoke_signed(
    accounts: RedeemTokensAccounts<'_, '_>,
    args: RedeemTokensIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redeem_tokens_invoke_signed_with_program_id(DAOS_BURNED_PROGRAM, accounts, args, seeds)
}
pub fn redeem_tokens_verify_account_keys(
    accounts: RedeemTokensAccounts<'_, '_>,
    keys: RedeemTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.redeem_mint.key, keys.redeem_mint),
        (*accounts.wallet.key, keys.wallet),
        (
            *accounts.redeem_wallet_token_acc.key,
            keys.redeem_wallet_token_acc,
        ),
        (*accounts.redeem_signer_ata.key, keys.redeem_signer_ata),
        (*accounts.user_dao_burned.key, keys.user_dao_burned),
        (
            *accounts.token_account_redemption.key,
            keys.token_account_redemption,
        ),
        (
            *accounts.user_dao_burn_redeemed.key,
            keys.user_dao_burn_redeemed,
        ),
        (
            *accounts.redeem_token_program.key,
            keys.redeem_token_program,
        ),
        (*accounts.system_program.key, keys.system_program),
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
pub fn redeem_tokens_verify_writable_privileges<'me, 'info>(
    accounts: RedeemTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.wallet,
        accounts.redeem_wallet_token_acc,
        accounts.redeem_signer_ata,
        accounts.user_dao_burned,
        accounts.token_account_redemption,
        accounts.user_dao_burn_redeemed,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redeem_tokens_verify_signer_privileges<'me, 'info>(
    accounts: RedeemTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redeem_tokens_verify_account_privileges<'me, 'info>(
    accounts: RedeemTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redeem_tokens_verify_writable_privileges(accounts)?;
    redeem_tokens_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct RevokeDelegateAuthorityAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub dao_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RevokeDelegateAuthorityKeys {
    pub signer: Pubkey,
    pub dao_mint: Pubkey,
    pub state: Pubkey,
}
impl From<RevokeDelegateAuthorityAccounts<'_, '_>> for RevokeDelegateAuthorityKeys {
    fn from(accounts: RevokeDelegateAuthorityAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            dao_mint: *accounts.dao_mint.key,
            state: *accounts.state.key,
        }
    }
}
impl From<RevokeDelegateAuthorityKeys>
    for [AccountMeta; REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(keys: RevokeDelegateAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dao_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]> for RevokeDelegateAuthorityKeys {
    fn from(pubkeys: [Pubkey; REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            dao_mint: pubkeys[1],
            state: pubkeys[2],
        }
    }
}
impl<'info> From<RevokeDelegateAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RevokeDelegateAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.dao_mint.clone(),
            accounts.state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]>
    for RevokeDelegateAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            dao_mint: &arr[1],
            state: &arr[2],
        }
    }
}
pub const REVOKE_DELEGATE_AUTHORITY_IX_DISCM: [u8; 8] = [49, 15, 12, 61, 75, 221, 229, 154];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RevokeDelegateAuthorityIxArgs {
    pub authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeDelegateAuthorityIxData(pub RevokeDelegateAuthorityIxArgs);
impl From<RevokeDelegateAuthorityIxArgs> for RevokeDelegateAuthorityIxData {
    fn from(args: RevokeDelegateAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl RevokeDelegateAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REVOKE_DELEGATE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REVOKE_DELEGATE_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RevokeDelegateAuthorityIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REVOKE_DELEGATE_AUTHORITY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn revoke_delegate_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: RevokeDelegateAuthorityKeys,
    args: RevokeDelegateAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REVOKE_DELEGATE_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    let data: RevokeDelegateAuthorityIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_delegate_authority_ix(
    keys: RevokeDelegateAuthorityKeys,
    args: RevokeDelegateAuthorityIxArgs,
) -> std::io::Result<Instruction> {
    revoke_delegate_authority_ix_with_program_id(DAOS_BURNED_PROGRAM, keys, args)
}
pub fn revoke_delegate_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RevokeDelegateAuthorityAccounts<'_, '_>,
    args: RevokeDelegateAuthorityIxArgs,
) -> ProgramResult {
    let keys: RevokeDelegateAuthorityKeys = accounts.into();
    let ix = revoke_delegate_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn revoke_delegate_authority_invoke(
    accounts: RevokeDelegateAuthorityAccounts<'_, '_>,
    args: RevokeDelegateAuthorityIxArgs,
) -> ProgramResult {
    revoke_delegate_authority_invoke_with_program_id(DAOS_BURNED_PROGRAM, accounts, args)
}
pub fn revoke_delegate_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RevokeDelegateAuthorityAccounts<'_, '_>,
    args: RevokeDelegateAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RevokeDelegateAuthorityKeys = accounts.into();
    let ix = revoke_delegate_authority_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn revoke_delegate_authority_invoke_signed(
    accounts: RevokeDelegateAuthorityAccounts<'_, '_>,
    args: RevokeDelegateAuthorityIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    revoke_delegate_authority_invoke_signed_with_program_id(
        DAOS_BURNED_PROGRAM,
        accounts,
        args,
        seeds,
    )
}
pub fn revoke_delegate_authority_verify_account_keys(
    accounts: RevokeDelegateAuthorityAccounts<'_, '_>,
    keys: RevokeDelegateAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.dao_mint.key, keys.dao_mint),
        (*accounts.state.key, keys.state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn revoke_delegate_authority_verify_writable_privileges<'me, 'info>(
    accounts: RevokeDelegateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.signer, accounts.state] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn revoke_delegate_authority_verify_signer_privileges<'me, 'info>(
    accounts: RevokeDelegateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn revoke_delegate_authority_verify_account_privileges<'me, 'info>(
    accounts: RevokeDelegateAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    revoke_delegate_authority_verify_writable_privileges(accounts)?;
    revoke_delegate_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
