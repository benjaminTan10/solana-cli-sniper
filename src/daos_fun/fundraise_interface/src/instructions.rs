use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum FundraiseProgramIx {
    CommitFunds(CommitFundsIxArgs),
    Finalize,
    Initialize(InitializeIxArgs),
    Refund(RefundIxArgs),
}
impl FundraiseProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            COMMIT_FUNDS_IX_DISCM => {
                Ok(Self::CommitFunds(CommitFundsIxArgs::deserialize(&mut reader)?))
            }
            FINALIZE_IX_DISCM => Ok(Self::Finalize),
            INITIALIZE_IX_DISCM => {
                Ok(Self::Initialize(InitializeIxArgs::deserialize(&mut reader)?))
            }
            REFUND_IX_DISCM => Ok(Self::Refund(RefundIxArgs::deserialize(&mut reader)?)),
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CommitFunds(args) => {
                writer.write_all(&COMMIT_FUNDS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Finalize => writer.write_all(&FINALIZE_IX_DISCM),
            Self::Initialize(args) => {
                writer.write_all(&INITIALIZE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Refund(args) => {
                writer.write_all(&REFUND_IX_DISCM)?;
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
pub const COMMIT_FUNDS_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CommitFundsAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub signer_token_ata: &'me AccountInfo<'info>,
    pub signer_funding_ata: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub funding_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub funding_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CommitFundsKeys {
    pub signer: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub state: Pubkey,
    pub signer_token_ata: Pubkey,
    pub signer_funding_ata: Pubkey,
    pub token_vault: Pubkey,
    pub funding_vault: Pubkey,
    pub token_program: Pubkey,
    pub funding_token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<CommitFundsAccounts<'_, '_>> for CommitFundsKeys {
    fn from(accounts: CommitFundsAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            token_mint: *accounts.token_mint.key,
            funding_mint: *accounts.funding_mint.key,
            state: *accounts.state.key,
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
impl From<CommitFundsKeys> for [AccountMeta; COMMIT_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: CommitFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.state,
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
impl From<[Pubkey; COMMIT_FUNDS_IX_ACCOUNTS_LEN]> for CommitFundsKeys {
    fn from(pubkeys: [Pubkey; COMMIT_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            token_mint: pubkeys[1],
            funding_mint: pubkeys[2],
            state: pubkeys[3],
            signer_token_ata: pubkeys[4],
            signer_funding_ata: pubkeys[5],
            token_vault: pubkeys[6],
            funding_vault: pubkeys[7],
            token_program: pubkeys[8],
            funding_token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
        }
    }
}
impl<'info> From<CommitFundsAccounts<'_, 'info>>
for [AccountInfo<'info>; COMMIT_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(accounts: CommitFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.token_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.state.clone(),
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; COMMIT_FUNDS_IX_ACCOUNTS_LEN]>
for CommitFundsAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; COMMIT_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            token_mint: &arr[1],
            funding_mint: &arr[2],
            state: &arr[3],
            signer_token_ata: &arr[4],
            signer_funding_ata: &arr[5],
            token_vault: &arr[6],
            funding_vault: &arr[7],
            token_program: &arr[8],
            funding_token_program: &arr[9],
            associated_token_program: &arr[10],
        }
    }
}
pub const COMMIT_FUNDS_IX_DISCM: [u8; 8] = [242, 226, 172, 204, 143, 241, 207, 248];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommitFundsIxArgs {
    pub funding_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CommitFundsIxData(pub CommitFundsIxArgs);
impl From<CommitFundsIxArgs> for CommitFundsIxData {
    fn from(args: CommitFundsIxArgs) -> Self {
        Self(args)
    }
}
impl CommitFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != COMMIT_FUNDS_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        COMMIT_FUNDS_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CommitFundsIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&COMMIT_FUNDS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn commit_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: CommitFundsKeys,
    args: CommitFundsIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; COMMIT_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CommitFundsIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn commit_funds_ix(
    keys: CommitFundsKeys,
    args: CommitFundsIxArgs,
) -> std::io::Result<Instruction> {
    commit_funds_ix_with_program_id(crate::ID, keys, args)
}
pub fn commit_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CommitFundsAccounts<'_, '_>,
    args: CommitFundsIxArgs,
) -> ProgramResult {
    let keys: CommitFundsKeys = accounts.into();
    let ix = commit_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn commit_funds_invoke(
    accounts: CommitFundsAccounts<'_, '_>,
    args: CommitFundsIxArgs,
) -> ProgramResult {
    commit_funds_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn commit_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CommitFundsAccounts<'_, '_>,
    args: CommitFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CommitFundsKeys = accounts.into();
    let ix = commit_funds_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn commit_funds_invoke_signed(
    accounts: CommitFundsAccounts<'_, '_>,
    args: CommitFundsIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    commit_funds_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn commit_funds_verify_account_keys(
    accounts: CommitFundsAccounts<'_, '_>,
    keys: CommitFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.state.key, keys.state),
        (*accounts.signer_token_ata.key, keys.signer_token_ata),
        (*accounts.signer_funding_ata.key, keys.signer_funding_ata),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.funding_vault.key, keys.funding_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.funding_token_program.key, keys.funding_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn commit_funds_verify_writable_privileges<'me, 'info>(
    accounts: CommitFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.state,
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
pub fn commit_funds_verify_signer_privileges<'me, 'info>(
    accounts: CommitFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn commit_funds_verify_account_privileges<'me, 'info>(
    accounts: CommitFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    commit_funds_verify_writable_privileges(accounts)?;
    commit_funds_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const FINALIZE_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct FinalizeAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub recipient_funding_ata: &'me AccountInfo<'info>,
    pub funding_vault: &'me AccountInfo<'info>,
    pub funding_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FinalizeKeys {
    pub signer: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub state: Pubkey,
    pub recipient_funding_ata: Pubkey,
    pub funding_vault: Pubkey,
    pub funding_token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<FinalizeAccounts<'_, '_>> for FinalizeKeys {
    fn from(accounts: FinalizeAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            token_mint: *accounts.token_mint.key,
            funding_mint: *accounts.funding_mint.key,
            state: *accounts.state.key,
            recipient_funding_ata: *accounts.recipient_funding_ata.key,
            funding_vault: *accounts.funding_vault.key,
            funding_token_program: *accounts.funding_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<FinalizeKeys> for [AccountMeta; FINALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: FinalizeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient_funding_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_vault,
                is_signer: false,
                is_writable: true,
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
impl From<[Pubkey; FINALIZE_IX_ACCOUNTS_LEN]> for FinalizeKeys {
    fn from(pubkeys: [Pubkey; FINALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            token_mint: pubkeys[1],
            funding_mint: pubkeys[2],
            state: pubkeys[3],
            recipient_funding_ata: pubkeys[4],
            funding_vault: pubkeys[5],
            funding_token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
        }
    }
}
impl<'info> From<FinalizeAccounts<'_, 'info>>
for [AccountInfo<'info>; FINALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: FinalizeAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.token_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.state.clone(),
            accounts.recipient_funding_ata.clone(),
            accounts.funding_vault.clone(),
            accounts.funding_token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FINALIZE_IX_ACCOUNTS_LEN]>
for FinalizeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; FINALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            token_mint: &arr[1],
            funding_mint: &arr[2],
            state: &arr[3],
            recipient_funding_ata: &arr[4],
            funding_vault: &arr[5],
            funding_token_program: &arr[6],
            associated_token_program: &arr[7],
        }
    }
}
pub const FINALIZE_IX_DISCM: [u8; 8] = [171, 61, 218, 56, 127, 115, 12, 217];
#[derive(Clone, Debug, PartialEq)]
pub struct FinalizeIxData;
impl FinalizeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FINALIZE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FINALIZE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FINALIZE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn finalize_ix_with_program_id(
    program_id: Pubkey,
    keys: FinalizeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; FINALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: FinalizeIxData.try_to_vec()?,
    })
}
pub fn finalize_ix(keys: FinalizeKeys) -> std::io::Result<Instruction> {
    finalize_ix_with_program_id(crate::ID, keys)
}
pub fn finalize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: FinalizeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: FinalizeKeys = accounts.into();
    let ix = finalize_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn finalize_invoke(accounts: FinalizeAccounts<'_, '_>) -> ProgramResult {
    finalize_invoke_with_program_id(crate::ID, accounts)
}
pub fn finalize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: FinalizeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: FinalizeKeys = accounts.into();
    let ix = finalize_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn finalize_invoke_signed(
    accounts: FinalizeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    finalize_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn finalize_verify_account_keys(
    accounts: FinalizeAccounts<'_, '_>,
    keys: FinalizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.state.key, keys.state),
        (*accounts.recipient_funding_ata.key, keys.recipient_funding_ata),
        (*accounts.funding_vault.key, keys.funding_vault),
        (*accounts.funding_token_program.key, keys.funding_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn finalize_verify_writable_privileges<'me, 'info>(
    accounts: FinalizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.state,
        accounts.recipient_funding_ata,
        accounts.funding_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn finalize_verify_signer_privileges<'me, 'info>(
    accounts: FinalizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn finalize_verify_account_privileges<'me, 'info>(
    accounts: FinalizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    finalize_verify_writable_privileges(accounts)?;
    finalize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub admin_token_ata: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub recipient: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeKeys {
    pub signer: Pubkey,
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub state: Pubkey,
    pub admin_token_ata: Pubkey,
    pub token_vault: Pubkey,
    pub recipient: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            admin: *accounts.admin.key,
            token_mint: *accounts.token_mint.key,
            funding_mint: *accounts.funding_mint.key,
            state: *accounts.state.key,
            admin_token_ata: *accounts.admin_token_ata.key,
            token_vault: *accounts.token_vault.key,
            recipient: *accounts.recipient.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recipient,
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
            signer: pubkeys[0],
            admin: pubkeys[1],
            token_mint: pubkeys[2],
            funding_mint: pubkeys[3],
            state: pubkeys[4],
            admin_token_ata: pubkeys[5],
            token_vault: pubkeys[6],
            recipient: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.admin.clone(),
            accounts.token_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.state.clone(),
            accounts.admin_token_ata.clone(),
            accounts.token_vault.clone(),
            accounts.recipient.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
for InitializeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            admin: &arr[1],
            token_mint: &arr[2],
            funding_mint: &arr[3],
            state: &arr[4],
            admin_token_ata: &arr[5],
            token_vault: &arr[6],
            recipient: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
            system_program: &arr[10],
        }
    }
}
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub token_deposit: u64,
    pub funding_goal: u64,
    pub expiration_seconds: u32,
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
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_IX_DISCM, maybe_discm
                    ),
                ),
            );
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
pub fn initialize_ix(
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(crate::ID, keys, args)
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
    initialize_invoke_with_program_id(crate::ID, accounts, args)
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
    initialize_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.admin.key, keys.admin),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.state.key, keys.state),
        (*accounts.admin_token_ata.key, keys.admin_token_ata),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.recipient.key, keys.recipient),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
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
    for should_be_writable in [
        accounts.signer,
        accounts.admin,
        accounts.state,
        accounts.admin_token_ata,
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
    for should_be_signer in [accounts.signer, accounts.admin] {
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
pub const REFUND_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct RefundAccounts<'me, 'info> {
    pub signer: &'me AccountInfo<'info>,
    pub token_mint: &'me AccountInfo<'info>,
    pub funding_mint: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub signer_token_ata: &'me AccountInfo<'info>,
    pub signer_funding_ata: &'me AccountInfo<'info>,
    pub token_vault: &'me AccountInfo<'info>,
    pub funding_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub funding_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RefundKeys {
    pub signer: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub state: Pubkey,
    pub signer_token_ata: Pubkey,
    pub signer_funding_ata: Pubkey,
    pub token_vault: Pubkey,
    pub funding_vault: Pubkey,
    pub token_program: Pubkey,
    pub funding_token_program: Pubkey,
    pub associated_token_program: Pubkey,
}
impl From<RefundAccounts<'_, '_>> for RefundKeys {
    fn from(accounts: RefundAccounts) -> Self {
        Self {
            signer: *accounts.signer.key,
            token_mint: *accounts.token_mint.key,
            funding_mint: *accounts.funding_mint.key,
            state: *accounts.state.key,
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
impl From<RefundKeys> for [AccountMeta; REFUND_IX_ACCOUNTS_LEN] {
    fn from(keys: RefundKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: true,
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
                pubkey: keys.state,
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
impl From<[Pubkey; REFUND_IX_ACCOUNTS_LEN]> for RefundKeys {
    fn from(pubkeys: [Pubkey; REFUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: pubkeys[0],
            token_mint: pubkeys[1],
            funding_mint: pubkeys[2],
            state: pubkeys[3],
            signer_token_ata: pubkeys[4],
            signer_funding_ata: pubkeys[5],
            token_vault: pubkeys[6],
            funding_vault: pubkeys[7],
            token_program: pubkeys[8],
            funding_token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
        }
    }
}
impl<'info> From<RefundAccounts<'_, 'info>>
for [AccountInfo<'info>; REFUND_IX_ACCOUNTS_LEN] {
    fn from(accounts: RefundAccounts<'_, 'info>) -> Self {
        [
            accounts.signer.clone(),
            accounts.token_mint.clone(),
            accounts.funding_mint.clone(),
            accounts.state.clone(),
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; REFUND_IX_ACCOUNTS_LEN]>
for RefundAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REFUND_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            signer: &arr[0],
            token_mint: &arr[1],
            funding_mint: &arr[2],
            state: &arr[3],
            signer_token_ata: &arr[4],
            signer_funding_ata: &arr[5],
            token_vault: &arr[6],
            funding_vault: &arr[7],
            token_program: &arr[8],
            funding_token_program: &arr[9],
            associated_token_program: &arr[10],
        }
    }
}
pub const REFUND_IX_DISCM: [u8; 8] = [2, 96, 183, 251, 63, 208, 46, 46];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefundIxArgs {
    pub token_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RefundIxData(pub RefundIxArgs);
impl From<RefundIxArgs> for RefundIxData {
    fn from(args: RefundIxArgs) -> Self {
        Self(args)
    }
}
impl RefundIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REFUND_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REFUND_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RefundIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REFUND_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn refund_ix_with_program_id(
    program_id: Pubkey,
    keys: RefundKeys,
    args: RefundIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REFUND_IX_ACCOUNTS_LEN] = keys.into();
    let data: RefundIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn refund_ix(keys: RefundKeys, args: RefundIxArgs) -> std::io::Result<Instruction> {
    refund_ix_with_program_id(crate::ID, keys, args)
}
pub fn refund_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RefundAccounts<'_, '_>,
    args: RefundIxArgs,
) -> ProgramResult {
    let keys: RefundKeys = accounts.into();
    let ix = refund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn refund_invoke(
    accounts: RefundAccounts<'_, '_>,
    args: RefundIxArgs,
) -> ProgramResult {
    refund_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn refund_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RefundAccounts<'_, '_>,
    args: RefundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RefundKeys = accounts.into();
    let ix = refund_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn refund_invoke_signed(
    accounts: RefundAccounts<'_, '_>,
    args: RefundIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    refund_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn refund_verify_account_keys(
    accounts: RefundAccounts<'_, '_>,
    keys: RefundKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.signer.key, keys.signer),
        (*accounts.token_mint.key, keys.token_mint),
        (*accounts.funding_mint.key, keys.funding_mint),
        (*accounts.state.key, keys.state),
        (*accounts.signer_token_ata.key, keys.signer_token_ata),
        (*accounts.signer_funding_ata.key, keys.signer_funding_ata),
        (*accounts.token_vault.key, keys.token_vault),
        (*accounts.funding_vault.key, keys.funding_vault),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.funding_token_program.key, keys.funding_token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn refund_verify_writable_privileges<'me, 'info>(
    accounts: RefundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.state,
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
pub fn refund_verify_signer_privileges<'me, 'info>(
    accounts: RefundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn refund_verify_account_privileges<'me, 'info>(
    accounts: RefundAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    refund_verify_writable_privileges(accounts)?;
    refund_verify_signer_privileges(accounts)?;
    Ok(())
}
