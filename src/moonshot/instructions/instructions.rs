use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::io::Read;
pub const MOONSHOT_TOKEN_LAUNCHPAD: Pubkey = pubkey!("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG");

use super::typedefs::{ConfigParams, TokenMintParams, TradeParams};
#[derive(Clone, Debug, PartialEq)]
pub enum TokenLaunchpadProgramIx {
    TokenMint(TokenMintIxArgs),
    Buy(BuyIxArgs),
    Sell(SellIxArgs),
    MigrateFunds,
    ConfigInit(ConfigInitIxArgs),
    ConfigUpdate(ConfigUpdateIxArgs),
}
impl TokenLaunchpadProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            TOKEN_MINT_IX_DISCM => Ok(Self::TokenMint(TokenMintIxArgs::deserialize(&mut reader)?)),
            BUY_IX_DISCM => Ok(Self::Buy(BuyIxArgs::deserialize(&mut reader)?)),
            SELL_IX_DISCM => Ok(Self::Sell(SellIxArgs::deserialize(&mut reader)?)),
            MIGRATE_FUNDS_IX_DISCM => Ok(Self::MigrateFunds),
            CONFIG_INIT_IX_DISCM => Ok(Self::ConfigInit(ConfigInitIxArgs::deserialize(
                &mut reader,
            )?)),
            CONFIG_UPDATE_IX_DISCM => Ok(Self::ConfigUpdate(ConfigUpdateIxArgs::deserialize(
                &mut reader,
            )?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::TokenMint(args) => {
                writer.write_all(&TOKEN_MINT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Buy(args) => {
                writer.write_all(&BUY_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Sell(args) => {
                writer.write_all(&SELL_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::MigrateFunds => writer.write_all(&MIGRATE_FUNDS_IX_DISCM),
            Self::ConfigInit(args) => {
                writer.write_all(&CONFIG_INIT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ConfigUpdate(args) => {
                writer.write_all(&CONFIG_UPDATE_IX_DISCM)?;
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
pub const TOKEN_MINT_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct TokenMintAccounts<'me, 'info> {
    pub sender: &'me AccountInfo<'info>,
    pub backend_authority: &'me AccountInfo<'info>,
    pub curve_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub mint_metadata: &'me AccountInfo<'info>,
    pub curve_token_account: &'me AccountInfo<'info>,
    pub config_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub mpl_token_metadata: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TokenMintKeys {
    pub sender: Pubkey,
    pub backend_authority: Pubkey,
    pub curve_account: Pubkey,
    pub mint: Pubkey,
    pub mint_metadata: Pubkey,
    pub curve_token_account: Pubkey,
    pub config_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub mpl_token_metadata: Pubkey,
    pub system_program: Pubkey,
}
impl From<TokenMintAccounts<'_, '_>> for TokenMintKeys {
    fn from(accounts: TokenMintAccounts) -> Self {
        Self {
            sender: *accounts.sender.key,
            backend_authority: *accounts.backend_authority.key,
            curve_account: *accounts.curve_account.key,
            mint: *accounts.mint.key,
            mint_metadata: *accounts.mint_metadata.key,
            curve_token_account: *accounts.curve_token_account.key,
            config_account: *accounts.config_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            mpl_token_metadata: *accounts.mpl_token_metadata.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<TokenMintKeys> for [AccountMeta; TOKEN_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: TokenMintKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.backend_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.curve_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_metadata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config_account,
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
                pubkey: keys.mpl_token_metadata,
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
impl From<[Pubkey; TOKEN_MINT_IX_ACCOUNTS_LEN]> for TokenMintKeys {
    fn from(pubkeys: [Pubkey; TOKEN_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sender: pubkeys[0],
            backend_authority: pubkeys[1],
            curve_account: pubkeys[2],
            mint: pubkeys[3],
            mint_metadata: pubkeys[4],
            curve_token_account: pubkeys[5],
            config_account: pubkeys[6],
            token_program: pubkeys[7],
            associated_token_program: pubkeys[8],
            mpl_token_metadata: pubkeys[9],
            system_program: pubkeys[10],
        }
    }
}
impl<'info> From<TokenMintAccounts<'_, 'info>>
    for [AccountInfo<'info>; TOKEN_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: TokenMintAccounts<'_, 'info>) -> Self {
        [
            accounts.sender.clone(),
            accounts.backend_authority.clone(),
            accounts.curve_account.clone(),
            accounts.mint.clone(),
            accounts.mint_metadata.clone(),
            accounts.curve_token_account.clone(),
            accounts.config_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.mpl_token_metadata.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOKEN_MINT_IX_ACCOUNTS_LEN]>
    for TokenMintAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TOKEN_MINT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sender: &arr[0],
            backend_authority: &arr[1],
            curve_account: &arr[2],
            mint: &arr[3],
            mint_metadata: &arr[4],
            curve_token_account: &arr[5],
            config_account: &arr[6],
            token_program: &arr[7],
            associated_token_program: &arr[8],
            mpl_token_metadata: &arr[9],
            system_program: &arr[10],
        }
    }
}
pub const TOKEN_MINT_IX_DISCM: [u8; 8] = [3, 44, 164, 184, 123, 13, 245, 179];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenMintIxArgs {
    pub mint_params: TokenMintParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenMintIxData(pub TokenMintIxArgs);
impl From<TokenMintIxArgs> for TokenMintIxData {
    fn from(args: TokenMintIxArgs) -> Self {
        Self(args)
    }
}
impl TokenMintIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_MINT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TOKEN_MINT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(TokenMintIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_MINT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn token_mint_ix_with_program_id(
    program_id: Pubkey,
    keys: TokenMintKeys,
    args: TokenMintIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOKEN_MINT_IX_ACCOUNTS_LEN] = keys.into();
    let data: TokenMintIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn token_mint_ix(keys: TokenMintKeys, args: TokenMintIxArgs) -> std::io::Result<Instruction> {
    token_mint_ix_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, keys, args)
}
pub fn token_mint_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TokenMintAccounts<'_, '_>,
    args: TokenMintIxArgs,
) -> ProgramResult {
    let keys: TokenMintKeys = accounts.into();
    let ix = token_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn token_mint_invoke(
    accounts: TokenMintAccounts<'_, '_>,
    args: TokenMintIxArgs,
) -> ProgramResult {
    token_mint_invoke_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args)
}
pub fn token_mint_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TokenMintAccounts<'_, '_>,
    args: TokenMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TokenMintKeys = accounts.into();
    let ix = token_mint_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn token_mint_invoke_signed(
    accounts: TokenMintAccounts<'_, '_>,
    args: TokenMintIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    token_mint_invoke_signed_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args, seeds)
}
pub fn token_mint_verify_account_keys(
    accounts: TokenMintAccounts<'_, '_>,
    keys: TokenMintKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.sender.key, keys.sender),
        (*accounts.backend_authority.key, keys.backend_authority),
        (*accounts.curve_account.key, keys.curve_account),
        (*accounts.mint.key, keys.mint),
        (*accounts.mint_metadata.key, keys.mint_metadata),
        (*accounts.curve_token_account.key, keys.curve_token_account),
        (*accounts.config_account.key, keys.config_account),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.mpl_token_metadata.key, keys.mpl_token_metadata),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn token_mint_verify_writable_privileges<'me, 'info>(
    accounts: TokenMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.sender,
        accounts.curve_account,
        accounts.mint,
        accounts.mint_metadata,
        accounts.curve_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn token_mint_verify_signer_privileges<'me, 'info>(
    accounts: TokenMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender, accounts.backend_authority, accounts.mint] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn token_mint_verify_account_privileges<'me, 'info>(
    accounts: TokenMintAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    token_mint_verify_writable_privileges(accounts)?;
    token_mint_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const BUY_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct BuyAccounts<'me, 'info> {
    pub sender: &'me AccountInfo<'info>,
    pub backend_authority: &'me AccountInfo<'info>,
    pub sender_token_account: &'me AccountInfo<'info>,
    pub curve_account: &'me AccountInfo<'info>,
    pub curve_token_account: &'me AccountInfo<'info>,
    pub dex_fee: &'me AccountInfo<'info>,
    pub helio_fee: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub config_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuyKeys {
    pub sender: Pubkey,
    pub backend_authority: Pubkey,
    pub sender_token_account: Pubkey,
    pub curve_account: Pubkey,
    pub curve_token_account: Pubkey,
    pub dex_fee: Pubkey,
    pub helio_fee: Pubkey,
    pub mint: Pubkey,
    pub config_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<BuyAccounts<'_, '_>> for BuyKeys {
    fn from(accounts: BuyAccounts) -> Self {
        Self {
            sender: *accounts.sender.key,
            backend_authority: *accounts.backend_authority.key,
            sender_token_account: *accounts.sender_token_account.key,
            curve_account: *accounts.curve_account.key,
            curve_token_account: *accounts.curve_token_account.key,
            dex_fee: *accounts.dex_fee.key,
            helio_fee: *accounts.helio_fee.key,
            mint: *accounts.mint.key,
            config_account: *accounts.config_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}

impl From<BuyKeys> for [AccountMeta; BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: BuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.backend_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.helio_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config_account,
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
impl From<[Pubkey; BUY_IX_ACCOUNTS_LEN]> for BuyKeys {
    fn from(pubkeys: [Pubkey; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sender: pubkeys[0],
            backend_authority: pubkeys[1],
            sender_token_account: pubkeys[2],
            curve_account: pubkeys[3],
            curve_token_account: pubkeys[4],
            dex_fee: pubkeys[5],
            helio_fee: pubkeys[6],
            mint: pubkeys[7],
            config_account: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<BuyAccounts<'_, 'info>> for [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN] {
    fn from(accounts: BuyAccounts<'_, 'info>) -> Self {
        [
            accounts.sender.clone(),
            accounts.backend_authority.clone(),
            accounts.sender_token_account.clone(),
            accounts.curve_account.clone(),
            accounts.curve_token_account.clone(),
            accounts.dex_fee.clone(),
            accounts.helio_fee.clone(),
            accounts.mint.clone(),
            accounts.config_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]> for BuyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sender: &arr[0],
            backend_authority: &arr[1],
            sender_token_account: &arr[2],
            curve_account: &arr[3],
            curve_token_account: &arr[4],
            dex_fee: &arr[5],
            helio_fee: &arr[6],
            mint: &arr[7],
            config_account: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const BUY_IX_DISCM: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyIxArgs {
    pub data: TradeParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct BuyIxData(pub BuyIxArgs);
impl From<BuyIxArgs> for BuyIxData {
    fn from(args: BuyIxArgs) -> Self {
        Self(args)
    }
}
impl BuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BUY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BUY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(BuyIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BUY_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn buy_ix_with_program_id(
    program_id: Pubkey,
    keys: BuyKeys,
    args: BuyIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BUY_IX_ACCOUNTS_LEN] = keys.into();
    let data: BuyIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn buy_ix(keys: BuyKeys, args: BuyIxArgs) -> std::io::Result<Instruction> {
    buy_ix_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, keys, args)
}
pub fn buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn buy_invoke(accounts: BuyAccounts<'_, '_>, args: BuyIxArgs) -> ProgramResult {
    buy_invoke_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args)
}
pub fn buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BuyKeys = accounts.into();
    let ix = buy_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn buy_invoke_signed(
    accounts: BuyAccounts<'_, '_>,
    args: BuyIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    buy_invoke_signed_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args, seeds)
}
pub fn buy_verify_account_keys(
    accounts: BuyAccounts<'_, '_>,
    keys: BuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.sender.key, keys.sender),
        (*accounts.backend_authority.key, keys.backend_authority),
        (
            *accounts.sender_token_account.key,
            keys.sender_token_account,
        ),
        (*accounts.curve_account.key, keys.curve_account),
        (*accounts.curve_token_account.key, keys.curve_token_account),
        (*accounts.dex_fee.key, keys.dex_fee),
        (*accounts.helio_fee.key, keys.helio_fee),
        (*accounts.mint.key, keys.mint),
        (*accounts.config_account.key, keys.config_account),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn buy_verify_writable_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.sender,
        accounts.sender_token_account,
        accounts.curve_account,
        accounts.curve_token_account,
        accounts.dex_fee,
        accounts.helio_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn buy_verify_signer_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender, accounts.backend_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn buy_verify_account_privileges<'me, 'info>(
    accounts: BuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    buy_verify_writable_privileges(accounts)?;
    buy_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SELL_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct SellAccounts<'me, 'info> {
    pub sender: &'me AccountInfo<'info>,
    pub backend_authority: &'me AccountInfo<'info>,
    pub sender_token_account: &'me AccountInfo<'info>,
    pub curve_account: &'me AccountInfo<'info>,
    pub curve_token_account: &'me AccountInfo<'info>,
    pub dex_fee: &'me AccountInfo<'info>,
    pub helio_fee: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub config_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SellKeys {
    pub sender: Pubkey,
    pub backend_authority: Pubkey,
    pub sender_token_account: Pubkey,
    pub curve_account: Pubkey,
    pub curve_token_account: Pubkey,
    pub dex_fee: Pubkey,
    pub helio_fee: Pubkey,
    pub mint: Pubkey,
    pub config_account: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<SellAccounts<'_, '_>> for SellKeys {
    fn from(accounts: SellAccounts) -> Self {
        Self {
            sender: *accounts.sender.key,
            backend_authority: *accounts.backend_authority.key,
            sender_token_account: *accounts.sender_token_account.key,
            curve_account: *accounts.curve_account.key,
            curve_token_account: *accounts.curve_token_account.key,
            dex_fee: *accounts.dex_fee.key,
            helio_fee: *accounts.helio_fee.key,
            mint: *accounts.mint.key,
            config_account: *accounts.config_account.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<SellKeys> for [AccountMeta; SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: SellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.sender,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.backend_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.helio_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config_account,
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
impl From<[Pubkey; SELL_IX_ACCOUNTS_LEN]> for SellKeys {
    fn from(pubkeys: [Pubkey; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sender: pubkeys[0],
            backend_authority: pubkeys[1],
            sender_token_account: pubkeys[2],
            curve_account: pubkeys[3],
            curve_token_account: pubkeys[4],
            dex_fee: pubkeys[5],
            helio_fee: pubkeys[6],
            mint: pubkeys[7],
            config_account: pubkeys[8],
            token_program: pubkeys[9],
            associated_token_program: pubkeys[10],
            system_program: pubkeys[11],
        }
    }
}
impl<'info> From<SellAccounts<'_, 'info>> for [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN] {
    fn from(accounts: SellAccounts<'_, 'info>) -> Self {
        [
            accounts.sender.clone(),
            accounts.backend_authority.clone(),
            accounts.sender_token_account.clone(),
            accounts.curve_account.clone(),
            accounts.curve_token_account.clone(),
            accounts.dex_fee.clone(),
            accounts.helio_fee.clone(),
            accounts.mint.clone(),
            accounts.config_account.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]>
    for SellAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            sender: &arr[0],
            backend_authority: &arr[1],
            sender_token_account: &arr[2],
            curve_account: &arr[3],
            curve_token_account: &arr[4],
            dex_fee: &arr[5],
            helio_fee: &arr[6],
            mint: &arr[7],
            config_account: &arr[8],
            token_program: &arr[9],
            associated_token_program: &arr[10],
            system_program: &arr[11],
        }
    }
}
pub const SELL_IX_DISCM: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellIxArgs {
    pub data: TradeParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SellIxData(pub SellIxArgs);
impl From<SellIxArgs> for SellIxData {
    fn from(args: SellIxArgs) -> Self {
        Self(args)
    }
}
impl SellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SELL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SELL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SellIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SELL_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sell_ix_with_program_id(
    program_id: Pubkey,
    keys: SellKeys,
    args: SellIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SELL_IX_ACCOUNTS_LEN] = keys.into();
    let data: SellIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sell_ix(keys: SellKeys, args: SellIxArgs) -> std::io::Result<Instruction> {
    sell_ix_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, keys, args)
}
pub fn sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn sell_invoke(accounts: SellAccounts<'_, '_>, args: SellIxArgs) -> ProgramResult {
    sell_invoke_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args)
}
pub fn sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SellKeys = accounts.into();
    let ix = sell_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sell_invoke_signed(
    accounts: SellAccounts<'_, '_>,
    args: SellIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sell_invoke_signed_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args, seeds)
}
pub fn sell_verify_account_keys(
    accounts: SellAccounts<'_, '_>,
    keys: SellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.sender.key, keys.sender),
        (*accounts.backend_authority.key, keys.backend_authority),
        (
            *accounts.sender_token_account.key,
            keys.sender_token_account,
        ),
        (*accounts.curve_account.key, keys.curve_account),
        (*accounts.curve_token_account.key, keys.curve_token_account),
        (*accounts.dex_fee.key, keys.dex_fee),
        (*accounts.helio_fee.key, keys.helio_fee),
        (*accounts.mint.key, keys.mint),
        (*accounts.config_account.key, keys.config_account),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sell_verify_writable_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.sender,
        accounts.sender_token_account,
        accounts.curve_account,
        accounts.curve_token_account,
        accounts.dex_fee,
        accounts.helio_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sell_verify_signer_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.sender, accounts.backend_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn sell_verify_account_privileges<'me, 'info>(
    accounts: SellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sell_verify_writable_privileges(accounts)?;
    sell_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MIGRATE_FUNDS_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct MigrateFundsAccounts<'me, 'info> {
    pub backend_authority: &'me AccountInfo<'info>,
    pub migration_authority: &'me AccountInfo<'info>,
    pub curve_account: &'me AccountInfo<'info>,
    pub curve_token_account: &'me AccountInfo<'info>,
    pub migration_authority_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub config_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MigrateFundsKeys {
    pub backend_authority: Pubkey,
    pub migration_authority: Pubkey,
    pub curve_account: Pubkey,
    pub curve_token_account: Pubkey,
    pub migration_authority_token_account: Pubkey,
    pub mint: Pubkey,
    pub config_account: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}

impl From<MigrateFundsAccounts<'_, '_>> for MigrateFundsKeys {
    fn from(accounts: MigrateFundsAccounts) -> Self {
        Self {
            backend_authority: *accounts.backend_authority.key,
            migration_authority: *accounts.migration_authority.key,
            curve_account: *accounts.curve_account.key,
            curve_token_account: *accounts.curve_token_account.key,
            migration_authority_token_account: *accounts.migration_authority_token_account.key,
            mint: *accounts.mint.key,
            config_account: *accounts.config_account.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
        }
    }
}
impl From<MigrateFundsKeys> for [AccountMeta; MIGRATE_FUNDS_IX_ACCOUNTS_LEN] {
    fn from(keys: MigrateFundsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.backend_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.migration_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.migration_authority_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config_account,
                is_signer: false,
                is_writable: false,
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
impl From<[Pubkey; MIGRATE_FUNDS_IX_ACCOUNTS_LEN]> for MigrateFundsKeys {
    fn from(pubkeys: [Pubkey; MIGRATE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            backend_authority: pubkeys[0],
            migration_authority: pubkeys[1],
            curve_account: pubkeys[2],
            curve_token_account: pubkeys[3],
            migration_authority_token_account: pubkeys[4],
            mint: pubkeys[5],
            config_account: pubkeys[6],
            system_program: pubkeys[7],
            token_program: pubkeys[8],
            associated_token_program: pubkeys[9],
        }
    }
}
impl<'info> From<MigrateFundsAccounts<'_, 'info>>
    for [AccountInfo<'info>; MIGRATE_FUNDS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MigrateFundsAccounts<'_, 'info>) -> Self {
        [
            accounts.backend_authority.clone(),
            accounts.migration_authority.clone(),
            accounts.curve_account.clone(),
            accounts.curve_token_account.clone(),
            accounts.migration_authority_token_account.clone(),
            accounts.mint.clone(),
            accounts.config_account.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MIGRATE_FUNDS_IX_ACCOUNTS_LEN]>
    for MigrateFundsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MIGRATE_FUNDS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            backend_authority: &arr[0],
            migration_authority: &arr[1],
            curve_account: &arr[2],
            curve_token_account: &arr[3],
            migration_authority_token_account: &arr[4],
            mint: &arr[5],
            config_account: &arr[6],
            system_program: &arr[7],
            token_program: &arr[8],
            associated_token_program: &arr[9],
        }
    }
}
pub const MIGRATE_FUNDS_IX_DISCM: [u8; 8] = [42, 229, 10, 231, 189, 62, 193, 174];
#[derive(Clone, Debug, PartialEq)]
pub struct MigrateFundsIxData;
impl MigrateFundsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MIGRATE_FUNDS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MIGRATE_FUNDS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MIGRATE_FUNDS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn migrate_funds_ix_with_program_id(
    program_id: Pubkey,
    keys: MigrateFundsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MIGRATE_FUNDS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MigrateFundsIxData.try_to_vec()?,
    })
}
pub fn migrate_funds_ix(keys: MigrateFundsKeys) -> std::io::Result<Instruction> {
    migrate_funds_ix_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, keys)
}
pub fn migrate_funds_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MigrateFundsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MigrateFundsKeys = accounts.into();
    let ix = migrate_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn migrate_funds_invoke(accounts: MigrateFundsAccounts<'_, '_>) -> ProgramResult {
    migrate_funds_invoke_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts)
}
pub fn migrate_funds_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MigrateFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MigrateFundsKeys = accounts.into();
    let ix = migrate_funds_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn migrate_funds_invoke_signed(
    accounts: MigrateFundsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    migrate_funds_invoke_signed_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, seeds)
}
pub fn migrate_funds_verify_account_keys(
    accounts: MigrateFundsAccounts<'_, '_>,
    keys: MigrateFundsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.backend_authority.key, keys.backend_authority),
        (*accounts.migration_authority.key, keys.migration_authority),
        (*accounts.curve_account.key, keys.curve_account),
        (*accounts.curve_token_account.key, keys.curve_token_account),
        (
            *accounts.migration_authority_token_account.key,
            keys.migration_authority_token_account,
        ),
        (*accounts.mint.key, keys.mint),
        (*accounts.config_account.key, keys.config_account),
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
pub fn migrate_funds_verify_writable_privileges<'me, 'info>(
    accounts: MigrateFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.migration_authority,
        accounts.curve_account,
        accounts.curve_token_account,
        accounts.migration_authority_token_account,
        accounts.mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn migrate_funds_verify_signer_privileges<'me, 'info>(
    accounts: MigrateFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.backend_authority, accounts.migration_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn migrate_funds_verify_account_privileges<'me, 'info>(
    accounts: MigrateFundsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    migrate_funds_verify_writable_privileges(accounts)?;
    migrate_funds_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIG_INIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ConfigInitAccounts<'me, 'info> {
    pub config_authority: &'me AccountInfo<'info>,
    pub config_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigInitKeys {
    pub config_authority: Pubkey,
    pub config_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<ConfigInitAccounts<'_, '_>> for ConfigInitKeys {
    fn from(accounts: ConfigInitAccounts) -> Self {
        Self {
            config_authority: *accounts.config_authority.key,
            config_account: *accounts.config_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ConfigInitKeys> for [AccountMeta; CONFIG_INIT_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigInitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config_authority,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.config_account,
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
impl From<[Pubkey; CONFIG_INIT_IX_ACCOUNTS_LEN]> for ConfigInitKeys {
    fn from(pubkeys: [Pubkey; CONFIG_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config_authority: pubkeys[0],
            config_account: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<ConfigInitAccounts<'_, 'info>>
    for [AccountInfo<'info>; CONFIG_INIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ConfigInitAccounts<'_, 'info>) -> Self {
        [
            accounts.config_authority.clone(),
            accounts.config_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONFIG_INIT_IX_ACCOUNTS_LEN]>
    for ConfigInitAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CONFIG_INIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config_authority: &arr[0],
            config_account: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CONFIG_INIT_IX_DISCM: [u8; 8] = [13, 236, 164, 173, 106, 253, 164, 185];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigInitIxArgs {
    pub data: ConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigInitIxData(pub ConfigInitIxArgs);
impl From<ConfigInitIxArgs> for ConfigInitIxData {
    fn from(args: ConfigInitIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigInitIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_INIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_INIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigInitIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_INIT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn config_init_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigInitKeys,
    args: ConfigInitIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIG_INIT_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigInitIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn config_init_ix(
    keys: ConfigInitKeys,
    args: ConfigInitIxArgs,
) -> std::io::Result<Instruction> {
    config_init_ix_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, keys, args)
}
pub fn config_init_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigInitAccounts<'_, '_>,
    args: ConfigInitIxArgs,
) -> ProgramResult {
    let keys: ConfigInitKeys = accounts.into();
    let ix = config_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn config_init_invoke(
    accounts: ConfigInitAccounts<'_, '_>,
    args: ConfigInitIxArgs,
) -> ProgramResult {
    config_init_invoke_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args)
}
pub fn config_init_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigInitAccounts<'_, '_>,
    args: ConfigInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigInitKeys = accounts.into();
    let ix = config_init_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn config_init_invoke_signed(
    accounts: ConfigInitAccounts<'_, '_>,
    args: ConfigInitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    config_init_invoke_signed_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args, seeds)
}
pub fn config_init_verify_account_keys(
    accounts: ConfigInitAccounts<'_, '_>,
    keys: ConfigInitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config_authority.key, keys.config_authority),
        (*accounts.config_account.key, keys.config_account),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn config_init_verify_writable_privileges<'me, 'info>(
    accounts: ConfigInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config_authority, accounts.config_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn config_init_verify_signer_privileges<'me, 'info>(
    accounts: ConfigInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.config_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn config_init_verify_account_privileges<'me, 'info>(
    accounts: ConfigInitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    config_init_verify_writable_privileges(accounts)?;
    config_init_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CONFIG_UPDATE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct ConfigUpdateAccounts<'me, 'info> {
    pub config_authority: &'me AccountInfo<'info>,
    pub config_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConfigUpdateKeys {
    pub config_authority: Pubkey,
    pub config_account: Pubkey,
}
impl From<ConfigUpdateAccounts<'_, '_>> for ConfigUpdateKeys {
    fn from(accounts: ConfigUpdateAccounts) -> Self {
        Self {
            config_authority: *accounts.config_authority.key,
            config_account: *accounts.config_account.key,
        }
    }
}
impl From<ConfigUpdateKeys> for [AccountMeta; CONFIG_UPDATE_IX_ACCOUNTS_LEN] {
    fn from(keys: ConfigUpdateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.config_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.config_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CONFIG_UPDATE_IX_ACCOUNTS_LEN]> for ConfigUpdateKeys {
    fn from(pubkeys: [Pubkey; CONFIG_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config_authority: pubkeys[0],
            config_account: pubkeys[1],
        }
    }
}
impl<'info> From<ConfigUpdateAccounts<'_, 'info>>
    for [AccountInfo<'info>; CONFIG_UPDATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ConfigUpdateAccounts<'_, 'info>) -> Self {
        [
            accounts.config_authority.clone(),
            accounts.config_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONFIG_UPDATE_IX_ACCOUNTS_LEN]>
    for ConfigUpdateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CONFIG_UPDATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            config_authority: &arr[0],
            config_account: &arr[1],
        }
    }
}
pub const CONFIG_UPDATE_IX_DISCM: [u8; 8] = [80, 37, 109, 136, 82, 135, 89, 241];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigUpdateIxArgs {
    pub data: ConfigParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigUpdateIxData(pub ConfigUpdateIxArgs);
impl From<ConfigUpdateIxArgs> for ConfigUpdateIxData {
    fn from(args: ConfigUpdateIxArgs) -> Self {
        Self(args)
    }
}
impl ConfigUpdateIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_UPDATE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_UPDATE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigUpdateIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_UPDATE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn config_update_ix_with_program_id(
    program_id: Pubkey,
    keys: ConfigUpdateKeys,
    args: ConfigUpdateIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CONFIG_UPDATE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ConfigUpdateIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn config_update_ix(
    keys: ConfigUpdateKeys,
    args: ConfigUpdateIxArgs,
) -> std::io::Result<Instruction> {
    config_update_ix_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, keys, args)
}
pub fn config_update_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ConfigUpdateAccounts<'_, '_>,
    args: ConfigUpdateIxArgs,
) -> ProgramResult {
    let keys: ConfigUpdateKeys = accounts.into();
    let ix = config_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn config_update_invoke(
    accounts: ConfigUpdateAccounts<'_, '_>,
    args: ConfigUpdateIxArgs,
) -> ProgramResult {
    config_update_invoke_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args)
}
pub fn config_update_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ConfigUpdateAccounts<'_, '_>,
    args: ConfigUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ConfigUpdateKeys = accounts.into();
    let ix = config_update_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn config_update_invoke_signed(
    accounts: ConfigUpdateAccounts<'_, '_>,
    args: ConfigUpdateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    config_update_invoke_signed_with_program_id(MOONSHOT_TOKEN_LAUNCHPAD, accounts, args, seeds)
}
pub fn config_update_verify_account_keys(
    accounts: ConfigUpdateAccounts<'_, '_>,
    keys: ConfigUpdateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.config_authority.key, keys.config_authority),
        (*accounts.config_account.key, keys.config_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn config_update_verify_writable_privileges<'me, 'info>(
    accounts: ConfigUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.config_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn config_update_verify_signer_privileges<'me, 'info>(
    accounts: ConfigUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.config_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn config_update_verify_account_privileges<'me, 'info>(
    accounts: ConfigUpdateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    config_update_verify_writable_privileges(accounts)?;
    config_update_verify_signer_privileges(accounts)?;
    Ok(())
}
