use crate::*;
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

use super::{RoutePlanStep, JUPITER_ID};
#[derive(Clone, Debug, PartialEq)]
pub enum JupiterProgramIx {
    Route(RouteIxArgs),
    RouteWithTokenLedger(RouteWithTokenLedgerIxArgs),
    ExactOutRoute(ExactOutRouteIxArgs),
    SharedAccountsRoute(SharedAccountsRouteIxArgs),
    SharedAccountsRouteWithTokenLedger(SharedAccountsRouteWithTokenLedgerIxArgs),
    SharedAccountsExactOutRoute(SharedAccountsExactOutRouteIxArgs),
    SetTokenLedger,
    CreateOpenOrders,
    CreateTokenAccount(CreateTokenAccountIxArgs),
    CreateProgramOpenOrders(CreateProgramOpenOrdersIxArgs),
    Claim(ClaimIxArgs),
    ClaimToken(ClaimTokenIxArgs),
    CreateTokenLedger,
    MercurialSwap,
    CykuraSwap,
    SerumSwap,
    SaberSwap,
    SaberAddDecimals,
    TokenSwap,
    TokenSwapV2,
    SenchaSwap,
    StepSwap,
    CropperSwap,
    RaydiumSwap,
    CremaSwap,
    LifinitySwap,
    MarinadeDeposit,
    MarinadeUnstake,
    AldrinSwap,
    AldrinV2Swap,
    WhirlpoolSwap,
    WhirlpoolSwapV2,
    InvariantSwap,
    MeteoraSwap,
    GoosefxSwap,
    DeltafiSwap,
    BalansolSwap,
    MarcoPoloSwap,
    DradexSwap,
    LifinityV2Swap,
    RaydiumClmmSwap,
    RaydiumClmmSwapV2,
    PhoenixSwap,
    SymmetrySwap,
    HeliumTreasuryManagementRedeemV0,
    GoosefxV2Swap,
    PerpsSwap,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmmSwap,
    OpenBookV2Swap,
    CloneSwap,
    RaydiumCpSwap,
    OneIntroSwap,
    PumpdotfunWrappedBuy,
    PumpdotfunWrappedSell,
    PerpsV2Swap,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell,
    StabbleStableSwap,
    StabbleWeightedSwap,
    ObricSwap,
}
impl JupiterProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            ROUTE_IX_DISCM => Ok(Self::Route(RouteIxArgs::deserialize(&mut reader)?)),
            ROUTE_WITH_TOKEN_LEDGER_IX_DISCM => Ok(Self::RouteWithTokenLedger(
                RouteWithTokenLedgerIxArgs::deserialize(&mut reader)?,
            )),
            EXACT_OUT_ROUTE_IX_DISCM => Ok(Self::ExactOutRoute(ExactOutRouteIxArgs::deserialize(
                &mut reader,
            )?)),
            SHARED_ACCOUNTS_ROUTE_IX_DISCM => Ok(Self::SharedAccountsRoute(
                SharedAccountsRouteIxArgs::deserialize(&mut reader)?,
            )),
            SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM => {
                Ok(Self::SharedAccountsRouteWithTokenLedger(
                    SharedAccountsRouteWithTokenLedgerIxArgs::deserialize(&mut reader)?,
                ))
            }
            SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM => Ok(Self::SharedAccountsExactOutRoute(
                SharedAccountsExactOutRouteIxArgs::deserialize(&mut reader)?,
            )),
            SET_TOKEN_LEDGER_IX_DISCM => Ok(Self::SetTokenLedger),
            CREATE_OPEN_ORDERS_IX_DISCM => Ok(Self::CreateOpenOrders),
            CREATE_TOKEN_ACCOUNT_IX_DISCM => Ok(Self::CreateTokenAccount(
                CreateTokenAccountIxArgs::deserialize(&mut reader)?,
            )),
            CREATE_PROGRAM_OPEN_ORDERS_IX_DISCM => Ok(Self::CreateProgramOpenOrders(
                CreateProgramOpenOrdersIxArgs::deserialize(&mut reader)?,
            )),
            CLAIM_IX_DISCM => Ok(Self::Claim(ClaimIxArgs::deserialize(&mut reader)?)),
            CLAIM_TOKEN_IX_DISCM => Ok(Self::ClaimToken(ClaimTokenIxArgs::deserialize(
                &mut reader,
            )?)),
            CREATE_TOKEN_LEDGER_IX_DISCM => Ok(Self::CreateTokenLedger),
            MERCURIAL_SWAP_IX_DISCM => Ok(Self::MercurialSwap),
            CYKURA_SWAP_IX_DISCM => Ok(Self::CykuraSwap),
            SERUM_SWAP_IX_DISCM => Ok(Self::SerumSwap),
            SABER_SWAP_IX_DISCM => Ok(Self::SaberSwap),
            SABER_ADD_DECIMALS_IX_DISCM => Ok(Self::SaberAddDecimals),
            TOKEN_SWAP_IX_DISCM => Ok(Self::TokenSwap),
            TOKEN_SWAP_V2_IX_DISCM => Ok(Self::TokenSwapV2),
            SENCHA_SWAP_IX_DISCM => Ok(Self::SenchaSwap),
            STEP_SWAP_IX_DISCM => Ok(Self::StepSwap),
            CROPPER_SWAP_IX_DISCM => Ok(Self::CropperSwap),
            RAYDIUM_SWAP_IX_DISCM => Ok(Self::RaydiumSwap),
            CREMA_SWAP_IX_DISCM => Ok(Self::CremaSwap),
            LIFINITY_SWAP_IX_DISCM => Ok(Self::LifinitySwap),
            MARINADE_DEPOSIT_IX_DISCM => Ok(Self::MarinadeDeposit),
            MARINADE_UNSTAKE_IX_DISCM => Ok(Self::MarinadeUnstake),
            ALDRIN_SWAP_IX_DISCM => Ok(Self::AldrinSwap),
            ALDRIN_V2_SWAP_IX_DISCM => Ok(Self::AldrinV2Swap),
            WHIRLPOOL_SWAP_IX_DISCM => Ok(Self::WhirlpoolSwap),
            WHIRLPOOL_SWAP_V2_IX_DISCM => Ok(Self::WhirlpoolSwapV2),
            INVARIANT_SWAP_IX_DISCM => Ok(Self::InvariantSwap),
            METEORA_SWAP_IX_DISCM => Ok(Self::MeteoraSwap),
            GOOSEFX_SWAP_IX_DISCM => Ok(Self::GoosefxSwap),
            DELTAFI_SWAP_IX_DISCM => Ok(Self::DeltafiSwap),
            BALANSOL_SWAP_IX_DISCM => Ok(Self::BalansolSwap),
            MARCO_POLO_SWAP_IX_DISCM => Ok(Self::MarcoPoloSwap),
            DRADEX_SWAP_IX_DISCM => Ok(Self::DradexSwap),
            LIFINITY_V2_SWAP_IX_DISCM => Ok(Self::LifinityV2Swap),
            RAYDIUM_CLMM_SWAP_IX_DISCM => Ok(Self::RaydiumClmmSwap),
            RAYDIUM_CLMM_SWAP_V2_IX_DISCM => Ok(Self::RaydiumClmmSwapV2),
            PHOENIX_SWAP_IX_DISCM => Ok(Self::PhoenixSwap),
            SYMMETRY_SWAP_IX_DISCM => Ok(Self::SymmetrySwap),
            HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_DISCM => {
                Ok(Self::HeliumTreasuryManagementRedeemV0)
            }
            GOOSEFX_V2_SWAP_IX_DISCM => Ok(Self::GoosefxV2Swap),
            PERPS_SWAP_IX_DISCM => Ok(Self::PerpsSwap),
            PERPS_ADD_LIQUIDITY_IX_DISCM => Ok(Self::PerpsAddLiquidity),
            PERPS_REMOVE_LIQUIDITY_IX_DISCM => Ok(Self::PerpsRemoveLiquidity),
            METEORA_DLMM_SWAP_IX_DISCM => Ok(Self::MeteoraDlmmSwap),
            OPEN_BOOK_V2_SWAP_IX_DISCM => Ok(Self::OpenBookV2Swap),
            CLONE_SWAP_IX_DISCM => Ok(Self::CloneSwap),
            RAYDIUM_CP_SWAP_IX_DISCM => Ok(Self::RaydiumCpSwap),
            ONE_INTRO_SWAP_IX_DISCM => Ok(Self::OneIntroSwap),
            PUMPDOTFUN_WRAPPED_BUY_IX_DISCM => Ok(Self::PumpdotfunWrappedBuy),
            PUMPDOTFUN_WRAPPED_SELL_IX_DISCM => Ok(Self::PumpdotfunWrappedSell),
            PERPS_V2_SWAP_IX_DISCM => Ok(Self::PerpsV2Swap),
            PERPS_V2_ADD_LIQUIDITY_IX_DISCM => Ok(Self::PerpsV2AddLiquidity),
            PERPS_V2_REMOVE_LIQUIDITY_IX_DISCM => Ok(Self::PerpsV2RemoveLiquidity),
            MOONSHOT_WRAPPED_BUY_IX_DISCM => Ok(Self::MoonshotWrappedBuy),
            MOONSHOT_WRAPPED_SELL_IX_DISCM => Ok(Self::MoonshotWrappedSell),
            STABBLE_STABLE_SWAP_IX_DISCM => Ok(Self::StabbleStableSwap),
            STABBLE_WEIGHTED_SWAP_IX_DISCM => Ok(Self::StabbleWeightedSwap),
            OBRIC_SWAP_IX_DISCM => Ok(Self::ObricSwap),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Route(args) => {
                writer.write_all(&ROUTE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RouteWithTokenLedger(args) => {
                writer.write_all(&ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ExactOutRoute(args) => {
                writer.write_all(&EXACT_OUT_ROUTE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SharedAccountsRoute(args) => {
                writer.write_all(&SHARED_ACCOUNTS_ROUTE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SharedAccountsRouteWithTokenLedger(args) => {
                writer.write_all(&SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SharedAccountsExactOutRoute(args) => {
                writer.write_all(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::SetTokenLedger => writer.write_all(&SET_TOKEN_LEDGER_IX_DISCM),
            Self::CreateOpenOrders => writer.write_all(&CREATE_OPEN_ORDERS_IX_DISCM),
            Self::CreateTokenAccount(args) => {
                writer.write_all(&CREATE_TOKEN_ACCOUNT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateProgramOpenOrders(args) => {
                writer.write_all(&CREATE_PROGRAM_OPEN_ORDERS_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Claim(args) => {
                writer.write_all(&CLAIM_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimToken(args) => {
                writer.write_all(&CLAIM_TOKEN_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CreateTokenLedger => writer.write_all(&CREATE_TOKEN_LEDGER_IX_DISCM),
            Self::MercurialSwap => writer.write_all(&MERCURIAL_SWAP_IX_DISCM),
            Self::CykuraSwap => writer.write_all(&CYKURA_SWAP_IX_DISCM),
            Self::SerumSwap => writer.write_all(&SERUM_SWAP_IX_DISCM),
            Self::SaberSwap => writer.write_all(&SABER_SWAP_IX_DISCM),
            Self::SaberAddDecimals => writer.write_all(&SABER_ADD_DECIMALS_IX_DISCM),
            Self::TokenSwap => writer.write_all(&TOKEN_SWAP_IX_DISCM),
            Self::TokenSwapV2 => writer.write_all(&TOKEN_SWAP_V2_IX_DISCM),
            Self::SenchaSwap => writer.write_all(&SENCHA_SWAP_IX_DISCM),
            Self::StepSwap => writer.write_all(&STEP_SWAP_IX_DISCM),
            Self::CropperSwap => writer.write_all(&CROPPER_SWAP_IX_DISCM),
            Self::RaydiumSwap => writer.write_all(&RAYDIUM_SWAP_IX_DISCM),
            Self::CremaSwap => writer.write_all(&CREMA_SWAP_IX_DISCM),
            Self::LifinitySwap => writer.write_all(&LIFINITY_SWAP_IX_DISCM),
            Self::MarinadeDeposit => writer.write_all(&MARINADE_DEPOSIT_IX_DISCM),
            Self::MarinadeUnstake => writer.write_all(&MARINADE_UNSTAKE_IX_DISCM),
            Self::AldrinSwap => writer.write_all(&ALDRIN_SWAP_IX_DISCM),
            Self::AldrinV2Swap => writer.write_all(&ALDRIN_V2_SWAP_IX_DISCM),
            Self::WhirlpoolSwap => writer.write_all(&WHIRLPOOL_SWAP_IX_DISCM),
            Self::WhirlpoolSwapV2 => writer.write_all(&WHIRLPOOL_SWAP_V2_IX_DISCM),
            Self::InvariantSwap => writer.write_all(&INVARIANT_SWAP_IX_DISCM),
            Self::MeteoraSwap => writer.write_all(&METEORA_SWAP_IX_DISCM),
            Self::GoosefxSwap => writer.write_all(&GOOSEFX_SWAP_IX_DISCM),
            Self::DeltafiSwap => writer.write_all(&DELTAFI_SWAP_IX_DISCM),
            Self::BalansolSwap => writer.write_all(&BALANSOL_SWAP_IX_DISCM),
            Self::MarcoPoloSwap => writer.write_all(&MARCO_POLO_SWAP_IX_DISCM),
            Self::DradexSwap => writer.write_all(&DRADEX_SWAP_IX_DISCM),
            Self::LifinityV2Swap => writer.write_all(&LIFINITY_V2_SWAP_IX_DISCM),
            Self::RaydiumClmmSwap => writer.write_all(&RAYDIUM_CLMM_SWAP_IX_DISCM),
            Self::RaydiumClmmSwapV2 => writer.write_all(&RAYDIUM_CLMM_SWAP_V2_IX_DISCM),
            Self::PhoenixSwap => writer.write_all(&PHOENIX_SWAP_IX_DISCM),
            Self::SymmetrySwap => writer.write_all(&SYMMETRY_SWAP_IX_DISCM),
            Self::HeliumTreasuryManagementRedeemV0 => {
                writer.write_all(&HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_DISCM)
            }
            Self::GoosefxV2Swap => writer.write_all(&GOOSEFX_V2_SWAP_IX_DISCM),
            Self::PerpsSwap => writer.write_all(&PERPS_SWAP_IX_DISCM),
            Self::PerpsAddLiquidity => writer.write_all(&PERPS_ADD_LIQUIDITY_IX_DISCM),
            Self::PerpsRemoveLiquidity => writer.write_all(&PERPS_REMOVE_LIQUIDITY_IX_DISCM),
            Self::MeteoraDlmmSwap => writer.write_all(&METEORA_DLMM_SWAP_IX_DISCM),
            Self::OpenBookV2Swap => writer.write_all(&OPEN_BOOK_V2_SWAP_IX_DISCM),
            Self::CloneSwap => writer.write_all(&CLONE_SWAP_IX_DISCM),
            Self::RaydiumCpSwap => writer.write_all(&RAYDIUM_CP_SWAP_IX_DISCM),
            Self::OneIntroSwap => writer.write_all(&ONE_INTRO_SWAP_IX_DISCM),
            Self::PumpdotfunWrappedBuy => writer.write_all(&PUMPDOTFUN_WRAPPED_BUY_IX_DISCM),
            Self::PumpdotfunWrappedSell => writer.write_all(&PUMPDOTFUN_WRAPPED_SELL_IX_DISCM),
            Self::PerpsV2Swap => writer.write_all(&PERPS_V2_SWAP_IX_DISCM),
            Self::PerpsV2AddLiquidity => writer.write_all(&PERPS_V2_ADD_LIQUIDITY_IX_DISCM),
            Self::PerpsV2RemoveLiquidity => writer.write_all(&PERPS_V2_REMOVE_LIQUIDITY_IX_DISCM),
            Self::MoonshotWrappedBuy => writer.write_all(&MOONSHOT_WRAPPED_BUY_IX_DISCM),
            Self::MoonshotWrappedSell => writer.write_all(&MOONSHOT_WRAPPED_SELL_IX_DISCM),
            Self::StabbleStableSwap => writer.write_all(&STABBLE_STABLE_SWAP_IX_DISCM),
            Self::StabbleWeightedSwap => writer.write_all(&STABBLE_WEIGHTED_SWAP_IX_DISCM),
            Self::ObricSwap => writer.write_all(&OBRIC_SWAP_IX_DISCM),
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
pub const ROUTE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RouteKeys {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RouteAccounts<'_, '_>> for RouteKeys {
    fn from(accounts: RouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RouteKeys> for [AccountMeta; ROUTE_IX_ACCOUNTS_LEN] {
    fn from(keys: RouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ROUTE_IX_ACCOUNTS_LEN]> for RouteKeys {
    fn from(pubkeys: [Pubkey; ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            user_source_token_account: pubkeys[2],
            user_destination_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            destination_mint: pubkeys[5],
            platform_fee_account: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<RouteAccounts<'_, 'info>> for [AccountInfo<'info>; ROUTE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ROUTE_IX_ACCOUNTS_LEN]>
    for RouteAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            user_transfer_authority: &arr[1],
            user_source_token_account: &arr[2],
            user_destination_token_account: &arr[3],
            destination_token_account: &arr[4],
            destination_mint: &arr[5],
            platform_fee_account: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const ROUTE_IX_DISCM: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RouteIxArgs {
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RouteIxData(pub RouteIxArgs);
impl From<RouteIxArgs> for RouteIxData {
    fn from(args: RouteIxArgs) -> Self {
        Self(args)
    }
}
impl RouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROUTE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ROUTE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RouteIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROUTE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn route_ix_with_program_id(
    program_id: Pubkey,
    keys: RouteKeys,
    args: RouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ROUTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn route_ix(keys: RouteKeys, args: RouteIxArgs) -> std::io::Result<Instruction> {
    route_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RouteAccounts<'_, '_>,
    args: RouteIxArgs,
) -> ProgramResult {
    let keys: RouteKeys = accounts.into();
    let ix = route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn route_invoke(accounts: RouteAccounts<'_, '_>, args: RouteIxArgs) -> ProgramResult {
    route_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RouteAccounts<'_, '_>,
    args: RouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RouteKeys = accounts.into();
    let ix = route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn route_invoke_signed(
    accounts: RouteAccounts<'_, '_>,
    args: RouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    route_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn route_verify_account_keys(
    accounts: RouteAccounts<'_, '_>,
    keys: RouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (
            *accounts.user_source_token_account.key,
            keys.user_source_token_account,
        ),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
        (*accounts.destination_mint.key, keys.destination_mint),
        (
            *accounts.platform_fee_account.key,
            keys.platform_fee_account,
        ),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn route_verify_writable_privileges<'me, 'info>(
    accounts: RouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn route_verify_signer_privileges<'me, 'info>(
    accounts: RouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn route_verify_account_privileges<'me, 'info>(
    accounts: RouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    route_verify_writable_privileges(accounts)?;
    route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct RouteWithTokenLedgerAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token_ledger: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RouteWithTokenLedgerKeys {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token_ledger: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RouteWithTokenLedgerAccounts<'_, '_>> for RouteWithTokenLedgerKeys {
    fn from(accounts: RouteWithTokenLedgerAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token_ledger: *accounts.token_ledger.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RouteWithTokenLedgerKeys> for [AccountMeta; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: RouteWithTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for RouteWithTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            user_source_token_account: pubkeys[2],
            user_destination_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            destination_mint: pubkeys[5],
            platform_fee_account: pubkeys[6],
            token_ledger: pubkeys[7],
            event_authority: pubkeys[8],
            program: pubkeys[9],
        }
    }
}
impl<'info> From<RouteWithTokenLedgerAccounts<'_, 'info>>
    for [AccountInfo<'info>; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RouteWithTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token_ledger.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
    for RouteWithTokenLedgerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            user_transfer_authority: &arr[1],
            user_source_token_account: &arr[2],
            user_destination_token_account: &arr[3],
            destination_token_account: &arr[4],
            destination_mint: &arr[5],
            platform_fee_account: &arr[6],
            token_ledger: &arr[7],
            event_authority: &arr[8],
            program: &arr[9],
        }
    }
}
pub const ROUTE_WITH_TOKEN_LEDGER_IX_DISCM: [u8; 8] = [150, 86, 71, 116, 167, 93, 14, 104];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RouteWithTokenLedgerIxArgs {
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RouteWithTokenLedgerIxData(pub RouteWithTokenLedgerIxArgs);
impl From<RouteWithTokenLedgerIxArgs> for RouteWithTokenLedgerIxData {
    fn from(args: RouteWithTokenLedgerIxArgs) -> Self {
        Self(args)
    }
}
impl RouteWithTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ROUTE_WITH_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ROUTE_WITH_TOKEN_LEDGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RouteWithTokenLedgerIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn route_with_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: RouteWithTokenLedgerKeys,
    args: RouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: RouteWithTokenLedgerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn route_with_token_ledger_ix(
    keys: RouteWithTokenLedgerKeys,
    args: RouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    route_with_token_ledger_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn route_with_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    let keys: RouteWithTokenLedgerKeys = accounts.into();
    let ix = route_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn route_with_token_ledger_invoke(
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    route_with_token_ledger_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn route_with_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RouteWithTokenLedgerKeys = accounts.into();
    let ix = route_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn route_with_token_ledger_invoke_signed(
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    args: RouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    route_with_token_ledger_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn route_with_token_ledger_verify_account_keys(
    accounts: RouteWithTokenLedgerAccounts<'_, '_>,
    keys: RouteWithTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (
            *accounts.user_source_token_account.key,
            keys.user_source_token_account,
        ),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
        (*accounts.destination_mint.key, keys.destination_mint),
        (
            *accounts.platform_fee_account.key,
            keys.platform_fee_account,
        ),
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn route_with_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: RouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn route_with_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: RouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn route_with_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: RouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    route_with_token_ledger_verify_writable_privileges(accounts)?;
    route_with_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ExactOutRouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExactOutRouteKeys {
    pub token_program: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ExactOutRouteAccounts<'_, '_>> for ExactOutRouteKeys {
    fn from(accounts: ExactOutRouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token2022_program: *accounts.token2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ExactOutRouteKeys> for [AccountMeta; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(keys: ExactOutRouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]> for ExactOutRouteKeys {
    fn from(pubkeys: [Pubkey; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            user_transfer_authority: pubkeys[1],
            user_source_token_account: pubkeys[2],
            user_destination_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            source_mint: pubkeys[5],
            destination_mint: pubkeys[6],
            platform_fee_account: pubkeys[7],
            token2022_program: pubkeys[8],
            event_authority: pubkeys[9],
            program: pubkeys[10],
        }
    }
}
impl<'info> From<ExactOutRouteAccounts<'_, 'info>>
    for [AccountInfo<'info>; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ExactOutRouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]>
    for ExactOutRouteAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            user_transfer_authority: &arr[1],
            user_source_token_account: &arr[2],
            user_destination_token_account: &arr[3],
            destination_token_account: &arr[4],
            source_mint: &arr[5],
            destination_mint: &arr[6],
            platform_fee_account: &arr[7],
            token2022_program: &arr[8],
            event_authority: &arr[9],
            program: &arr[10],
        }
    }
}
pub const EXACT_OUT_ROUTE_IX_DISCM: [u8; 8] = [208, 51, 239, 151, 123, 43, 237, 92];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExactOutRouteIxArgs {
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ExactOutRouteIxData(pub ExactOutRouteIxArgs);
impl From<ExactOutRouteIxArgs> for ExactOutRouteIxData {
    fn from(args: ExactOutRouteIxArgs) -> Self {
        Self(args)
    }
}
impl ExactOutRouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != EXACT_OUT_ROUTE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EXACT_OUT_ROUTE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ExactOutRouteIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&EXACT_OUT_ROUTE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn exact_out_route_ix_with_program_id(
    program_id: Pubkey,
    keys: ExactOutRouteKeys,
    args: ExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ExactOutRouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn exact_out_route_ix(
    keys: ExactOutRouteKeys,
    args: ExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    exact_out_route_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn exact_out_route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
) -> ProgramResult {
    let keys: ExactOutRouteKeys = accounts.into();
    let ix = exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn exact_out_route_invoke(
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
) -> ProgramResult {
    exact_out_route_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn exact_out_route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ExactOutRouteKeys = accounts.into();
    let ix = exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn exact_out_route_invoke_signed(
    accounts: ExactOutRouteAccounts<'_, '_>,
    args: ExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    exact_out_route_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn exact_out_route_verify_account_keys(
    accounts: ExactOutRouteAccounts<'_, '_>,
    keys: ExactOutRouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (
            *accounts.user_source_token_account.key,
            keys.user_source_token_account,
        ),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (
            *accounts.platform_fee_account.key,
            keys.platform_fee_account,
        ),
        (*accounts.token2022_program.key, keys.token2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn exact_out_route_verify_writable_privileges<'me, 'info>(
    accounts: ExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn exact_out_route_verify_signer_privileges<'me, 'info>(
    accounts: ExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn exact_out_route_verify_account_privileges<'me, 'info>(
    accounts: ExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    exact_out_route_verify_writable_privileges(accounts)?;
    exact_out_route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsRouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteKeys {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsRouteAccounts<'_, '_>> for SharedAccountsRouteKeys {
    fn from(accounts: SharedAccountsRouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts.program_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token2022_program: *accounts.token2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsRouteKeys> for [AccountMeta; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN] {
    fn from(keys: SharedAccountsRouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]> for SharedAccountsRouteKeys {
    fn from(pubkeys: [Pubkey; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            program_authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source_token_account: pubkeys[3],
            program_source_token_account: pubkeys[4],
            program_destination_token_account: pubkeys[5],
            destination_token_account: pubkeys[6],
            source_mint: pubkeys[7],
            destination_mint: pubkeys[8],
            platform_fee_account: pubkeys[9],
            token2022_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<SharedAccountsRouteAccounts<'_, 'info>>
    for [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SharedAccountsRouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]>
    for SharedAccountsRouteAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: &arr[0],
            program_authority: &arr[1],
            user_transfer_authority: &arr[2],
            source_token_account: &arr[3],
            program_source_token_account: &arr[4],
            program_destination_token_account: &arr[5],
            destination_token_account: &arr[6],
            source_mint: &arr[7],
            destination_mint: &arr[8],
            platform_fee_account: &arr[9],
            token2022_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const SHARED_ACCOUNTS_ROUTE_IX_DISCM: [u8; 8] = [193, 32, 155, 51, 65, 214, 156, 129];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedAccountsRouteIxArgs {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub in_amount: u64,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteIxData(pub SharedAccountsRouteIxArgs);
impl From<SharedAccountsRouteIxArgs> for SharedAccountsRouteIxData {
    fn from(args: SharedAccountsRouteIxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsRouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_ROUTE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SHARED_ACCOUNTS_ROUTE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SharedAccountsRouteIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_ROUTE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_route_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsRouteKeys,
    args: SharedAccountsRouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_ROUTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SharedAccountsRouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_route_ix(
    keys: SharedAccountsRouteKeys,
    args: SharedAccountsRouteIxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_route_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn shared_accounts_route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
) -> ProgramResult {
    let keys: SharedAccountsRouteKeys = accounts.into();
    let ix = shared_accounts_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_route_invoke(
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
) -> ProgramResult {
    shared_accounts_route_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn shared_accounts_route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsRouteKeys = accounts.into();
    let ix = shared_accounts_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_route_invoke_signed(
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    args: SharedAccountsRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_route_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn shared_accounts_route_verify_account_keys(
    accounts: SharedAccountsRouteAccounts<'_, '_>,
    keys: SharedAccountsRouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program_authority.key, keys.program_authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (
            *accounts.source_token_account.key,
            keys.source_token_account,
        ),
        (
            *accounts.program_source_token_account.key,
            keys.program_source_token_account,
        ),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (
            *accounts.platform_fee_account.key,
            keys.platform_fee_account,
        ),
        (*accounts.token2022_program.key, keys.token2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_route_verify_writable_privileges(accounts)?;
    shared_accounts_route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token2022_program: &'me AccountInfo<'info>,
    pub token_ledger: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteWithTokenLedgerKeys {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token2022_program: Pubkey,
    pub token_ledger: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>>
    for SharedAccountsRouteWithTokenLedgerKeys
{
    fn from(accounts: SharedAccountsRouteWithTokenLedgerAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts.program_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token2022_program: *accounts.token2022_program.key,
            token_ledger: *accounts.token_ledger.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsRouteWithTokenLedgerKeys>
    for [AccountMeta; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]
{
    fn from(keys: SharedAccountsRouteWithTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
    for SharedAccountsRouteWithTokenLedgerKeys
{
    fn from(pubkeys: [Pubkey; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            program_authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source_token_account: pubkeys[3],
            program_source_token_account: pubkeys[4],
            program_destination_token_account: pubkeys[5],
            destination_token_account: pubkeys[6],
            source_mint: pubkeys[7],
            destination_mint: pubkeys[8],
            platform_fee_account: pubkeys[9],
            token2022_program: pubkeys[10],
            token_ledger: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<SharedAccountsRouteWithTokenLedgerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token2022_program.clone(),
            accounts.token_ledger.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
    for SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            program_authority: &arr[1],
            user_transfer_authority: &arr[2],
            source_token_account: &arr[3],
            program_source_token_account: &arr[4],
            program_destination_token_account: &arr[5],
            destination_token_account: &arr[6],
            source_mint: &arr[7],
            destination_mint: &arr[8],
            platform_fee_account: &arr[9],
            token2022_program: &arr[10],
            token_ledger: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM: [u8; 8] =
    [230, 121, 143, 80, 119, 159, 106, 170];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedAccountsRouteWithTokenLedgerIxArgs {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsRouteWithTokenLedgerIxData(pub SharedAccountsRouteWithTokenLedgerIxArgs);
impl From<SharedAccountsRouteWithTokenLedgerIxArgs> for SharedAccountsRouteWithTokenLedgerIxData {
    fn from(args: SharedAccountsRouteWithTokenLedgerIxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsRouteWithTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SharedAccountsRouteWithTokenLedgerIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_route_with_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsRouteWithTokenLedgerKeys,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_ROUTE_WITH_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    let data: SharedAccountsRouteWithTokenLedgerIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_route_with_token_ledger_ix(
    keys: SharedAccountsRouteWithTokenLedgerKeys,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_route_with_token_ledger_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn shared_accounts_route_with_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    let keys: SharedAccountsRouteWithTokenLedgerKeys = accounts.into();
    let ix = shared_accounts_route_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_route_with_token_ledger_invoke(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
) -> ProgramResult {
    shared_accounts_route_with_token_ledger_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn shared_accounts_route_with_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsRouteWithTokenLedgerKeys = accounts.into();
    let ix = shared_accounts_route_with_token_ledger_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_route_with_token_ledger_invoke_signed(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    args: SharedAccountsRouteWithTokenLedgerIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_route_with_token_ledger_invoke_signed_with_program_id(
        JUPITER_ID, accounts, args, seeds,
    )
}
pub fn shared_accounts_route_with_token_ledger_verify_account_keys(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'_, '_>,
    keys: SharedAccountsRouteWithTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program_authority.key, keys.program_authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (
            *accounts.source_token_account.key,
            keys.source_token_account,
        ),
        (
            *accounts.program_source_token_account.key,
            keys.program_source_token_account,
        ),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (
            *accounts.platform_fee_account.key,
            keys.platform_fee_account,
        ),
        (*accounts.token2022_program.key, keys.token2022_program),
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_with_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_with_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_route_with_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsRouteWithTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_route_with_token_ledger_verify_writable_privileges(accounts)?;
    shared_accounts_route_with_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct SharedAccountsExactOutRouteAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub program_source_token_account: &'me AccountInfo<'info>,
    pub program_destination_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub platform_fee_account: &'me AccountInfo<'info>,
    pub token2022_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SharedAccountsExactOutRouteKeys {
    pub token_program: Pubkey,
    pub program_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub program_source_token_account: Pubkey,
    pub program_destination_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub platform_fee_account: Pubkey,
    pub token2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<SharedAccountsExactOutRouteAccounts<'_, '_>> for SharedAccountsExactOutRouteKeys {
    fn from(accounts: SharedAccountsExactOutRouteAccounts) -> Self {
        Self {
            token_program: *accounts.token_program.key,
            program_authority: *accounts.program_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            program_source_token_account: *accounts.program_source_token_account.key,
            program_destination_token_account: *accounts.program_destination_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            platform_fee_account: *accounts.platform_fee_account.key,
            token2022_program: *accounts.token2022_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<SharedAccountsExactOutRouteKeys>
    for [AccountMeta; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]
{
    fn from(keys: SharedAccountsExactOutRouteKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.platform_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token2022_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]>
    for SharedAccountsExactOutRouteKeys
{
    fn from(pubkeys: [Pubkey; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_program: pubkeys[0],
            program_authority: pubkeys[1],
            user_transfer_authority: pubkeys[2],
            source_token_account: pubkeys[3],
            program_source_token_account: pubkeys[4],
            program_destination_token_account: pubkeys[5],
            destination_token_account: pubkeys[6],
            source_mint: pubkeys[7],
            destination_mint: pubkeys[8],
            platform_fee_account: pubkeys[9],
            token2022_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
        }
    }
}
impl<'info> From<SharedAccountsExactOutRouteAccounts<'_, 'info>>
    for [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SharedAccountsExactOutRouteAccounts<'_, 'info>) -> Self {
        [
            accounts.token_program.clone(),
            accounts.program_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.program_source_token_account.clone(),
            accounts.program_destination_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.platform_fee_account.clone(),
            accounts.token2022_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN]>
    for SharedAccountsExactOutRouteAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            token_program: &arr[0],
            program_authority: &arr[1],
            user_transfer_authority: &arr[2],
            source_token_account: &arr[3],
            program_source_token_account: &arr[4],
            program_destination_token_account: &arr[5],
            destination_token_account: &arr[6],
            source_mint: &arr[7],
            destination_mint: &arr[8],
            platform_fee_account: &arr[9],
            token2022_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
        }
    }
}
pub const SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM: [u8; 8] =
    [176, 209, 105, 168, 154, 125, 69, 62];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SharedAccountsExactOutRouteIxArgs {
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub out_amount: u64,
    pub quoted_in_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SharedAccountsExactOutRouteIxData(pub SharedAccountsExactOutRouteIxArgs);
impl From<SharedAccountsExactOutRouteIxArgs> for SharedAccountsExactOutRouteIxData {
    fn from(args: SharedAccountsExactOutRouteIxArgs) -> Self {
        Self(args)
    }
}
impl SharedAccountsExactOutRouteIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SharedAccountsExactOutRouteIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn shared_accounts_exact_out_route_ix_with_program_id(
    program_id: Pubkey,
    keys: SharedAccountsExactOutRouteKeys,
    args: SharedAccountsExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SHARED_ACCOUNTS_EXACT_OUT_ROUTE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SharedAccountsExactOutRouteIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn shared_accounts_exact_out_route_ix(
    keys: SharedAccountsExactOutRouteKeys,
    args: SharedAccountsExactOutRouteIxArgs,
) -> std::io::Result<Instruction> {
    shared_accounts_exact_out_route_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn shared_accounts_exact_out_route_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
) -> ProgramResult {
    let keys: SharedAccountsExactOutRouteKeys = accounts.into();
    let ix = shared_accounts_exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn shared_accounts_exact_out_route_invoke(
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
) -> ProgramResult {
    shared_accounts_exact_out_route_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn shared_accounts_exact_out_route_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SharedAccountsExactOutRouteKeys = accounts.into();
    let ix = shared_accounts_exact_out_route_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn shared_accounts_exact_out_route_invoke_signed(
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    args: SharedAccountsExactOutRouteIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    shared_accounts_exact_out_route_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn shared_accounts_exact_out_route_verify_account_keys(
    accounts: SharedAccountsExactOutRouteAccounts<'_, '_>,
    keys: SharedAccountsExactOutRouteKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_program.key, keys.token_program),
        (*accounts.program_authority.key, keys.program_authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (
            *accounts.source_token_account.key,
            keys.source_token_account,
        ),
        (
            *accounts.program_source_token_account.key,
            keys.program_source_token_account,
        ),
        (
            *accounts.program_destination_token_account.key,
            keys.program_destination_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (
            *accounts.platform_fee_account.key,
            keys.platform_fee_account,
        ),
        (*accounts.token2022_program.key, keys.token2022_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_verify_writable_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.program_source_token_account,
        accounts.program_destination_token_account,
        accounts.destination_token_account,
        accounts.platform_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_verify_signer_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user_transfer_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn shared_accounts_exact_out_route_verify_account_privileges<'me, 'info>(
    accounts: SharedAccountsExactOutRouteAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    shared_accounts_exact_out_route_verify_writable_privileges(accounts)?;
    shared_accounts_exact_out_route_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetTokenLedgerAccounts<'me, 'info> {
    pub token_ledger: &'me AccountInfo<'info>,
    pub token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetTokenLedgerKeys {
    pub token_ledger: Pubkey,
    pub token_account: Pubkey,
}
impl From<SetTokenLedgerAccounts<'_, '_>> for SetTokenLedgerKeys {
    fn from(accounts: SetTokenLedgerAccounts) -> Self {
        Self {
            token_ledger: *accounts.token_ledger.key,
            token_account: *accounts.token_account.key,
        }
    }
}
impl From<SetTokenLedgerKeys> for [AccountMeta; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for SetTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: pubkeys[0],
            token_account: pubkeys[1],
        }
    }
}
impl<'info> From<SetTokenLedgerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_ledger.clone(),
            accounts.token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
    for SetTokenLedgerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: &arr[0],
            token_account: &arr[1],
        }
    }
}
pub const SET_TOKEN_LEDGER_IX_DISCM: [u8; 8] = [228, 85, 185, 112, 78, 79, 77, 2];
#[derive(Clone, Debug, PartialEq)]
pub struct SetTokenLedgerIxData;
impl SetTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SET_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_TOKEN_LEDGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SET_TOKEN_LEDGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: SetTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetTokenLedgerIxData.try_to_vec()?,
    })
}
pub fn set_token_ledger_ix(keys: SetTokenLedgerKeys) -> std::io::Result<Instruction> {
    set_token_ledger_ix_with_program_id(JUPITER_ID, keys)
}
pub fn set_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetTokenLedgerKeys = accounts.into();
    let ix = set_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_token_ledger_invoke(accounts: SetTokenLedgerAccounts<'_, '_>) -> ProgramResult {
    set_token_ledger_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn set_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetTokenLedgerKeys = accounts.into();
    let ix = set_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_token_ledger_invoke_signed(
    accounts: SetTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_token_ledger_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn set_token_ledger_verify_account_keys(
    accounts: SetTokenLedgerAccounts<'_, '_>,
    keys: SetTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.token_account.key, keys.token_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn set_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: SetTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_ledger] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: SetTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_token_ledger_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct CreateOpenOrdersAccounts<'me, 'info> {
    pub open_orders: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub dex_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateOpenOrdersKeys {
    pub open_orders: Pubkey,
    pub payer: Pubkey,
    pub dex_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub market: Pubkey,
}
impl From<CreateOpenOrdersAccounts<'_, '_>> for CreateOpenOrdersKeys {
    fn from(accounts: CreateOpenOrdersAccounts) -> Self {
        Self {
            open_orders: *accounts.open_orders.key,
            payer: *accounts.payer.key,
            dex_program: *accounts.dex_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            market: *accounts.market.key,
        }
    }
}
impl From<CreateOpenOrdersKeys> for [AccountMeta; CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateOpenOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN]> for CreateOpenOrdersKeys {
    fn from(pubkeys: [Pubkey; CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: pubkeys[0],
            payer: pubkeys[1],
            dex_program: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
            market: pubkeys[5],
        }
    }
}
impl<'info> From<CreateOpenOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateOpenOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.open_orders.clone(),
            accounts.payer.clone(),
            accounts.dex_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN]>
    for CreateOpenOrdersAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: &arr[0],
            payer: &arr[1],
            dex_program: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
            market: &arr[5],
        }
    }
}
pub const CREATE_OPEN_ORDERS_IX_DISCM: [u8; 8] = [229, 194, 212, 172, 8, 10, 134, 147];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateOpenOrdersIxData;
impl CreateOpenOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_OPEN_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_OPEN_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_OPEN_ORDERS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_open_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateOpenOrdersKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_OPEN_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateOpenOrdersIxData.try_to_vec()?,
    })
}
pub fn create_open_orders_ix(keys: CreateOpenOrdersKeys) -> std::io::Result<Instruction> {
    create_open_orders_ix_with_program_id(JUPITER_ID, keys)
}
pub fn create_open_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateOpenOrdersAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateOpenOrdersKeys = accounts.into();
    let ix = create_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_open_orders_invoke(accounts: CreateOpenOrdersAccounts<'_, '_>) -> ProgramResult {
    create_open_orders_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn create_open_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateOpenOrdersKeys = accounts.into();
    let ix = create_open_orders_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_open_orders_invoke_signed(
    accounts: CreateOpenOrdersAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_open_orders_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn create_open_orders_verify_account_keys(
    accounts: CreateOpenOrdersAccounts<'_, '_>,
    keys: CreateOpenOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.payer.key, keys.payer),
        (*accounts.dex_program.key, keys.dex_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.market.key, keys.market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_open_orders_verify_writable_privileges<'me, 'info>(
    accounts: CreateOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.open_orders, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_open_orders_verify_signer_privileges<'me, 'info>(
    accounts: CreateOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_open_orders_verify_account_privileges<'me, 'info>(
    accounts: CreateOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_open_orders_verify_writable_privileges(accounts)?;
    create_open_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenAccountAccounts<'me, 'info> {
    pub token_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenAccountKeys {
    pub token_account: Pubkey,
    pub user: Pubkey,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTokenAccountAccounts<'_, '_>> for CreateTokenAccountKeys {
    fn from(accounts: CreateTokenAccountAccounts) -> Self {
        Self {
            token_account: *accounts.token_account.key,
            user: *accounts.user.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTokenAccountKeys> for [AccountMeta; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
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
        ]
    }
}
impl From<[Pubkey; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateTokenAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: pubkeys[0],
            user: pubkeys[1],
            mint: pubkeys[2],
            token_program: pubkeys[3],
            system_program: pubkeys[4],
        }
    }
}
impl<'info> From<CreateTokenAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateTokenAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.token_account.clone(),
            accounts.user.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]>
    for CreateTokenAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_account: &arr[0],
            user: &arr[1],
            mint: &arr[2],
            token_program: &arr[3],
            system_program: &arr[4],
        }
    }
}
pub const CREATE_TOKEN_ACCOUNT_IX_DISCM: [u8; 8] = [147, 241, 123, 100, 244, 132, 174, 118];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateTokenAccountIxArgs {
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenAccountIxData(pub CreateTokenAccountIxArgs);
impl From<CreateTokenAccountIxArgs> for CreateTokenAccountIxData {
    fn from(args: CreateTokenAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateTokenAccountIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_TOKEN_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateTokenAccountIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_ACCOUNT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenAccountKeys,
    args: CreateTokenAccountIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateTokenAccountIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_token_account_ix(
    keys: CreateTokenAccountKeys,
    args: CreateTokenAccountIxArgs,
) -> std::io::Result<Instruction> {
    create_token_account_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn create_token_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
) -> ProgramResult {
    let keys: CreateTokenAccountKeys = accounts.into();
    let ix = create_token_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_account_invoke(
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
) -> ProgramResult {
    create_token_account_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn create_token_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenAccountKeys = accounts.into();
    let ix = create_token_account_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_account_invoke_signed(
    accounts: CreateTokenAccountAccounts<'_, '_>,
    args: CreateTokenAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_account_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn create_token_account_verify_account_keys(
    accounts: CreateTokenAccountAccounts<'_, '_>,
    keys: CreateTokenAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_account.key, keys.token_account),
        (*accounts.user.key, keys.user),
        (*accounts.mint.key, keys.mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account, accounts.user] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.user] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_account_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_account_verify_writable_privileges(accounts)?;
    create_token_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateProgramOpenOrdersAccounts<'me, 'info> {
    pub open_orders: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub dex_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateProgramOpenOrdersKeys {
    pub open_orders: Pubkey,
    pub payer: Pubkey,
    pub program_authority: Pubkey,
    pub dex_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub market: Pubkey,
}
impl From<CreateProgramOpenOrdersAccounts<'_, '_>> for CreateProgramOpenOrdersKeys {
    fn from(accounts: CreateProgramOpenOrdersAccounts) -> Self {
        Self {
            open_orders: *accounts.open_orders.key,
            payer: *accounts.payer.key,
            program_authority: *accounts.program_authority.key,
            dex_program: *accounts.dex_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            market: *accounts.market.key,
        }
    }
}
impl From<CreateProgramOpenOrdersKeys>
    for [AccountMeta; CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(keys: CreateProgramOpenOrdersKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dex_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN]> for CreateProgramOpenOrdersKeys {
    fn from(pubkeys: [Pubkey; CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: pubkeys[0],
            payer: pubkeys[1],
            program_authority: pubkeys[2],
            dex_program: pubkeys[3],
            system_program: pubkeys[4],
            rent: pubkeys[5],
            market: pubkeys[6],
        }
    }
}
impl<'info> From<CreateProgramOpenOrdersAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateProgramOpenOrdersAccounts<'_, 'info>) -> Self {
        [
            accounts.open_orders.clone(),
            accounts.payer.clone(),
            accounts.program_authority.clone(),
            accounts.dex_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.market.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN]>
    for CreateProgramOpenOrdersAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            open_orders: &arr[0],
            payer: &arr[1],
            program_authority: &arr[2],
            dex_program: &arr[3],
            system_program: &arr[4],
            rent: &arr[5],
            market: &arr[6],
        }
    }
}
pub const CREATE_PROGRAM_OPEN_ORDERS_IX_DISCM: [u8; 8] = [28, 226, 32, 148, 188, 136, 113, 171];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateProgramOpenOrdersIxArgs {
    pub id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateProgramOpenOrdersIxData(pub CreateProgramOpenOrdersIxArgs);
impl From<CreateProgramOpenOrdersIxArgs> for CreateProgramOpenOrdersIxData {
    fn from(args: CreateProgramOpenOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl CreateProgramOpenOrdersIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_PROGRAM_OPEN_ORDERS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_PROGRAM_OPEN_ORDERS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateProgramOpenOrdersIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_PROGRAM_OPEN_ORDERS_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_program_open_orders_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateProgramOpenOrdersKeys,
    args: CreateProgramOpenOrdersIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_PROGRAM_OPEN_ORDERS_IX_ACCOUNTS_LEN] = keys.into();
    let data: CreateProgramOpenOrdersIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_program_open_orders_ix(
    keys: CreateProgramOpenOrdersKeys,
    args: CreateProgramOpenOrdersIxArgs,
) -> std::io::Result<Instruction> {
    create_program_open_orders_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn create_program_open_orders_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateProgramOpenOrdersAccounts<'_, '_>,
    args: CreateProgramOpenOrdersIxArgs,
) -> ProgramResult {
    let keys: CreateProgramOpenOrdersKeys = accounts.into();
    let ix = create_program_open_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_program_open_orders_invoke(
    accounts: CreateProgramOpenOrdersAccounts<'_, '_>,
    args: CreateProgramOpenOrdersIxArgs,
) -> ProgramResult {
    create_program_open_orders_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn create_program_open_orders_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateProgramOpenOrdersAccounts<'_, '_>,
    args: CreateProgramOpenOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateProgramOpenOrdersKeys = accounts.into();
    let ix = create_program_open_orders_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_program_open_orders_invoke_signed(
    accounts: CreateProgramOpenOrdersAccounts<'_, '_>,
    args: CreateProgramOpenOrdersIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_program_open_orders_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn create_program_open_orders_verify_account_keys(
    accounts: CreateProgramOpenOrdersAccounts<'_, '_>,
    keys: CreateProgramOpenOrdersKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.open_orders.key, keys.open_orders),
        (*accounts.payer.key, keys.payer),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.dex_program.key, keys.dex_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.market.key, keys.market),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_program_open_orders_verify_writable_privileges<'me, 'info>(
    accounts: CreateProgramOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.open_orders, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_program_open_orders_verify_signer_privileges<'me, 'info>(
    accounts: CreateProgramOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_program_open_orders_verify_account_privileges<'me, 'info>(
    accounts: CreateProgramOpenOrdersAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_program_open_orders_verify_writable_privileges(accounts)?;
    create_program_open_orders_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ClaimAccounts<'me, 'info> {
    pub wallet: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimKeys {
    pub wallet: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
}
impl From<ClaimAccounts<'_, '_>> for ClaimKeys {
    fn from(accounts: ClaimAccounts) -> Self {
        Self {
            wallet: *accounts.wallet.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ClaimKeys> for [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
impl From<[Pubkey; CLAIM_IX_ACCOUNTS_LEN]> for ClaimKeys {
    fn from(pubkeys: [Pubkey; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: pubkeys[0],
            program_authority: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<ClaimAccounts<'_, 'info>> for [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimAccounts<'_, 'info>) -> Self {
        [
            accounts.wallet.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]>
    for ClaimAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            wallet: &arr[0],
            program_authority: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CLAIM_IX_DISCM: [u8; 8] = [62, 198, 214, 193, 213, 159, 108, 210];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimIxArgs {
    pub id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimIxData(pub ClaimIxArgs);
impl From<ClaimIxArgs> for ClaimIxData {
    fn from(args: ClaimIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimKeys,
    args: ClaimIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_ix(keys: ClaimKeys, args: ClaimIxArgs) -> std::io::Result<Instruction> {
    claim_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn claim_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAccounts<'_, '_>,
    args: ClaimIxArgs,
) -> ProgramResult {
    let keys: ClaimKeys = accounts.into();
    let ix = claim_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_invoke(accounts: ClaimAccounts<'_, '_>, args: ClaimIxArgs) -> ProgramResult {
    claim_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn claim_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimAccounts<'_, '_>,
    args: ClaimIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimKeys = accounts.into();
    let ix = claim_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_invoke_signed(
    accounts: ClaimAccounts<'_, '_>,
    args: ClaimIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn claim_verify_account_keys(
    accounts: ClaimAccounts<'_, '_>,
    keys: ClaimKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.wallet.key, keys.wallet),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_verify_writable_privileges<'me, 'info>(
    accounts: ClaimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.wallet, accounts.program_authority] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_verify_account_privileges<'me, 'info>(
    accounts: ClaimAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_TOKEN_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct ClaimTokenAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub wallet: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub program_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub associated_token_token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimTokenKeys {
    pub payer: Pubkey,
    pub wallet: Pubkey,
    pub program_authority: Pubkey,
    pub program_token_account: Pubkey,
    pub destination_token_account: Pubkey,
    pub mint: Pubkey,
    pub associated_token_token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<ClaimTokenAccounts<'_, '_>> for ClaimTokenKeys {
    fn from(accounts: ClaimTokenAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            wallet: *accounts.wallet.key,
            program_authority: *accounts.program_authority.key,
            program_token_account: *accounts.program_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
            mint: *accounts.mint.key,
            associated_token_token_program: *accounts.associated_token_token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<ClaimTokenKeys> for [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimTokenKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_token_program,
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
impl From<[Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]> for ClaimTokenKeys {
    fn from(pubkeys: [Pubkey; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            wallet: pubkeys[1],
            program_authority: pubkeys[2],
            program_token_account: pubkeys[3],
            destination_token_account: pubkeys[4],
            mint: pubkeys[5],
            associated_token_token_program: pubkeys[6],
            associated_token_program: pubkeys[7],
            system_program: pubkeys[8],
        }
    }
}
impl<'info> From<ClaimTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ClaimTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.wallet.clone(),
            accounts.program_authority.clone(),
            accounts.program_token_account.clone(),
            accounts.destination_token_account.clone(),
            accounts.mint.clone(),
            accounts.associated_token_token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]>
    for ClaimTokenAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            wallet: &arr[1],
            program_authority: &arr[2],
            program_token_account: &arr[3],
            destination_token_account: &arr[4],
            mint: &arr[5],
            associated_token_token_program: &arr[6],
            associated_token_program: &arr[7],
            system_program: &arr[8],
        }
    }
}
pub const CLAIM_TOKEN_IX_DISCM: [u8; 8] = [116, 206, 27, 191, 166, 19, 0, 73];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimTokenIxArgs {
    pub id: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimTokenIxData(pub ClaimTokenIxArgs);
impl From<ClaimTokenIxArgs> for ClaimTokenIxData {
    fn from(args: ClaimTokenIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimTokenIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimTokenIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_TOKEN_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_token_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimTokenKeys,
    args: ClaimTokenIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_TOKEN_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimTokenIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_token_ix(
    keys: ClaimTokenKeys,
    args: ClaimTokenIxArgs,
) -> std::io::Result<Instruction> {
    claim_token_ix_with_program_id(JUPITER_ID, keys, args)
}
pub fn claim_token_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_token_invoke(
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
) -> ProgramResult {
    claim_token_invoke_with_program_id(JUPITER_ID, accounts, args)
}
pub fn claim_token_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimTokenKeys = accounts.into();
    let ix = claim_token_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_token_invoke_signed(
    accounts: ClaimTokenAccounts<'_, '_>,
    args: ClaimTokenIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_token_invoke_signed_with_program_id(JUPITER_ID, accounts, args, seeds)
}
pub fn claim_token_verify_account_keys(
    accounts: ClaimTokenAccounts<'_, '_>,
    keys: ClaimTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.payer.key, keys.payer),
        (*accounts.wallet.key, keys.wallet),
        (*accounts.program_authority.key, keys.program_authority),
        (
            *accounts.program_token_account.key,
            keys.program_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
        (*accounts.mint.key, keys.mint),
        (
            *accounts.associated_token_token_program.key,
            keys.associated_token_token_program,
        ),
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
pub fn claim_token_verify_writable_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.payer,
        accounts.program_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_token_verify_signer_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_token_verify_account_privileges<'me, 'info>(
    accounts: ClaimTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_token_verify_writable_privileges(accounts)?;
    claim_token_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateTokenLedgerAccounts<'me, 'info> {
    pub token_ledger: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CreateTokenLedgerKeys {
    pub token_ledger: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
}
impl From<CreateTokenLedgerAccounts<'_, '_>> for CreateTokenLedgerKeys {
    fn from(accounts: CreateTokenLedgerAccounts) -> Self {
        Self {
            token_ledger: *accounts.token_ledger.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<CreateTokenLedgerKeys> for [AccountMeta; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateTokenLedgerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_ledger,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
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
impl From<[Pubkey; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]> for CreateTokenLedgerKeys {
    fn from(pubkeys: [Pubkey; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: pubkeys[0],
            payer: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<CreateTokenLedgerAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateTokenLedgerAccounts<'_, 'info>) -> Self {
        [
            accounts.token_ledger.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]>
    for CreateTokenLedgerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_ledger: &arr[0],
            payer: &arr[1],
            system_program: &arr[2],
        }
    }
}
pub const CREATE_TOKEN_LEDGER_IX_DISCM: [u8; 8] = [232, 242, 197, 253, 240, 143, 129, 52];
#[derive(Clone, Debug, PartialEq)]
pub struct CreateTokenLedgerIxData;
impl CreateTokenLedgerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREATE_TOKEN_LEDGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_TOKEN_LEDGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREATE_TOKEN_LEDGER_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn create_token_ledger_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateTokenLedgerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREATE_TOKEN_LEDGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CreateTokenLedgerIxData.try_to_vec()?,
    })
}
pub fn create_token_ledger_ix(keys: CreateTokenLedgerKeys) -> std::io::Result<Instruction> {
    create_token_ledger_ix_with_program_id(JUPITER_ID, keys)
}
pub fn create_token_ledger_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenLedgerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CreateTokenLedgerKeys = accounts.into();
    let ix = create_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn create_token_ledger_invoke(accounts: CreateTokenLedgerAccounts<'_, '_>) -> ProgramResult {
    create_token_ledger_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn create_token_ledger_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateTokenLedgerKeys = accounts.into();
    let ix = create_token_ledger_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_token_ledger_invoke_signed(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_token_ledger_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn create_token_ledger_verify_account_keys(
    accounts: CreateTokenLedgerAccounts<'_, '_>,
    keys: CreateTokenLedgerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_ledger.key, keys.token_ledger),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_writable_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_ledger, accounts.payer] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_signer_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.token_ledger, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_token_ledger_verify_account_privileges<'me, 'info>(
    accounts: CreateTokenLedgerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_token_ledger_verify_writable_privileges(accounts)?;
    create_token_ledger_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MERCURIAL_SWAP_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct MercurialSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub swap_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub pool_authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_token_account: &'me AccountInfo<'info>,
    pub destination_token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MercurialSwapKeys {
    pub swap_program: Pubkey,
    pub swap_state: Pubkey,
    pub token_program: Pubkey,
    pub pool_authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_token_account: Pubkey,
    pub destination_token_account: Pubkey,
}
impl From<MercurialSwapAccounts<'_, '_>> for MercurialSwapKeys {
    fn from(accounts: MercurialSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            swap_state: *accounts.swap_state.key,
            token_program: *accounts.token_program.key,
            pool_authority: *accounts.pool_authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_token_account: *accounts.source_token_account.key,
            destination_token_account: *accounts.destination_token_account.key,
        }
    }
}
impl From<MercurialSwapKeys> for [AccountMeta; MERCURIAL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: MercurialSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_token_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MERCURIAL_SWAP_IX_ACCOUNTS_LEN]> for MercurialSwapKeys {
    fn from(pubkeys: [Pubkey; MERCURIAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            swap_state: pubkeys[1],
            token_program: pubkeys[2],
            pool_authority: pubkeys[3],
            user_transfer_authority: pubkeys[4],
            source_token_account: pubkeys[5],
            destination_token_account: pubkeys[6],
        }
    }
}
impl<'info> From<MercurialSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; MERCURIAL_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MercurialSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.swap_state.clone(),
            accounts.token_program.clone(),
            accounts.pool_authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_token_account.clone(),
            accounts.destination_token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MERCURIAL_SWAP_IX_ACCOUNTS_LEN]>
    for MercurialSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MERCURIAL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            swap_state: &arr[1],
            token_program: &arr[2],
            pool_authority: &arr[3],
            user_transfer_authority: &arr[4],
            source_token_account: &arr[5],
            destination_token_account: &arr[6],
        }
    }
}
pub const MERCURIAL_SWAP_IX_DISCM: [u8; 8] = [2, 5, 77, 173, 197, 0, 7, 157];
#[derive(Clone, Debug, PartialEq)]
pub struct MercurialSwapIxData;
impl MercurialSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MERCURIAL_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MERCURIAL_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MERCURIAL_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn mercurial_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: MercurialSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MERCURIAL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MercurialSwapIxData.try_to_vec()?,
    })
}
pub fn mercurial_swap_ix(keys: MercurialSwapKeys) -> std::io::Result<Instruction> {
    mercurial_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn mercurial_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MercurialSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MercurialSwapKeys = accounts.into();
    let ix = mercurial_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn mercurial_swap_invoke(accounts: MercurialSwapAccounts<'_, '_>) -> ProgramResult {
    mercurial_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn mercurial_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MercurialSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MercurialSwapKeys = accounts.into();
    let ix = mercurial_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn mercurial_swap_invoke_signed(
    accounts: MercurialSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    mercurial_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn mercurial_swap_verify_account_keys(
    accounts: MercurialSwapAccounts<'_, '_>,
    keys: MercurialSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.swap_state.key, keys.swap_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.pool_authority.key, keys.pool_authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (
            *accounts.source_token_account.key,
            keys.source_token_account,
        ),
        (
            *accounts.destination_token_account.key,
            keys.destination_token_account,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn mercurial_swap_verify_writable_privileges<'me, 'info>(
    accounts: MercurialSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_token_account,
        accounts.destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn mercurial_swap_verify_account_privileges<'me, 'info>(
    accounts: MercurialSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    mercurial_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CYKURA_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct CykuraSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub factory_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub last_observation_state: &'me AccountInfo<'info>,
    pub core_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CykuraSwapKeys {
    pub swap_program: Pubkey,
    pub signer: Pubkey,
    pub factory_state: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub last_observation_state: Pubkey,
    pub core_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<CykuraSwapAccounts<'_, '_>> for CykuraSwapKeys {
    fn from(accounts: CykuraSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            signer: *accounts.signer.key,
            factory_state: *accounts.factory_state.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            last_observation_state: *accounts.last_observation_state.key,
            core_program: *accounts.core_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CykuraSwapKeys> for [AccountMeta; CYKURA_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: CykuraSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.factory_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.last_observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.core_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CYKURA_SWAP_IX_ACCOUNTS_LEN]> for CykuraSwapKeys {
    fn from(pubkeys: [Pubkey; CYKURA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            signer: pubkeys[1],
            factory_state: pubkeys[2],
            pool_state: pubkeys[3],
            input_token_account: pubkeys[4],
            output_token_account: pubkeys[5],
            input_vault: pubkeys[6],
            output_vault: pubkeys[7],
            last_observation_state: pubkeys[8],
            core_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<CykuraSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; CYKURA_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CykuraSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.signer.clone(),
            accounts.factory_state.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.last_observation_state.clone(),
            accounts.core_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CYKURA_SWAP_IX_ACCOUNTS_LEN]>
    for CykuraSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CYKURA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            signer: &arr[1],
            factory_state: &arr[2],
            pool_state: &arr[3],
            input_token_account: &arr[4],
            output_token_account: &arr[5],
            input_vault: &arr[6],
            output_vault: &arr[7],
            last_observation_state: &arr[8],
            core_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const CYKURA_SWAP_IX_DISCM: [u8; 8] = [38, 241, 21, 107, 120, 59, 184, 249];
#[derive(Clone, Debug, PartialEq)]
pub struct CykuraSwapIxData;
impl CykuraSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CYKURA_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CYKURA_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CYKURA_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cykura_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: CykuraSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CYKURA_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CykuraSwapIxData.try_to_vec()?,
    })
}
pub fn cykura_swap_ix(keys: CykuraSwapKeys) -> std::io::Result<Instruction> {
    cykura_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn cykura_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CykuraSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CykuraSwapKeys = accounts.into();
    let ix = cykura_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cykura_swap_invoke(accounts: CykuraSwapAccounts<'_, '_>) -> ProgramResult {
    cykura_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn cykura_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CykuraSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CykuraSwapKeys = accounts.into();
    let ix = cykura_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cykura_swap_invoke_signed(
    accounts: CykuraSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cykura_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn cykura_swap_verify_account_keys(
    accounts: CykuraSwapAccounts<'_, '_>,
    keys: CykuraSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.signer.key, keys.signer),
        (*accounts.factory_state.key, keys.factory_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (
            *accounts.output_token_account.key,
            keys.output_token_account,
        ),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (
            *accounts.last_observation_state.key,
            keys.last_observation_state,
        ),
        (*accounts.core_program.key, keys.core_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cykura_swap_verify_writable_privileges<'me, 'info>(
    accounts: CykuraSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.last_observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cykura_swap_verify_account_privileges<'me, 'info>(
    accounts: CykuraSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cykura_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SERUM_SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct SerumSwapAccounts<'me, 'info> {
    pub market_market: &'me AccountInfo<'info>,
    pub market_open_orders: &'me AccountInfo<'info>,
    pub market_request_queue: &'me AccountInfo<'info>,
    pub market_event_queue: &'me AccountInfo<'info>,
    pub market_bids: &'me AccountInfo<'info>,
    pub market_asks: &'me AccountInfo<'info>,
    pub market_coin_vault: &'me AccountInfo<'info>,
    pub market_pc_vault: &'me AccountInfo<'info>,
    pub market_vault_signer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub order_payer_token_account: &'me AccountInfo<'info>,
    pub coin_wallet: &'me AccountInfo<'info>,
    pub pc_wallet: &'me AccountInfo<'info>,
    pub dex_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SerumSwapKeys {
    pub market_market: Pubkey,
    pub market_open_orders: Pubkey,
    pub market_request_queue: Pubkey,
    pub market_event_queue: Pubkey,
    pub market_bids: Pubkey,
    pub market_asks: Pubkey,
    pub market_coin_vault: Pubkey,
    pub market_pc_vault: Pubkey,
    pub market_vault_signer: Pubkey,
    pub authority: Pubkey,
    pub order_payer_token_account: Pubkey,
    pub coin_wallet: Pubkey,
    pub pc_wallet: Pubkey,
    pub dex_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<SerumSwapAccounts<'_, '_>> for SerumSwapKeys {
    fn from(accounts: SerumSwapAccounts) -> Self {
        Self {
            market_market: *accounts.market_market.key,
            market_open_orders: *accounts.market_open_orders.key,
            market_request_queue: *accounts.market_request_queue.key,
            market_event_queue: *accounts.market_event_queue.key,
            market_bids: *accounts.market_bids.key,
            market_asks: *accounts.market_asks.key,
            market_coin_vault: *accounts.market_coin_vault.key,
            market_pc_vault: *accounts.market_pc_vault.key,
            market_vault_signer: *accounts.market_vault_signer.key,
            authority: *accounts.authority.key,
            order_payer_token_account: *accounts.order_payer_token_account.key,
            coin_wallet: *accounts.coin_wallet.key,
            pc_wallet: *accounts.pc_wallet.key,
            dex_program: *accounts.dex_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<SerumSwapKeys> for [AccountMeta; SERUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SerumSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.market_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_request_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_coin_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_pc_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.order_payer_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.coin_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pc_wallet,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SERUM_SWAP_IX_ACCOUNTS_LEN]> for SerumSwapKeys {
    fn from(pubkeys: [Pubkey; SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_market: pubkeys[0],
            market_open_orders: pubkeys[1],
            market_request_queue: pubkeys[2],
            market_event_queue: pubkeys[3],
            market_bids: pubkeys[4],
            market_asks: pubkeys[5],
            market_coin_vault: pubkeys[6],
            market_pc_vault: pubkeys[7],
            market_vault_signer: pubkeys[8],
            authority: pubkeys[9],
            order_payer_token_account: pubkeys[10],
            coin_wallet: pubkeys[11],
            pc_wallet: pubkeys[12],
            dex_program: pubkeys[13],
            token_program: pubkeys[14],
            rent: pubkeys[15],
        }
    }
}
impl<'info> From<SerumSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; SERUM_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SerumSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.market_market.clone(),
            accounts.market_open_orders.clone(),
            accounts.market_request_queue.clone(),
            accounts.market_event_queue.clone(),
            accounts.market_bids.clone(),
            accounts.market_asks.clone(),
            accounts.market_coin_vault.clone(),
            accounts.market_pc_vault.clone(),
            accounts.market_vault_signer.clone(),
            accounts.authority.clone(),
            accounts.order_payer_token_account.clone(),
            accounts.coin_wallet.clone(),
            accounts.pc_wallet.clone(),
            accounts.dex_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SERUM_SWAP_IX_ACCOUNTS_LEN]>
    for SerumSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SERUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            market_market: &arr[0],
            market_open_orders: &arr[1],
            market_request_queue: &arr[2],
            market_event_queue: &arr[3],
            market_bids: &arr[4],
            market_asks: &arr[5],
            market_coin_vault: &arr[6],
            market_pc_vault: &arr[7],
            market_vault_signer: &arr[8],
            authority: &arr[9],
            order_payer_token_account: &arr[10],
            coin_wallet: &arr[11],
            pc_wallet: &arr[12],
            dex_program: &arr[13],
            token_program: &arr[14],
            rent: &arr[15],
        }
    }
}
pub const SERUM_SWAP_IX_DISCM: [u8; 8] = [88, 183, 70, 249, 214, 118, 82, 210];
#[derive(Clone, Debug, PartialEq)]
pub struct SerumSwapIxData;
impl SerumSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SERUM_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SERUM_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SERUM_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn serum_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SerumSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SERUM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SerumSwapIxData.try_to_vec()?,
    })
}
pub fn serum_swap_ix(keys: SerumSwapKeys) -> std::io::Result<Instruction> {
    serum_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn serum_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SerumSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SerumSwapKeys = accounts.into();
    let ix = serum_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn serum_swap_invoke(accounts: SerumSwapAccounts<'_, '_>) -> ProgramResult {
    serum_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn serum_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SerumSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SerumSwapKeys = accounts.into();
    let ix = serum_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn serum_swap_invoke_signed(
    accounts: SerumSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    serum_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn serum_swap_verify_account_keys(
    accounts: SerumSwapAccounts<'_, '_>,
    keys: SerumSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.market_market.key, keys.market_market),
        (*accounts.market_open_orders.key, keys.market_open_orders),
        (
            *accounts.market_request_queue.key,
            keys.market_request_queue,
        ),
        (*accounts.market_event_queue.key, keys.market_event_queue),
        (*accounts.market_bids.key, keys.market_bids),
        (*accounts.market_asks.key, keys.market_asks),
        (*accounts.market_coin_vault.key, keys.market_coin_vault),
        (*accounts.market_pc_vault.key, keys.market_pc_vault),
        (*accounts.market_vault_signer.key, keys.market_vault_signer),
        (*accounts.authority.key, keys.authority),
        (
            *accounts.order_payer_token_account.key,
            keys.order_payer_token_account,
        ),
        (*accounts.coin_wallet.key, keys.coin_wallet),
        (*accounts.pc_wallet.key, keys.pc_wallet),
        (*accounts.dex_program.key, keys.dex_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn serum_swap_verify_writable_privileges<'me, 'info>(
    accounts: SerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market_market,
        accounts.market_open_orders,
        accounts.market_request_queue,
        accounts.market_event_queue,
        accounts.market_bids,
        accounts.market_asks,
        accounts.market_coin_vault,
        accounts.market_pc_vault,
        accounts.order_payer_token_account,
        accounts.coin_wallet,
        accounts.pc_wallet,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn serum_swap_verify_account_privileges<'me, 'info>(
    accounts: SerumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    serum_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SABER_SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SaberSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub swap: &'me AccountInfo<'info>,
    pub swap_authority: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub input_user_account: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_user_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub fees_token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SaberSwapKeys {
    pub swap_program: Pubkey,
    pub token_program: Pubkey,
    pub swap: Pubkey,
    pub swap_authority: Pubkey,
    pub user_authority: Pubkey,
    pub input_user_account: Pubkey,
    pub input_token_account: Pubkey,
    pub output_user_account: Pubkey,
    pub output_token_account: Pubkey,
    pub fees_token_account: Pubkey,
}
impl From<SaberSwapAccounts<'_, '_>> for SaberSwapKeys {
    fn from(accounts: SaberSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            token_program: *accounts.token_program.key,
            swap: *accounts.swap.key,
            swap_authority: *accounts.swap_authority.key,
            user_authority: *accounts.user_authority.key,
            input_user_account: *accounts.input_user_account.key,
            input_token_account: *accounts.input_token_account.key,
            output_user_account: *accounts.output_user_account.key,
            output_token_account: *accounts.output_token_account.key,
            fees_token_account: *accounts.fees_token_account.key,
        }
    }
}
impl From<SaberSwapKeys> for [AccountMeta; SABER_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SaberSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fees_token_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SABER_SWAP_IX_ACCOUNTS_LEN]> for SaberSwapKeys {
    fn from(pubkeys: [Pubkey; SABER_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            token_program: pubkeys[1],
            swap: pubkeys[2],
            swap_authority: pubkeys[3],
            user_authority: pubkeys[4],
            input_user_account: pubkeys[5],
            input_token_account: pubkeys[6],
            output_user_account: pubkeys[7],
            output_token_account: pubkeys[8],
            fees_token_account: pubkeys[9],
        }
    }
}
impl<'info> From<SaberSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; SABER_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SaberSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.token_program.clone(),
            accounts.swap.clone(),
            accounts.swap_authority.clone(),
            accounts.user_authority.clone(),
            accounts.input_user_account.clone(),
            accounts.input_token_account.clone(),
            accounts.output_user_account.clone(),
            accounts.output_token_account.clone(),
            accounts.fees_token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SABER_SWAP_IX_ACCOUNTS_LEN]>
    for SaberSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SABER_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            token_program: &arr[1],
            swap: &arr[2],
            swap_authority: &arr[3],
            user_authority: &arr[4],
            input_user_account: &arr[5],
            input_token_account: &arr[6],
            output_user_account: &arr[7],
            output_token_account: &arr[8],
            fees_token_account: &arr[9],
        }
    }
}
pub const SABER_SWAP_IX_DISCM: [u8; 8] = [64, 62, 98, 226, 52, 74, 37, 178];
#[derive(Clone, Debug, PartialEq)]
pub struct SaberSwapIxData;
impl SaberSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SABER_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SABER_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SABER_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn saber_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SaberSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SABER_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SaberSwapIxData.try_to_vec()?,
    })
}
pub fn saber_swap_ix(keys: SaberSwapKeys) -> std::io::Result<Instruction> {
    saber_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn saber_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SaberSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SaberSwapKeys = accounts.into();
    let ix = saber_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn saber_swap_invoke(accounts: SaberSwapAccounts<'_, '_>) -> ProgramResult {
    saber_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn saber_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SaberSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SaberSwapKeys = accounts.into();
    let ix = saber_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn saber_swap_invoke_signed(
    accounts: SaberSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    saber_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn saber_swap_verify_account_keys(
    accounts: SaberSwapAccounts<'_, '_>,
    keys: SaberSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.swap.key, keys.swap),
        (*accounts.swap_authority.key, keys.swap_authority),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.input_user_account.key, keys.input_user_account),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.output_user_account.key, keys.output_user_account),
        (
            *accounts.output_token_account.key,
            keys.output_token_account,
        ),
        (*accounts.fees_token_account.key, keys.fees_token_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn saber_swap_verify_writable_privileges<'me, 'info>(
    accounts: SaberSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.input_user_account,
        accounts.input_token_account,
        accounts.output_user_account,
        accounts.output_token_account,
        accounts.fees_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn saber_swap_verify_account_privileges<'me, 'info>(
    accounts: SaberSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    saber_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct SaberAddDecimalsAccounts<'me, 'info> {
    pub add_decimals_program: &'me AccountInfo<'info>,
    pub wrapper: &'me AccountInfo<'info>,
    pub wrapper_mint: &'me AccountInfo<'info>,
    pub wrapper_underlying_tokens: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub user_underlying_tokens: &'me AccountInfo<'info>,
    pub user_wrapped_tokens: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SaberAddDecimalsKeys {
    pub add_decimals_program: Pubkey,
    pub wrapper: Pubkey,
    pub wrapper_mint: Pubkey,
    pub wrapper_underlying_tokens: Pubkey,
    pub owner: Pubkey,
    pub user_underlying_tokens: Pubkey,
    pub user_wrapped_tokens: Pubkey,
    pub token_program: Pubkey,
}
impl From<SaberAddDecimalsAccounts<'_, '_>> for SaberAddDecimalsKeys {
    fn from(accounts: SaberAddDecimalsAccounts) -> Self {
        Self {
            add_decimals_program: *accounts.add_decimals_program.key,
            wrapper: *accounts.wrapper.key,
            wrapper_mint: *accounts.wrapper_mint.key,
            wrapper_underlying_tokens: *accounts.wrapper_underlying_tokens.key,
            owner: *accounts.owner.key,
            user_underlying_tokens: *accounts.user_underlying_tokens.key,
            user_wrapped_tokens: *accounts.user_wrapped_tokens.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SaberAddDecimalsKeys> for [AccountMeta; SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN] {
    fn from(keys: SaberAddDecimalsKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.add_decimals_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.wrapper_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wrapper_underlying_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_underlying_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wrapped_tokens,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN]> for SaberAddDecimalsKeys {
    fn from(pubkeys: [Pubkey; SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            add_decimals_program: pubkeys[0],
            wrapper: pubkeys[1],
            wrapper_mint: pubkeys[2],
            wrapper_underlying_tokens: pubkeys[3],
            owner: pubkeys[4],
            user_underlying_tokens: pubkeys[5],
            user_wrapped_tokens: pubkeys[6],
            token_program: pubkeys[7],
        }
    }
}
impl<'info> From<SaberAddDecimalsAccounts<'_, 'info>>
    for [AccountInfo<'info>; SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SaberAddDecimalsAccounts<'_, 'info>) -> Self {
        [
            accounts.add_decimals_program.clone(),
            accounts.wrapper.clone(),
            accounts.wrapper_mint.clone(),
            accounts.wrapper_underlying_tokens.clone(),
            accounts.owner.clone(),
            accounts.user_underlying_tokens.clone(),
            accounts.user_wrapped_tokens.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN]>
    for SaberAddDecimalsAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            add_decimals_program: &arr[0],
            wrapper: &arr[1],
            wrapper_mint: &arr[2],
            wrapper_underlying_tokens: &arr[3],
            owner: &arr[4],
            user_underlying_tokens: &arr[5],
            user_wrapped_tokens: &arr[6],
            token_program: &arr[7],
        }
    }
}
pub const SABER_ADD_DECIMALS_IX_DISCM: [u8; 8] = [36, 53, 231, 184, 7, 181, 5, 238];
#[derive(Clone, Debug, PartialEq)]
pub struct SaberAddDecimalsIxData;
impl SaberAddDecimalsIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SABER_ADD_DECIMALS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SABER_ADD_DECIMALS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SABER_ADD_DECIMALS_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn saber_add_decimals_ix_with_program_id(
    program_id: Pubkey,
    keys: SaberAddDecimalsKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SABER_ADD_DECIMALS_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SaberAddDecimalsIxData.try_to_vec()?,
    })
}
pub fn saber_add_decimals_ix(keys: SaberAddDecimalsKeys) -> std::io::Result<Instruction> {
    saber_add_decimals_ix_with_program_id(JUPITER_ID, keys)
}
pub fn saber_add_decimals_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SaberAddDecimalsAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SaberAddDecimalsKeys = accounts.into();
    let ix = saber_add_decimals_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn saber_add_decimals_invoke(accounts: SaberAddDecimalsAccounts<'_, '_>) -> ProgramResult {
    saber_add_decimals_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn saber_add_decimals_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SaberAddDecimalsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SaberAddDecimalsKeys = accounts.into();
    let ix = saber_add_decimals_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn saber_add_decimals_invoke_signed(
    accounts: SaberAddDecimalsAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    saber_add_decimals_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn saber_add_decimals_verify_account_keys(
    accounts: SaberAddDecimalsAccounts<'_, '_>,
    keys: SaberAddDecimalsKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            *accounts.add_decimals_program.key,
            keys.add_decimals_program,
        ),
        (*accounts.wrapper.key, keys.wrapper),
        (*accounts.wrapper_mint.key, keys.wrapper_mint),
        (
            *accounts.wrapper_underlying_tokens.key,
            keys.wrapper_underlying_tokens,
        ),
        (*accounts.owner.key, keys.owner),
        (
            *accounts.user_underlying_tokens.key,
            keys.user_underlying_tokens,
        ),
        (*accounts.user_wrapped_tokens.key, keys.user_wrapped_tokens),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn saber_add_decimals_verify_writable_privileges<'me, 'info>(
    accounts: SaberAddDecimalsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.wrapper_mint,
        accounts.wrapper_underlying_tokens,
        accounts.user_underlying_tokens,
        accounts.user_wrapped_tokens,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn saber_add_decimals_verify_account_privileges<'me, 'info>(
    accounts: SaberAddDecimalsAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    saber_add_decimals_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const TOKEN_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct TokenSwapAccounts<'me, 'info> {
    pub token_swap_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source: &'me AccountInfo<'info>,
    pub swap_source: &'me AccountInfo<'info>,
    pub swap_destination: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub pool_fee: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TokenSwapKeys {
    pub token_swap_program: Pubkey,
    pub token_program: Pubkey,
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
}
impl From<TokenSwapAccounts<'_, '_>> for TokenSwapKeys {
    fn from(accounts: TokenSwapAccounts) -> Self {
        Self {
            token_swap_program: *accounts.token_swap_program.key,
            token_program: *accounts.token_program.key,
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source: *accounts.source.key,
            swap_source: *accounts.swap_source.key,
            swap_destination: *accounts.swap_destination.key,
            destination: *accounts.destination.key,
            pool_mint: *accounts.pool_mint.key,
            pool_fee: *accounts.pool_fee.key,
        }
    }
}
impl From<TokenSwapKeys> for [AccountMeta; TOKEN_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: TokenSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TOKEN_SWAP_IX_ACCOUNTS_LEN]> for TokenSwapKeys {
    fn from(pubkeys: [Pubkey; TOKEN_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_swap_program: pubkeys[0],
            token_program: pubkeys[1],
            swap: pubkeys[2],
            authority: pubkeys[3],
            user_transfer_authority: pubkeys[4],
            source: pubkeys[5],
            swap_source: pubkeys[6],
            swap_destination: pubkeys[7],
            destination: pubkeys[8],
            pool_mint: pubkeys[9],
            pool_fee: pubkeys[10],
        }
    }
}
impl<'info> From<TokenSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; TOKEN_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: TokenSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.token_swap_program.clone(),
            accounts.token_program.clone(),
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source.clone(),
            accounts.swap_source.clone(),
            accounts.swap_destination.clone(),
            accounts.destination.clone(),
            accounts.pool_mint.clone(),
            accounts.pool_fee.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOKEN_SWAP_IX_ACCOUNTS_LEN]>
    for TokenSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TOKEN_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_swap_program: &arr[0],
            token_program: &arr[1],
            swap: &arr[2],
            authority: &arr[3],
            user_transfer_authority: &arr[4],
            source: &arr[5],
            swap_source: &arr[6],
            swap_destination: &arr[7],
            destination: &arr[8],
            pool_mint: &arr[9],
            pool_fee: &arr[10],
        }
    }
}
pub const TOKEN_SWAP_IX_DISCM: [u8; 8] = [187, 192, 118, 212, 62, 109, 28, 213];
#[derive(Clone, Debug, PartialEq)]
pub struct TokenSwapIxData;
impl TokenSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TOKEN_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn token_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: TokenSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOKEN_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TokenSwapIxData.try_to_vec()?,
    })
}
pub fn token_swap_ix(keys: TokenSwapKeys) -> std::io::Result<Instruction> {
    token_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn token_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TokenSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: TokenSwapKeys = accounts.into();
    let ix = token_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn token_swap_invoke(accounts: TokenSwapAccounts<'_, '_>) -> ProgramResult {
    token_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn token_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TokenSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TokenSwapKeys = accounts.into();
    let ix = token_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn token_swap_invoke_signed(
    accounts: TokenSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    token_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn token_swap_verify_account_keys(
    accounts: TokenSwapAccounts<'_, '_>,
    keys: TokenSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_swap_program.key, keys.token_swap_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (*accounts.source.key, keys.source),
        (*accounts.swap_source.key, keys.swap_source),
        (*accounts.swap_destination.key, keys.swap_destination),
        (*accounts.destination.key, keys.destination),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.pool_fee.key, keys.pool_fee),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn token_swap_verify_writable_privileges<'me, 'info>(
    accounts: TokenSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source,
        accounts.swap_source,
        accounts.swap_destination,
        accounts.destination,
        accounts.pool_mint,
        accounts.pool_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn token_swap_verify_account_privileges<'me, 'info>(
    accounts: TokenSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    token_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const TOKEN_SWAP_V2_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct TokenSwapV2Accounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source: &'me AccountInfo<'info>,
    pub swap_source: &'me AccountInfo<'info>,
    pub swap_destination: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub pool_fee: &'me AccountInfo<'info>,
    pub source_mint: &'me AccountInfo<'info>,
    pub destination_mint: &'me AccountInfo<'info>,
    pub source_token_program: &'me AccountInfo<'info>,
    pub destination_token_program: &'me AccountInfo<'info>,
    pub pool_token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TokenSwapV2Keys {
    pub swap_program: Pubkey,
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
    pub source_mint: Pubkey,
    pub destination_mint: Pubkey,
    pub source_token_program: Pubkey,
    pub destination_token_program: Pubkey,
    pub pool_token_program: Pubkey,
}
impl From<TokenSwapV2Accounts<'_, '_>> for TokenSwapV2Keys {
    fn from(accounts: TokenSwapV2Accounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source: *accounts.source.key,
            swap_source: *accounts.swap_source.key,
            swap_destination: *accounts.swap_destination.key,
            destination: *accounts.destination.key,
            pool_mint: *accounts.pool_mint.key,
            pool_fee: *accounts.pool_fee.key,
            source_mint: *accounts.source_mint.key,
            destination_mint: *accounts.destination_mint.key,
            source_token_program: *accounts.source_token_program.key,
            destination_token_program: *accounts.destination_token_program.key,
            pool_token_program: *accounts.pool_token_program.key,
        }
    }
}
impl From<TokenSwapV2Keys> for [AccountMeta; TOKEN_SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: TokenSwapV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.source_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.destination_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; TOKEN_SWAP_V2_IX_ACCOUNTS_LEN]> for TokenSwapV2Keys {
    fn from(pubkeys: [Pubkey; TOKEN_SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            swap: pubkeys[1],
            authority: pubkeys[2],
            user_transfer_authority: pubkeys[3],
            source: pubkeys[4],
            swap_source: pubkeys[5],
            swap_destination: pubkeys[6],
            destination: pubkeys[7],
            pool_mint: pubkeys[8],
            pool_fee: pubkeys[9],
            source_mint: pubkeys[10],
            destination_mint: pubkeys[11],
            source_token_program: pubkeys[12],
            destination_token_program: pubkeys[13],
            pool_token_program: pubkeys[14],
        }
    }
}
impl<'info> From<TokenSwapV2Accounts<'_, 'info>>
    for [AccountInfo<'info>; TOKEN_SWAP_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: TokenSwapV2Accounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source.clone(),
            accounts.swap_source.clone(),
            accounts.swap_destination.clone(),
            accounts.destination.clone(),
            accounts.pool_mint.clone(),
            accounts.pool_fee.clone(),
            accounts.source_mint.clone(),
            accounts.destination_mint.clone(),
            accounts.source_token_program.clone(),
            accounts.destination_token_program.clone(),
            accounts.pool_token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TOKEN_SWAP_V2_IX_ACCOUNTS_LEN]>
    for TokenSwapV2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TOKEN_SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            swap: &arr[1],
            authority: &arr[2],
            user_transfer_authority: &arr[3],
            source: &arr[4],
            swap_source: &arr[5],
            swap_destination: &arr[6],
            destination: &arr[7],
            pool_mint: &arr[8],
            pool_fee: &arr[9],
            source_mint: &arr[10],
            destination_mint: &arr[11],
            source_token_program: &arr[12],
            destination_token_program: &arr[13],
            pool_token_program: &arr[14],
        }
    }
}
pub const TOKEN_SWAP_V2_IX_DISCM: [u8; 8] = [51, 48, 145, 115, 123, 95, 71, 138];
#[derive(Clone, Debug, PartialEq)]
pub struct TokenSwapV2IxData;
impl TokenSwapV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_SWAP_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    TOKEN_SWAP_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_SWAP_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn token_swap_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: TokenSwapV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; TOKEN_SWAP_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: TokenSwapV2IxData.try_to_vec()?,
    })
}
pub fn token_swap_v2_ix(keys: TokenSwapV2Keys) -> std::io::Result<Instruction> {
    token_swap_v2_ix_with_program_id(JUPITER_ID, keys)
}
pub fn token_swap_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TokenSwapV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: TokenSwapV2Keys = accounts.into();
    let ix = token_swap_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn token_swap_v2_invoke(accounts: TokenSwapV2Accounts<'_, '_>) -> ProgramResult {
    token_swap_v2_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn token_swap_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TokenSwapV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TokenSwapV2Keys = accounts.into();
    let ix = token_swap_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn token_swap_v2_invoke_signed(
    accounts: TokenSwapV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    token_swap_v2_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn token_swap_v2_verify_account_keys(
    accounts: TokenSwapV2Accounts<'_, '_>,
    keys: TokenSwapV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (*accounts.source.key, keys.source),
        (*accounts.swap_source.key, keys.swap_source),
        (*accounts.swap_destination.key, keys.swap_destination),
        (*accounts.destination.key, keys.destination),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.pool_fee.key, keys.pool_fee),
        (*accounts.source_mint.key, keys.source_mint),
        (*accounts.destination_mint.key, keys.destination_mint),
        (
            *accounts.source_token_program.key,
            keys.source_token_program,
        ),
        (
            *accounts.destination_token_program.key,
            keys.destination_token_program,
        ),
        (*accounts.pool_token_program.key, keys.pool_token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn token_swap_v2_verify_writable_privileges<'me, 'info>(
    accounts: TokenSwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source,
        accounts.swap_source,
        accounts.swap_destination,
        accounts.destination,
        accounts.pool_mint,
        accounts.pool_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn token_swap_v2_verify_account_privileges<'me, 'info>(
    accounts: TokenSwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    token_swap_v2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SENCHA_SWAP_IX_ACCOUNTS_LEN: usize = 10;
#[derive(Copy, Clone, Debug)]
pub struct SenchaSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub swap: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub input_user_account: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub input_fees_account: &'me AccountInfo<'info>,
    pub output_user_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub output_fees_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SenchaSwapKeys {
    pub swap_program: Pubkey,
    pub token_program: Pubkey,
    pub swap: Pubkey,
    pub user_authority: Pubkey,
    pub input_user_account: Pubkey,
    pub input_token_account: Pubkey,
    pub input_fees_account: Pubkey,
    pub output_user_account: Pubkey,
    pub output_token_account: Pubkey,
    pub output_fees_account: Pubkey,
}
impl From<SenchaSwapAccounts<'_, '_>> for SenchaSwapKeys {
    fn from(accounts: SenchaSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            token_program: *accounts.token_program.key,
            swap: *accounts.swap.key,
            user_authority: *accounts.user_authority.key,
            input_user_account: *accounts.input_user_account.key,
            input_token_account: *accounts.input_token_account.key,
            input_fees_account: *accounts.input_fees_account.key,
            output_user_account: *accounts.output_user_account.key,
            output_token_account: *accounts.output_token_account.key,
            output_fees_account: *accounts.output_fees_account.key,
        }
    }
}
impl From<SenchaSwapKeys> for [AccountMeta; SENCHA_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SenchaSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_fees_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_user_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_fees_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; SENCHA_SWAP_IX_ACCOUNTS_LEN]> for SenchaSwapKeys {
    fn from(pubkeys: [Pubkey; SENCHA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            token_program: pubkeys[1],
            swap: pubkeys[2],
            user_authority: pubkeys[3],
            input_user_account: pubkeys[4],
            input_token_account: pubkeys[5],
            input_fees_account: pubkeys[6],
            output_user_account: pubkeys[7],
            output_token_account: pubkeys[8],
            output_fees_account: pubkeys[9],
        }
    }
}
impl<'info> From<SenchaSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; SENCHA_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SenchaSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.token_program.clone(),
            accounts.swap.clone(),
            accounts.user_authority.clone(),
            accounts.input_user_account.clone(),
            accounts.input_token_account.clone(),
            accounts.input_fees_account.clone(),
            accounts.output_user_account.clone(),
            accounts.output_token_account.clone(),
            accounts.output_fees_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SENCHA_SWAP_IX_ACCOUNTS_LEN]>
    for SenchaSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SENCHA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            token_program: &arr[1],
            swap: &arr[2],
            user_authority: &arr[3],
            input_user_account: &arr[4],
            input_token_account: &arr[5],
            input_fees_account: &arr[6],
            output_user_account: &arr[7],
            output_token_account: &arr[8],
            output_fees_account: &arr[9],
        }
    }
}
pub const SENCHA_SWAP_IX_DISCM: [u8; 8] = [25, 50, 7, 21, 207, 248, 230, 194];
#[derive(Clone, Debug, PartialEq)]
pub struct SenchaSwapIxData;
impl SenchaSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SENCHA_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SENCHA_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SENCHA_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sencha_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SenchaSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SENCHA_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SenchaSwapIxData.try_to_vec()?,
    })
}
pub fn sencha_swap_ix(keys: SenchaSwapKeys) -> std::io::Result<Instruction> {
    sencha_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn sencha_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SenchaSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SenchaSwapKeys = accounts.into();
    let ix = sencha_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn sencha_swap_invoke(accounts: SenchaSwapAccounts<'_, '_>) -> ProgramResult {
    sencha_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn sencha_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SenchaSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SenchaSwapKeys = accounts.into();
    let ix = sencha_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn sencha_swap_invoke_signed(
    accounts: SenchaSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    sencha_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn sencha_swap_verify_account_keys(
    accounts: SenchaSwapAccounts<'_, '_>,
    keys: SenchaSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.swap.key, keys.swap),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.input_user_account.key, keys.input_user_account),
        (*accounts.input_token_account.key, keys.input_token_account),
        (*accounts.input_fees_account.key, keys.input_fees_account),
        (*accounts.output_user_account.key, keys.output_user_account),
        (
            *accounts.output_token_account.key,
            keys.output_token_account,
        ),
        (*accounts.output_fees_account.key, keys.output_fees_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn sencha_swap_verify_writable_privileges<'me, 'info>(
    accounts: SenchaSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap,
        accounts.input_user_account,
        accounts.input_token_account,
        accounts.input_fees_account,
        accounts.output_user_account,
        accounts.output_token_account,
        accounts.output_fees_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn sencha_swap_verify_account_privileges<'me, 'info>(
    accounts: SenchaSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    sencha_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const STEP_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct StepSwapAccounts<'me, 'info> {
    pub token_swap_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub swap: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source: &'me AccountInfo<'info>,
    pub swap_source: &'me AccountInfo<'info>,
    pub swap_destination: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub pool_fee: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StepSwapKeys {
    pub token_swap_program: Pubkey,
    pub token_program: Pubkey,
    pub swap: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
}
impl From<StepSwapAccounts<'_, '_>> for StepSwapKeys {
    fn from(accounts: StepSwapAccounts) -> Self {
        Self {
            token_swap_program: *accounts.token_swap_program.key,
            token_program: *accounts.token_program.key,
            swap: *accounts.swap.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source: *accounts.source.key,
            swap_source: *accounts.swap_source.key,
            swap_destination: *accounts.swap_destination.key,
            destination: *accounts.destination.key,
            pool_mint: *accounts.pool_mint.key,
            pool_fee: *accounts.pool_fee.key,
        }
    }
}
impl From<StepSwapKeys> for [AccountMeta; STEP_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: StepSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; STEP_SWAP_IX_ACCOUNTS_LEN]> for StepSwapKeys {
    fn from(pubkeys: [Pubkey; STEP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_swap_program: pubkeys[0],
            token_program: pubkeys[1],
            swap: pubkeys[2],
            authority: pubkeys[3],
            user_transfer_authority: pubkeys[4],
            source: pubkeys[5],
            swap_source: pubkeys[6],
            swap_destination: pubkeys[7],
            destination: pubkeys[8],
            pool_mint: pubkeys[9],
            pool_fee: pubkeys[10],
        }
    }
}
impl<'info> From<StepSwapAccounts<'_, 'info>> for [AccountInfo<'info>; STEP_SWAP_IX_ACCOUNTS_LEN] {
    fn from(accounts: StepSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.token_swap_program.clone(),
            accounts.token_program.clone(),
            accounts.swap.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source.clone(),
            accounts.swap_source.clone(),
            accounts.swap_destination.clone(),
            accounts.destination.clone(),
            accounts.pool_mint.clone(),
            accounts.pool_fee.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STEP_SWAP_IX_ACCOUNTS_LEN]>
    for StepSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; STEP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_swap_program: &arr[0],
            token_program: &arr[1],
            swap: &arr[2],
            authority: &arr[3],
            user_transfer_authority: &arr[4],
            source: &arr[5],
            swap_source: &arr[6],
            swap_destination: &arr[7],
            destination: &arr[8],
            pool_mint: &arr[9],
            pool_fee: &arr[10],
        }
    }
}
pub const STEP_SWAP_IX_DISCM: [u8; 8] = [155, 56, 208, 198, 27, 61, 149, 233];
#[derive(Clone, Debug, PartialEq)]
pub struct StepSwapIxData;
impl StepSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STEP_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STEP_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STEP_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn step_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: StepSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STEP_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: StepSwapIxData.try_to_vec()?,
    })
}
pub fn step_swap_ix(keys: StepSwapKeys) -> std::io::Result<Instruction> {
    step_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn step_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StepSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: StepSwapKeys = accounts.into();
    let ix = step_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn step_swap_invoke(accounts: StepSwapAccounts<'_, '_>) -> ProgramResult {
    step_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn step_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StepSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StepSwapKeys = accounts.into();
    let ix = step_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn step_swap_invoke_signed(
    accounts: StepSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    step_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn step_swap_verify_account_keys(
    accounts: StepSwapAccounts<'_, '_>,
    keys: StepSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_swap_program.key, keys.token_swap_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.swap.key, keys.swap),
        (*accounts.authority.key, keys.authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (*accounts.source.key, keys.source),
        (*accounts.swap_source.key, keys.swap_source),
        (*accounts.swap_destination.key, keys.swap_destination),
        (*accounts.destination.key, keys.destination),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.pool_fee.key, keys.pool_fee),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn step_swap_verify_writable_privileges<'me, 'info>(
    accounts: StepSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source,
        accounts.swap_source,
        accounts.swap_destination,
        accounts.destination,
        accounts.pool_mint,
        accounts.pool_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn step_swap_verify_account_privileges<'me, 'info>(
    accounts: StepSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    step_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CROPPER_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct CropperSwapAccounts<'me, 'info> {
    pub token_swap_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub swap: &'me AccountInfo<'info>,
    pub swap_state: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source: &'me AccountInfo<'info>,
    pub swap_source: &'me AccountInfo<'info>,
    pub swap_destination: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub pool_fee: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CropperSwapKeys {
    pub token_swap_program: Pubkey,
    pub token_program: Pubkey,
    pub swap: Pubkey,
    pub swap_state: Pubkey,
    pub authority: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub destination: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_fee: Pubkey,
}
impl From<CropperSwapAccounts<'_, '_>> for CropperSwapKeys {
    fn from(accounts: CropperSwapAccounts) -> Self {
        Self {
            token_swap_program: *accounts.token_swap_program.key,
            token_program: *accounts.token_program.key,
            swap: *accounts.swap.key,
            swap_state: *accounts.swap_state.key,
            authority: *accounts.authority.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source: *accounts.source.key,
            swap_source: *accounts.swap_source.key,
            swap_destination: *accounts.swap_destination.key,
            destination: *accounts.destination.key,
            pool_mint: *accounts.pool_mint.key,
            pool_fee: *accounts.pool_fee.key,
        }
    }
}
impl From<CropperSwapKeys> for [AccountMeta; CROPPER_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: CropperSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.token_swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_fee,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CROPPER_SWAP_IX_ACCOUNTS_LEN]> for CropperSwapKeys {
    fn from(pubkeys: [Pubkey; CROPPER_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_swap_program: pubkeys[0],
            token_program: pubkeys[1],
            swap: pubkeys[2],
            swap_state: pubkeys[3],
            authority: pubkeys[4],
            user_transfer_authority: pubkeys[5],
            source: pubkeys[6],
            swap_source: pubkeys[7],
            swap_destination: pubkeys[8],
            destination: pubkeys[9],
            pool_mint: pubkeys[10],
            pool_fee: pubkeys[11],
        }
    }
}
impl<'info> From<CropperSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; CROPPER_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CropperSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.token_swap_program.clone(),
            accounts.token_program.clone(),
            accounts.swap.clone(),
            accounts.swap_state.clone(),
            accounts.authority.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source.clone(),
            accounts.swap_source.clone(),
            accounts.swap_destination.clone(),
            accounts.destination.clone(),
            accounts.pool_mint.clone(),
            accounts.pool_fee.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CROPPER_SWAP_IX_ACCOUNTS_LEN]>
    for CropperSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CROPPER_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            token_swap_program: &arr[0],
            token_program: &arr[1],
            swap: &arr[2],
            swap_state: &arr[3],
            authority: &arr[4],
            user_transfer_authority: &arr[5],
            source: &arr[6],
            swap_source: &arr[7],
            swap_destination: &arr[8],
            destination: &arr[9],
            pool_mint: &arr[10],
            pool_fee: &arr[11],
        }
    }
}
pub const CROPPER_SWAP_IX_DISCM: [u8; 8] = [230, 216, 47, 182, 165, 117, 210, 103];
#[derive(Clone, Debug, PartialEq)]
pub struct CropperSwapIxData;
impl CropperSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CROPPER_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CROPPER_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CROPPER_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cropper_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: CropperSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CROPPER_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CropperSwapIxData.try_to_vec()?,
    })
}
pub fn cropper_swap_ix(keys: CropperSwapKeys) -> std::io::Result<Instruction> {
    cropper_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn cropper_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CropperSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CropperSwapKeys = accounts.into();
    let ix = cropper_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cropper_swap_invoke(accounts: CropperSwapAccounts<'_, '_>) -> ProgramResult {
    cropper_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn cropper_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CropperSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CropperSwapKeys = accounts.into();
    let ix = cropper_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cropper_swap_invoke_signed(
    accounts: CropperSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cropper_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn cropper_swap_verify_account_keys(
    accounts: CropperSwapAccounts<'_, '_>,
    keys: CropperSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.token_swap_program.key, keys.token_swap_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.swap.key, keys.swap),
        (*accounts.swap_state.key, keys.swap_state),
        (*accounts.authority.key, keys.authority),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (*accounts.source.key, keys.source),
        (*accounts.swap_source.key, keys.swap_source),
        (*accounts.swap_destination.key, keys.swap_destination),
        (*accounts.destination.key, keys.destination),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.pool_fee.key, keys.pool_fee),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cropper_swap_verify_writable_privileges<'me, 'info>(
    accounts: CropperSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source,
        accounts.swap_source,
        accounts.swap_destination,
        accounts.destination,
        accounts.pool_mint,
        accounts.pool_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cropper_swap_verify_account_privileges<'me, 'info>(
    accounts: CropperSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cropper_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const RAYDIUM_SWAP_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct RaydiumSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub amm_id: &'me AccountInfo<'info>,
    pub amm_authority: &'me AccountInfo<'info>,
    pub amm_open_orders: &'me AccountInfo<'info>,
    pub pool_coin_token_account: &'me AccountInfo<'info>,
    pub pool_pc_token_account: &'me AccountInfo<'info>,
    pub serum_program_id: &'me AccountInfo<'info>,
    pub serum_market: &'me AccountInfo<'info>,
    pub serum_bids: &'me AccountInfo<'info>,
    pub serum_asks: &'me AccountInfo<'info>,
    pub serum_event_queue: &'me AccountInfo<'info>,
    pub serum_coin_vault_account: &'me AccountInfo<'info>,
    pub serum_pc_vault_account: &'me AccountInfo<'info>,
    pub serum_vault_signer: &'me AccountInfo<'info>,
    pub user_source_token_account: &'me AccountInfo<'info>,
    pub user_destination_token_account: &'me AccountInfo<'info>,
    pub user_source_owner: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RaydiumSwapKeys {
    pub swap_program: Pubkey,
    pub token_program: Pubkey,
    pub amm_id: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_open_orders: Pubkey,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub serum_program_id: Pubkey,
    pub serum_market: Pubkey,
    pub serum_bids: Pubkey,
    pub serum_asks: Pubkey,
    pub serum_event_queue: Pubkey,
    pub serum_coin_vault_account: Pubkey,
    pub serum_pc_vault_account: Pubkey,
    pub serum_vault_signer: Pubkey,
    pub user_source_token_account: Pubkey,
    pub user_destination_token_account: Pubkey,
    pub user_source_owner: Pubkey,
}
impl From<RaydiumSwapAccounts<'_, '_>> for RaydiumSwapKeys {
    fn from(accounts: RaydiumSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            token_program: *accounts.token_program.key,
            amm_id: *accounts.amm_id.key,
            amm_authority: *accounts.amm_authority.key,
            amm_open_orders: *accounts.amm_open_orders.key,
            pool_coin_token_account: *accounts.pool_coin_token_account.key,
            pool_pc_token_account: *accounts.pool_pc_token_account.key,
            serum_program_id: *accounts.serum_program_id.key,
            serum_market: *accounts.serum_market.key,
            serum_bids: *accounts.serum_bids.key,
            serum_asks: *accounts.serum_asks.key,
            serum_event_queue: *accounts.serum_event_queue.key,
            serum_coin_vault_account: *accounts.serum_coin_vault_account.key,
            serum_pc_vault_account: *accounts.serum_pc_vault_account.key,
            serum_vault_signer: *accounts.serum_vault_signer.key,
            user_source_token_account: *accounts.user_source_token_account.key,
            user_destination_token_account: *accounts.user_destination_token_account.key,
            user_source_owner: *accounts.user_source_owner.key,
        }
    }
}
impl From<RaydiumSwapKeys> for [AccountMeta; RAYDIUM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: RaydiumSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_id,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_open_orders,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_coin_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_pc_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_program_id,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.serum_market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_coin_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_pc_vault_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.serum_vault_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_source_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_owner,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; RAYDIUM_SWAP_IX_ACCOUNTS_LEN]> for RaydiumSwapKeys {
    fn from(pubkeys: [Pubkey; RAYDIUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            token_program: pubkeys[1],
            amm_id: pubkeys[2],
            amm_authority: pubkeys[3],
            amm_open_orders: pubkeys[4],
            pool_coin_token_account: pubkeys[5],
            pool_pc_token_account: pubkeys[6],
            serum_program_id: pubkeys[7],
            serum_market: pubkeys[8],
            serum_bids: pubkeys[9],
            serum_asks: pubkeys[10],
            serum_event_queue: pubkeys[11],
            serum_coin_vault_account: pubkeys[12],
            serum_pc_vault_account: pubkeys[13],
            serum_vault_signer: pubkeys[14],
            user_source_token_account: pubkeys[15],
            user_destination_token_account: pubkeys[16],
            user_source_owner: pubkeys[17],
        }
    }
}
impl<'info> From<RaydiumSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; RAYDIUM_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RaydiumSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.token_program.clone(),
            accounts.amm_id.clone(),
            accounts.amm_authority.clone(),
            accounts.amm_open_orders.clone(),
            accounts.pool_coin_token_account.clone(),
            accounts.pool_pc_token_account.clone(),
            accounts.serum_program_id.clone(),
            accounts.serum_market.clone(),
            accounts.serum_bids.clone(),
            accounts.serum_asks.clone(),
            accounts.serum_event_queue.clone(),
            accounts.serum_coin_vault_account.clone(),
            accounts.serum_pc_vault_account.clone(),
            accounts.serum_vault_signer.clone(),
            accounts.user_source_token_account.clone(),
            accounts.user_destination_token_account.clone(),
            accounts.user_source_owner.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RAYDIUM_SWAP_IX_ACCOUNTS_LEN]>
    for RaydiumSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RAYDIUM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            token_program: &arr[1],
            amm_id: &arr[2],
            amm_authority: &arr[3],
            amm_open_orders: &arr[4],
            pool_coin_token_account: &arr[5],
            pool_pc_token_account: &arr[6],
            serum_program_id: &arr[7],
            serum_market: &arr[8],
            serum_bids: &arr[9],
            serum_asks: &arr[10],
            serum_event_queue: &arr[11],
            serum_coin_vault_account: &arr[12],
            serum_pc_vault_account: &arr[13],
            serum_vault_signer: &arr[14],
            user_source_token_account: &arr[15],
            user_destination_token_account: &arr[16],
            user_source_owner: &arr[17],
        }
    }
}
pub const RAYDIUM_SWAP_IX_DISCM: [u8; 8] = [177, 173, 42, 240, 184, 4, 124, 81];
#[derive(Clone, Debug, PartialEq)]
pub struct RaydiumSwapIxData;
impl RaydiumSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RAYDIUM_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RAYDIUM_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RAYDIUM_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn raydium_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: RaydiumSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RAYDIUM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RaydiumSwapIxData.try_to_vec()?,
    })
}
pub fn raydium_swap_ix(keys: RaydiumSwapKeys) -> std::io::Result<Instruction> {
    raydium_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn raydium_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RaydiumSwapKeys = accounts.into();
    let ix = raydium_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn raydium_swap_invoke(accounts: RaydiumSwapAccounts<'_, '_>) -> ProgramResult {
    raydium_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn raydium_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RaydiumSwapKeys = accounts.into();
    let ix = raydium_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn raydium_swap_invoke_signed(
    accounts: RaydiumSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    raydium_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn raydium_swap_verify_account_keys(
    accounts: RaydiumSwapAccounts<'_, '_>,
    keys: RaydiumSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.amm_id.key, keys.amm_id),
        (*accounts.amm_authority.key, keys.amm_authority),
        (*accounts.amm_open_orders.key, keys.amm_open_orders),
        (
            *accounts.pool_coin_token_account.key,
            keys.pool_coin_token_account,
        ),
        (
            *accounts.pool_pc_token_account.key,
            keys.pool_pc_token_account,
        ),
        (*accounts.serum_program_id.key, keys.serum_program_id),
        (*accounts.serum_market.key, keys.serum_market),
        (*accounts.serum_bids.key, keys.serum_bids),
        (*accounts.serum_asks.key, keys.serum_asks),
        (*accounts.serum_event_queue.key, keys.serum_event_queue),
        (
            *accounts.serum_coin_vault_account.key,
            keys.serum_coin_vault_account,
        ),
        (
            *accounts.serum_pc_vault_account.key,
            keys.serum_pc_vault_account,
        ),
        (*accounts.serum_vault_signer.key, keys.serum_vault_signer),
        (
            *accounts.user_source_token_account.key,
            keys.user_source_token_account,
        ),
        (
            *accounts.user_destination_token_account.key,
            keys.user_destination_token_account,
        ),
        (*accounts.user_source_owner.key, keys.user_source_owner),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn raydium_swap_verify_writable_privileges<'me, 'info>(
    accounts: RaydiumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm_id,
        accounts.amm_open_orders,
        accounts.pool_coin_token_account,
        accounts.pool_pc_token_account,
        accounts.serum_market,
        accounts.serum_bids,
        accounts.serum_asks,
        accounts.serum_event_queue,
        accounts.serum_coin_vault_account,
        accounts.serum_pc_vault_account,
        accounts.user_source_token_account,
        accounts.user_destination_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn raydium_swap_verify_account_privileges<'me, 'info>(
    accounts: RaydiumSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    raydium_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CREMA_SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct CremaSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub clmm_config: &'me AccountInfo<'info>,
    pub clmmpool: &'me AccountInfo<'info>,
    pub token_a: &'me AccountInfo<'info>,
    pub token_b: &'me AccountInfo<'info>,
    pub account_a: &'me AccountInfo<'info>,
    pub account_b: &'me AccountInfo<'info>,
    pub token_a_vault: &'me AccountInfo<'info>,
    pub token_b_vault: &'me AccountInfo<'info>,
    pub tick_array_map: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub partner: &'me AccountInfo<'info>,
    pub partner_ata_a: &'me AccountInfo<'info>,
    pub partner_ata_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CremaSwapKeys {
    pub swap_program: Pubkey,
    pub clmm_config: Pubkey,
    pub clmmpool: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub account_a: Pubkey,
    pub account_b: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub tick_array_map: Pubkey,
    pub owner: Pubkey,
    pub partner: Pubkey,
    pub partner_ata_a: Pubkey,
    pub partner_ata_b: Pubkey,
    pub token_program: Pubkey,
}
impl From<CremaSwapAccounts<'_, '_>> for CremaSwapKeys {
    fn from(accounts: CremaSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            clmm_config: *accounts.clmm_config.key,
            clmmpool: *accounts.clmmpool.key,
            token_a: *accounts.token_a.key,
            token_b: *accounts.token_b.key,
            account_a: *accounts.account_a.key,
            account_b: *accounts.account_b.key,
            token_a_vault: *accounts.token_a_vault.key,
            token_b_vault: *accounts.token_b_vault.key,
            tick_array_map: *accounts.tick_array_map.key,
            owner: *accounts.owner.key,
            partner: *accounts.partner.key,
            partner_ata_a: *accounts.partner_ata_a.key,
            partner_ata_b: *accounts.partner_ata_b.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<CremaSwapKeys> for [AccountMeta; CREMA_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: CremaSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clmmpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array_map,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.partner_ata_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.partner_ata_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREMA_SWAP_IX_ACCOUNTS_LEN]> for CremaSwapKeys {
    fn from(pubkeys: [Pubkey; CREMA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            clmm_config: pubkeys[1],
            clmmpool: pubkeys[2],
            token_a: pubkeys[3],
            token_b: pubkeys[4],
            account_a: pubkeys[5],
            account_b: pubkeys[6],
            token_a_vault: pubkeys[7],
            token_b_vault: pubkeys[8],
            tick_array_map: pubkeys[9],
            owner: pubkeys[10],
            partner: pubkeys[11],
            partner_ata_a: pubkeys[12],
            partner_ata_b: pubkeys[13],
            token_program: pubkeys[14],
        }
    }
}
impl<'info> From<CremaSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREMA_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CremaSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.clmm_config.clone(),
            accounts.clmmpool.clone(),
            accounts.token_a.clone(),
            accounts.token_b.clone(),
            accounts.account_a.clone(),
            accounts.account_b.clone(),
            accounts.token_a_vault.clone(),
            accounts.token_b_vault.clone(),
            accounts.tick_array_map.clone(),
            accounts.owner.clone(),
            accounts.partner.clone(),
            accounts.partner_ata_a.clone(),
            accounts.partner_ata_b.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREMA_SWAP_IX_ACCOUNTS_LEN]>
    for CremaSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREMA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            clmm_config: &arr[1],
            clmmpool: &arr[2],
            token_a: &arr[3],
            token_b: &arr[4],
            account_a: &arr[5],
            account_b: &arr[6],
            token_a_vault: &arr[7],
            token_b_vault: &arr[8],
            tick_array_map: &arr[9],
            owner: &arr[10],
            partner: &arr[11],
            partner_ata_a: &arr[12],
            partner_ata_b: &arr[13],
            token_program: &arr[14],
        }
    }
}
pub const CREMA_SWAP_IX_DISCM: [u8; 8] = [169, 220, 41, 250, 35, 190, 133, 198];
#[derive(Clone, Debug, PartialEq)]
pub struct CremaSwapIxData;
impl CremaSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CREMA_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREMA_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CREMA_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn crema_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: CremaSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CREMA_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CremaSwapIxData.try_to_vec()?,
    })
}
pub fn crema_swap_ix(keys: CremaSwapKeys) -> std::io::Result<Instruction> {
    crema_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn crema_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CremaSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CremaSwapKeys = accounts.into();
    let ix = crema_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn crema_swap_invoke(accounts: CremaSwapAccounts<'_, '_>) -> ProgramResult {
    crema_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn crema_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CremaSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CremaSwapKeys = accounts.into();
    let ix = crema_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn crema_swap_invoke_signed(
    accounts: CremaSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    crema_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn crema_swap_verify_account_keys(
    accounts: CremaSwapAccounts<'_, '_>,
    keys: CremaSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.clmm_config.key, keys.clmm_config),
        (*accounts.clmmpool.key, keys.clmmpool),
        (*accounts.token_a.key, keys.token_a),
        (*accounts.token_b.key, keys.token_b),
        (*accounts.account_a.key, keys.account_a),
        (*accounts.account_b.key, keys.account_b),
        (*accounts.token_a_vault.key, keys.token_a_vault),
        (*accounts.token_b_vault.key, keys.token_b_vault),
        (*accounts.tick_array_map.key, keys.tick_array_map),
        (*accounts.owner.key, keys.owner),
        (*accounts.partner.key, keys.partner),
        (*accounts.partner_ata_a.key, keys.partner_ata_a),
        (*accounts.partner_ata_b.key, keys.partner_ata_b),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn crema_swap_verify_writable_privileges<'me, 'info>(
    accounts: CremaSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.clmmpool,
        accounts.account_a,
        accounts.account_b,
        accounts.token_a_vault,
        accounts.token_b_vault,
        accounts.tick_array_map,
        accounts.partner_ata_a,
        accounts.partner_ata_b,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn crema_swap_verify_account_privileges<'me, 'info>(
    accounts: CremaSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    crema_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LIFINITY_SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct LifinitySwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_info: &'me AccountInfo<'info>,
    pub destination_info: &'me AccountInfo<'info>,
    pub swap_source: &'me AccountInfo<'info>,
    pub swap_destination: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub pyth_account: &'me AccountInfo<'info>,
    pub pyth_pc_account: &'me AccountInfo<'info>,
    pub config_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LifinitySwapKeys {
    pub swap_program: Pubkey,
    pub authority: Pubkey,
    pub amm: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_info: Pubkey,
    pub destination_info: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub fee_account: Pubkey,
    pub token_program: Pubkey,
    pub pyth_account: Pubkey,
    pub pyth_pc_account: Pubkey,
    pub config_account: Pubkey,
}
impl From<LifinitySwapAccounts<'_, '_>> for LifinitySwapKeys {
    fn from(accounts: LifinitySwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            authority: *accounts.authority.key,
            amm: *accounts.amm.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_info: *accounts.source_info.key,
            destination_info: *accounts.destination_info.key,
            swap_source: *accounts.swap_source.key,
            swap_destination: *accounts.swap_destination.key,
            pool_mint: *accounts.pool_mint.key,
            fee_account: *accounts.fee_account.key,
            token_program: *accounts.token_program.key,
            pyth_account: *accounts.pyth_account.key,
            pyth_pc_account: *accounts.pyth_pc_account.key,
            config_account: *accounts.config_account.key,
        }
    }
}
impl From<LifinitySwapKeys> for [AccountMeta; LIFINITY_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: LifinitySwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_pc_account,
                is_signer: false,
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
impl From<[Pubkey; LIFINITY_SWAP_IX_ACCOUNTS_LEN]> for LifinitySwapKeys {
    fn from(pubkeys: [Pubkey; LIFINITY_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            authority: pubkeys[1],
            amm: pubkeys[2],
            user_transfer_authority: pubkeys[3],
            source_info: pubkeys[4],
            destination_info: pubkeys[5],
            swap_source: pubkeys[6],
            swap_destination: pubkeys[7],
            pool_mint: pubkeys[8],
            fee_account: pubkeys[9],
            token_program: pubkeys[10],
            pyth_account: pubkeys[11],
            pyth_pc_account: pubkeys[12],
            config_account: pubkeys[13],
        }
    }
}
impl<'info> From<LifinitySwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIFINITY_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: LifinitySwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.authority.clone(),
            accounts.amm.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_info.clone(),
            accounts.destination_info.clone(),
            accounts.swap_source.clone(),
            accounts.swap_destination.clone(),
            accounts.pool_mint.clone(),
            accounts.fee_account.clone(),
            accounts.token_program.clone(),
            accounts.pyth_account.clone(),
            accounts.pyth_pc_account.clone(),
            accounts.config_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIFINITY_SWAP_IX_ACCOUNTS_LEN]>
    for LifinitySwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LIFINITY_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            authority: &arr[1],
            amm: &arr[2],
            user_transfer_authority: &arr[3],
            source_info: &arr[4],
            destination_info: &arr[5],
            swap_source: &arr[6],
            swap_destination: &arr[7],
            pool_mint: &arr[8],
            fee_account: &arr[9],
            token_program: &arr[10],
            pyth_account: &arr[11],
            pyth_pc_account: &arr[12],
            config_account: &arr[13],
        }
    }
}
pub const LIFINITY_SWAP_IX_DISCM: [u8; 8] = [23, 96, 165, 33, 90, 214, 96, 153];
#[derive(Clone, Debug, PartialEq)]
pub struct LifinitySwapIxData;
impl LifinitySwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIFINITY_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIFINITY_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIFINITY_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lifinity_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: LifinitySwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIFINITY_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LifinitySwapIxData.try_to_vec()?,
    })
}
pub fn lifinity_swap_ix(keys: LifinitySwapKeys) -> std::io::Result<Instruction> {
    lifinity_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn lifinity_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LifinitySwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LifinitySwapKeys = accounts.into();
    let ix = lifinity_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lifinity_swap_invoke(accounts: LifinitySwapAccounts<'_, '_>) -> ProgramResult {
    lifinity_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn lifinity_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LifinitySwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LifinitySwapKeys = accounts.into();
    let ix = lifinity_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lifinity_swap_invoke_signed(
    accounts: LifinitySwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lifinity_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn lifinity_swap_verify_account_keys(
    accounts: LifinitySwapAccounts<'_, '_>,
    keys: LifinitySwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.amm.key, keys.amm),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (*accounts.source_info.key, keys.source_info),
        (*accounts.destination_info.key, keys.destination_info),
        (*accounts.swap_source.key, keys.swap_source),
        (*accounts.swap_destination.key, keys.swap_destination),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.fee_account.key, keys.fee_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.pyth_account.key, keys.pyth_account),
        (*accounts.pyth_pc_account.key, keys.pyth_pc_account),
        (*accounts.config_account.key, keys.config_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lifinity_swap_verify_writable_privileges<'me, 'info>(
    accounts: LifinitySwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.source_info,
        accounts.destination_info,
        accounts.swap_source,
        accounts.swap_destination,
        accounts.pool_mint,
        accounts.fee_account,
        accounts.config_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lifinity_swap_verify_account_privileges<'me, 'info>(
    accounts: LifinitySwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lifinity_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MARINADE_DEPOSIT_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct MarinadeDepositAccounts<'me, 'info> {
    pub marinade_finance_program: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub msol_mint: &'me AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg_authority: &'me AccountInfo<'info>,
    pub reserve_pda: &'me AccountInfo<'info>,
    pub transfer_from: &'me AccountInfo<'info>,
    pub mint_to: &'me AccountInfo<'info>,
    pub msol_mint_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub user_wsol_token_account: &'me AccountInfo<'info>,
    pub temp_wsol_token_account: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarinadeDepositKeys {
    pub marinade_finance_program: Pubkey,
    pub state: Pubkey,
    pub msol_mint: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_msol_leg_authority: Pubkey,
    pub reserve_pda: Pubkey,
    pub transfer_from: Pubkey,
    pub mint_to: Pubkey,
    pub msol_mint_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub user_wsol_token_account: Pubkey,
    pub temp_wsol_token_account: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub payer: Pubkey,
    pub wsol_mint: Pubkey,
    pub rent: Pubkey,
}
impl From<MarinadeDepositAccounts<'_, '_>> for MarinadeDepositKeys {
    fn from(accounts: MarinadeDepositAccounts) -> Self {
        Self {
            marinade_finance_program: *accounts.marinade_finance_program.key,
            state: *accounts.state.key,
            msol_mint: *accounts.msol_mint.key,
            liq_pool_sol_leg_pda: *accounts.liq_pool_sol_leg_pda.key,
            liq_pool_msol_leg: *accounts.liq_pool_msol_leg.key,
            liq_pool_msol_leg_authority: *accounts.liq_pool_msol_leg_authority.key,
            reserve_pda: *accounts.reserve_pda.key,
            transfer_from: *accounts.transfer_from.key,
            mint_to: *accounts.mint_to.key,
            msol_mint_authority: *accounts.msol_mint_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            user_wsol_token_account: *accounts.user_wsol_token_account.key,
            temp_wsol_token_account: *accounts.temp_wsol_token_account.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            payer: *accounts.payer.key,
            wsol_mint: *accounts.wsol_mint.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<MarinadeDepositKeys> for [AccountMeta; MARINADE_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: MarinadeDepositKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marinade_finance_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.msol_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liq_pool_sol_leg_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liq_pool_msol_leg,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liq_pool_msol_leg_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.msol_mint_authority,
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
                pubkey: keys.user_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.temp_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARINADE_DEPOSIT_IX_ACCOUNTS_LEN]> for MarinadeDepositKeys {
    fn from(pubkeys: [Pubkey; MARINADE_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_finance_program: pubkeys[0],
            state: pubkeys[1],
            msol_mint: pubkeys[2],
            liq_pool_sol_leg_pda: pubkeys[3],
            liq_pool_msol_leg: pubkeys[4],
            liq_pool_msol_leg_authority: pubkeys[5],
            reserve_pda: pubkeys[6],
            transfer_from: pubkeys[7],
            mint_to: pubkeys[8],
            msol_mint_authority: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            user_wsol_token_account: pubkeys[12],
            temp_wsol_token_account: pubkeys[13],
            user_transfer_authority: pubkeys[14],
            payer: pubkeys[15],
            wsol_mint: pubkeys[16],
            rent: pubkeys[17],
        }
    }
}
impl<'info> From<MarinadeDepositAccounts<'_, 'info>>
    for [AccountInfo<'info>; MARINADE_DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MarinadeDepositAccounts<'_, 'info>) -> Self {
        [
            accounts.marinade_finance_program.clone(),
            accounts.state.clone(),
            accounts.msol_mint.clone(),
            accounts.liq_pool_sol_leg_pda.clone(),
            accounts.liq_pool_msol_leg.clone(),
            accounts.liq_pool_msol_leg_authority.clone(),
            accounts.reserve_pda.clone(),
            accounts.transfer_from.clone(),
            accounts.mint_to.clone(),
            accounts.msol_mint_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.user_wsol_token_account.clone(),
            accounts.temp_wsol_token_account.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.payer.clone(),
            accounts.wsol_mint.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MARINADE_DEPOSIT_IX_ACCOUNTS_LEN]>
    for MarinadeDepositAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MARINADE_DEPOSIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_finance_program: &arr[0],
            state: &arr[1],
            msol_mint: &arr[2],
            liq_pool_sol_leg_pda: &arr[3],
            liq_pool_msol_leg: &arr[4],
            liq_pool_msol_leg_authority: &arr[5],
            reserve_pda: &arr[6],
            transfer_from: &arr[7],
            mint_to: &arr[8],
            msol_mint_authority: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            user_wsol_token_account: &arr[12],
            temp_wsol_token_account: &arr[13],
            user_transfer_authority: &arr[14],
            payer: &arr[15],
            wsol_mint: &arr[16],
            rent: &arr[17],
        }
    }
}
pub const MARINADE_DEPOSIT_IX_DISCM: [u8; 8] = [62, 236, 248, 28, 222, 232, 182, 73];
#[derive(Clone, Debug, PartialEq)]
pub struct MarinadeDepositIxData;
impl MarinadeDepositIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARINADE_DEPOSIT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MARINADE_DEPOSIT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARINADE_DEPOSIT_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marinade_deposit_ix_with_program_id(
    program_id: Pubkey,
    keys: MarinadeDepositKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARINADE_DEPOSIT_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarinadeDepositIxData.try_to_vec()?,
    })
}
pub fn marinade_deposit_ix(keys: MarinadeDepositKeys) -> std::io::Result<Instruction> {
    marinade_deposit_ix_with_program_id(JUPITER_ID, keys)
}
pub fn marinade_deposit_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeDepositAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarinadeDepositKeys = accounts.into();
    let ix = marinade_deposit_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marinade_deposit_invoke(accounts: MarinadeDepositAccounts<'_, '_>) -> ProgramResult {
    marinade_deposit_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn marinade_deposit_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeDepositAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarinadeDepositKeys = accounts.into();
    let ix = marinade_deposit_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marinade_deposit_invoke_signed(
    accounts: MarinadeDepositAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marinade_deposit_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn marinade_deposit_verify_account_keys(
    accounts: MarinadeDepositAccounts<'_, '_>,
    keys: MarinadeDepositKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            *accounts.marinade_finance_program.key,
            keys.marinade_finance_program,
        ),
        (*accounts.state.key, keys.state),
        (*accounts.msol_mint.key, keys.msol_mint),
        (
            *accounts.liq_pool_sol_leg_pda.key,
            keys.liq_pool_sol_leg_pda,
        ),
        (*accounts.liq_pool_msol_leg.key, keys.liq_pool_msol_leg),
        (
            *accounts.liq_pool_msol_leg_authority.key,
            keys.liq_pool_msol_leg_authority,
        ),
        (*accounts.reserve_pda.key, keys.reserve_pda),
        (*accounts.transfer_from.key, keys.transfer_from),
        (*accounts.mint_to.key, keys.mint_to),
        (*accounts.msol_mint_authority.key, keys.msol_mint_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.user_wsol_token_account.key,
            keys.user_wsol_token_account,
        ),
        (
            *accounts.temp_wsol_token_account.key,
            keys.temp_wsol_token_account,
        ),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (*accounts.payer.key, keys.payer),
        (*accounts.wsol_mint.key, keys.wsol_mint),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marinade_deposit_verify_writable_privileges<'me, 'info>(
    accounts: MarinadeDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.msol_mint,
        accounts.liq_pool_sol_leg_pda,
        accounts.liq_pool_msol_leg,
        accounts.reserve_pda,
        accounts.transfer_from,
        accounts.mint_to,
        accounts.user_wsol_token_account,
        accounts.temp_wsol_token_account,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marinade_deposit_verify_account_privileges<'me, 'info>(
    accounts: MarinadeDepositAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marinade_deposit_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MARINADE_UNSTAKE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct MarinadeUnstakeAccounts<'me, 'info> {
    pub marinade_finance_program: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub msol_mint: &'me AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: &'me AccountInfo<'info>,
    pub liq_pool_msol_leg: &'me AccountInfo<'info>,
    pub treasury_msol_account: &'me AccountInfo<'info>,
    pub get_msol_from: &'me AccountInfo<'info>,
    pub get_msol_from_authority: &'me AccountInfo<'info>,
    pub transfer_sol_to: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub user_wsol_token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarinadeUnstakeKeys {
    pub marinade_finance_program: Pubkey,
    pub state: Pubkey,
    pub msol_mint: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub treasury_msol_account: Pubkey,
    pub get_msol_from: Pubkey,
    pub get_msol_from_authority: Pubkey,
    pub transfer_sol_to: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub user_wsol_token_account: Pubkey,
}
impl From<MarinadeUnstakeAccounts<'_, '_>> for MarinadeUnstakeKeys {
    fn from(accounts: MarinadeUnstakeAccounts) -> Self {
        Self {
            marinade_finance_program: *accounts.marinade_finance_program.key,
            state: *accounts.state.key,
            msol_mint: *accounts.msol_mint.key,
            liq_pool_sol_leg_pda: *accounts.liq_pool_sol_leg_pda.key,
            liq_pool_msol_leg: *accounts.liq_pool_msol_leg.key,
            treasury_msol_account: *accounts.treasury_msol_account.key,
            get_msol_from: *accounts.get_msol_from.key,
            get_msol_from_authority: *accounts.get_msol_from_authority.key,
            transfer_sol_to: *accounts.transfer_sol_to.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            user_wsol_token_account: *accounts.user_wsol_token_account.key,
        }
    }
}
impl From<MarinadeUnstakeKeys> for [AccountMeta; MARINADE_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: MarinadeUnstakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.marinade_finance_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.msol_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liq_pool_sol_leg_pda,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liq_pool_msol_leg,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_msol_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.get_msol_from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.get_msol_from_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.transfer_sol_to,
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
                pubkey: keys.user_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MARINADE_UNSTAKE_IX_ACCOUNTS_LEN]> for MarinadeUnstakeKeys {
    fn from(pubkeys: [Pubkey; MARINADE_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_finance_program: pubkeys[0],
            state: pubkeys[1],
            msol_mint: pubkeys[2],
            liq_pool_sol_leg_pda: pubkeys[3],
            liq_pool_msol_leg: pubkeys[4],
            treasury_msol_account: pubkeys[5],
            get_msol_from: pubkeys[6],
            get_msol_from_authority: pubkeys[7],
            transfer_sol_to: pubkeys[8],
            system_program: pubkeys[9],
            token_program: pubkeys[10],
            user_wsol_token_account: pubkeys[11],
        }
    }
}
impl<'info> From<MarinadeUnstakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; MARINADE_UNSTAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MarinadeUnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.marinade_finance_program.clone(),
            accounts.state.clone(),
            accounts.msol_mint.clone(),
            accounts.liq_pool_sol_leg_pda.clone(),
            accounts.liq_pool_msol_leg.clone(),
            accounts.treasury_msol_account.clone(),
            accounts.get_msol_from.clone(),
            accounts.get_msol_from_authority.clone(),
            accounts.transfer_sol_to.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.user_wsol_token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MARINADE_UNSTAKE_IX_ACCOUNTS_LEN]>
    for MarinadeUnstakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MARINADE_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            marinade_finance_program: &arr[0],
            state: &arr[1],
            msol_mint: &arr[2],
            liq_pool_sol_leg_pda: &arr[3],
            liq_pool_msol_leg: &arr[4],
            treasury_msol_account: &arr[5],
            get_msol_from: &arr[6],
            get_msol_from_authority: &arr[7],
            transfer_sol_to: &arr[8],
            system_program: &arr[9],
            token_program: &arr[10],
            user_wsol_token_account: &arr[11],
        }
    }
}
pub const MARINADE_UNSTAKE_IX_DISCM: [u8; 8] = [41, 120, 15, 0, 113, 219, 42, 1];
#[derive(Clone, Debug, PartialEq)]
pub struct MarinadeUnstakeIxData;
impl MarinadeUnstakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARINADE_UNSTAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MARINADE_UNSTAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARINADE_UNSTAKE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marinade_unstake_ix_with_program_id(
    program_id: Pubkey,
    keys: MarinadeUnstakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARINADE_UNSTAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarinadeUnstakeIxData.try_to_vec()?,
    })
}
pub fn marinade_unstake_ix(keys: MarinadeUnstakeKeys) -> std::io::Result<Instruction> {
    marinade_unstake_ix_with_program_id(JUPITER_ID, keys)
}
pub fn marinade_unstake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeUnstakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarinadeUnstakeKeys = accounts.into();
    let ix = marinade_unstake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marinade_unstake_invoke(accounts: MarinadeUnstakeAccounts<'_, '_>) -> ProgramResult {
    marinade_unstake_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn marinade_unstake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarinadeUnstakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarinadeUnstakeKeys = accounts.into();
    let ix = marinade_unstake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marinade_unstake_invoke_signed(
    accounts: MarinadeUnstakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marinade_unstake_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn marinade_unstake_verify_account_keys(
    accounts: MarinadeUnstakeAccounts<'_, '_>,
    keys: MarinadeUnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            *accounts.marinade_finance_program.key,
            keys.marinade_finance_program,
        ),
        (*accounts.state.key, keys.state),
        (*accounts.msol_mint.key, keys.msol_mint),
        (
            *accounts.liq_pool_sol_leg_pda.key,
            keys.liq_pool_sol_leg_pda,
        ),
        (*accounts.liq_pool_msol_leg.key, keys.liq_pool_msol_leg),
        (
            *accounts.treasury_msol_account.key,
            keys.treasury_msol_account,
        ),
        (*accounts.get_msol_from.key, keys.get_msol_from),
        (
            *accounts.get_msol_from_authority.key,
            keys.get_msol_from_authority,
        ),
        (*accounts.transfer_sol_to.key, keys.transfer_sol_to),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.user_wsol_token_account.key,
            keys.user_wsol_token_account,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marinade_unstake_verify_writable_privileges<'me, 'info>(
    accounts: MarinadeUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.state,
        accounts.msol_mint,
        accounts.liq_pool_sol_leg_pda,
        accounts.liq_pool_msol_leg,
        accounts.treasury_msol_account,
        accounts.get_msol_from,
        accounts.transfer_sol_to,
        accounts.user_wsol_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marinade_unstake_verify_account_privileges<'me, 'info>(
    accounts: MarinadeUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marinade_unstake_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const ALDRIN_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct AldrinSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_signer: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub base_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub fee_pool_token_account: &'me AccountInfo<'info>,
    pub wallet_authority: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AldrinSwapKeys {
    pub swap_program: Pubkey,
    pub pool: Pubkey,
    pub pool_signer: Pubkey,
    pub pool_mint: Pubkey,
    pub base_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub fee_pool_token_account: Pubkey,
    pub wallet_authority: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<AldrinSwapAccounts<'_, '_>> for AldrinSwapKeys {
    fn from(accounts: AldrinSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            pool: *accounts.pool.key,
            pool_signer: *accounts.pool_signer.key,
            pool_mint: *accounts.pool_mint.key,
            base_token_vault: *accounts.base_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            fee_pool_token_account: *accounts.fee_pool_token_account.key,
            wallet_authority: *accounts.wallet_authority.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AldrinSwapKeys> for [AccountMeta; ALDRIN_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: AldrinSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ALDRIN_SWAP_IX_ACCOUNTS_LEN]> for AldrinSwapKeys {
    fn from(pubkeys: [Pubkey; ALDRIN_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            pool: pubkeys[1],
            pool_signer: pubkeys[2],
            pool_mint: pubkeys[3],
            base_token_vault: pubkeys[4],
            quote_token_vault: pubkeys[5],
            fee_pool_token_account: pubkeys[6],
            wallet_authority: pubkeys[7],
            user_base_token_account: pubkeys[8],
            user_quote_token_account: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<AldrinSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; ALDRIN_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AldrinSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.pool.clone(),
            accounts.pool_signer.clone(),
            accounts.pool_mint.clone(),
            accounts.base_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.fee_pool_token_account.clone(),
            accounts.wallet_authority.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ALDRIN_SWAP_IX_ACCOUNTS_LEN]>
    for AldrinSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ALDRIN_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            pool: &arr[1],
            pool_signer: &arr[2],
            pool_mint: &arr[3],
            base_token_vault: &arr[4],
            quote_token_vault: &arr[5],
            fee_pool_token_account: &arr[6],
            wallet_authority: &arr[7],
            user_base_token_account: &arr[8],
            user_quote_token_account: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const ALDRIN_SWAP_IX_DISCM: [u8; 8] = [251, 232, 119, 166, 225, 185, 169, 161];
#[derive(Clone, Debug, PartialEq)]
pub struct AldrinSwapIxData;
impl AldrinSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ALDRIN_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ALDRIN_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ALDRIN_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn aldrin_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: AldrinSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ALDRIN_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AldrinSwapIxData.try_to_vec()?,
    })
}
pub fn aldrin_swap_ix(keys: AldrinSwapKeys) -> std::io::Result<Instruction> {
    aldrin_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn aldrin_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AldrinSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AldrinSwapKeys = accounts.into();
    let ix = aldrin_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn aldrin_swap_invoke(accounts: AldrinSwapAccounts<'_, '_>) -> ProgramResult {
    aldrin_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn aldrin_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AldrinSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AldrinSwapKeys = accounts.into();
    let ix = aldrin_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn aldrin_swap_invoke_signed(
    accounts: AldrinSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    aldrin_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn aldrin_swap_verify_account_keys(
    accounts: AldrinSwapAccounts<'_, '_>,
    keys: AldrinSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_signer.key, keys.pool_signer),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.base_token_vault.key, keys.base_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (
            *accounts.fee_pool_token_account.key,
            keys.fee_pool_token_account,
        ),
        (*accounts.wallet_authority.key, keys.wallet_authority),
        (
            *accounts.user_base_token_account.key,
            keys.user_base_token_account,
        ),
        (
            *accounts.user_quote_token_account.key,
            keys.user_quote_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn aldrin_swap_verify_writable_privileges<'me, 'info>(
    accounts: AldrinSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_mint,
        accounts.base_token_vault,
        accounts.quote_token_vault,
        accounts.fee_pool_token_account,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn aldrin_swap_verify_account_privileges<'me, 'info>(
    accounts: AldrinSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    aldrin_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct AldrinV2SwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub pool_signer: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub base_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub fee_pool_token_account: &'me AccountInfo<'info>,
    pub wallet_authority: &'me AccountInfo<'info>,
    pub user_base_token_account: &'me AccountInfo<'info>,
    pub user_quote_token_account: &'me AccountInfo<'info>,
    pub curve: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AldrinV2SwapKeys {
    pub swap_program: Pubkey,
    pub pool: Pubkey,
    pub pool_signer: Pubkey,
    pub pool_mint: Pubkey,
    pub base_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub fee_pool_token_account: Pubkey,
    pub wallet_authority: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub curve: Pubkey,
    pub token_program: Pubkey,
}
impl From<AldrinV2SwapAccounts<'_, '_>> for AldrinV2SwapKeys {
    fn from(accounts: AldrinV2SwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            pool: *accounts.pool.key,
            pool_signer: *accounts.pool_signer.key,
            pool_mint: *accounts.pool_mint.key,
            base_token_vault: *accounts.base_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            fee_pool_token_account: *accounts.fee_pool_token_account.key,
            wallet_authority: *accounts.wallet_authority.key,
            user_base_token_account: *accounts.user_base_token_account.key,
            user_quote_token_account: *accounts.user_quote_token_account.key,
            curve: *accounts.curve.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<AldrinV2SwapKeys> for [AccountMeta; ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: AldrinV2SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_pool_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wallet_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_base_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.curve,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN]> for AldrinV2SwapKeys {
    fn from(pubkeys: [Pubkey; ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            pool: pubkeys[1],
            pool_signer: pubkeys[2],
            pool_mint: pubkeys[3],
            base_token_vault: pubkeys[4],
            quote_token_vault: pubkeys[5],
            fee_pool_token_account: pubkeys[6],
            wallet_authority: pubkeys[7],
            user_base_token_account: pubkeys[8],
            user_quote_token_account: pubkeys[9],
            curve: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<AldrinV2SwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AldrinV2SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.pool.clone(),
            accounts.pool_signer.clone(),
            accounts.pool_mint.clone(),
            accounts.base_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.fee_pool_token_account.clone(),
            accounts.wallet_authority.clone(),
            accounts.user_base_token_account.clone(),
            accounts.user_quote_token_account.clone(),
            accounts.curve.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN]>
    for AldrinV2SwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            pool: &arr[1],
            pool_signer: &arr[2],
            pool_mint: &arr[3],
            base_token_vault: &arr[4],
            quote_token_vault: &arr[5],
            fee_pool_token_account: &arr[6],
            wallet_authority: &arr[7],
            user_base_token_account: &arr[8],
            user_quote_token_account: &arr[9],
            curve: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const ALDRIN_V2_SWAP_IX_DISCM: [u8; 8] = [190, 166, 89, 139, 33, 152, 16, 10];
#[derive(Clone, Debug, PartialEq)]
pub struct AldrinV2SwapIxData;
impl AldrinV2SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ALDRIN_V2_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ALDRIN_V2_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ALDRIN_V2_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn aldrin_v2_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: AldrinV2SwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ALDRIN_V2_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: AldrinV2SwapIxData.try_to_vec()?,
    })
}
pub fn aldrin_v2_swap_ix(keys: AldrinV2SwapKeys) -> std::io::Result<Instruction> {
    aldrin_v2_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn aldrin_v2_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AldrinV2SwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AldrinV2SwapKeys = accounts.into();
    let ix = aldrin_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn aldrin_v2_swap_invoke(accounts: AldrinV2SwapAccounts<'_, '_>) -> ProgramResult {
    aldrin_v2_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn aldrin_v2_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AldrinV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AldrinV2SwapKeys = accounts.into();
    let ix = aldrin_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn aldrin_v2_swap_invoke_signed(
    accounts: AldrinV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    aldrin_v2_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn aldrin_v2_swap_verify_account_keys(
    accounts: AldrinV2SwapAccounts<'_, '_>,
    keys: AldrinV2SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.pool.key, keys.pool),
        (*accounts.pool_signer.key, keys.pool_signer),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.base_token_vault.key, keys.base_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (
            *accounts.fee_pool_token_account.key,
            keys.fee_pool_token_account,
        ),
        (*accounts.wallet_authority.key, keys.wallet_authority),
        (
            *accounts.user_base_token_account.key,
            keys.user_base_token_account,
        ),
        (
            *accounts.user_quote_token_account.key,
            keys.user_quote_token_account,
        ),
        (*accounts.curve.key, keys.curve),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn aldrin_v2_swap_verify_writable_privileges<'me, 'info>(
    accounts: AldrinV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_mint,
        accounts.base_token_vault,
        accounts.quote_token_vault,
        accounts.fee_pool_token_account,
        accounts.user_base_token_account,
        accounts.user_quote_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn aldrin_v2_swap_verify_account_privileges<'me, 'info>(
    accounts: AldrinV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    aldrin_v2_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct WhirlpoolSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_authority: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array0: &'me AccountInfo<'info>,
    pub tick_array1: &'me AccountInfo<'info>,
    pub tick_array2: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WhirlpoolSwapKeys {
    pub swap_program: Pubkey,
    pub token_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
}
impl From<WhirlpoolSwapAccounts<'_, '_>> for WhirlpoolSwapKeys {
    fn from(accounts: WhirlpoolSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            token_program: *accounts.token_program.key,
            token_authority: *accounts.token_authority.key,
            whirlpool: *accounts.whirlpool.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array0: *accounts.tick_array0.key,
            tick_array1: *accounts.tick_array1.key,
            tick_array2: *accounts.tick_array2.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<WhirlpoolSwapKeys> for [AccountMeta; WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: WhirlpoolSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN]> for WhirlpoolSwapKeys {
    fn from(pubkeys: [Pubkey; WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            token_program: pubkeys[1],
            token_authority: pubkeys[2],
            whirlpool: pubkeys[3],
            token_owner_account_a: pubkeys[4],
            token_vault_a: pubkeys[5],
            token_owner_account_b: pubkeys[6],
            token_vault_b: pubkeys[7],
            tick_array0: pubkeys[8],
            tick_array1: pubkeys[9],
            tick_array2: pubkeys[10],
            oracle: pubkeys[11],
        }
    }
}
impl<'info> From<WhirlpoolSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: WhirlpoolSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.token_program.clone(),
            accounts.token_authority.clone(),
            accounts.whirlpool.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array0.clone(),
            accounts.tick_array1.clone(),
            accounts.tick_array2.clone(),
            accounts.oracle.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN]>
    for WhirlpoolSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            token_program: &arr[1],
            token_authority: &arr[2],
            whirlpool: &arr[3],
            token_owner_account_a: &arr[4],
            token_vault_a: &arr[5],
            token_owner_account_b: &arr[6],
            token_vault_b: &arr[7],
            tick_array0: &arr[8],
            tick_array1: &arr[9],
            tick_array2: &arr[10],
            oracle: &arr[11],
        }
    }
}
pub const WHIRLPOOL_SWAP_IX_DISCM: [u8; 8] = [123, 229, 184, 63, 12, 0, 92, 145];
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolSwapIxData;
impl WhirlpoolSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOL_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WHIRLPOOL_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOL_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn whirlpool_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: WhirlpoolSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WHIRLPOOL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WhirlpoolSwapIxData.try_to_vec()?,
    })
}
pub fn whirlpool_swap_ix(keys: WhirlpoolSwapKeys) -> std::io::Result<Instruction> {
    whirlpool_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn whirlpool_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WhirlpoolSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WhirlpoolSwapKeys = accounts.into();
    let ix = whirlpool_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn whirlpool_swap_invoke(accounts: WhirlpoolSwapAccounts<'_, '_>) -> ProgramResult {
    whirlpool_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn whirlpool_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WhirlpoolSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WhirlpoolSwapKeys = accounts.into();
    let ix = whirlpool_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn whirlpool_swap_invoke_signed(
    accounts: WhirlpoolSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    whirlpool_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn whirlpool_swap_verify_account_keys(
    accounts: WhirlpoolSwapAccounts<'_, '_>,
    keys: WhirlpoolSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_authority.key, keys.token_authority),
        (*accounts.whirlpool.key, keys.whirlpool),
        (
            *accounts.token_owner_account_a.key,
            keys.token_owner_account_a,
        ),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (
            *accounts.token_owner_account_b.key,
            keys.token_owner_account_b,
        ),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array0.key, keys.tick_array0),
        (*accounts.tick_array1.key, keys.tick_array1),
        (*accounts.tick_array2.key, keys.tick_array2),
        (*accounts.oracle.key, keys.oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn whirlpool_swap_verify_writable_privileges<'me, 'info>(
    accounts: WhirlpoolSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.whirlpool,
        accounts.token_owner_account_a,
        accounts.token_vault_a,
        accounts.token_owner_account_b,
        accounts.token_vault_b,
        accounts.tick_array0,
        accounts.tick_array1,
        accounts.tick_array2,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn whirlpool_swap_verify_account_privileges<'me, 'info>(
    accounts: WhirlpoolSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    whirlpool_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct WhirlpoolSwapV2Accounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub token_program_a: &'me AccountInfo<'info>,
    pub token_program_b: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub token_authority: &'me AccountInfo<'info>,
    pub whirlpool: &'me AccountInfo<'info>,
    pub token_mint_a: &'me AccountInfo<'info>,
    pub token_mint_b: &'me AccountInfo<'info>,
    pub token_owner_account_a: &'me AccountInfo<'info>,
    pub token_vault_a: &'me AccountInfo<'info>,
    pub token_owner_account_b: &'me AccountInfo<'info>,
    pub token_vault_b: &'me AccountInfo<'info>,
    pub tick_array0: &'me AccountInfo<'info>,
    pub tick_array1: &'me AccountInfo<'info>,
    pub tick_array2: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WhirlpoolSwapV2Keys {
    pub swap_program: Pubkey,
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub memo_program: Pubkey,
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array0: Pubkey,
    pub tick_array1: Pubkey,
    pub tick_array2: Pubkey,
    pub oracle: Pubkey,
}
impl From<WhirlpoolSwapV2Accounts<'_, '_>> for WhirlpoolSwapV2Keys {
    fn from(accounts: WhirlpoolSwapV2Accounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            token_program_a: *accounts.token_program_a.key,
            token_program_b: *accounts.token_program_b.key,
            memo_program: *accounts.memo_program.key,
            token_authority: *accounts.token_authority.key,
            whirlpool: *accounts.whirlpool.key,
            token_mint_a: *accounts.token_mint_a.key,
            token_mint_b: *accounts.token_mint_b.key,
            token_owner_account_a: *accounts.token_owner_account_a.key,
            token_vault_a: *accounts.token_vault_a.key,
            token_owner_account_b: *accounts.token_owner_account_b.key,
            token_vault_b: *accounts.token_vault_b.key,
            tick_array0: *accounts.tick_array0.key,
            tick_array1: *accounts.tick_array1.key,
            tick_array2: *accounts.tick_array2.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<WhirlpoolSwapV2Keys> for [AccountMeta; WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: WhirlpoolSwapV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.whirlpool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_mint_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_mint_b,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_a,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_owner_account_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_vault_b,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array1,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tick_array2,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN]> for WhirlpoolSwapV2Keys {
    fn from(pubkeys: [Pubkey; WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            token_program_a: pubkeys[1],
            token_program_b: pubkeys[2],
            memo_program: pubkeys[3],
            token_authority: pubkeys[4],
            whirlpool: pubkeys[5],
            token_mint_a: pubkeys[6],
            token_mint_b: pubkeys[7],
            token_owner_account_a: pubkeys[8],
            token_vault_a: pubkeys[9],
            token_owner_account_b: pubkeys[10],
            token_vault_b: pubkeys[11],
            tick_array0: pubkeys[12],
            tick_array1: pubkeys[13],
            tick_array2: pubkeys[14],
            oracle: pubkeys[15],
        }
    }
}
impl<'info> From<WhirlpoolSwapV2Accounts<'_, 'info>>
    for [AccountInfo<'info>; WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: WhirlpoolSwapV2Accounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.token_program_a.clone(),
            accounts.token_program_b.clone(),
            accounts.memo_program.clone(),
            accounts.token_authority.clone(),
            accounts.whirlpool.clone(),
            accounts.token_mint_a.clone(),
            accounts.token_mint_b.clone(),
            accounts.token_owner_account_a.clone(),
            accounts.token_vault_a.clone(),
            accounts.token_owner_account_b.clone(),
            accounts.token_vault_b.clone(),
            accounts.tick_array0.clone(),
            accounts.tick_array1.clone(),
            accounts.tick_array2.clone(),
            accounts.oracle.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN]>
    for WhirlpoolSwapV2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            token_program_a: &arr[1],
            token_program_b: &arr[2],
            memo_program: &arr[3],
            token_authority: &arr[4],
            whirlpool: &arr[5],
            token_mint_a: &arr[6],
            token_mint_b: &arr[7],
            token_owner_account_a: &arr[8],
            token_vault_a: &arr[9],
            token_owner_account_b: &arr[10],
            token_vault_b: &arr[11],
            tick_array0: &arr[12],
            tick_array1: &arr[13],
            tick_array2: &arr[14],
            oracle: &arr[15],
        }
    }
}
pub const WHIRLPOOL_SWAP_V2_IX_DISCM: [u8; 8] = [56, 166, 129, 9, 157, 205, 118, 217];
#[derive(Clone, Debug, PartialEq)]
pub struct WhirlpoolSwapV2IxData;
impl WhirlpoolSwapV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WHIRLPOOL_SWAP_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WHIRLPOOL_SWAP_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WHIRLPOOL_SWAP_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn whirlpool_swap_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: WhirlpoolSwapV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WHIRLPOOL_SWAP_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WhirlpoolSwapV2IxData.try_to_vec()?,
    })
}
pub fn whirlpool_swap_v2_ix(keys: WhirlpoolSwapV2Keys) -> std::io::Result<Instruction> {
    whirlpool_swap_v2_ix_with_program_id(JUPITER_ID, keys)
}
pub fn whirlpool_swap_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WhirlpoolSwapV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: WhirlpoolSwapV2Keys = accounts.into();
    let ix = whirlpool_swap_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn whirlpool_swap_v2_invoke(accounts: WhirlpoolSwapV2Accounts<'_, '_>) -> ProgramResult {
    whirlpool_swap_v2_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn whirlpool_swap_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WhirlpoolSwapV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WhirlpoolSwapV2Keys = accounts.into();
    let ix = whirlpool_swap_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn whirlpool_swap_v2_invoke_signed(
    accounts: WhirlpoolSwapV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    whirlpool_swap_v2_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn whirlpool_swap_v2_verify_account_keys(
    accounts: WhirlpoolSwapV2Accounts<'_, '_>,
    keys: WhirlpoolSwapV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.token_program_a.key, keys.token_program_a),
        (*accounts.token_program_b.key, keys.token_program_b),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.token_authority.key, keys.token_authority),
        (*accounts.whirlpool.key, keys.whirlpool),
        (*accounts.token_mint_a.key, keys.token_mint_a),
        (*accounts.token_mint_b.key, keys.token_mint_b),
        (
            *accounts.token_owner_account_a.key,
            keys.token_owner_account_a,
        ),
        (*accounts.token_vault_a.key, keys.token_vault_a),
        (
            *accounts.token_owner_account_b.key,
            keys.token_owner_account_b,
        ),
        (*accounts.token_vault_b.key, keys.token_vault_b),
        (*accounts.tick_array0.key, keys.tick_array0),
        (*accounts.tick_array1.key, keys.tick_array1),
        (*accounts.tick_array2.key, keys.tick_array2),
        (*accounts.oracle.key, keys.oracle),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn whirlpool_swap_v2_verify_writable_privileges<'me, 'info>(
    accounts: WhirlpoolSwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.whirlpool,
        accounts.token_owner_account_a,
        accounts.token_vault_a,
        accounts.token_owner_account_b,
        accounts.token_vault_b,
        accounts.tick_array0,
        accounts.tick_array1,
        accounts.tick_array2,
        accounts.oracle,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn whirlpool_swap_v2_verify_account_privileges<'me, 'info>(
    accounts: WhirlpoolSwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    whirlpool_swap_v2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const INVARIANT_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct InvariantSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub tickmap: &'me AccountInfo<'info>,
    pub account_x: &'me AccountInfo<'info>,
    pub account_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InvariantSwapKeys {
    pub swap_program: Pubkey,
    pub state: Pubkey,
    pub pool: Pubkey,
    pub tickmap: Pubkey,
    pub account_x: Pubkey,
    pub account_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub owner: Pubkey,
    pub program_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<InvariantSwapAccounts<'_, '_>> for InvariantSwapKeys {
    fn from(accounts: InvariantSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            tickmap: *accounts.tickmap.key,
            account_x: *accounts.account_x.key,
            account_y: *accounts.account_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            owner: *accounts.owner.key,
            program_authority: *accounts.program_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InvariantSwapKeys> for [AccountMeta; INVARIANT_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: InvariantSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tickmap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.account_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INVARIANT_SWAP_IX_ACCOUNTS_LEN]> for InvariantSwapKeys {
    fn from(pubkeys: [Pubkey; INVARIANT_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            state: pubkeys[1],
            pool: pubkeys[2],
            tickmap: pubkeys[3],
            account_x: pubkeys[4],
            account_y: pubkeys[5],
            reserve_x: pubkeys[6],
            reserve_y: pubkeys[7],
            owner: pubkeys[8],
            program_authority: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<InvariantSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; INVARIANT_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InvariantSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.tickmap.clone(),
            accounts.account_x.clone(),
            accounts.account_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.owner.clone(),
            accounts.program_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INVARIANT_SWAP_IX_ACCOUNTS_LEN]>
    for InvariantSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INVARIANT_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            state: &arr[1],
            pool: &arr[2],
            tickmap: &arr[3],
            account_x: &arr[4],
            account_y: &arr[5],
            reserve_x: &arr[6],
            reserve_y: &arr[7],
            owner: &arr[8],
            program_authority: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const INVARIANT_SWAP_IX_DISCM: [u8; 8] = [187, 193, 40, 121, 47, 73, 144, 177];
#[derive(Clone, Debug, PartialEq)]
pub struct InvariantSwapIxData;
impl InvariantSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INVARIANT_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INVARIANT_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INVARIANT_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn invariant_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: InvariantSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INVARIANT_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InvariantSwapIxData.try_to_vec()?,
    })
}
pub fn invariant_swap_ix(keys: InvariantSwapKeys) -> std::io::Result<Instruction> {
    invariant_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn invariant_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InvariantSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InvariantSwapKeys = accounts.into();
    let ix = invariant_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn invariant_swap_invoke(accounts: InvariantSwapAccounts<'_, '_>) -> ProgramResult {
    invariant_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn invariant_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InvariantSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InvariantSwapKeys = accounts.into();
    let ix = invariant_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn invariant_swap_invoke_signed(
    accounts: InvariantSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    invariant_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn invariant_swap_verify_account_keys(
    accounts: InvariantSwapAccounts<'_, '_>,
    keys: InvariantSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.tickmap.key, keys.tickmap),
        (*accounts.account_x.key, keys.account_x),
        (*accounts.account_y.key, keys.account_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.owner.key, keys.owner),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn invariant_swap_verify_writable_privileges<'me, 'info>(
    accounts: InvariantSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.tickmap,
        accounts.account_x,
        accounts.account_y,
        accounts.reserve_x,
        accounts.reserve_y,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn invariant_swap_verify_account_privileges<'me, 'info>(
    accounts: InvariantSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    invariant_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const METEORA_SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct MeteoraSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub admin_token_fee: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MeteoraSwapKeys {
    pub swap_program: Pubkey,
    pub pool: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub admin_token_fee: Pubkey,
    pub user: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<MeteoraSwapAccounts<'_, '_>> for MeteoraSwapKeys {
    fn from(accounts: MeteoraSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            pool: *accounts.pool.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            admin_token_fee: *accounts.admin_token_fee.key,
            user: *accounts.user.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<MeteoraSwapKeys> for [AccountMeta; METEORA_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: MeteoraSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_token_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; METEORA_SWAP_IX_ACCOUNTS_LEN]> for MeteoraSwapKeys {
    fn from(pubkeys: [Pubkey; METEORA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            pool: pubkeys[1],
            user_source_token: pubkeys[2],
            user_destination_token: pubkeys[3],
            a_vault: pubkeys[4],
            b_vault: pubkeys[5],
            a_token_vault: pubkeys[6],
            b_token_vault: pubkeys[7],
            a_vault_lp_mint: pubkeys[8],
            b_vault_lp_mint: pubkeys[9],
            a_vault_lp: pubkeys[10],
            b_vault_lp: pubkeys[11],
            admin_token_fee: pubkeys[12],
            user: pubkeys[13],
            vault_program: pubkeys[14],
            token_program: pubkeys[15],
        }
    }
}
impl<'info> From<MeteoraSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; METEORA_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MeteoraSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.pool.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.admin_token_fee.clone(),
            accounts.user.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; METEORA_SWAP_IX_ACCOUNTS_LEN]>
    for MeteoraSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; METEORA_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            pool: &arr[1],
            user_source_token: &arr[2],
            user_destination_token: &arr[3],
            a_vault: &arr[4],
            b_vault: &arr[5],
            a_token_vault: &arr[6],
            b_token_vault: &arr[7],
            a_vault_lp_mint: &arr[8],
            b_vault_lp_mint: &arr[9],
            a_vault_lp: &arr[10],
            b_vault_lp: &arr[11],
            admin_token_fee: &arr[12],
            user: &arr[13],
            vault_program: &arr[14],
            token_program: &arr[15],
        }
    }
}
pub const METEORA_SWAP_IX_DISCM: [u8; 8] = [127, 125, 226, 12, 81, 24, 204, 35];
#[derive(Clone, Debug, PartialEq)]
pub struct MeteoraSwapIxData;
impl MeteoraSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != METEORA_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    METEORA_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&METEORA_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn meteora_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: MeteoraSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; METEORA_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MeteoraSwapIxData.try_to_vec()?,
    })
}
pub fn meteora_swap_ix(keys: MeteoraSwapKeys) -> std::io::Result<Instruction> {
    meteora_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn meteora_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MeteoraSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MeteoraSwapKeys = accounts.into();
    let ix = meteora_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn meteora_swap_invoke(accounts: MeteoraSwapAccounts<'_, '_>) -> ProgramResult {
    meteora_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn meteora_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MeteoraSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MeteoraSwapKeys = accounts.into();
    let ix = meteora_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn meteora_swap_invoke_signed(
    accounts: MeteoraSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    meteora_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn meteora_swap_verify_account_keys(
    accounts: MeteoraSwapAccounts<'_, '_>,
    keys: MeteoraSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.pool.key, keys.pool),
        (*accounts.user_source_token.key, keys.user_source_token),
        (
            *accounts.user_destination_token.key,
            keys.user_destination_token,
        ),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.admin_token_fee.key, keys.admin_token_fee),
        (*accounts.user.key, keys.user),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn meteora_swap_verify_writable_privileges<'me, 'info>(
    accounts: MeteoraSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.admin_token_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn meteora_swap_verify_account_privileges<'me, 'info>(
    accounts: MeteoraSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    meteora_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const GOOSEFX_SWAP_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct GoosefxSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub controller: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub ssl_in: &'me AccountInfo<'info>,
    pub ssl_out: &'me AccountInfo<'info>,
    pub liability_vault_in: &'me AccountInfo<'info>,
    pub swapped_liability_vault_in: &'me AccountInfo<'info>,
    pub liability_vault_out: &'me AccountInfo<'info>,
    pub swapped_liability_vault_out: &'me AccountInfo<'info>,
    pub user_in_ata: &'me AccountInfo<'info>,
    pub user_out_ata: &'me AccountInfo<'info>,
    pub fee_collector_ata: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub fee_collector: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoosefxSwapKeys {
    pub swap_program: Pubkey,
    pub controller: Pubkey,
    pub pair: Pubkey,
    pub ssl_in: Pubkey,
    pub ssl_out: Pubkey,
    pub liability_vault_in: Pubkey,
    pub swapped_liability_vault_in: Pubkey,
    pub liability_vault_out: Pubkey,
    pub swapped_liability_vault_out: Pubkey,
    pub user_in_ata: Pubkey,
    pub user_out_ata: Pubkey,
    pub fee_collector_ata: Pubkey,
    pub user_wallet: Pubkey,
    pub fee_collector: Pubkey,
    pub token_program: Pubkey,
}
impl From<GoosefxSwapAccounts<'_, '_>> for GoosefxSwapKeys {
    fn from(accounts: GoosefxSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            controller: *accounts.controller.key,
            pair: *accounts.pair.key,
            ssl_in: *accounts.ssl_in.key,
            ssl_out: *accounts.ssl_out.key,
            liability_vault_in: *accounts.liability_vault_in.key,
            swapped_liability_vault_in: *accounts.swapped_liability_vault_in.key,
            liability_vault_out: *accounts.liability_vault_out.key,
            swapped_liability_vault_out: *accounts.swapped_liability_vault_out.key,
            user_in_ata: *accounts.user_in_ata.key,
            user_out_ata: *accounts.user_out_ata.key,
            fee_collector_ata: *accounts.fee_collector_ata.key,
            user_wallet: *accounts.user_wallet.key,
            fee_collector: *accounts.fee_collector.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<GoosefxSwapKeys> for [AccountMeta; GOOSEFX_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: GoosefxSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.controller,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liability_vault_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapped_liability_vault_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.liability_vault_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapped_liability_vault_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_in_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_out_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_collector_ata,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_collector,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GOOSEFX_SWAP_IX_ACCOUNTS_LEN]> for GoosefxSwapKeys {
    fn from(pubkeys: [Pubkey; GOOSEFX_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            controller: pubkeys[1],
            pair: pubkeys[2],
            ssl_in: pubkeys[3],
            ssl_out: pubkeys[4],
            liability_vault_in: pubkeys[5],
            swapped_liability_vault_in: pubkeys[6],
            liability_vault_out: pubkeys[7],
            swapped_liability_vault_out: pubkeys[8],
            user_in_ata: pubkeys[9],
            user_out_ata: pubkeys[10],
            fee_collector_ata: pubkeys[11],
            user_wallet: pubkeys[12],
            fee_collector: pubkeys[13],
            token_program: pubkeys[14],
        }
    }
}
impl<'info> From<GoosefxSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; GOOSEFX_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: GoosefxSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.controller.clone(),
            accounts.pair.clone(),
            accounts.ssl_in.clone(),
            accounts.ssl_out.clone(),
            accounts.liability_vault_in.clone(),
            accounts.swapped_liability_vault_in.clone(),
            accounts.liability_vault_out.clone(),
            accounts.swapped_liability_vault_out.clone(),
            accounts.user_in_ata.clone(),
            accounts.user_out_ata.clone(),
            accounts.fee_collector_ata.clone(),
            accounts.user_wallet.clone(),
            accounts.fee_collector.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GOOSEFX_SWAP_IX_ACCOUNTS_LEN]>
    for GoosefxSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; GOOSEFX_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            controller: &arr[1],
            pair: &arr[2],
            ssl_in: &arr[3],
            ssl_out: &arr[4],
            liability_vault_in: &arr[5],
            swapped_liability_vault_in: &arr[6],
            liability_vault_out: &arr[7],
            swapped_liability_vault_out: &arr[8],
            user_in_ata: &arr[9],
            user_out_ata: &arr[10],
            fee_collector_ata: &arr[11],
            user_wallet: &arr[12],
            fee_collector: &arr[13],
            token_program: &arr[14],
        }
    }
}
pub const GOOSEFX_SWAP_IX_DISCM: [u8; 8] = [222, 136, 46, 123, 189, 125, 124, 122];
#[derive(Clone, Debug, PartialEq)]
pub struct GoosefxSwapIxData;
impl GoosefxSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GOOSEFX_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    GOOSEFX_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GOOSEFX_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn goosefx_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: GoosefxSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GOOSEFX_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GoosefxSwapIxData.try_to_vec()?,
    })
}
pub fn goosefx_swap_ix(keys: GoosefxSwapKeys) -> std::io::Result<Instruction> {
    goosefx_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn goosefx_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GoosefxSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GoosefxSwapKeys = accounts.into();
    let ix = goosefx_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn goosefx_swap_invoke(accounts: GoosefxSwapAccounts<'_, '_>) -> ProgramResult {
    goosefx_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn goosefx_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GoosefxSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GoosefxSwapKeys = accounts.into();
    let ix = goosefx_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn goosefx_swap_invoke_signed(
    accounts: GoosefxSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    goosefx_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn goosefx_swap_verify_account_keys(
    accounts: GoosefxSwapAccounts<'_, '_>,
    keys: GoosefxSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.controller.key, keys.controller),
        (*accounts.pair.key, keys.pair),
        (*accounts.ssl_in.key, keys.ssl_in),
        (*accounts.ssl_out.key, keys.ssl_out),
        (*accounts.liability_vault_in.key, keys.liability_vault_in),
        (
            *accounts.swapped_liability_vault_in.key,
            keys.swapped_liability_vault_in,
        ),
        (*accounts.liability_vault_out.key, keys.liability_vault_out),
        (
            *accounts.swapped_liability_vault_out.key,
            keys.swapped_liability_vault_out,
        ),
        (*accounts.user_in_ata.key, keys.user_in_ata),
        (*accounts.user_out_ata.key, keys.user_out_ata),
        (*accounts.fee_collector_ata.key, keys.fee_collector_ata),
        (*accounts.user_wallet.key, keys.user_wallet),
        (*accounts.fee_collector.key, keys.fee_collector),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn goosefx_swap_verify_writable_privileges<'me, 'info>(
    accounts: GoosefxSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.ssl_in,
        accounts.ssl_out,
        accounts.liability_vault_in,
        accounts.swapped_liability_vault_in,
        accounts.liability_vault_out,
        accounts.swapped_liability_vault_out,
        accounts.user_in_ata,
        accounts.user_out_ata,
        accounts.fee_collector_ata,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn goosefx_swap_verify_account_privileges<'me, 'info>(
    accounts: GoosefxSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    goosefx_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const DELTAFI_SWAP_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct DeltafiSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub market_config: &'me AccountInfo<'info>,
    pub swap_info: &'me AccountInfo<'info>,
    pub user_source_token: &'me AccountInfo<'info>,
    pub user_destination_token: &'me AccountInfo<'info>,
    pub swap_source_token: &'me AccountInfo<'info>,
    pub swap_destination_token: &'me AccountInfo<'info>,
    pub deltafi_user: &'me AccountInfo<'info>,
    pub admin_destination_token: &'me AccountInfo<'info>,
    pub pyth_price_base: &'me AccountInfo<'info>,
    pub pyth_price_quote: &'me AccountInfo<'info>,
    pub user_authority: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeltafiSwapKeys {
    pub swap_program: Pubkey,
    pub market_config: Pubkey,
    pub swap_info: Pubkey,
    pub user_source_token: Pubkey,
    pub user_destination_token: Pubkey,
    pub swap_source_token: Pubkey,
    pub swap_destination_token: Pubkey,
    pub deltafi_user: Pubkey,
    pub admin_destination_token: Pubkey,
    pub pyth_price_base: Pubkey,
    pub pyth_price_quote: Pubkey,
    pub user_authority: Pubkey,
    pub token_program: Pubkey,
}
impl From<DeltafiSwapAccounts<'_, '_>> for DeltafiSwapKeys {
    fn from(accounts: DeltafiSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            market_config: *accounts.market_config.key,
            swap_info: *accounts.swap_info.key,
            user_source_token: *accounts.user_source_token.key,
            user_destination_token: *accounts.user_destination_token.key,
            swap_source_token: *accounts.swap_source_token.key,
            swap_destination_token: *accounts.swap_destination_token.key,
            deltafi_user: *accounts.deltafi_user.key,
            admin_destination_token: *accounts.admin_destination_token.key,
            pyth_price_base: *accounts.pyth_price_base.key,
            pyth_price_quote: *accounts.pyth_price_quote.key,
            user_authority: *accounts.user_authority.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<DeltafiSwapKeys> for [AccountMeta; DELTAFI_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: DeltafiSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.swap_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.deltafi_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin_destination_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pyth_price_base,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pyth_price_quote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DELTAFI_SWAP_IX_ACCOUNTS_LEN]> for DeltafiSwapKeys {
    fn from(pubkeys: [Pubkey; DELTAFI_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            market_config: pubkeys[1],
            swap_info: pubkeys[2],
            user_source_token: pubkeys[3],
            user_destination_token: pubkeys[4],
            swap_source_token: pubkeys[5],
            swap_destination_token: pubkeys[6],
            deltafi_user: pubkeys[7],
            admin_destination_token: pubkeys[8],
            pyth_price_base: pubkeys[9],
            pyth_price_quote: pubkeys[10],
            user_authority: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<DeltafiSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; DELTAFI_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DeltafiSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.market_config.clone(),
            accounts.swap_info.clone(),
            accounts.user_source_token.clone(),
            accounts.user_destination_token.clone(),
            accounts.swap_source_token.clone(),
            accounts.swap_destination_token.clone(),
            accounts.deltafi_user.clone(),
            accounts.admin_destination_token.clone(),
            accounts.pyth_price_base.clone(),
            accounts.pyth_price_quote.clone(),
            accounts.user_authority.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELTAFI_SWAP_IX_ACCOUNTS_LEN]>
    for DeltafiSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DELTAFI_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            market_config: &arr[1],
            swap_info: &arr[2],
            user_source_token: &arr[3],
            user_destination_token: &arr[4],
            swap_source_token: &arr[5],
            swap_destination_token: &arr[6],
            deltafi_user: &arr[7],
            admin_destination_token: &arr[8],
            pyth_price_base: &arr[9],
            pyth_price_quote: &arr[10],
            user_authority: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const DELTAFI_SWAP_IX_DISCM: [u8; 8] = [132, 230, 102, 120, 205, 9, 237, 190];
#[derive(Clone, Debug, PartialEq)]
pub struct DeltafiSwapIxData;
impl DeltafiSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DELTAFI_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DELTAFI_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DELTAFI_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn deltafi_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: DeltafiSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DELTAFI_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DeltafiSwapIxData.try_to_vec()?,
    })
}
pub fn deltafi_swap_ix(keys: DeltafiSwapKeys) -> std::io::Result<Instruction> {
    deltafi_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn deltafi_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeltafiSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeltafiSwapKeys = accounts.into();
    let ix = deltafi_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn deltafi_swap_invoke(accounts: DeltafiSwapAccounts<'_, '_>) -> ProgramResult {
    deltafi_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn deltafi_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeltafiSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeltafiSwapKeys = accounts.into();
    let ix = deltafi_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deltafi_swap_invoke_signed(
    accounts: DeltafiSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deltafi_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn deltafi_swap_verify_account_keys(
    accounts: DeltafiSwapAccounts<'_, '_>,
    keys: DeltafiSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.market_config.key, keys.market_config),
        (*accounts.swap_info.key, keys.swap_info),
        (*accounts.user_source_token.key, keys.user_source_token),
        (
            *accounts.user_destination_token.key,
            keys.user_destination_token,
        ),
        (*accounts.swap_source_token.key, keys.swap_source_token),
        (
            *accounts.swap_destination_token.key,
            keys.swap_destination_token,
        ),
        (*accounts.deltafi_user.key, keys.deltafi_user),
        (
            *accounts.admin_destination_token.key,
            keys.admin_destination_token,
        ),
        (*accounts.pyth_price_base.key, keys.pyth_price_base),
        (*accounts.pyth_price_quote.key, keys.pyth_price_quote),
        (*accounts.user_authority.key, keys.user_authority),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn deltafi_swap_verify_writable_privileges<'me, 'info>(
    accounts: DeltafiSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.swap_info,
        accounts.user_source_token,
        accounts.user_destination_token,
        accounts.swap_source_token,
        accounts.swap_destination_token,
        accounts.deltafi_user,
        accounts.admin_destination_token,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deltafi_swap_verify_account_privileges<'me, 'info>(
    accounts: DeltafiSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deltafi_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const BALANSOL_SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct BalansolSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub tax_man: &'me AccountInfo<'info>,
    pub bid_mint: &'me AccountInfo<'info>,
    pub treasurer: &'me AccountInfo<'info>,
    pub src_treasury: &'me AccountInfo<'info>,
    pub src_associated_token_account: &'me AccountInfo<'info>,
    pub ask_mint: &'me AccountInfo<'info>,
    pub dst_treasury: &'me AccountInfo<'info>,
    pub dst_associated_token_account: &'me AccountInfo<'info>,
    pub dst_token_account_taxman: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BalansolSwapKeys {
    pub swap_program: Pubkey,
    pub authority: Pubkey,
    pub pool: Pubkey,
    pub tax_man: Pubkey,
    pub bid_mint: Pubkey,
    pub treasurer: Pubkey,
    pub src_treasury: Pubkey,
    pub src_associated_token_account: Pubkey,
    pub ask_mint: Pubkey,
    pub dst_treasury: Pubkey,
    pub dst_associated_token_account: Pubkey,
    pub dst_token_account_taxman: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<BalansolSwapAccounts<'_, '_>> for BalansolSwapKeys {
    fn from(accounts: BalansolSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            authority: *accounts.authority.key,
            pool: *accounts.pool.key,
            tax_man: *accounts.tax_man.key,
            bid_mint: *accounts.bid_mint.key,
            treasurer: *accounts.treasurer.key,
            src_treasury: *accounts.src_treasury.key,
            src_associated_token_account: *accounts.src_associated_token_account.key,
            ask_mint: *accounts.ask_mint.key,
            dst_treasury: *accounts.dst_treasury.key,
            dst_associated_token_account: *accounts.dst_associated_token_account.key,
            dst_token_account_taxman: *accounts.dst_token_account_taxman.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<BalansolSwapKeys> for [AccountMeta; BALANSOL_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: BalansolSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.tax_man,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bid_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasurer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.src_treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.src_associated_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ask_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dst_treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dst_associated_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dst_token_account_taxman,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; BALANSOL_SWAP_IX_ACCOUNTS_LEN]> for BalansolSwapKeys {
    fn from(pubkeys: [Pubkey; BALANSOL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            authority: pubkeys[1],
            pool: pubkeys[2],
            tax_man: pubkeys[3],
            bid_mint: pubkeys[4],
            treasurer: pubkeys[5],
            src_treasury: pubkeys[6],
            src_associated_token_account: pubkeys[7],
            ask_mint: pubkeys[8],
            dst_treasury: pubkeys[9],
            dst_associated_token_account: pubkeys[10],
            dst_token_account_taxman: pubkeys[11],
            system_program: pubkeys[12],
            token_program: pubkeys[13],
            associated_token_program: pubkeys[14],
            rent: pubkeys[15],
        }
    }
}
impl<'info> From<BalansolSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; BALANSOL_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: BalansolSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.authority.clone(),
            accounts.pool.clone(),
            accounts.tax_man.clone(),
            accounts.bid_mint.clone(),
            accounts.treasurer.clone(),
            accounts.src_treasury.clone(),
            accounts.src_associated_token_account.clone(),
            accounts.ask_mint.clone(),
            accounts.dst_treasury.clone(),
            accounts.dst_associated_token_account.clone(),
            accounts.dst_token_account_taxman.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; BALANSOL_SWAP_IX_ACCOUNTS_LEN]>
    for BalansolSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; BALANSOL_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            authority: &arr[1],
            pool: &arr[2],
            tax_man: &arr[3],
            bid_mint: &arr[4],
            treasurer: &arr[5],
            src_treasury: &arr[6],
            src_associated_token_account: &arr[7],
            ask_mint: &arr[8],
            dst_treasury: &arr[9],
            dst_associated_token_account: &arr[10],
            dst_token_account_taxman: &arr[11],
            system_program: &arr[12],
            token_program: &arr[13],
            associated_token_program: &arr[14],
            rent: &arr[15],
        }
    }
}
pub const BALANSOL_SWAP_IX_DISCM: [u8; 8] = [137, 109, 253, 253, 70, 109, 11, 100];
#[derive(Clone, Debug, PartialEq)]
pub struct BalansolSwapIxData;
impl BalansolSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != BALANSOL_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    BALANSOL_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&BALANSOL_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn balansol_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: BalansolSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; BALANSOL_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: BalansolSwapIxData.try_to_vec()?,
    })
}
pub fn balansol_swap_ix(keys: BalansolSwapKeys) -> std::io::Result<Instruction> {
    balansol_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn balansol_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: BalansolSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: BalansolSwapKeys = accounts.into();
    let ix = balansol_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn balansol_swap_invoke(accounts: BalansolSwapAccounts<'_, '_>) -> ProgramResult {
    balansol_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn balansol_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: BalansolSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: BalansolSwapKeys = accounts.into();
    let ix = balansol_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn balansol_swap_invoke_signed(
    accounts: BalansolSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    balansol_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn balansol_swap_verify_account_keys(
    accounts: BalansolSwapAccounts<'_, '_>,
    keys: BalansolSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.pool.key, keys.pool),
        (*accounts.tax_man.key, keys.tax_man),
        (*accounts.bid_mint.key, keys.bid_mint),
        (*accounts.treasurer.key, keys.treasurer),
        (*accounts.src_treasury.key, keys.src_treasury),
        (
            *accounts.src_associated_token_account.key,
            keys.src_associated_token_account,
        ),
        (*accounts.ask_mint.key, keys.ask_mint),
        (*accounts.dst_treasury.key, keys.dst_treasury),
        (
            *accounts.dst_associated_token_account.key,
            keys.dst_associated_token_account,
        ),
        (
            *accounts.dst_token_account_taxman.key,
            keys.dst_token_account_taxman,
        ),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn balansol_swap_verify_writable_privileges<'me, 'info>(
    accounts: BalansolSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.authority,
        accounts.pool,
        accounts.tax_man,
        accounts.src_treasury,
        accounts.src_associated_token_account,
        accounts.dst_treasury,
        accounts.dst_associated_token_account,
        accounts.dst_token_account_taxman,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn balansol_swap_verify_account_privileges<'me, 'info>(
    accounts: BalansolSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    balansol_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MARCO_POLO_SWAP_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct MarcoPoloSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub state: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub token_x: &'me AccountInfo<'info>,
    pub token_y: &'me AccountInfo<'info>,
    pub pool_x_account: &'me AccountInfo<'info>,
    pub pool_y_account: &'me AccountInfo<'info>,
    pub swapper_x_account: &'me AccountInfo<'info>,
    pub swapper_y_account: &'me AccountInfo<'info>,
    pub swapper: &'me AccountInfo<'info>,
    pub referrer_x_account: &'me AccountInfo<'info>,
    pub referrer_y_account: &'me AccountInfo<'info>,
    pub referrer: &'me AccountInfo<'info>,
    pub program_authority: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarcoPoloSwapKeys {
    pub swap_program: Pubkey,
    pub state: Pubkey,
    pub pool: Pubkey,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub swapper_x_account: Pubkey,
    pub swapper_y_account: Pubkey,
    pub swapper: Pubkey,
    pub referrer_x_account: Pubkey,
    pub referrer_y_account: Pubkey,
    pub referrer: Pubkey,
    pub program_authority: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub rent: Pubkey,
}
impl From<MarcoPoloSwapAccounts<'_, '_>> for MarcoPoloSwapKeys {
    fn from(accounts: MarcoPoloSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            state: *accounts.state.key,
            pool: *accounts.pool.key,
            token_x: *accounts.token_x.key,
            token_y: *accounts.token_y.key,
            pool_x_account: *accounts.pool_x_account.key,
            pool_y_account: *accounts.pool_y_account.key,
            swapper_x_account: *accounts.swapper_x_account.key,
            swapper_y_account: *accounts.swapper_y_account.key,
            swapper: *accounts.swapper.key,
            referrer_x_account: *accounts.referrer_x_account.key,
            referrer_y_account: *accounts.referrer_y_account.key,
            referrer: *accounts.referrer.key,
            program_authority: *accounts.program_authority.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<MarcoPoloSwapKeys> for [AccountMeta; MARCO_POLO_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: MarcoPoloSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapper_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapper_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swapper,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_x_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer_y_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referrer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.program_authority,
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
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MARCO_POLO_SWAP_IX_ACCOUNTS_LEN]> for MarcoPoloSwapKeys {
    fn from(pubkeys: [Pubkey; MARCO_POLO_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            state: pubkeys[1],
            pool: pubkeys[2],
            token_x: pubkeys[3],
            token_y: pubkeys[4],
            pool_x_account: pubkeys[5],
            pool_y_account: pubkeys[6],
            swapper_x_account: pubkeys[7],
            swapper_y_account: pubkeys[8],
            swapper: pubkeys[9],
            referrer_x_account: pubkeys[10],
            referrer_y_account: pubkeys[11],
            referrer: pubkeys[12],
            program_authority: pubkeys[13],
            system_program: pubkeys[14],
            token_program: pubkeys[15],
            associated_token_program: pubkeys[16],
            rent: pubkeys[17],
        }
    }
}
impl<'info> From<MarcoPoloSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; MARCO_POLO_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MarcoPoloSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.state.clone(),
            accounts.pool.clone(),
            accounts.token_x.clone(),
            accounts.token_y.clone(),
            accounts.pool_x_account.clone(),
            accounts.pool_y_account.clone(),
            accounts.swapper_x_account.clone(),
            accounts.swapper_y_account.clone(),
            accounts.swapper.clone(),
            accounts.referrer_x_account.clone(),
            accounts.referrer_y_account.clone(),
            accounts.referrer.clone(),
            accounts.program_authority.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MARCO_POLO_SWAP_IX_ACCOUNTS_LEN]>
    for MarcoPoloSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MARCO_POLO_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            state: &arr[1],
            pool: &arr[2],
            token_x: &arr[3],
            token_y: &arr[4],
            pool_x_account: &arr[5],
            pool_y_account: &arr[6],
            swapper_x_account: &arr[7],
            swapper_y_account: &arr[8],
            swapper: &arr[9],
            referrer_x_account: &arr[10],
            referrer_y_account: &arr[11],
            referrer: &arr[12],
            program_authority: &arr[13],
            system_program: &arr[14],
            token_program: &arr[15],
            associated_token_program: &arr[16],
            rent: &arr[17],
        }
    }
}
pub const MARCO_POLO_SWAP_IX_DISCM: [u8; 8] = [241, 147, 94, 15, 58, 108, 179, 68];
#[derive(Clone, Debug, PartialEq)]
pub struct MarcoPoloSwapIxData;
impl MarcoPoloSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MARCO_POLO_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MARCO_POLO_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MARCO_POLO_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn marco_polo_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: MarcoPoloSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MARCO_POLO_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MarcoPoloSwapIxData.try_to_vec()?,
    })
}
pub fn marco_polo_swap_ix(keys: MarcoPoloSwapKeys) -> std::io::Result<Instruction> {
    marco_polo_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn marco_polo_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MarcoPoloSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MarcoPoloSwapKeys = accounts.into();
    let ix = marco_polo_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn marco_polo_swap_invoke(accounts: MarcoPoloSwapAccounts<'_, '_>) -> ProgramResult {
    marco_polo_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn marco_polo_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MarcoPoloSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MarcoPoloSwapKeys = accounts.into();
    let ix = marco_polo_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn marco_polo_swap_invoke_signed(
    accounts: MarcoPoloSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    marco_polo_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn marco_polo_swap_verify_account_keys(
    accounts: MarcoPoloSwapAccounts<'_, '_>,
    keys: MarcoPoloSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.state.key, keys.state),
        (*accounts.pool.key, keys.pool),
        (*accounts.token_x.key, keys.token_x),
        (*accounts.token_y.key, keys.token_y),
        (*accounts.pool_x_account.key, keys.pool_x_account),
        (*accounts.pool_y_account.key, keys.pool_y_account),
        (*accounts.swapper_x_account.key, keys.swapper_x_account),
        (*accounts.swapper_y_account.key, keys.swapper_y_account),
        (*accounts.swapper.key, keys.swapper),
        (*accounts.referrer_x_account.key, keys.referrer_x_account),
        (*accounts.referrer_y_account.key, keys.referrer_y_account),
        (*accounts.referrer.key, keys.referrer),
        (*accounts.program_authority.key, keys.program_authority),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.rent.key, keys.rent),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn marco_polo_swap_verify_writable_privileges<'me, 'info>(
    accounts: MarcoPoloSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool,
        accounts.pool_x_account,
        accounts.pool_y_account,
        accounts.swapper_x_account,
        accounts.swapper_y_account,
        accounts.swapper,
        accounts.referrer_x_account,
        accounts.referrer_y_account,
        accounts.referrer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn marco_polo_swap_verify_account_privileges<'me, 'info>(
    accounts: MarcoPoloSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    marco_polo_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const DRADEX_SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct DradexSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub event_queue: &'me AccountInfo<'info>,
    pub dex_user: &'me AccountInfo<'info>,
    pub market_user: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub t0_vault: &'me AccountInfo<'info>,
    pub t1_vault: &'me AccountInfo<'info>,
    pub t0_user: &'me AccountInfo<'info>,
    pub t1_user: &'me AccountInfo<'info>,
    pub master: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub logger: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DradexSwapKeys {
    pub swap_program: Pubkey,
    pub pair: Pubkey,
    pub market: Pubkey,
    pub event_queue: Pubkey,
    pub dex_user: Pubkey,
    pub market_user: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub t0_vault: Pubkey,
    pub t1_vault: Pubkey,
    pub t0_user: Pubkey,
    pub t1_user: Pubkey,
    pub master: Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub logger: Pubkey,
}
impl From<DradexSwapAccounts<'_, '_>> for DradexSwapKeys {
    fn from(accounts: DradexSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            pair: *accounts.pair.key,
            market: *accounts.market.key,
            event_queue: *accounts.event_queue.key,
            dex_user: *accounts.dex_user.key,
            market_user: *accounts.market_user.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            t0_vault: *accounts.t0_vault.key,
            t1_vault: *accounts.t1_vault.key,
            t0_user: *accounts.t0_user.key,
            t1_user: *accounts.t1_user.key,
            master: *accounts.master.key,
            signer: *accounts.signer.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            logger: *accounts.logger.key,
        }
    }
}
impl From<DradexSwapKeys> for [AccountMeta; DRADEX_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: DradexSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_queue,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dex_user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t0_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.t1_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.master,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
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
                pubkey: keys.logger,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DRADEX_SWAP_IX_ACCOUNTS_LEN]> for DradexSwapKeys {
    fn from(pubkeys: [Pubkey; DRADEX_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            pair: pubkeys[1],
            market: pubkeys[2],
            event_queue: pubkeys[3],
            dex_user: pubkeys[4],
            market_user: pubkeys[5],
            bids: pubkeys[6],
            asks: pubkeys[7],
            t0_vault: pubkeys[8],
            t1_vault: pubkeys[9],
            t0_user: pubkeys[10],
            t1_user: pubkeys[11],
            master: pubkeys[12],
            signer: pubkeys[13],
            system_program: pubkeys[14],
            token_program: pubkeys[15],
            logger: pubkeys[16],
        }
    }
}
impl<'info> From<DradexSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; DRADEX_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DradexSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.pair.clone(),
            accounts.market.clone(),
            accounts.event_queue.clone(),
            accounts.dex_user.clone(),
            accounts.market_user.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.t0_vault.clone(),
            accounts.t1_vault.clone(),
            accounts.t0_user.clone(),
            accounts.t1_user.clone(),
            accounts.master.clone(),
            accounts.signer.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.logger.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DRADEX_SWAP_IX_ACCOUNTS_LEN]>
    for DradexSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DRADEX_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            pair: &arr[1],
            market: &arr[2],
            event_queue: &arr[3],
            dex_user: &arr[4],
            market_user: &arr[5],
            bids: &arr[6],
            asks: &arr[7],
            t0_vault: &arr[8],
            t1_vault: &arr[9],
            t0_user: &arr[10],
            t1_user: &arr[11],
            master: &arr[12],
            signer: &arr[13],
            system_program: &arr[14],
            token_program: &arr[15],
            logger: &arr[16],
        }
    }
}
pub const DRADEX_SWAP_IX_DISCM: [u8; 8] = [34, 146, 160, 38, 51, 85, 58, 151];
#[derive(Clone, Debug, PartialEq)]
pub struct DradexSwapIxData;
impl DradexSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DRADEX_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DRADEX_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DRADEX_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn dradex_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: DradexSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DRADEX_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DradexSwapIxData.try_to_vec()?,
    })
}
pub fn dradex_swap_ix(keys: DradexSwapKeys) -> std::io::Result<Instruction> {
    dradex_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn dradex_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DradexSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DradexSwapKeys = accounts.into();
    let ix = dradex_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn dradex_swap_invoke(accounts: DradexSwapAccounts<'_, '_>) -> ProgramResult {
    dradex_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn dradex_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DradexSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DradexSwapKeys = accounts.into();
    let ix = dradex_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn dradex_swap_invoke_signed(
    accounts: DradexSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    dradex_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn dradex_swap_verify_account_keys(
    accounts: DradexSwapAccounts<'_, '_>,
    keys: DradexSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.pair.key, keys.pair),
        (*accounts.market.key, keys.market),
        (*accounts.event_queue.key, keys.event_queue),
        (*accounts.dex_user.key, keys.dex_user),
        (*accounts.market_user.key, keys.market_user),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.t0_vault.key, keys.t0_vault),
        (*accounts.t1_vault.key, keys.t1_vault),
        (*accounts.t0_user.key, keys.t0_user),
        (*accounts.t1_user.key, keys.t1_user),
        (*accounts.master.key, keys.master),
        (*accounts.signer.key, keys.signer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.logger.key, keys.logger),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn dradex_swap_verify_writable_privileges<'me, 'info>(
    accounts: DradexSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.market,
        accounts.event_queue,
        accounts.market_user,
        accounts.bids,
        accounts.asks,
        accounts.t0_vault,
        accounts.t1_vault,
        accounts.t0_user,
        accounts.t1_user,
        accounts.signer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn dradex_swap_verify_account_privileges<'me, 'info>(
    accounts: DradexSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    dradex_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct LifinityV2SwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub amm: &'me AccountInfo<'info>,
    pub user_transfer_authority: &'me AccountInfo<'info>,
    pub source_info: &'me AccountInfo<'info>,
    pub destination_info: &'me AccountInfo<'info>,
    pub swap_source: &'me AccountInfo<'info>,
    pub swap_destination: &'me AccountInfo<'info>,
    pub pool_mint: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub oracle_main_account: &'me AccountInfo<'info>,
    pub oracle_sub_account: &'me AccountInfo<'info>,
    pub oracle_pc_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LifinityV2SwapKeys {
    pub swap_program: Pubkey,
    pub authority: Pubkey,
    pub amm: Pubkey,
    pub user_transfer_authority: Pubkey,
    pub source_info: Pubkey,
    pub destination_info: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub pool_mint: Pubkey,
    pub fee_account: Pubkey,
    pub token_program: Pubkey,
    pub oracle_main_account: Pubkey,
    pub oracle_sub_account: Pubkey,
    pub oracle_pc_account: Pubkey,
}
impl From<LifinityV2SwapAccounts<'_, '_>> for LifinityV2SwapKeys {
    fn from(accounts: LifinityV2SwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            authority: *accounts.authority.key,
            amm: *accounts.amm.key,
            user_transfer_authority: *accounts.user_transfer_authority.key,
            source_info: *accounts.source_info.key,
            destination_info: *accounts.destination_info.key,
            swap_source: *accounts.swap_source.key,
            swap_destination: *accounts.swap_destination.key,
            pool_mint: *accounts.pool_mint.key,
            fee_account: *accounts.fee_account.key,
            token_program: *accounts.token_program.key,
            oracle_main_account: *accounts.oracle_main_account.key,
            oracle_sub_account: *accounts.oracle_sub_account.key,
            oracle_pc_account: *accounts.oracle_pc_account.key,
        }
    }
}
impl From<LifinityV2SwapKeys> for [AccountMeta; LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: LifinityV2SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.source_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.destination_info,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_source,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_main_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_sub_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_pc_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN]> for LifinityV2SwapKeys {
    fn from(pubkeys: [Pubkey; LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            authority: pubkeys[1],
            amm: pubkeys[2],
            user_transfer_authority: pubkeys[3],
            source_info: pubkeys[4],
            destination_info: pubkeys[5],
            swap_source: pubkeys[6],
            swap_destination: pubkeys[7],
            pool_mint: pubkeys[8],
            fee_account: pubkeys[9],
            token_program: pubkeys[10],
            oracle_main_account: pubkeys[11],
            oracle_sub_account: pubkeys[12],
            oracle_pc_account: pubkeys[13],
        }
    }
}
impl<'info> From<LifinityV2SwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: LifinityV2SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.authority.clone(),
            accounts.amm.clone(),
            accounts.user_transfer_authority.clone(),
            accounts.source_info.clone(),
            accounts.destination_info.clone(),
            accounts.swap_source.clone(),
            accounts.swap_destination.clone(),
            accounts.pool_mint.clone(),
            accounts.fee_account.clone(),
            accounts.token_program.clone(),
            accounts.oracle_main_account.clone(),
            accounts.oracle_sub_account.clone(),
            accounts.oracle_pc_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN]>
    for LifinityV2SwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            authority: &arr[1],
            amm: &arr[2],
            user_transfer_authority: &arr[3],
            source_info: &arr[4],
            destination_info: &arr[5],
            swap_source: &arr[6],
            swap_destination: &arr[7],
            pool_mint: &arr[8],
            fee_account: &arr[9],
            token_program: &arr[10],
            oracle_main_account: &arr[11],
            oracle_sub_account: &arr[12],
            oracle_pc_account: &arr[13],
        }
    }
}
pub const LIFINITY_V2_SWAP_IX_DISCM: [u8; 8] = [19, 152, 195, 245, 187, 144, 74, 227];
#[derive(Clone, Debug, PartialEq)]
pub struct LifinityV2SwapIxData;
impl LifinityV2SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != LIFINITY_V2_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIFINITY_V2_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&LIFINITY_V2_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn lifinity_v2_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: LifinityV2SwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; LIFINITY_V2_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: LifinityV2SwapIxData.try_to_vec()?,
    })
}
pub fn lifinity_v2_swap_ix(keys: LifinityV2SwapKeys) -> std::io::Result<Instruction> {
    lifinity_v2_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn lifinity_v2_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: LifinityV2SwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: LifinityV2SwapKeys = accounts.into();
    let ix = lifinity_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn lifinity_v2_swap_invoke(accounts: LifinityV2SwapAccounts<'_, '_>) -> ProgramResult {
    lifinity_v2_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn lifinity_v2_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: LifinityV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: LifinityV2SwapKeys = accounts.into();
    let ix = lifinity_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn lifinity_v2_swap_invoke_signed(
    accounts: LifinityV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    lifinity_v2_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn lifinity_v2_swap_verify_account_keys(
    accounts: LifinityV2SwapAccounts<'_, '_>,
    keys: LifinityV2SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.authority.key, keys.authority),
        (*accounts.amm.key, keys.amm),
        (
            *accounts.user_transfer_authority.key,
            keys.user_transfer_authority,
        ),
        (*accounts.source_info.key, keys.source_info),
        (*accounts.destination_info.key, keys.destination_info),
        (*accounts.swap_source.key, keys.swap_source),
        (*accounts.swap_destination.key, keys.swap_destination),
        (*accounts.pool_mint.key, keys.pool_mint),
        (*accounts.fee_account.key, keys.fee_account),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.oracle_main_account.key, keys.oracle_main_account),
        (*accounts.oracle_sub_account.key, keys.oracle_sub_account),
        (*accounts.oracle_pc_account.key, keys.oracle_pc_account),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn lifinity_v2_swap_verify_writable_privileges<'me, 'info>(
    accounts: LifinityV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.amm,
        accounts.source_info,
        accounts.destination_info,
        accounts.swap_source,
        accounts.swap_destination,
        accounts.pool_mint,
        accounts.fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn lifinity_v2_swap_verify_account_privileges<'me, 'info>(
    accounts: LifinityV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    lifinity_v2_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct RaydiumClmmSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub tick_array: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RaydiumClmmSwapKeys {
    pub swap_program: Pubkey,
    pub payer: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub observation_state: Pubkey,
    pub token_program: Pubkey,
    pub tick_array: Pubkey,
}
impl From<RaydiumClmmSwapAccounts<'_, '_>> for RaydiumClmmSwapKeys {
    fn from(accounts: RaydiumClmmSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            payer: *accounts.payer.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            observation_state: *accounts.observation_state.key,
            token_program: *accounts.token_program.key,
            tick_array: *accounts.tick_array.key,
        }
    }
}
impl From<RaydiumClmmSwapKeys> for [AccountMeta; RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: RaydiumClmmSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.tick_array,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN]> for RaydiumClmmSwapKeys {
    fn from(pubkeys: [Pubkey; RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            payer: pubkeys[1],
            amm_config: pubkeys[2],
            pool_state: pubkeys[3],
            input_token_account: pubkeys[4],
            output_token_account: pubkeys[5],
            input_vault: pubkeys[6],
            output_vault: pubkeys[7],
            observation_state: pubkeys[8],
            token_program: pubkeys[9],
            tick_array: pubkeys[10],
        }
    }
}
impl<'info> From<RaydiumClmmSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RaydiumClmmSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.payer.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.observation_state.clone(),
            accounts.token_program.clone(),
            accounts.tick_array.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN]>
    for RaydiumClmmSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            payer: &arr[1],
            amm_config: &arr[2],
            pool_state: &arr[3],
            input_token_account: &arr[4],
            output_token_account: &arr[5],
            input_vault: &arr[6],
            output_vault: &arr[7],
            observation_state: &arr[8],
            token_program: &arr[9],
            tick_array: &arr[10],
        }
    }
}
pub const RAYDIUM_CLMM_SWAP_IX_DISCM: [u8; 8] = [47, 184, 213, 193, 35, 210, 87, 4];
#[derive(Clone, Debug, PartialEq)]
pub struct RaydiumClmmSwapIxData;
impl RaydiumClmmSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RAYDIUM_CLMM_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RAYDIUM_CLMM_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RAYDIUM_CLMM_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn raydium_clmm_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: RaydiumClmmSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RAYDIUM_CLMM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RaydiumClmmSwapIxData.try_to_vec()?,
    })
}
pub fn raydium_clmm_swap_ix(keys: RaydiumClmmSwapKeys) -> std::io::Result<Instruction> {
    raydium_clmm_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn raydium_clmm_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumClmmSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RaydiumClmmSwapKeys = accounts.into();
    let ix = raydium_clmm_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn raydium_clmm_swap_invoke(accounts: RaydiumClmmSwapAccounts<'_, '_>) -> ProgramResult {
    raydium_clmm_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn raydium_clmm_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumClmmSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RaydiumClmmSwapKeys = accounts.into();
    let ix = raydium_clmm_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn raydium_clmm_swap_invoke_signed(
    accounts: RaydiumClmmSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    raydium_clmm_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn raydium_clmm_swap_verify_account_keys(
    accounts: RaydiumClmmSwapAccounts<'_, '_>,
    keys: RaydiumClmmSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.payer.key, keys.payer),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (
            *accounts.output_token_account.key,
            keys.output_token_account,
        ),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.tick_array.key, keys.tick_array),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn raydium_clmm_swap_verify_writable_privileges<'me, 'info>(
    accounts: RaydiumClmmSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.observation_state,
        accounts.tick_array,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn raydium_clmm_swap_verify_account_privileges<'me, 'info>(
    accounts: RaydiumClmmSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    raydium_clmm_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct RaydiumClmmSwapV2Accounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub token_program2022: &'me AccountInfo<'info>,
    pub memo_program: &'me AccountInfo<'info>,
    pub input_vault_mint: &'me AccountInfo<'info>,
    pub output_vault_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RaydiumClmmSwapV2Keys {
    pub swap_program: Pubkey,
    pub payer: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub observation_state: Pubkey,
    pub token_program: Pubkey,
    pub token_program2022: Pubkey,
    pub memo_program: Pubkey,
    pub input_vault_mint: Pubkey,
    pub output_vault_mint: Pubkey,
}
impl From<RaydiumClmmSwapV2Accounts<'_, '_>> for RaydiumClmmSwapV2Keys {
    fn from(accounts: RaydiumClmmSwapV2Accounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            payer: *accounts.payer.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            observation_state: *accounts.observation_state.key,
            token_program: *accounts.token_program.key,
            token_program2022: *accounts.token_program2022.key,
            memo_program: *accounts.memo_program.key,
            input_vault_mint: *accounts.input_vault_mint.key,
            output_vault_mint: *accounts.output_vault_mint.key,
        }
    }
}
impl From<RaydiumClmmSwapV2Keys> for [AccountMeta; RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN] {
    fn from(keys: RaydiumClmmSwapV2Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program2022,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.memo_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_vault_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_vault_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN]> for RaydiumClmmSwapV2Keys {
    fn from(pubkeys: [Pubkey; RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            payer: pubkeys[1],
            amm_config: pubkeys[2],
            pool_state: pubkeys[3],
            input_token_account: pubkeys[4],
            output_token_account: pubkeys[5],
            input_vault: pubkeys[6],
            output_vault: pubkeys[7],
            observation_state: pubkeys[8],
            token_program: pubkeys[9],
            token_program2022: pubkeys[10],
            memo_program: pubkeys[11],
            input_vault_mint: pubkeys[12],
            output_vault_mint: pubkeys[13],
        }
    }
}
impl<'info> From<RaydiumClmmSwapV2Accounts<'_, 'info>>
    for [AccountInfo<'info>; RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RaydiumClmmSwapV2Accounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.payer.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.observation_state.clone(),
            accounts.token_program.clone(),
            accounts.token_program2022.clone(),
            accounts.memo_program.clone(),
            accounts.input_vault_mint.clone(),
            accounts.output_vault_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN]>
    for RaydiumClmmSwapV2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            payer: &arr[1],
            amm_config: &arr[2],
            pool_state: &arr[3],
            input_token_account: &arr[4],
            output_token_account: &arr[5],
            input_vault: &arr[6],
            output_vault: &arr[7],
            observation_state: &arr[8],
            token_program: &arr[9],
            token_program2022: &arr[10],
            memo_program: &arr[11],
            input_vault_mint: &arr[12],
            output_vault_mint: &arr[13],
        }
    }
}
pub const RAYDIUM_CLMM_SWAP_V2_IX_DISCM: [u8; 8] = [86, 108, 246, 93, 88, 47, 114, 90];
#[derive(Clone, Debug, PartialEq)]
pub struct RaydiumClmmSwapV2IxData;
impl RaydiumClmmSwapV2IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RAYDIUM_CLMM_SWAP_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RAYDIUM_CLMM_SWAP_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RAYDIUM_CLMM_SWAP_V2_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn raydium_clmm_swap_v2_ix_with_program_id(
    program_id: Pubkey,
    keys: RaydiumClmmSwapV2Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RAYDIUM_CLMM_SWAP_V2_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RaydiumClmmSwapV2IxData.try_to_vec()?,
    })
}
pub fn raydium_clmm_swap_v2_ix(keys: RaydiumClmmSwapV2Keys) -> std::io::Result<Instruction> {
    raydium_clmm_swap_v2_ix_with_program_id(JUPITER_ID, keys)
}
pub fn raydium_clmm_swap_v2_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumClmmSwapV2Accounts<'_, '_>,
) -> ProgramResult {
    let keys: RaydiumClmmSwapV2Keys = accounts.into();
    let ix = raydium_clmm_swap_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn raydium_clmm_swap_v2_invoke(accounts: RaydiumClmmSwapV2Accounts<'_, '_>) -> ProgramResult {
    raydium_clmm_swap_v2_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn raydium_clmm_swap_v2_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumClmmSwapV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RaydiumClmmSwapV2Keys = accounts.into();
    let ix = raydium_clmm_swap_v2_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn raydium_clmm_swap_v2_invoke_signed(
    accounts: RaydiumClmmSwapV2Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    raydium_clmm_swap_v2_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn raydium_clmm_swap_v2_verify_account_keys(
    accounts: RaydiumClmmSwapV2Accounts<'_, '_>,
    keys: RaydiumClmmSwapV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.payer.key, keys.payer),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (
            *accounts.output_token_account.key,
            keys.output_token_account,
        ),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.observation_state.key, keys.observation_state),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.token_program2022.key, keys.token_program2022),
        (*accounts.memo_program.key, keys.memo_program),
        (*accounts.input_vault_mint.key, keys.input_vault_mint),
        (*accounts.output_vault_mint.key, keys.output_vault_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn raydium_clmm_swap_v2_verify_writable_privileges<'me, 'info>(
    accounts: RaydiumClmmSwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn raydium_clmm_swap_v2_verify_account_privileges<'me, 'info>(
    accounts: RaydiumClmmSwapV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    raydium_clmm_swap_v2_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PHOENIX_SWAP_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct PhoenixSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub log_authority: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub trader: &'me AccountInfo<'info>,
    pub base_account: &'me AccountInfo<'info>,
    pub quote_account: &'me AccountInfo<'info>,
    pub base_vault: &'me AccountInfo<'info>,
    pub quote_vault: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PhoenixSwapKeys {
    pub swap_program: Pubkey,
    pub log_authority: Pubkey,
    pub market: Pubkey,
    pub trader: Pubkey,
    pub base_account: Pubkey,
    pub quote_account: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub token_program: Pubkey,
}
impl From<PhoenixSwapAccounts<'_, '_>> for PhoenixSwapKeys {
    fn from(accounts: PhoenixSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            log_authority: *accounts.log_authority.key,
            market: *accounts.market.key,
            trader: *accounts.trader.key,
            base_account: *accounts.base_account.key,
            quote_account: *accounts.quote_account.key,
            base_vault: *accounts.base_vault.key,
            quote_vault: *accounts.quote_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<PhoenixSwapKeys> for [AccountMeta; PHOENIX_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: PhoenixSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.log_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.trader,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PHOENIX_SWAP_IX_ACCOUNTS_LEN]> for PhoenixSwapKeys {
    fn from(pubkeys: [Pubkey; PHOENIX_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            log_authority: pubkeys[1],
            market: pubkeys[2],
            trader: pubkeys[3],
            base_account: pubkeys[4],
            quote_account: pubkeys[5],
            base_vault: pubkeys[6],
            quote_vault: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<PhoenixSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; PHOENIX_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PhoenixSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.log_authority.clone(),
            accounts.market.clone(),
            accounts.trader.clone(),
            accounts.base_account.clone(),
            accounts.quote_account.clone(),
            accounts.base_vault.clone(),
            accounts.quote_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PHOENIX_SWAP_IX_ACCOUNTS_LEN]>
    for PhoenixSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PHOENIX_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            log_authority: &arr[1],
            market: &arr[2],
            trader: &arr[3],
            base_account: &arr[4],
            quote_account: &arr[5],
            base_vault: &arr[6],
            quote_vault: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const PHOENIX_SWAP_IX_DISCM: [u8; 8] = [99, 66, 223, 95, 236, 131, 26, 140];
#[derive(Clone, Debug, PartialEq)]
pub struct PhoenixSwapIxData;
impl PhoenixSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PHOENIX_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PHOENIX_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PHOENIX_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn phoenix_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: PhoenixSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PHOENIX_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PhoenixSwapIxData.try_to_vec()?,
    })
}
pub fn phoenix_swap_ix(keys: PhoenixSwapKeys) -> std::io::Result<Instruction> {
    phoenix_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn phoenix_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PhoenixSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PhoenixSwapKeys = accounts.into();
    let ix = phoenix_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn phoenix_swap_invoke(accounts: PhoenixSwapAccounts<'_, '_>) -> ProgramResult {
    phoenix_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn phoenix_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PhoenixSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PhoenixSwapKeys = accounts.into();
    let ix = phoenix_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn phoenix_swap_invoke_signed(
    accounts: PhoenixSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    phoenix_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn phoenix_swap_verify_account_keys(
    accounts: PhoenixSwapAccounts<'_, '_>,
    keys: PhoenixSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.log_authority.key, keys.log_authority),
        (*accounts.market.key, keys.market),
        (*accounts.trader.key, keys.trader),
        (*accounts.base_account.key, keys.base_account),
        (*accounts.quote_account.key, keys.quote_account),
        (*accounts.base_vault.key, keys.base_vault),
        (*accounts.quote_vault.key, keys.quote_vault),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn phoenix_swap_verify_writable_privileges<'me, 'info>(
    accounts: PhoenixSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.market,
        accounts.base_account,
        accounts.quote_account,
        accounts.base_vault,
        accounts.quote_vault,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn phoenix_swap_verify_account_privileges<'me, 'info>(
    accounts: PhoenixSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    phoenix_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SYMMETRY_SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct SymmetrySwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub buyer: &'me AccountInfo<'info>,
    pub fund_state: &'me AccountInfo<'info>,
    pub pda_account: &'me AccountInfo<'info>,
    pub pda_from_token_account: &'me AccountInfo<'info>,
    pub buyer_from_token_account: &'me AccountInfo<'info>,
    pub pda_to_token_account: &'me AccountInfo<'info>,
    pub buyer_to_token_account: &'me AccountInfo<'info>,
    pub swap_fee_account: &'me AccountInfo<'info>,
    pub host_fee_account: &'me AccountInfo<'info>,
    pub manager_fee_account: &'me AccountInfo<'info>,
    pub token_list: &'me AccountInfo<'info>,
    pub prism_data: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SymmetrySwapKeys {
    pub swap_program: Pubkey,
    pub buyer: Pubkey,
    pub fund_state: Pubkey,
    pub pda_account: Pubkey,
    pub pda_from_token_account: Pubkey,
    pub buyer_from_token_account: Pubkey,
    pub pda_to_token_account: Pubkey,
    pub buyer_to_token_account: Pubkey,
    pub swap_fee_account: Pubkey,
    pub host_fee_account: Pubkey,
    pub manager_fee_account: Pubkey,
    pub token_list: Pubkey,
    pub prism_data: Pubkey,
    pub token_program: Pubkey,
}
impl From<SymmetrySwapAccounts<'_, '_>> for SymmetrySwapKeys {
    fn from(accounts: SymmetrySwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            buyer: *accounts.buyer.key,
            fund_state: *accounts.fund_state.key,
            pda_account: *accounts.pda_account.key,
            pda_from_token_account: *accounts.pda_from_token_account.key,
            buyer_from_token_account: *accounts.buyer_from_token_account.key,
            pda_to_token_account: *accounts.pda_to_token_account.key,
            buyer_to_token_account: *accounts.buyer_to_token_account.key,
            swap_fee_account: *accounts.swap_fee_account.key,
            host_fee_account: *accounts.host_fee_account.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            token_list: *accounts.token_list.key,
            prism_data: *accounts.prism_data.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<SymmetrySwapKeys> for [AccountMeta; SYMMETRY_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: SymmetrySwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.buyer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fund_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pda_from_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_from_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pda_to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.buyer_to_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.swap_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_list,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.prism_data,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SYMMETRY_SWAP_IX_ACCOUNTS_LEN]> for SymmetrySwapKeys {
    fn from(pubkeys: [Pubkey; SYMMETRY_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            buyer: pubkeys[1],
            fund_state: pubkeys[2],
            pda_account: pubkeys[3],
            pda_from_token_account: pubkeys[4],
            buyer_from_token_account: pubkeys[5],
            pda_to_token_account: pubkeys[6],
            buyer_to_token_account: pubkeys[7],
            swap_fee_account: pubkeys[8],
            host_fee_account: pubkeys[9],
            manager_fee_account: pubkeys[10],
            token_list: pubkeys[11],
            prism_data: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<SymmetrySwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; SYMMETRY_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SymmetrySwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.buyer.clone(),
            accounts.fund_state.clone(),
            accounts.pda_account.clone(),
            accounts.pda_from_token_account.clone(),
            accounts.buyer_from_token_account.clone(),
            accounts.pda_to_token_account.clone(),
            accounts.buyer_to_token_account.clone(),
            accounts.swap_fee_account.clone(),
            accounts.host_fee_account.clone(),
            accounts.manager_fee_account.clone(),
            accounts.token_list.clone(),
            accounts.prism_data.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SYMMETRY_SWAP_IX_ACCOUNTS_LEN]>
    for SymmetrySwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SYMMETRY_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            buyer: &arr[1],
            fund_state: &arr[2],
            pda_account: &arr[3],
            pda_from_token_account: &arr[4],
            buyer_from_token_account: &arr[5],
            pda_to_token_account: &arr[6],
            buyer_to_token_account: &arr[7],
            swap_fee_account: &arr[8],
            host_fee_account: &arr[9],
            manager_fee_account: &arr[10],
            token_list: &arr[11],
            prism_data: &arr[12],
            token_program: &arr[13],
        }
    }
}
pub const SYMMETRY_SWAP_IX_DISCM: [u8; 8] = [17, 114, 237, 234, 154, 12, 185, 116];
#[derive(Clone, Debug, PartialEq)]
pub struct SymmetrySwapIxData;
impl SymmetrySwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != SYMMETRY_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SYMMETRY_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&SYMMETRY_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn symmetry_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: SymmetrySwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SYMMETRY_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SymmetrySwapIxData.try_to_vec()?,
    })
}
pub fn symmetry_swap_ix(keys: SymmetrySwapKeys) -> std::io::Result<Instruction> {
    symmetry_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn symmetry_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SymmetrySwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SymmetrySwapKeys = accounts.into();
    let ix = symmetry_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn symmetry_swap_invoke(accounts: SymmetrySwapAccounts<'_, '_>) -> ProgramResult {
    symmetry_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn symmetry_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SymmetrySwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SymmetrySwapKeys = accounts.into();
    let ix = symmetry_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn symmetry_swap_invoke_signed(
    accounts: SymmetrySwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    symmetry_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn symmetry_swap_verify_account_keys(
    accounts: SymmetrySwapAccounts<'_, '_>,
    keys: SymmetrySwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.buyer.key, keys.buyer),
        (*accounts.fund_state.key, keys.fund_state),
        (*accounts.pda_account.key, keys.pda_account),
        (
            *accounts.pda_from_token_account.key,
            keys.pda_from_token_account,
        ),
        (
            *accounts.buyer_from_token_account.key,
            keys.buyer_from_token_account,
        ),
        (
            *accounts.pda_to_token_account.key,
            keys.pda_to_token_account,
        ),
        (
            *accounts.buyer_to_token_account.key,
            keys.buyer_to_token_account,
        ),
        (*accounts.swap_fee_account.key, keys.swap_fee_account),
        (*accounts.host_fee_account.key, keys.host_fee_account),
        (*accounts.manager_fee_account.key, keys.manager_fee_account),
        (*accounts.token_list.key, keys.token_list),
        (*accounts.prism_data.key, keys.prism_data),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn symmetry_swap_verify_writable_privileges<'me, 'info>(
    accounts: SymmetrySwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fund_state,
        accounts.pda_from_token_account,
        accounts.buyer_from_token_account,
        accounts.pda_to_token_account,
        accounts.buyer_to_token_account,
        accounts.swap_fee_account,
        accounts.host_fee_account,
        accounts.manager_fee_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn symmetry_swap_verify_account_privileges<'me, 'info>(
    accounts: SymmetrySwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    symmetry_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct HeliumTreasuryManagementRedeemV0Accounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub treasury_management: &'me AccountInfo<'info>,
    pub treasury_mint: &'me AccountInfo<'info>,
    pub supply_mint: &'me AccountInfo<'info>,
    pub treasury: &'me AccountInfo<'info>,
    pub circuit_breaker: &'me AccountInfo<'info>,
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub circuit_breaker_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HeliumTreasuryManagementRedeemV0Keys {
    pub swap_program: Pubkey,
    pub treasury_management: Pubkey,
    pub treasury_mint: Pubkey,
    pub supply_mint: Pubkey,
    pub treasury: Pubkey,
    pub circuit_breaker: Pubkey,
    pub from: Pubkey,
    pub to: Pubkey,
    pub owner: Pubkey,
    pub circuit_breaker_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<HeliumTreasuryManagementRedeemV0Accounts<'_, '_>>
    for HeliumTreasuryManagementRedeemV0Keys
{
    fn from(accounts: HeliumTreasuryManagementRedeemV0Accounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            treasury_management: *accounts.treasury_management.key,
            treasury_mint: *accounts.treasury_mint.key,
            supply_mint: *accounts.supply_mint.key,
            treasury: *accounts.treasury.key,
            circuit_breaker: *accounts.circuit_breaker.key,
            from: *accounts.from.key,
            to: *accounts.to.key,
            owner: *accounts.owner.key,
            circuit_breaker_program: *accounts.circuit_breaker_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<HeliumTreasuryManagementRedeemV0Keys>
    for [AccountMeta; HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN]
{
    fn from(keys: HeliumTreasuryManagementRedeemV0Keys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasury_management,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.treasury_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.supply_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.circuit_breaker,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.circuit_breaker_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN]>
    for HeliumTreasuryManagementRedeemV0Keys
{
    fn from(pubkeys: [Pubkey; HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            treasury_management: pubkeys[1],
            treasury_mint: pubkeys[2],
            supply_mint: pubkeys[3],
            treasury: pubkeys[4],
            circuit_breaker: pubkeys[5],
            from: pubkeys[6],
            to: pubkeys[7],
            owner: pubkeys[8],
            circuit_breaker_program: pubkeys[9],
            token_program: pubkeys[10],
        }
    }
}
impl<'info> From<HeliumTreasuryManagementRedeemV0Accounts<'_, 'info>>
    for [AccountInfo<'info>; HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN]
{
    fn from(accounts: HeliumTreasuryManagementRedeemV0Accounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.treasury_management.clone(),
            accounts.treasury_mint.clone(),
            accounts.supply_mint.clone(),
            accounts.treasury.clone(),
            accounts.circuit_breaker.clone(),
            accounts.from.clone(),
            accounts.to.clone(),
            accounts.owner.clone(),
            accounts.circuit_breaker_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN]>
    for HeliumTreasuryManagementRedeemV0Accounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            swap_program: &arr[0],
            treasury_management: &arr[1],
            treasury_mint: &arr[2],
            supply_mint: &arr[3],
            treasury: &arr[4],
            circuit_breaker: &arr[5],
            from: &arr[6],
            to: &arr[7],
            owner: &arr[8],
            circuit_breaker_program: &arr[9],
            token_program: &arr[10],
        }
    }
}
pub const HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_DISCM: [u8; 8] =
    [163, 159, 163, 25, 243, 161, 108, 74];
#[derive(Clone, Debug, PartialEq)]
pub struct HeliumTreasuryManagementRedeemV0IxData;
impl HeliumTreasuryManagementRedeemV0IxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn helium_treasury_management_redeem_v0_ix_with_program_id(
    program_id: Pubkey,
    keys: HeliumTreasuryManagementRedeemV0Keys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; HELIUM_TREASURY_MANAGEMENT_REDEEM_V0_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: HeliumTreasuryManagementRedeemV0IxData.try_to_vec()?,
    })
}
pub fn helium_treasury_management_redeem_v0_ix(
    keys: HeliumTreasuryManagementRedeemV0Keys,
) -> std::io::Result<Instruction> {
    helium_treasury_management_redeem_v0_ix_with_program_id(JUPITER_ID, keys)
}
pub fn helium_treasury_management_redeem_v0_invoke_with_program_id(
    program_id: Pubkey,
    accounts: HeliumTreasuryManagementRedeemV0Accounts<'_, '_>,
) -> ProgramResult {
    let keys: HeliumTreasuryManagementRedeemV0Keys = accounts.into();
    let ix = helium_treasury_management_redeem_v0_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn helium_treasury_management_redeem_v0_invoke(
    accounts: HeliumTreasuryManagementRedeemV0Accounts<'_, '_>,
) -> ProgramResult {
    helium_treasury_management_redeem_v0_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn helium_treasury_management_redeem_v0_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: HeliumTreasuryManagementRedeemV0Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: HeliumTreasuryManagementRedeemV0Keys = accounts.into();
    let ix = helium_treasury_management_redeem_v0_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn helium_treasury_management_redeem_v0_invoke_signed(
    accounts: HeliumTreasuryManagementRedeemV0Accounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    helium_treasury_management_redeem_v0_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn helium_treasury_management_redeem_v0_verify_account_keys(
    accounts: HeliumTreasuryManagementRedeemV0Accounts<'_, '_>,
    keys: HeliumTreasuryManagementRedeemV0Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.treasury_management.key, keys.treasury_management),
        (*accounts.treasury_mint.key, keys.treasury_mint),
        (*accounts.supply_mint.key, keys.supply_mint),
        (*accounts.treasury.key, keys.treasury),
        (*accounts.circuit_breaker.key, keys.circuit_breaker),
        (*accounts.from.key, keys.from),
        (*accounts.to.key, keys.to),
        (*accounts.owner.key, keys.owner),
        (
            *accounts.circuit_breaker_program.key,
            keys.circuit_breaker_program,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn helium_treasury_management_redeem_v0_verify_writable_privileges<'me, 'info>(
    accounts: HeliumTreasuryManagementRedeemV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.supply_mint,
        accounts.treasury,
        accounts.circuit_breaker,
        accounts.from,
        accounts.to,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn helium_treasury_management_redeem_v0_verify_account_privileges<'me, 'info>(
    accounts: HeliumTreasuryManagementRedeemV0Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    helium_treasury_management_redeem_v0_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct GoosefxV2SwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub pair: &'me AccountInfo<'info>,
    pub pool_registry: &'me AccountInfo<'info>,
    pub user_wallet: &'me AccountInfo<'info>,
    pub ssl_pool_in_signer: &'me AccountInfo<'info>,
    pub ssl_pool_out_signer: &'me AccountInfo<'info>,
    pub user_ata_in: &'me AccountInfo<'info>,
    pub user_ata_out: &'me AccountInfo<'info>,
    pub ssl_out_main_vault: &'me AccountInfo<'info>,
    pub ssl_out_secondary_vault: &'me AccountInfo<'info>,
    pub ssl_in_main_vault: &'me AccountInfo<'info>,
    pub ssl_in_secondary_vault: &'me AccountInfo<'info>,
    pub ssl_out_fee_vault: &'me AccountInfo<'info>,
    pub fee_destination: &'me AccountInfo<'info>,
    pub output_token_price_history: &'me AccountInfo<'info>,
    pub output_token_oracle: &'me AccountInfo<'info>,
    pub input_token_price_history: &'me AccountInfo<'info>,
    pub input_token_oracle: &'me AccountInfo<'info>,
    pub event_emitter: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GoosefxV2SwapKeys {
    pub swap_program: Pubkey,
    pub pair: Pubkey,
    pub pool_registry: Pubkey,
    pub user_wallet: Pubkey,
    pub ssl_pool_in_signer: Pubkey,
    pub ssl_pool_out_signer: Pubkey,
    pub user_ata_in: Pubkey,
    pub user_ata_out: Pubkey,
    pub ssl_out_main_vault: Pubkey,
    pub ssl_out_secondary_vault: Pubkey,
    pub ssl_in_main_vault: Pubkey,
    pub ssl_in_secondary_vault: Pubkey,
    pub ssl_out_fee_vault: Pubkey,
    pub fee_destination: Pubkey,
    pub output_token_price_history: Pubkey,
    pub output_token_oracle: Pubkey,
    pub input_token_price_history: Pubkey,
    pub input_token_oracle: Pubkey,
    pub event_emitter: Pubkey,
    pub token_program: Pubkey,
}
impl From<GoosefxV2SwapAccounts<'_, '_>> for GoosefxV2SwapKeys {
    fn from(accounts: GoosefxV2SwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            pair: *accounts.pair.key,
            pool_registry: *accounts.pool_registry.key,
            user_wallet: *accounts.user_wallet.key,
            ssl_pool_in_signer: *accounts.ssl_pool_in_signer.key,
            ssl_pool_out_signer: *accounts.ssl_pool_out_signer.key,
            user_ata_in: *accounts.user_ata_in.key,
            user_ata_out: *accounts.user_ata_out.key,
            ssl_out_main_vault: *accounts.ssl_out_main_vault.key,
            ssl_out_secondary_vault: *accounts.ssl_out_secondary_vault.key,
            ssl_in_main_vault: *accounts.ssl_in_main_vault.key,
            ssl_in_secondary_vault: *accounts.ssl_in_secondary_vault.key,
            ssl_out_fee_vault: *accounts.ssl_out_fee_vault.key,
            fee_destination: *accounts.fee_destination.key,
            output_token_price_history: *accounts.output_token_price_history.key,
            output_token_oracle: *accounts.output_token_oracle.key,
            input_token_price_history: *accounts.input_token_price_history.key,
            input_token_oracle: *accounts.input_token_oracle.key,
            event_emitter: *accounts.event_emitter.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<GoosefxV2SwapKeys> for [AccountMeta; GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: GoosefxV2SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_registry,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_wallet,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_in_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.ssl_pool_out_signer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_ata_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_ata_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out_main_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out_secondary_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_in_main_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_in_secondary_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ssl_out_fee_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.fee_destination,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_price_history,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_price_history,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_oracle,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_emitter,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN]> for GoosefxV2SwapKeys {
    fn from(pubkeys: [Pubkey; GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            pair: pubkeys[1],
            pool_registry: pubkeys[2],
            user_wallet: pubkeys[3],
            ssl_pool_in_signer: pubkeys[4],
            ssl_pool_out_signer: pubkeys[5],
            user_ata_in: pubkeys[6],
            user_ata_out: pubkeys[7],
            ssl_out_main_vault: pubkeys[8],
            ssl_out_secondary_vault: pubkeys[9],
            ssl_in_main_vault: pubkeys[10],
            ssl_in_secondary_vault: pubkeys[11],
            ssl_out_fee_vault: pubkeys[12],
            fee_destination: pubkeys[13],
            output_token_price_history: pubkeys[14],
            output_token_oracle: pubkeys[15],
            input_token_price_history: pubkeys[16],
            input_token_oracle: pubkeys[17],
            event_emitter: pubkeys[18],
            token_program: pubkeys[19],
        }
    }
}
impl<'info> From<GoosefxV2SwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: GoosefxV2SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.pair.clone(),
            accounts.pool_registry.clone(),
            accounts.user_wallet.clone(),
            accounts.ssl_pool_in_signer.clone(),
            accounts.ssl_pool_out_signer.clone(),
            accounts.user_ata_in.clone(),
            accounts.user_ata_out.clone(),
            accounts.ssl_out_main_vault.clone(),
            accounts.ssl_out_secondary_vault.clone(),
            accounts.ssl_in_main_vault.clone(),
            accounts.ssl_in_secondary_vault.clone(),
            accounts.ssl_out_fee_vault.clone(),
            accounts.fee_destination.clone(),
            accounts.output_token_price_history.clone(),
            accounts.output_token_oracle.clone(),
            accounts.input_token_price_history.clone(),
            accounts.input_token_oracle.clone(),
            accounts.event_emitter.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN]>
    for GoosefxV2SwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            pair: &arr[1],
            pool_registry: &arr[2],
            user_wallet: &arr[3],
            ssl_pool_in_signer: &arr[4],
            ssl_pool_out_signer: &arr[5],
            user_ata_in: &arr[6],
            user_ata_out: &arr[7],
            ssl_out_main_vault: &arr[8],
            ssl_out_secondary_vault: &arr[9],
            ssl_in_main_vault: &arr[10],
            ssl_in_secondary_vault: &arr[11],
            ssl_out_fee_vault: &arr[12],
            fee_destination: &arr[13],
            output_token_price_history: &arr[14],
            output_token_oracle: &arr[15],
            input_token_price_history: &arr[16],
            input_token_oracle: &arr[17],
            event_emitter: &arr[18],
            token_program: &arr[19],
        }
    }
}
pub const GOOSEFX_V2_SWAP_IX_DISCM: [u8; 8] = [178, 108, 208, 137, 154, 194, 168, 213];
#[derive(Clone, Debug, PartialEq)]
pub struct GoosefxV2SwapIxData;
impl GoosefxV2SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GOOSEFX_V2_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    GOOSEFX_V2_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GOOSEFX_V2_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn goosefx_v2_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: GoosefxV2SwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; GOOSEFX_V2_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: GoosefxV2SwapIxData.try_to_vec()?,
    })
}
pub fn goosefx_v2_swap_ix(keys: GoosefxV2SwapKeys) -> std::io::Result<Instruction> {
    goosefx_v2_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn goosefx_v2_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: GoosefxV2SwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: GoosefxV2SwapKeys = accounts.into();
    let ix = goosefx_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn goosefx_v2_swap_invoke(accounts: GoosefxV2SwapAccounts<'_, '_>) -> ProgramResult {
    goosefx_v2_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn goosefx_v2_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: GoosefxV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: GoosefxV2SwapKeys = accounts.into();
    let ix = goosefx_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn goosefx_v2_swap_invoke_signed(
    accounts: GoosefxV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    goosefx_v2_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn goosefx_v2_swap_verify_account_keys(
    accounts: GoosefxV2SwapAccounts<'_, '_>,
    keys: GoosefxV2SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.pair.key, keys.pair),
        (*accounts.pool_registry.key, keys.pool_registry),
        (*accounts.user_wallet.key, keys.user_wallet),
        (*accounts.ssl_pool_in_signer.key, keys.ssl_pool_in_signer),
        (*accounts.ssl_pool_out_signer.key, keys.ssl_pool_out_signer),
        (*accounts.user_ata_in.key, keys.user_ata_in),
        (*accounts.user_ata_out.key, keys.user_ata_out),
        (*accounts.ssl_out_main_vault.key, keys.ssl_out_main_vault),
        (
            *accounts.ssl_out_secondary_vault.key,
            keys.ssl_out_secondary_vault,
        ),
        (*accounts.ssl_in_main_vault.key, keys.ssl_in_main_vault),
        (
            *accounts.ssl_in_secondary_vault.key,
            keys.ssl_in_secondary_vault,
        ),
        (*accounts.ssl_out_fee_vault.key, keys.ssl_out_fee_vault),
        (*accounts.fee_destination.key, keys.fee_destination),
        (
            *accounts.output_token_price_history.key,
            keys.output_token_price_history,
        ),
        (*accounts.output_token_oracle.key, keys.output_token_oracle),
        (
            *accounts.input_token_price_history.key,
            keys.input_token_price_history,
        ),
        (*accounts.input_token_oracle.key, keys.input_token_oracle),
        (*accounts.event_emitter.key, keys.event_emitter),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn goosefx_v2_swap_verify_writable_privileges<'me, 'info>(
    accounts: GoosefxV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pair,
        accounts.pool_registry,
        accounts.user_ata_in,
        accounts.user_ata_out,
        accounts.ssl_out_main_vault,
        accounts.ssl_out_secondary_vault,
        accounts.ssl_in_main_vault,
        accounts.ssl_in_secondary_vault,
        accounts.ssl_out_fee_vault,
        accounts.fee_destination,
        accounts.output_token_price_history,
        accounts.input_token_price_history,
        accounts.event_emitter,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn goosefx_v2_swap_verify_account_privileges<'me, 'info>(
    accounts: GoosefxV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    goosefx_v2_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PERPS_SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct PerpsSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub receiving_custody: &'me AccountInfo<'info>,
    pub receiving_custody_oracle_account: &'me AccountInfo<'info>,
    pub receiving_custody_token_account: &'me AccountInfo<'info>,
    pub dispensing_custody: &'me AccountInfo<'info>,
    pub dispensing_custody_oracle_account: &'me AccountInfo<'info>,
    pub dispensing_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerpsSwapKeys {
    pub swap_program: Pubkey,
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub receiving_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub receiving_custody: Pubkey,
    pub receiving_custody_oracle_account: Pubkey,
    pub receiving_custody_token_account: Pubkey,
    pub dispensing_custody: Pubkey,
    pub dispensing_custody_oracle_account: Pubkey,
    pub dispensing_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PerpsSwapAccounts<'_, '_>> for PerpsSwapKeys {
    fn from(accounts: PerpsSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            receiving_account: *accounts.receiving_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            receiving_custody: *accounts.receiving_custody.key,
            receiving_custody_oracle_account: *accounts.receiving_custody_oracle_account.key,
            receiving_custody_token_account: *accounts.receiving_custody_token_account.key,
            dispensing_custody: *accounts.dispensing_custody.key,
            dispensing_custody_oracle_account: *accounts.dispensing_custody_oracle_account.key,
            dispensing_custody_token_account: *accounts.dispensing_custody_token_account.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PerpsSwapKeys> for [AccountMeta; PERPS_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: PerpsSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_oracle_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_oracle_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PERPS_SWAP_IX_ACCOUNTS_LEN]> for PerpsSwapKeys {
    fn from(pubkeys: [Pubkey; PERPS_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            owner: pubkeys[1],
            funding_account: pubkeys[2],
            receiving_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            receiving_custody: pubkeys[7],
            receiving_custody_oracle_account: pubkeys[8],
            receiving_custody_token_account: pubkeys[9],
            dispensing_custody: pubkeys[10],
            dispensing_custody_oracle_account: pubkeys[11],
            dispensing_custody_token_account: pubkeys[12],
            token_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<PerpsSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; PERPS_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PerpsSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.receiving_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.receiving_custody.clone(),
            accounts.receiving_custody_oracle_account.clone(),
            accounts.receiving_custody_token_account.clone(),
            accounts.dispensing_custody.clone(),
            accounts.dispensing_custody_oracle_account.clone(),
            accounts.dispensing_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PERPS_SWAP_IX_ACCOUNTS_LEN]>
    for PerpsSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PERPS_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            owner: &arr[1],
            funding_account: &arr[2],
            receiving_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            receiving_custody: &arr[7],
            receiving_custody_oracle_account: &arr[8],
            receiving_custody_token_account: &arr[9],
            dispensing_custody: &arr[10],
            dispensing_custody_oracle_account: &arr[11],
            dispensing_custody_token_account: &arr[12],
            token_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const PERPS_SWAP_IX_DISCM: [u8; 8] = [147, 22, 108, 178, 110, 18, 171, 34];
#[derive(Clone, Debug, PartialEq)]
pub struct PerpsSwapIxData;
impl PerpsSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERPS_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERPS_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERPS_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn perps_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: PerpsSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERPS_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PerpsSwapIxData.try_to_vec()?,
    })
}
pub fn perps_swap_ix(keys: PerpsSwapKeys) -> std::io::Result<Instruction> {
    perps_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn perps_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PerpsSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PerpsSwapKeys = accounts.into();
    let ix = perps_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn perps_swap_invoke(accounts: PerpsSwapAccounts<'_, '_>) -> ProgramResult {
    perps_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn perps_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PerpsSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PerpsSwapKeys = accounts.into();
    let ix = perps_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn perps_swap_invoke_signed(
    accounts: PerpsSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    perps_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn perps_swap_verify_account_keys(
    accounts: PerpsSwapAccounts<'_, '_>,
    keys: PerpsSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.receiving_custody.key, keys.receiving_custody),
        (
            *accounts.receiving_custody_oracle_account.key,
            keys.receiving_custody_oracle_account,
        ),
        (
            *accounts.receiving_custody_token_account.key,
            keys.receiving_custody_token_account,
        ),
        (*accounts.dispensing_custody.key, keys.dispensing_custody),
        (
            *accounts.dispensing_custody_oracle_account.key,
            keys.dispensing_custody_oracle_account,
        ),
        (
            *accounts.dispensing_custody_token_account.key,
            keys.dispensing_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn perps_swap_verify_writable_privileges<'me, 'info>(
    accounts: PerpsSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.funding_account,
        accounts.receiving_account,
        accounts.pool,
        accounts.receiving_custody,
        accounts.receiving_custody_token_account,
        accounts.dispensing_custody,
        accounts.dispensing_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn perps_swap_verify_account_privileges<'me, 'info>(
    accounts: PerpsSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    perps_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct PerpsAddLiquidityAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_or_receiving_account: &'me AccountInfo<'info>,
    pub lp_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_oracle_account: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerpsAddLiquidityKeys {
    pub swap_program: Pubkey,
    pub owner: Pubkey,
    pub funding_or_receiving_account: Pubkey,
    pub lp_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_oracle_account: Pubkey,
    pub custody_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PerpsAddLiquidityAccounts<'_, '_>> for PerpsAddLiquidityKeys {
    fn from(accounts: PerpsAddLiquidityAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            owner: *accounts.owner.key,
            funding_or_receiving_account: *accounts.funding_or_receiving_account.key,
            lp_token_account: *accounts.lp_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_oracle_account: *accounts.custody_oracle_account.key,
            custody_token_account: *accounts.custody_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PerpsAddLiquidityKeys> for [AccountMeta; PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: PerpsAddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_or_receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_oracle_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for PerpsAddLiquidityKeys {
    fn from(pubkeys: [Pubkey; PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            owner: pubkeys[1],
            funding_or_receiving_account: pubkeys[2],
            lp_token_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            custody: pubkeys[7],
            custody_oracle_account: pubkeys[8],
            custody_token_account: pubkeys[9],
            lp_token_mint: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<PerpsAddLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PerpsAddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.owner.clone(),
            accounts.funding_or_receiving_account.clone(),
            accounts.lp_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_oracle_account.clone(),
            accounts.custody_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for PerpsAddLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            owner: &arr[1],
            funding_or_receiving_account: &arr[2],
            lp_token_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            custody: &arr[7],
            custody_oracle_account: &arr[8],
            custody_token_account: &arr[9],
            lp_token_mint: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const PERPS_ADD_LIQUIDITY_IX_DISCM: [u8; 8] = [170, 238, 222, 214, 245, 202, 108, 155];
#[derive(Clone, Debug, PartialEq)]
pub struct PerpsAddLiquidityIxData;
impl PerpsAddLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERPS_ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERPS_ADD_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERPS_ADD_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn perps_add_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: PerpsAddLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERPS_ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PerpsAddLiquidityIxData.try_to_vec()?,
    })
}
pub fn perps_add_liquidity_ix(keys: PerpsAddLiquidityKeys) -> std::io::Result<Instruction> {
    perps_add_liquidity_ix_with_program_id(JUPITER_ID, keys)
}
pub fn perps_add_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PerpsAddLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PerpsAddLiquidityKeys = accounts.into();
    let ix = perps_add_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn perps_add_liquidity_invoke(accounts: PerpsAddLiquidityAccounts<'_, '_>) -> ProgramResult {
    perps_add_liquidity_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn perps_add_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PerpsAddLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PerpsAddLiquidityKeys = accounts.into();
    let ix = perps_add_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn perps_add_liquidity_invoke_signed(
    accounts: PerpsAddLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    perps_add_liquidity_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn perps_add_liquidity_verify_account_keys(
    accounts: PerpsAddLiquidityAccounts<'_, '_>,
    keys: PerpsAddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.owner.key, keys.owner),
        (
            *accounts.funding_or_receiving_account.key,
            keys.funding_or_receiving_account,
        ),
        (*accounts.lp_token_account.key, keys.lp_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (
            *accounts.custody_oracle_account.key,
            keys.custody_oracle_account,
        ),
        (
            *accounts.custody_token_account.key,
            keys.custody_token_account,
        ),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn perps_add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: PerpsAddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.funding_or_receiving_account,
        accounts.lp_token_account,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn perps_add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: PerpsAddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    perps_add_liquidity_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct PerpsRemoveLiquidityAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_or_receiving_account: &'me AccountInfo<'info>,
    pub lp_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_oracle_account: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerpsRemoveLiquidityKeys {
    pub swap_program: Pubkey,
    pub owner: Pubkey,
    pub funding_or_receiving_account: Pubkey,
    pub lp_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_oracle_account: Pubkey,
    pub custody_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PerpsRemoveLiquidityAccounts<'_, '_>> for PerpsRemoveLiquidityKeys {
    fn from(accounts: PerpsRemoveLiquidityAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            owner: *accounts.owner.key,
            funding_or_receiving_account: *accounts.funding_or_receiving_account.key,
            lp_token_account: *accounts.lp_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_oracle_account: *accounts.custody_oracle_account.key,
            custody_token_account: *accounts.custody_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PerpsRemoveLiquidityKeys> for [AccountMeta; PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: PerpsRemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.funding_or_receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_oracle_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for PerpsRemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            owner: pubkeys[1],
            funding_or_receiving_account: pubkeys[2],
            lp_token_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            custody: pubkeys[7],
            custody_oracle_account: pubkeys[8],
            custody_token_account: pubkeys[9],
            lp_token_mint: pubkeys[10],
            token_program: pubkeys[11],
            event_authority: pubkeys[12],
            program: pubkeys[13],
        }
    }
}
impl<'info> From<PerpsRemoveLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PerpsRemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.owner.clone(),
            accounts.funding_or_receiving_account.clone(),
            accounts.lp_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_oracle_account.clone(),
            accounts.custody_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for PerpsRemoveLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            owner: &arr[1],
            funding_or_receiving_account: &arr[2],
            lp_token_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            custody: &arr[7],
            custody_oracle_account: &arr[8],
            custody_token_account: &arr[9],
            lp_token_mint: &arr[10],
            token_program: &arr[11],
            event_authority: &arr[12],
            program: &arr[13],
        }
    }
}
pub const PERPS_REMOVE_LIQUIDITY_IX_DISCM: [u8; 8] = [79, 211, 232, 140, 8, 78, 220, 34];
#[derive(Clone, Debug, PartialEq)]
pub struct PerpsRemoveLiquidityIxData;
impl PerpsRemoveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERPS_REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERPS_REMOVE_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERPS_REMOVE_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn perps_remove_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: PerpsRemoveLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERPS_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PerpsRemoveLiquidityIxData.try_to_vec()?,
    })
}
pub fn perps_remove_liquidity_ix(keys: PerpsRemoveLiquidityKeys) -> std::io::Result<Instruction> {
    perps_remove_liquidity_ix_with_program_id(JUPITER_ID, keys)
}
pub fn perps_remove_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PerpsRemoveLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PerpsRemoveLiquidityKeys = accounts.into();
    let ix = perps_remove_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn perps_remove_liquidity_invoke(
    accounts: PerpsRemoveLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    perps_remove_liquidity_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn perps_remove_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PerpsRemoveLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PerpsRemoveLiquidityKeys = accounts.into();
    let ix = perps_remove_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn perps_remove_liquidity_invoke_signed(
    accounts: PerpsRemoveLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    perps_remove_liquidity_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn perps_remove_liquidity_verify_account_keys(
    accounts: PerpsRemoveLiquidityAccounts<'_, '_>,
    keys: PerpsRemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.owner.key, keys.owner),
        (
            *accounts.funding_or_receiving_account.key,
            keys.funding_or_receiving_account,
        ),
        (*accounts.lp_token_account.key, keys.lp_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (
            *accounts.custody_oracle_account.key,
            keys.custody_oracle_account,
        ),
        (
            *accounts.custody_token_account.key,
            keys.custody_token_account,
        ),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn perps_remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: PerpsRemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.owner,
        accounts.funding_or_receiving_account,
        accounts.lp_token_account,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn perps_remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: PerpsRemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    perps_remove_liquidity_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct MeteoraDlmmSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub lb_pair: &'me AccountInfo<'info>,
    pub bin_array_bitmap_extension: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub token_x_mint: &'me AccountInfo<'info>,
    pub token_y_mint: &'me AccountInfo<'info>,
    pub oracle: &'me AccountInfo<'info>,
    pub host_fee_in: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_x_program: &'me AccountInfo<'info>,
    pub token_y_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MeteoraDlmmSwapKeys {
    pub swap_program: Pubkey,
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_extension: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub user: Pubkey,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<MeteoraDlmmSwapAccounts<'_, '_>> for MeteoraDlmmSwapKeys {
    fn from(accounts: MeteoraDlmmSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            lb_pair: *accounts.lb_pair.key,
            bin_array_bitmap_extension: *accounts.bin_array_bitmap_extension.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            token_x_mint: *accounts.token_x_mint.key,
            token_y_mint: *accounts.token_y_mint.key,
            oracle: *accounts.oracle.key,
            host_fee_in: *accounts.host_fee_in.key,
            user: *accounts.user.key,
            token_x_program: *accounts.token_x_program.key,
            token_y_program: *accounts.token_y_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<MeteoraDlmmSwapKeys> for [AccountMeta; METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: MeteoraDlmmSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lb_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.bin_array_bitmap_extension,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_x_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.host_fee_in,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_x_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_y_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN]> for MeteoraDlmmSwapKeys {
    fn from(pubkeys: [Pubkey; METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            lb_pair: pubkeys[1],
            bin_array_bitmap_extension: pubkeys[2],
            reserve_x: pubkeys[3],
            reserve_y: pubkeys[4],
            user_token_in: pubkeys[5],
            user_token_out: pubkeys[6],
            token_x_mint: pubkeys[7],
            token_y_mint: pubkeys[8],
            oracle: pubkeys[9],
            host_fee_in: pubkeys[10],
            user: pubkeys[11],
            token_x_program: pubkeys[12],
            token_y_program: pubkeys[13],
            event_authority: pubkeys[14],
            program: pubkeys[15],
        }
    }
}
impl<'info> From<MeteoraDlmmSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MeteoraDlmmSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.lb_pair.clone(),
            accounts.bin_array_bitmap_extension.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.token_x_mint.clone(),
            accounts.token_y_mint.clone(),
            accounts.oracle.clone(),
            accounts.host_fee_in.clone(),
            accounts.user.clone(),
            accounts.token_x_program.clone(),
            accounts.token_y_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN]>
    for MeteoraDlmmSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            lb_pair: &arr[1],
            bin_array_bitmap_extension: &arr[2],
            reserve_x: &arr[3],
            reserve_y: &arr[4],
            user_token_in: &arr[5],
            user_token_out: &arr[6],
            token_x_mint: &arr[7],
            token_y_mint: &arr[8],
            oracle: &arr[9],
            host_fee_in: &arr[10],
            user: &arr[11],
            token_x_program: &arr[12],
            token_y_program: &arr[13],
            event_authority: &arr[14],
            program: &arr[15],
        }
    }
}
pub const METEORA_DLMM_SWAP_IX_DISCM: [u8; 8] = [127, 64, 37, 138, 173, 243, 207, 84];
#[derive(Clone, Debug, PartialEq)]
pub struct MeteoraDlmmSwapIxData;
impl MeteoraDlmmSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != METEORA_DLMM_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    METEORA_DLMM_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&METEORA_DLMM_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn meteora_dlmm_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: MeteoraDlmmSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; METEORA_DLMM_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MeteoraDlmmSwapIxData.try_to_vec()?,
    })
}
pub fn meteora_dlmm_swap_ix(keys: MeteoraDlmmSwapKeys) -> std::io::Result<Instruction> {
    meteora_dlmm_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn meteora_dlmm_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MeteoraDlmmSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MeteoraDlmmSwapKeys = accounts.into();
    let ix = meteora_dlmm_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn meteora_dlmm_swap_invoke(accounts: MeteoraDlmmSwapAccounts<'_, '_>) -> ProgramResult {
    meteora_dlmm_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn meteora_dlmm_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MeteoraDlmmSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MeteoraDlmmSwapKeys = accounts.into();
    let ix = meteora_dlmm_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn meteora_dlmm_swap_invoke_signed(
    accounts: MeteoraDlmmSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    meteora_dlmm_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn meteora_dlmm_swap_verify_account_keys(
    accounts: MeteoraDlmmSwapAccounts<'_, '_>,
    keys: MeteoraDlmmSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.lb_pair.key, keys.lb_pair),
        (
            *accounts.bin_array_bitmap_extension.key,
            keys.bin_array_bitmap_extension,
        ),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.token_x_mint.key, keys.token_x_mint),
        (*accounts.token_y_mint.key, keys.token_y_mint),
        (*accounts.oracle.key, keys.oracle),
        (*accounts.host_fee_in.key, keys.host_fee_in),
        (*accounts.user.key, keys.user),
        (*accounts.token_x_program.key, keys.token_x_program),
        (*accounts.token_y_program.key, keys.token_y_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn meteora_dlmm_swap_verify_writable_privileges<'me, 'info>(
    accounts: MeteoraDlmmSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.lb_pair,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.oracle,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn meteora_dlmm_swap_verify_account_privileges<'me, 'info>(
    accounts: MeteoraDlmmSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    meteora_dlmm_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct OpenBookV2SwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub signer: &'me AccountInfo<'info>,
    pub penalty_payer: &'me AccountInfo<'info>,
    pub market: &'me AccountInfo<'info>,
    pub market_authority: &'me AccountInfo<'info>,
    pub bids: &'me AccountInfo<'info>,
    pub asks: &'me AccountInfo<'info>,
    pub market_base_vault: &'me AccountInfo<'info>,
    pub market_quote_vault: &'me AccountInfo<'info>,
    pub event_heap: &'me AccountInfo<'info>,
    pub user_base_account: &'me AccountInfo<'info>,
    pub user_quote_account: &'me AccountInfo<'info>,
    pub oracle_a: &'me AccountInfo<'info>,
    pub oracle_b: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub open_orders_admin: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OpenBookV2SwapKeys {
    pub swap_program: Pubkey,
    pub signer: Pubkey,
    pub penalty_payer: Pubkey,
    pub market: Pubkey,
    pub market_authority: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub event_heap: Pubkey,
    pub user_base_account: Pubkey,
    pub user_quote_account: Pubkey,
    pub oracle_a: Pubkey,
    pub oracle_b: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub open_orders_admin: Pubkey,
}
impl From<OpenBookV2SwapAccounts<'_, '_>> for OpenBookV2SwapKeys {
    fn from(accounts: OpenBookV2SwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            signer: *accounts.signer.key,
            penalty_payer: *accounts.penalty_payer.key,
            market: *accounts.market.key,
            market_authority: *accounts.market_authority.key,
            bids: *accounts.bids.key,
            asks: *accounts.asks.key,
            market_base_vault: *accounts.market_base_vault.key,
            market_quote_vault: *accounts.market_quote_vault.key,
            event_heap: *accounts.event_heap.key,
            user_base_account: *accounts.user_base_account.key,
            user_quote_account: *accounts.user_quote_account.key,
            oracle_a: *accounts.oracle_a.key,
            oracle_b: *accounts.oracle_b.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            open_orders_admin: *accounts.open_orders_admin.key,
        }
    }
}
impl From<OpenBookV2SwapKeys> for [AccountMeta; OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: OpenBookV2SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.penalty_payer,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bids,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.asks,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_base_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.market_quote_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.event_heap,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_base_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracle_a,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.oracle_b,
                is_signer: false,
                is_writable: false,
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
                pubkey: keys.open_orders_admin,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN]> for OpenBookV2SwapKeys {
    fn from(pubkeys: [Pubkey; OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            signer: pubkeys[1],
            penalty_payer: pubkeys[2],
            market: pubkeys[3],
            market_authority: pubkeys[4],
            bids: pubkeys[5],
            asks: pubkeys[6],
            market_base_vault: pubkeys[7],
            market_quote_vault: pubkeys[8],
            event_heap: pubkeys[9],
            user_base_account: pubkeys[10],
            user_quote_account: pubkeys[11],
            oracle_a: pubkeys[12],
            oracle_b: pubkeys[13],
            token_program: pubkeys[14],
            system_program: pubkeys[15],
            open_orders_admin: pubkeys[16],
        }
    }
}
impl<'info> From<OpenBookV2SwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: OpenBookV2SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.signer.clone(),
            accounts.penalty_payer.clone(),
            accounts.market.clone(),
            accounts.market_authority.clone(),
            accounts.bids.clone(),
            accounts.asks.clone(),
            accounts.market_base_vault.clone(),
            accounts.market_quote_vault.clone(),
            accounts.event_heap.clone(),
            accounts.user_base_account.clone(),
            accounts.user_quote_account.clone(),
            accounts.oracle_a.clone(),
            accounts.oracle_b.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.open_orders_admin.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN]>
    for OpenBookV2SwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            signer: &arr[1],
            penalty_payer: &arr[2],
            market: &arr[3],
            market_authority: &arr[4],
            bids: &arr[5],
            asks: &arr[6],
            market_base_vault: &arr[7],
            market_quote_vault: &arr[8],
            event_heap: &arr[9],
            user_base_account: &arr[10],
            user_quote_account: &arr[11],
            oracle_a: &arr[12],
            oracle_b: &arr[13],
            token_program: &arr[14],
            system_program: &arr[15],
            open_orders_admin: &arr[16],
        }
    }
}
pub const OPEN_BOOK_V2_SWAP_IX_DISCM: [u8; 8] = [135, 26, 163, 43, 198, 221, 29, 67];
#[derive(Clone, Debug, PartialEq)]
pub struct OpenBookV2SwapIxData;
impl OpenBookV2SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OPEN_BOOK_V2_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    OPEN_BOOK_V2_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OPEN_BOOK_V2_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn open_book_v2_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: OpenBookV2SwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OPEN_BOOK_V2_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: OpenBookV2SwapIxData.try_to_vec()?,
    })
}
pub fn open_book_v2_swap_ix(keys: OpenBookV2SwapKeys) -> std::io::Result<Instruction> {
    open_book_v2_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn open_book_v2_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OpenBookV2SwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: OpenBookV2SwapKeys = accounts.into();
    let ix = open_book_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn open_book_v2_swap_invoke(accounts: OpenBookV2SwapAccounts<'_, '_>) -> ProgramResult {
    open_book_v2_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn open_book_v2_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OpenBookV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OpenBookV2SwapKeys = accounts.into();
    let ix = open_book_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn open_book_v2_swap_invoke_signed(
    accounts: OpenBookV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    open_book_v2_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn open_book_v2_swap_verify_account_keys(
    accounts: OpenBookV2SwapAccounts<'_, '_>,
    keys: OpenBookV2SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.signer.key, keys.signer),
        (*accounts.penalty_payer.key, keys.penalty_payer),
        (*accounts.market.key, keys.market),
        (*accounts.market_authority.key, keys.market_authority),
        (*accounts.bids.key, keys.bids),
        (*accounts.asks.key, keys.asks),
        (*accounts.market_base_vault.key, keys.market_base_vault),
        (*accounts.market_quote_vault.key, keys.market_quote_vault),
        (*accounts.event_heap.key, keys.event_heap),
        (*accounts.user_base_account.key, keys.user_base_account),
        (*accounts.user_quote_account.key, keys.user_quote_account),
        (*accounts.oracle_a.key, keys.oracle_a),
        (*accounts.oracle_b.key, keys.oracle_b),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.open_orders_admin.key, keys.open_orders_admin),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn open_book_v2_swap_verify_writable_privileges<'me, 'info>(
    accounts: OpenBookV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.signer,
        accounts.penalty_payer,
        accounts.market,
        accounts.bids,
        accounts.asks,
        accounts.market_base_vault,
        accounts.market_quote_vault,
        accounts.event_heap,
        accounts.user_base_account,
        accounts.user_quote_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn open_book_v2_swap_verify_account_privileges<'me, 'info>(
    accounts: OpenBookV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    open_book_v2_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLONE_SWAP_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct CloneSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub clone: &'me AccountInfo<'info>,
    pub pools: &'me AccountInfo<'info>,
    pub oracles: &'me AccountInfo<'info>,
    pub user_collateral_token_account: &'me AccountInfo<'info>,
    pub user_onasset_token_account: &'me AccountInfo<'info>,
    pub onasset_mint: &'me AccountInfo<'info>,
    pub collateral_mint: &'me AccountInfo<'info>,
    pub collateral_vault: &'me AccountInfo<'info>,
    pub treasury_onasset_token_account: &'me AccountInfo<'info>,
    pub treasury_collateral_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub clone_staking: &'me AccountInfo<'info>,
    pub user_staking_account: &'me AccountInfo<'info>,
    pub clone_staking_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CloneSwapKeys {
    pub swap_program: Pubkey,
    pub user: Pubkey,
    pub clone: Pubkey,
    pub pools: Pubkey,
    pub oracles: Pubkey,
    pub user_collateral_token_account: Pubkey,
    pub user_onasset_token_account: Pubkey,
    pub onasset_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub treasury_onasset_token_account: Pubkey,
    pub treasury_collateral_token_account: Pubkey,
    pub token_program: Pubkey,
    pub clone_staking: Pubkey,
    pub user_staking_account: Pubkey,
    pub clone_staking_program: Pubkey,
}
impl From<CloneSwapAccounts<'_, '_>> for CloneSwapKeys {
    fn from(accounts: CloneSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            user: *accounts.user.key,
            clone: *accounts.clone.key,
            pools: *accounts.pools.key,
            oracles: *accounts.oracles.key,
            user_collateral_token_account: *accounts.user_collateral_token_account.key,
            user_onasset_token_account: *accounts.user_onasset_token_account.key,
            onasset_mint: *accounts.onasset_mint.key,
            collateral_mint: *accounts.collateral_mint.key,
            collateral_vault: *accounts.collateral_vault.key,
            treasury_onasset_token_account: *accounts.treasury_onasset_token_account.key,
            treasury_collateral_token_account: *accounts.treasury_collateral_token_account.key,
            token_program: *accounts.token_program.key,
            clone_staking: *accounts.clone_staking.key,
            user_staking_account: *accounts.user_staking_account.key,
            clone_staking_program: *accounts.clone_staking_program.key,
        }
    }
}
impl From<CloneSwapKeys> for [AccountMeta; CLONE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: CloneSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pools,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.oracles,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_onasset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.onasset_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.collateral_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.collateral_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_onasset_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.treasury_collateral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone_staking,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_staking_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clone_staking_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLONE_SWAP_IX_ACCOUNTS_LEN]> for CloneSwapKeys {
    fn from(pubkeys: [Pubkey; CLONE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            user: pubkeys[1],
            clone: pubkeys[2],
            pools: pubkeys[3],
            oracles: pubkeys[4],
            user_collateral_token_account: pubkeys[5],
            user_onasset_token_account: pubkeys[6],
            onasset_mint: pubkeys[7],
            collateral_mint: pubkeys[8],
            collateral_vault: pubkeys[9],
            treasury_onasset_token_account: pubkeys[10],
            treasury_collateral_token_account: pubkeys[11],
            token_program: pubkeys[12],
            clone_staking: pubkeys[13],
            user_staking_account: pubkeys[14],
            clone_staking_program: pubkeys[15],
        }
    }
}
impl<'info> From<CloneSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLONE_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CloneSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.user.clone(),
            accounts.clone.clone(),
            accounts.pools.clone(),
            accounts.oracles.clone(),
            accounts.user_collateral_token_account.clone(),
            accounts.user_onasset_token_account.clone(),
            accounts.onasset_mint.clone(),
            accounts.collateral_mint.clone(),
            accounts.collateral_vault.clone(),
            accounts.treasury_onasset_token_account.clone(),
            accounts.treasury_collateral_token_account.clone(),
            accounts.token_program.clone(),
            accounts.clone_staking.clone(),
            accounts.user_staking_account.clone(),
            accounts.clone_staking_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLONE_SWAP_IX_ACCOUNTS_LEN]>
    for CloneSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CLONE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            user: &arr[1],
            clone: &arr[2],
            pools: &arr[3],
            oracles: &arr[4],
            user_collateral_token_account: &arr[5],
            user_onasset_token_account: &arr[6],
            onasset_mint: &arr[7],
            collateral_mint: &arr[8],
            collateral_vault: &arr[9],
            treasury_onasset_token_account: &arr[10],
            treasury_collateral_token_account: &arr[11],
            token_program: &arr[12],
            clone_staking: &arr[13],
            user_staking_account: &arr[14],
            clone_staking_program: &arr[15],
        }
    }
}
pub const CLONE_SWAP_IX_DISCM: [u8; 8] = [85, 201, 154, 92, 133, 31, 142, 85];
#[derive(Clone, Debug, PartialEq)]
pub struct CloneSwapIxData;
impl CloneSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLONE_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLONE_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLONE_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn clone_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: CloneSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLONE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CloneSwapIxData.try_to_vec()?,
    })
}
pub fn clone_swap_ix(keys: CloneSwapKeys) -> std::io::Result<Instruction> {
    clone_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn clone_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CloneSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CloneSwapKeys = accounts.into();
    let ix = clone_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn clone_swap_invoke(accounts: CloneSwapAccounts<'_, '_>) -> ProgramResult {
    clone_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn clone_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CloneSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CloneSwapKeys = accounts.into();
    let ix = clone_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn clone_swap_invoke_signed(
    accounts: CloneSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    clone_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn clone_swap_verify_account_keys(
    accounts: CloneSwapAccounts<'_, '_>,
    keys: CloneSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.user.key, keys.user),
        (*accounts.clone.key, keys.clone),
        (*accounts.pools.key, keys.pools),
        (*accounts.oracles.key, keys.oracles),
        (
            *accounts.user_collateral_token_account.key,
            keys.user_collateral_token_account,
        ),
        (
            *accounts.user_onasset_token_account.key,
            keys.user_onasset_token_account,
        ),
        (*accounts.onasset_mint.key, keys.onasset_mint),
        (*accounts.collateral_mint.key, keys.collateral_mint),
        (*accounts.collateral_vault.key, keys.collateral_vault),
        (
            *accounts.treasury_onasset_token_account.key,
            keys.treasury_onasset_token_account,
        ),
        (
            *accounts.treasury_collateral_token_account.key,
            keys.treasury_collateral_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.clone_staking.key, keys.clone_staking),
        (
            *accounts.user_staking_account.key,
            keys.user_staking_account,
        ),
        (
            *accounts.clone_staking_program.key,
            keys.clone_staking_program,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn clone_swap_verify_writable_privileges<'me, 'info>(
    accounts: CloneSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.clone,
        accounts.pools,
        accounts.oracles,
        accounts.user_collateral_token_account,
        accounts.user_onasset_token_account,
        accounts.onasset_mint,
        accounts.collateral_vault,
        accounts.treasury_onasset_token_account,
        accounts.treasury_collateral_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn clone_swap_verify_account_privileges<'me, 'info>(
    accounts: CloneSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    clone_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct RaydiumCpSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
    pub amm_config: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub input_token_account: &'me AccountInfo<'info>,
    pub output_token_account: &'me AccountInfo<'info>,
    pub input_vault: &'me AccountInfo<'info>,
    pub output_vault: &'me AccountInfo<'info>,
    pub input_token_program: &'me AccountInfo<'info>,
    pub output_token_program: &'me AccountInfo<'info>,
    pub input_token_mint: &'me AccountInfo<'info>,
    pub output_token_mint: &'me AccountInfo<'info>,
    pub observation_state: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RaydiumCpSwapKeys {
    pub swap_program: Pubkey,
    pub payer: Pubkey,
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub input_token_mint: Pubkey,
    pub output_token_mint: Pubkey,
    pub observation_state: Pubkey,
}
impl From<RaydiumCpSwapAccounts<'_, '_>> for RaydiumCpSwapKeys {
    fn from(accounts: RaydiumCpSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            payer: *accounts.payer.key,
            authority: *accounts.authority.key,
            amm_config: *accounts.amm_config.key,
            pool_state: *accounts.pool_state.key,
            input_token_account: *accounts.input_token_account.key,
            output_token_account: *accounts.output_token_account.key,
            input_vault: *accounts.input_vault.key,
            output_vault: *accounts.output_vault.key,
            input_token_program: *accounts.input_token_program.key,
            output_token_program: *accounts.output_token_program.key,
            input_token_mint: *accounts.input_token_mint.key,
            output_token_mint: *accounts.output_token_mint.key,
            observation_state: *accounts.observation_state.key,
        }
    }
}
impl From<RaydiumCpSwapKeys> for [AccountMeta; RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: RaydiumCpSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.amm_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.output_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.input_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.input_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.output_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.observation_state,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN]> for RaydiumCpSwapKeys {
    fn from(pubkeys: [Pubkey; RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            payer: pubkeys[1],
            authority: pubkeys[2],
            amm_config: pubkeys[3],
            pool_state: pubkeys[4],
            input_token_account: pubkeys[5],
            output_token_account: pubkeys[6],
            input_vault: pubkeys[7],
            output_vault: pubkeys[8],
            input_token_program: pubkeys[9],
            output_token_program: pubkeys[10],
            input_token_mint: pubkeys[11],
            output_token_mint: pubkeys[12],
            observation_state: pubkeys[13],
        }
    }
}
impl<'info> From<RaydiumCpSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RaydiumCpSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.payer.clone(),
            accounts.authority.clone(),
            accounts.amm_config.clone(),
            accounts.pool_state.clone(),
            accounts.input_token_account.clone(),
            accounts.output_token_account.clone(),
            accounts.input_vault.clone(),
            accounts.output_vault.clone(),
            accounts.input_token_program.clone(),
            accounts.output_token_program.clone(),
            accounts.input_token_mint.clone(),
            accounts.output_token_mint.clone(),
            accounts.observation_state.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN]>
    for RaydiumCpSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            payer: &arr[1],
            authority: &arr[2],
            amm_config: &arr[3],
            pool_state: &arr[4],
            input_token_account: &arr[5],
            output_token_account: &arr[6],
            input_vault: &arr[7],
            output_vault: &arr[8],
            input_token_program: &arr[9],
            output_token_program: &arr[10],
            input_token_mint: &arr[11],
            output_token_mint: &arr[12],
            observation_state: &arr[13],
        }
    }
}
pub const RAYDIUM_CP_SWAP_IX_DISCM: [u8; 8] = [54, 234, 83, 141, 52, 191, 46, 144];
#[derive(Clone, Debug, PartialEq)]
pub struct RaydiumCpSwapIxData;
impl RaydiumCpSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != RAYDIUM_CP_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RAYDIUM_CP_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&RAYDIUM_CP_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn raydium_cp_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: RaydiumCpSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; RAYDIUM_CP_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RaydiumCpSwapIxData.try_to_vec()?,
    })
}
pub fn raydium_cp_swap_ix(keys: RaydiumCpSwapKeys) -> std::io::Result<Instruction> {
    raydium_cp_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn raydium_cp_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumCpSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RaydiumCpSwapKeys = accounts.into();
    let ix = raydium_cp_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn raydium_cp_swap_invoke(accounts: RaydiumCpSwapAccounts<'_, '_>) -> ProgramResult {
    raydium_cp_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn raydium_cp_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RaydiumCpSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RaydiumCpSwapKeys = accounts.into();
    let ix = raydium_cp_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn raydium_cp_swap_invoke_signed(
    accounts: RaydiumCpSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    raydium_cp_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn raydium_cp_swap_verify_account_keys(
    accounts: RaydiumCpSwapAccounts<'_, '_>,
    keys: RaydiumCpSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.payer.key, keys.payer),
        (*accounts.authority.key, keys.authority),
        (*accounts.amm_config.key, keys.amm_config),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.input_token_account.key, keys.input_token_account),
        (
            *accounts.output_token_account.key,
            keys.output_token_account,
        ),
        (*accounts.input_vault.key, keys.input_vault),
        (*accounts.output_vault.key, keys.output_vault),
        (*accounts.input_token_program.key, keys.input_token_program),
        (
            *accounts.output_token_program.key,
            keys.output_token_program,
        ),
        (*accounts.input_token_mint.key, keys.input_token_mint),
        (*accounts.output_token_mint.key, keys.output_token_mint),
        (*accounts.observation_state.key, keys.observation_state),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn raydium_cp_swap_verify_writable_privileges<'me, 'info>(
    accounts: RaydiumCpSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.input_token_account,
        accounts.output_token_account,
        accounts.input_vault,
        accounts.output_vault,
        accounts.observation_state,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn raydium_cp_swap_verify_account_privileges<'me, 'info>(
    accounts: RaydiumCpSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    raydium_cp_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const ONE_INTRO_SWAP_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct OneIntroSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub metadata_state: &'me AccountInfo<'info>,
    pub pool_state: &'me AccountInfo<'info>,
    pub pool_auth_pda: &'me AccountInfo<'info>,
    pub pool_token_in_account: &'me AccountInfo<'info>,
    pub pool_token_out_account: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_token_in_account: &'me AccountInfo<'info>,
    pub user_token_out_account: &'me AccountInfo<'info>,
    pub metadata_swap_fee_account: &'me AccountInfo<'info>,
    pub referral_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OneIntroSwapKeys {
    pub swap_program: Pubkey,
    pub metadata_state: Pubkey,
    pub pool_state: Pubkey,
    pub pool_auth_pda: Pubkey,
    pub pool_token_in_account: Pubkey,
    pub pool_token_out_account: Pubkey,
    pub user: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub metadata_swap_fee_account: Pubkey,
    pub referral_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl From<OneIntroSwapAccounts<'_, '_>> for OneIntroSwapKeys {
    fn from(accounts: OneIntroSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            metadata_state: *accounts.metadata_state.key,
            pool_state: *accounts.pool_state.key,
            pool_auth_pda: *accounts.pool_auth_pda.key,
            pool_token_in_account: *accounts.pool_token_in_account.key,
            pool_token_out_account: *accounts.pool_token_out_account.key,
            user: *accounts.user.key,
            user_token_in_account: *accounts.user_token_in_account.key,
            user_token_out_account: *accounts.user_token_out_account.key,
            metadata_swap_fee_account: *accounts.metadata_swap_fee_account.key,
            referral_token_account: *accounts.referral_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<OneIntroSwapKeys> for [AccountMeta; ONE_INTRO_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: OneIntroSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.metadata_state,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_state,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_auth_pda,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_in_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.metadata_swap_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.referral_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ONE_INTRO_SWAP_IX_ACCOUNTS_LEN]> for OneIntroSwapKeys {
    fn from(pubkeys: [Pubkey; ONE_INTRO_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            metadata_state: pubkeys[1],
            pool_state: pubkeys[2],
            pool_auth_pda: pubkeys[3],
            pool_token_in_account: pubkeys[4],
            pool_token_out_account: pubkeys[5],
            user: pubkeys[6],
            user_token_in_account: pubkeys[7],
            user_token_out_account: pubkeys[8],
            metadata_swap_fee_account: pubkeys[9],
            referral_token_account: pubkeys[10],
            token_program: pubkeys[11],
        }
    }
}
impl<'info> From<OneIntroSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; ONE_INTRO_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: OneIntroSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.metadata_state.clone(),
            accounts.pool_state.clone(),
            accounts.pool_auth_pda.clone(),
            accounts.pool_token_in_account.clone(),
            accounts.pool_token_out_account.clone(),
            accounts.user.clone(),
            accounts.user_token_in_account.clone(),
            accounts.user_token_out_account.clone(),
            accounts.metadata_swap_fee_account.clone(),
            accounts.referral_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ONE_INTRO_SWAP_IX_ACCOUNTS_LEN]>
    for OneIntroSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ONE_INTRO_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            metadata_state: &arr[1],
            pool_state: &arr[2],
            pool_auth_pda: &arr[3],
            pool_token_in_account: &arr[4],
            pool_token_out_account: &arr[5],
            user: &arr[6],
            user_token_in_account: &arr[7],
            user_token_out_account: &arr[8],
            metadata_swap_fee_account: &arr[9],
            referral_token_account: &arr[10],
            token_program: &arr[11],
        }
    }
}
pub const ONE_INTRO_SWAP_IX_DISCM: [u8; 8] = [208, 212, 80, 169, 36, 148, 209, 35];
#[derive(Clone, Debug, PartialEq)]
pub struct OneIntroSwapIxData;
impl OneIntroSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != ONE_INTRO_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ONE_INTRO_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&ONE_INTRO_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn one_intro_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: OneIntroSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ONE_INTRO_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: OneIntroSwapIxData.try_to_vec()?,
    })
}
pub fn one_intro_swap_ix(keys: OneIntroSwapKeys) -> std::io::Result<Instruction> {
    one_intro_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn one_intro_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: OneIntroSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: OneIntroSwapKeys = accounts.into();
    let ix = one_intro_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn one_intro_swap_invoke(accounts: OneIntroSwapAccounts<'_, '_>) -> ProgramResult {
    one_intro_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn one_intro_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: OneIntroSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: OneIntroSwapKeys = accounts.into();
    let ix = one_intro_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn one_intro_swap_invoke_signed(
    accounts: OneIntroSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    one_intro_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn one_intro_swap_verify_account_keys(
    accounts: OneIntroSwapAccounts<'_, '_>,
    keys: OneIntroSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.metadata_state.key, keys.metadata_state),
        (*accounts.pool_state.key, keys.pool_state),
        (*accounts.pool_auth_pda.key, keys.pool_auth_pda),
        (
            *accounts.pool_token_in_account.key,
            keys.pool_token_in_account,
        ),
        (
            *accounts.pool_token_out_account.key,
            keys.pool_token_out_account,
        ),
        (*accounts.user.key, keys.user),
        (
            *accounts.user_token_in_account.key,
            keys.user_token_in_account,
        ),
        (
            *accounts.user_token_out_account.key,
            keys.user_token_out_account,
        ),
        (
            *accounts.metadata_swap_fee_account.key,
            keys.metadata_swap_fee_account,
        ),
        (
            *accounts.referral_token_account.key,
            keys.referral_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn one_intro_swap_verify_writable_privileges<'me, 'info>(
    accounts: OneIntroSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.pool_state,
        accounts.pool_token_in_account,
        accounts.pool_token_out_account,
        accounts.user,
        accounts.user_token_in_account,
        accounts.user_token_out_account,
        accounts.metadata_swap_fee_account,
        accounts.referral_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn one_intro_swap_verify_account_privileges<'me, 'info>(
    accounts: OneIntroSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    one_intro_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN: usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct PumpdotfunWrappedBuyAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub user_wsol_token_account: &'me AccountInfo<'info>,
    pub temp_wsol_token_account: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PumpdotfunWrappedBuyKeys {
    pub swap_program: Pubkey,
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub rent: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub user_wsol_token_account: Pubkey,
    pub temp_wsol_token_account: Pubkey,
    pub wsol_mint: Pubkey,
}
impl From<PumpdotfunWrappedBuyAccounts<'_, '_>> for PumpdotfunWrappedBuyKeys {
    fn from(accounts: PumpdotfunWrappedBuyAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            user_wsol_token_account: *accounts.user_wsol_token_account.key,
            temp_wsol_token_account: *accounts.temp_wsol_token_account.key,
            wsol_mint: *accounts.wsol_mint.key,
        }
    }
}
impl From<PumpdotfunWrappedBuyKeys> for [AccountMeta; PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: PumpdotfunWrappedBuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
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
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.temp_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN]> for PumpdotfunWrappedBuyKeys {
    fn from(pubkeys: [Pubkey; PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            global: pubkeys[1],
            fee_recipient: pubkeys[2],
            mint: pubkeys[3],
            bonding_curve: pubkeys[4],
            associated_bonding_curve: pubkeys[5],
            associated_user: pubkeys[6],
            user: pubkeys[7],
            system_program: pubkeys[8],
            token_program: pubkeys[9],
            rent: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
            user_wsol_token_account: pubkeys[13],
            temp_wsol_token_account: pubkeys[14],
            wsol_mint: pubkeys[15],
        }
    }
}
impl<'info> From<PumpdotfunWrappedBuyAccounts<'_, 'info>>
    for [AccountInfo<'info>; PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PumpdotfunWrappedBuyAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.user_wsol_token_account.clone(),
            accounts.temp_wsol_token_account.clone(),
            accounts.wsol_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN]>
    for PumpdotfunWrappedBuyAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            global: &arr[1],
            fee_recipient: &arr[2],
            mint: &arr[3],
            bonding_curve: &arr[4],
            associated_bonding_curve: &arr[5],
            associated_user: &arr[6],
            user: &arr[7],
            system_program: &arr[8],
            token_program: &arr[9],
            rent: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
            user_wsol_token_account: &arr[13],
            temp_wsol_token_account: &arr[14],
            wsol_mint: &arr[15],
        }
    }
}
pub const PUMPDOTFUN_WRAPPED_BUY_IX_DISCM: [u8; 8] = [138, 139, 167, 134, 208, 91, 138, 158];
#[derive(Clone, Debug, PartialEq)]
pub struct PumpdotfunWrappedBuyIxData;
impl PumpdotfunWrappedBuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PUMPDOTFUN_WRAPPED_BUY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PUMPDOTFUN_WRAPPED_BUY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PUMPDOTFUN_WRAPPED_BUY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pumpdotfun_wrapped_buy_ix_with_program_id(
    program_id: Pubkey,
    keys: PumpdotfunWrappedBuyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PUMPDOTFUN_WRAPPED_BUY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PumpdotfunWrappedBuyIxData.try_to_vec()?,
    })
}
pub fn pumpdotfun_wrapped_buy_ix(keys: PumpdotfunWrappedBuyKeys) -> std::io::Result<Instruction> {
    pumpdotfun_wrapped_buy_ix_with_program_id(JUPITER_ID, keys)
}
pub fn pumpdotfun_wrapped_buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PumpdotfunWrappedBuyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PumpdotfunWrappedBuyKeys = accounts.into();
    let ix = pumpdotfun_wrapped_buy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pumpdotfun_wrapped_buy_invoke(
    accounts: PumpdotfunWrappedBuyAccounts<'_, '_>,
) -> ProgramResult {
    pumpdotfun_wrapped_buy_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn pumpdotfun_wrapped_buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PumpdotfunWrappedBuyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PumpdotfunWrappedBuyKeys = accounts.into();
    let ix = pumpdotfun_wrapped_buy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pumpdotfun_wrapped_buy_invoke_signed(
    accounts: PumpdotfunWrappedBuyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pumpdotfun_wrapped_buy_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn pumpdotfun_wrapped_buy_verify_account_keys(
    accounts: PumpdotfunWrappedBuyAccounts<'_, '_>,
    keys: PumpdotfunWrappedBuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.associated_bonding_curve.key,
            keys.associated_bonding_curve,
        ),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.rent.key, keys.rent),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (
            *accounts.user_wsol_token_account.key,
            keys.user_wsol_token_account,
        ),
        (
            *accounts.temp_wsol_token_account.key,
            keys.temp_wsol_token_account,
        ),
        (*accounts.wsol_mint.key, keys.wsol_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pumpdotfun_wrapped_buy_verify_writable_privileges<'me, 'info>(
    accounts: PumpdotfunWrappedBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.user_wsol_token_account,
        accounts.temp_wsol_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pumpdotfun_wrapped_buy_verify_account_privileges<'me, 'info>(
    accounts: PumpdotfunWrappedBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pumpdotfun_wrapped_buy_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct PumpdotfunWrappedSellAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub global: &'me AccountInfo<'info>,
    pub fee_recipient: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub bonding_curve: &'me AccountInfo<'info>,
    pub associated_bonding_curve: &'me AccountInfo<'info>,
    pub associated_user: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
    pub user_wsol_token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PumpdotfunWrappedSellKeys {
    pub swap_program: Pubkey,
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
    pub system_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
    pub user_wsol_token_account: Pubkey,
}
impl From<PumpdotfunWrappedSellAccounts<'_, '_>> for PumpdotfunWrappedSellKeys {
    fn from(accounts: PumpdotfunWrappedSellAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            global: *accounts.global.key,
            fee_recipient: *accounts.fee_recipient.key,
            mint: *accounts.mint.key,
            bonding_curve: *accounts.bonding_curve.key,
            associated_bonding_curve: *accounts.associated_bonding_curve.key,
            associated_user: *accounts.associated_user.key,
            user: *accounts.user.key,
            system_program: *accounts.system_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
            user_wsol_token_account: *accounts.user_wsol_token_account.key,
        }
    }
}
impl From<PumpdotfunWrappedSellKeys> for [AccountMeta; PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: PumpdotfunWrappedSellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.global,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.fee_recipient,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_bonding_curve,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.associated_user,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: true,
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
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN]> for PumpdotfunWrappedSellKeys {
    fn from(pubkeys: [Pubkey; PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            global: pubkeys[1],
            fee_recipient: pubkeys[2],
            mint: pubkeys[3],
            bonding_curve: pubkeys[4],
            associated_bonding_curve: pubkeys[5],
            associated_user: pubkeys[6],
            user: pubkeys[7],
            system_program: pubkeys[8],
            associated_token_program: pubkeys[9],
            token_program: pubkeys[10],
            event_authority: pubkeys[11],
            program: pubkeys[12],
            user_wsol_token_account: pubkeys[13],
        }
    }
}
impl<'info> From<PumpdotfunWrappedSellAccounts<'_, 'info>>
    for [AccountInfo<'info>; PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PumpdotfunWrappedSellAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.global.clone(),
            accounts.fee_recipient.clone(),
            accounts.mint.clone(),
            accounts.bonding_curve.clone(),
            accounts.associated_bonding_curve.clone(),
            accounts.associated_user.clone(),
            accounts.user.clone(),
            accounts.system_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
            accounts.user_wsol_token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN]>
    for PumpdotfunWrappedSellAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            global: &arr[1],
            fee_recipient: &arr[2],
            mint: &arr[3],
            bonding_curve: &arr[4],
            associated_bonding_curve: &arr[5],
            associated_user: &arr[6],
            user: &arr[7],
            system_program: &arr[8],
            associated_token_program: &arr[9],
            token_program: &arr[10],
            event_authority: &arr[11],
            program: &arr[12],
            user_wsol_token_account: &arr[13],
        }
    }
}
pub const PUMPDOTFUN_WRAPPED_SELL_IX_DISCM: [u8; 8] = [255, 19, 99, 99, 40, 65, 83, 255];
#[derive(Clone, Debug, PartialEq)]
pub struct PumpdotfunWrappedSellIxData;
impl PumpdotfunWrappedSellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PUMPDOTFUN_WRAPPED_SELL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PUMPDOTFUN_WRAPPED_SELL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PUMPDOTFUN_WRAPPED_SELL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn pumpdotfun_wrapped_sell_ix_with_program_id(
    program_id: Pubkey,
    keys: PumpdotfunWrappedSellKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PUMPDOTFUN_WRAPPED_SELL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PumpdotfunWrappedSellIxData.try_to_vec()?,
    })
}
pub fn pumpdotfun_wrapped_sell_ix(keys: PumpdotfunWrappedSellKeys) -> std::io::Result<Instruction> {
    pumpdotfun_wrapped_sell_ix_with_program_id(JUPITER_ID, keys)
}
pub fn pumpdotfun_wrapped_sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PumpdotfunWrappedSellAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PumpdotfunWrappedSellKeys = accounts.into();
    let ix = pumpdotfun_wrapped_sell_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn pumpdotfun_wrapped_sell_invoke(
    accounts: PumpdotfunWrappedSellAccounts<'_, '_>,
) -> ProgramResult {
    pumpdotfun_wrapped_sell_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn pumpdotfun_wrapped_sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PumpdotfunWrappedSellAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PumpdotfunWrappedSellKeys = accounts.into();
    let ix = pumpdotfun_wrapped_sell_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn pumpdotfun_wrapped_sell_invoke_signed(
    accounts: PumpdotfunWrappedSellAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    pumpdotfun_wrapped_sell_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn pumpdotfun_wrapped_sell_verify_account_keys(
    accounts: PumpdotfunWrappedSellAccounts<'_, '_>,
    keys: PumpdotfunWrappedSellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.global.key, keys.global),
        (*accounts.fee_recipient.key, keys.fee_recipient),
        (*accounts.mint.key, keys.mint),
        (*accounts.bonding_curve.key, keys.bonding_curve),
        (
            *accounts.associated_bonding_curve.key,
            keys.associated_bonding_curve,
        ),
        (*accounts.associated_user.key, keys.associated_user),
        (*accounts.user.key, keys.user),
        (*accounts.system_program.key, keys.system_program),
        (
            *accounts.associated_token_program.key,
            keys.associated_token_program,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
        (
            *accounts.user_wsol_token_account.key,
            keys.user_wsol_token_account,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn pumpdotfun_wrapped_sell_verify_writable_privileges<'me, 'info>(
    accounts: PumpdotfunWrappedSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.fee_recipient,
        accounts.bonding_curve,
        accounts.associated_bonding_curve,
        accounts.associated_user,
        accounts.user,
        accounts.user_wsol_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn pumpdotfun_wrapped_sell_verify_account_privileges<'me, 'info>(
    accounts: PumpdotfunWrappedSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    pumpdotfun_wrapped_sell_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PERPS_V2_SWAP_IX_ACCOUNTS_LEN: usize = 18;
#[derive(Copy, Clone, Debug)]
pub struct PerpsV2SwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_account: &'me AccountInfo<'info>,
    pub receiving_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub receiving_custody: &'me AccountInfo<'info>,
    pub receiving_custody_doves_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub receiving_custody_token_account: &'me AccountInfo<'info>,
    pub dispensing_custody: &'me AccountInfo<'info>,
    pub dispensing_custody_doves_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub dispensing_custody_token_account: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerpsV2SwapKeys {
    pub swap_program: Pubkey,
    pub owner: Pubkey,
    pub funding_account: Pubkey,
    pub receiving_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub receiving_custody: Pubkey,
    pub receiving_custody_doves_price_account: Pubkey,
    pub receiving_custody_pythnet_price_account: Pubkey,
    pub receiving_custody_token_account: Pubkey,
    pub dispensing_custody: Pubkey,
    pub dispensing_custody_doves_price_account: Pubkey,
    pub dispensing_custody_pythnet_price_account: Pubkey,
    pub dispensing_custody_token_account: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PerpsV2SwapAccounts<'_, '_>> for PerpsV2SwapKeys {
    fn from(accounts: PerpsV2SwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            owner: *accounts.owner.key,
            funding_account: *accounts.funding_account.key,
            receiving_account: *accounts.receiving_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            receiving_custody: *accounts.receiving_custody.key,
            receiving_custody_doves_price_account: *accounts
                .receiving_custody_doves_price_account
                .key,
            receiving_custody_pythnet_price_account: *accounts
                .receiving_custody_pythnet_price_account
                .key,
            receiving_custody_token_account: *accounts.receiving_custody_token_account.key,
            dispensing_custody: *accounts.dispensing_custody.key,
            dispensing_custody_doves_price_account: *accounts
                .dispensing_custody_doves_price_account
                .key,
            dispensing_custody_pythnet_price_account: *accounts
                .dispensing_custody_pythnet_price_account
                .key,
            dispensing_custody_token_account: *accounts.dispensing_custody_token_account.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PerpsV2SwapKeys> for [AccountMeta; PERPS_V2_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: PerpsV2SwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.receiving_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.dispensing_custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PERPS_V2_SWAP_IX_ACCOUNTS_LEN]> for PerpsV2SwapKeys {
    fn from(pubkeys: [Pubkey; PERPS_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            owner: pubkeys[1],
            funding_account: pubkeys[2],
            receiving_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            receiving_custody: pubkeys[7],
            receiving_custody_doves_price_account: pubkeys[8],
            receiving_custody_pythnet_price_account: pubkeys[9],
            receiving_custody_token_account: pubkeys[10],
            dispensing_custody: pubkeys[11],
            dispensing_custody_doves_price_account: pubkeys[12],
            dispensing_custody_pythnet_price_account: pubkeys[13],
            dispensing_custody_token_account: pubkeys[14],
            token_program: pubkeys[15],
            event_authority: pubkeys[16],
            program: pubkeys[17],
        }
    }
}
impl<'info> From<PerpsV2SwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; PERPS_V2_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PerpsV2SwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.owner.clone(),
            accounts.funding_account.clone(),
            accounts.receiving_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.receiving_custody.clone(),
            accounts.receiving_custody_doves_price_account.clone(),
            accounts.receiving_custody_pythnet_price_account.clone(),
            accounts.receiving_custody_token_account.clone(),
            accounts.dispensing_custody.clone(),
            accounts.dispensing_custody_doves_price_account.clone(),
            accounts.dispensing_custody_pythnet_price_account.clone(),
            accounts.dispensing_custody_token_account.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PERPS_V2_SWAP_IX_ACCOUNTS_LEN]>
    for PerpsV2SwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PERPS_V2_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            owner: &arr[1],
            funding_account: &arr[2],
            receiving_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            receiving_custody: &arr[7],
            receiving_custody_doves_price_account: &arr[8],
            receiving_custody_pythnet_price_account: &arr[9],
            receiving_custody_token_account: &arr[10],
            dispensing_custody: &arr[11],
            dispensing_custody_doves_price_account: &arr[12],
            dispensing_custody_pythnet_price_account: &arr[13],
            dispensing_custody_token_account: &arr[14],
            token_program: &arr[15],
            event_authority: &arr[16],
            program: &arr[17],
        }
    }
}
pub const PERPS_V2_SWAP_IX_DISCM: [u8; 8] = [127, 245, 19, 158, 82, 250, 33, 18];
#[derive(Clone, Debug, PartialEq)]
pub struct PerpsV2SwapIxData;
impl PerpsV2SwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERPS_V2_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERPS_V2_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERPS_V2_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn perps_v2_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: PerpsV2SwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERPS_V2_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PerpsV2SwapIxData.try_to_vec()?,
    })
}
pub fn perps_v2_swap_ix(keys: PerpsV2SwapKeys) -> std::io::Result<Instruction> {
    perps_v2_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn perps_v2_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PerpsV2SwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PerpsV2SwapKeys = accounts.into();
    let ix = perps_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn perps_v2_swap_invoke(accounts: PerpsV2SwapAccounts<'_, '_>) -> ProgramResult {
    perps_v2_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn perps_v2_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PerpsV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PerpsV2SwapKeys = accounts.into();
    let ix = perps_v2_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn perps_v2_swap_invoke_signed(
    accounts: PerpsV2SwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    perps_v2_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn perps_v2_swap_verify_account_keys(
    accounts: PerpsV2SwapAccounts<'_, '_>,
    keys: PerpsV2SwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.owner.key, keys.owner),
        (*accounts.funding_account.key, keys.funding_account),
        (*accounts.receiving_account.key, keys.receiving_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.receiving_custody.key, keys.receiving_custody),
        (
            *accounts.receiving_custody_doves_price_account.key,
            keys.receiving_custody_doves_price_account,
        ),
        (
            *accounts.receiving_custody_pythnet_price_account.key,
            keys.receiving_custody_pythnet_price_account,
        ),
        (
            *accounts.receiving_custody_token_account.key,
            keys.receiving_custody_token_account,
        ),
        (*accounts.dispensing_custody.key, keys.dispensing_custody),
        (
            *accounts.dispensing_custody_doves_price_account.key,
            keys.dispensing_custody_doves_price_account,
        ),
        (
            *accounts.dispensing_custody_pythnet_price_account.key,
            keys.dispensing_custody_pythnet_price_account,
        ),
        (
            *accounts.dispensing_custody_token_account.key,
            keys.dispensing_custody_token_account,
        ),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn perps_v2_swap_verify_writable_privileges<'me, 'info>(
    accounts: PerpsV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funding_account,
        accounts.receiving_account,
        accounts.pool,
        accounts.receiving_custody,
        accounts.receiving_custody_token_account,
        accounts.dispensing_custody,
        accounts.dispensing_custody_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn perps_v2_swap_verify_account_privileges<'me, 'info>(
    accounts: PerpsV2SwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    perps_v2_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct PerpsV2AddLiquidityAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_or_receiving_account: &'me AccountInfo<'info>,
    pub lp_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerpsV2AddLiquidityKeys {
    pub swap_program: Pubkey,
    pub owner: Pubkey,
    pub funding_or_receiving_account: Pubkey,
    pub lp_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub custody_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PerpsV2AddLiquidityAccounts<'_, '_>> for PerpsV2AddLiquidityKeys {
    fn from(accounts: PerpsV2AddLiquidityAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            owner: *accounts.owner.key,
            funding_or_receiving_account: *accounts.funding_or_receiving_account.key,
            lp_token_account: *accounts.lp_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            custody_token_account: *accounts.custody_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PerpsV2AddLiquidityKeys> for [AccountMeta; PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: PerpsV2AddLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_or_receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for PerpsV2AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            owner: pubkeys[1],
            funding_or_receiving_account: pubkeys[2],
            lp_token_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            custody: pubkeys[7],
            custody_doves_price_account: pubkeys[8],
            custody_pythnet_price_account: pubkeys[9],
            custody_token_account: pubkeys[10],
            lp_token_mint: pubkeys[11],
            token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<PerpsV2AddLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PerpsV2AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.owner.clone(),
            accounts.funding_or_receiving_account.clone(),
            accounts.lp_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.custody_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for PerpsV2AddLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            owner: &arr[1],
            funding_or_receiving_account: &arr[2],
            lp_token_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            custody: &arr[7],
            custody_doves_price_account: &arr[8],
            custody_pythnet_price_account: &arr[9],
            custody_token_account: &arr[10],
            lp_token_mint: &arr[11],
            token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const PERPS_V2_ADD_LIQUIDITY_IX_DISCM: [u8; 8] = [18, 66, 88, 194, 197, 52, 116, 212];
#[derive(Clone, Debug, PartialEq)]
pub struct PerpsV2AddLiquidityIxData;
impl PerpsV2AddLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERPS_V2_ADD_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERPS_V2_ADD_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERPS_V2_ADD_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn perps_v2_add_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: PerpsV2AddLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERPS_V2_ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PerpsV2AddLiquidityIxData.try_to_vec()?,
    })
}
pub fn perps_v2_add_liquidity_ix(keys: PerpsV2AddLiquidityKeys) -> std::io::Result<Instruction> {
    perps_v2_add_liquidity_ix_with_program_id(JUPITER_ID, keys)
}
pub fn perps_v2_add_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PerpsV2AddLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PerpsV2AddLiquidityKeys = accounts.into();
    let ix = perps_v2_add_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn perps_v2_add_liquidity_invoke(
    accounts: PerpsV2AddLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    perps_v2_add_liquidity_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn perps_v2_add_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PerpsV2AddLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PerpsV2AddLiquidityKeys = accounts.into();
    let ix = perps_v2_add_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn perps_v2_add_liquidity_invoke_signed(
    accounts: PerpsV2AddLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    perps_v2_add_liquidity_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn perps_v2_add_liquidity_verify_account_keys(
    accounts: PerpsV2AddLiquidityAccounts<'_, '_>,
    keys: PerpsV2AddLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.owner.key, keys.owner),
        (
            *accounts.funding_or_receiving_account.key,
            keys.funding_or_receiving_account,
        ),
        (*accounts.lp_token_account.key, keys.lp_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (
            *accounts.custody_doves_price_account.key,
            keys.custody_doves_price_account,
        ),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (
            *accounts.custody_token_account.key,
            keys.custody_token_account,
        ),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn perps_v2_add_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: PerpsV2AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funding_or_receiving_account,
        accounts.lp_token_account,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn perps_v2_add_liquidity_verify_account_privileges<'me, 'info>(
    accounts: PerpsV2AddLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    perps_v2_add_liquidity_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct PerpsV2RemoveLiquidityAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub funding_or_receiving_account: &'me AccountInfo<'info>,
    pub lp_token_account: &'me AccountInfo<'info>,
    pub transfer_authority: &'me AccountInfo<'info>,
    pub perpetuals: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub custody: &'me AccountInfo<'info>,
    pub custody_doves_price_account: &'me AccountInfo<'info>,
    pub custody_pythnet_price_account: &'me AccountInfo<'info>,
    pub custody_token_account: &'me AccountInfo<'info>,
    pub lp_token_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerpsV2RemoveLiquidityKeys {
    pub swap_program: Pubkey,
    pub owner: Pubkey,
    pub funding_or_receiving_account: Pubkey,
    pub lp_token_account: Pubkey,
    pub transfer_authority: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub custody: Pubkey,
    pub custody_doves_price_account: Pubkey,
    pub custody_pythnet_price_account: Pubkey,
    pub custody_token_account: Pubkey,
    pub lp_token_mint: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<PerpsV2RemoveLiquidityAccounts<'_, '_>> for PerpsV2RemoveLiquidityKeys {
    fn from(accounts: PerpsV2RemoveLiquidityAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            owner: *accounts.owner.key,
            funding_or_receiving_account: *accounts.funding_or_receiving_account.key,
            lp_token_account: *accounts.lp_token_account.key,
            transfer_authority: *accounts.transfer_authority.key,
            perpetuals: *accounts.perpetuals.key,
            pool: *accounts.pool.key,
            custody: *accounts.custody.key,
            custody_doves_price_account: *accounts.custody_doves_price_account.key,
            custody_pythnet_price_account: *accounts.custody_pythnet_price_account.key,
            custody_token_account: *accounts.custody_token_account.key,
            lp_token_mint: *accounts.lp_token_mint.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<PerpsV2RemoveLiquidityKeys> for [AccountMeta; PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: PerpsV2RemoveLiquidityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.funding_or_receiving_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transfer_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.perpetuals,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.custody_doves_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_pythnet_price_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.custody_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_token_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for PerpsV2RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            owner: pubkeys[1],
            funding_or_receiving_account: pubkeys[2],
            lp_token_account: pubkeys[3],
            transfer_authority: pubkeys[4],
            perpetuals: pubkeys[5],
            pool: pubkeys[6],
            custody: pubkeys[7],
            custody_doves_price_account: pubkeys[8],
            custody_pythnet_price_account: pubkeys[9],
            custody_token_account: pubkeys[10],
            lp_token_mint: pubkeys[11],
            token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<PerpsV2RemoveLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: PerpsV2RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.owner.clone(),
            accounts.funding_or_receiving_account.clone(),
            accounts.lp_token_account.clone(),
            accounts.transfer_authority.clone(),
            accounts.perpetuals.clone(),
            accounts.pool.clone(),
            accounts.custody.clone(),
            accounts.custody_doves_price_account.clone(),
            accounts.custody_pythnet_price_account.clone(),
            accounts.custody_token_account.clone(),
            accounts.lp_token_mint.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for PerpsV2RemoveLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            owner: &arr[1],
            funding_or_receiving_account: &arr[2],
            lp_token_account: &arr[3],
            transfer_authority: &arr[4],
            perpetuals: &arr[5],
            pool: &arr[6],
            custody: &arr[7],
            custody_doves_price_account: &arr[8],
            custody_pythnet_price_account: &arr[9],
            custody_token_account: &arr[10],
            lp_token_mint: &arr[11],
            token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const PERPS_V2_REMOVE_LIQUIDITY_IX_DISCM: [u8; 8] = [16, 103, 98, 99, 106, 36, 5, 105];
#[derive(Clone, Debug, PartialEq)]
pub struct PerpsV2RemoveLiquidityIxData;
impl PerpsV2RemoveLiquidityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != PERPS_V2_REMOVE_LIQUIDITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PERPS_V2_REMOVE_LIQUIDITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&PERPS_V2_REMOVE_LIQUIDITY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn perps_v2_remove_liquidity_ix_with_program_id(
    program_id: Pubkey,
    keys: PerpsV2RemoveLiquidityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; PERPS_V2_REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: PerpsV2RemoveLiquidityIxData.try_to_vec()?,
    })
}
pub fn perps_v2_remove_liquidity_ix(
    keys: PerpsV2RemoveLiquidityKeys,
) -> std::io::Result<Instruction> {
    perps_v2_remove_liquidity_ix_with_program_id(JUPITER_ID, keys)
}
pub fn perps_v2_remove_liquidity_invoke_with_program_id(
    program_id: Pubkey,
    accounts: PerpsV2RemoveLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: PerpsV2RemoveLiquidityKeys = accounts.into();
    let ix = perps_v2_remove_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn perps_v2_remove_liquidity_invoke(
    accounts: PerpsV2RemoveLiquidityAccounts<'_, '_>,
) -> ProgramResult {
    perps_v2_remove_liquidity_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn perps_v2_remove_liquidity_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: PerpsV2RemoveLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: PerpsV2RemoveLiquidityKeys = accounts.into();
    let ix = perps_v2_remove_liquidity_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn perps_v2_remove_liquidity_invoke_signed(
    accounts: PerpsV2RemoveLiquidityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    perps_v2_remove_liquidity_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn perps_v2_remove_liquidity_verify_account_keys(
    accounts: PerpsV2RemoveLiquidityAccounts<'_, '_>,
    keys: PerpsV2RemoveLiquidityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.owner.key, keys.owner),
        (
            *accounts.funding_or_receiving_account.key,
            keys.funding_or_receiving_account,
        ),
        (*accounts.lp_token_account.key, keys.lp_token_account),
        (*accounts.transfer_authority.key, keys.transfer_authority),
        (*accounts.perpetuals.key, keys.perpetuals),
        (*accounts.pool.key, keys.pool),
        (*accounts.custody.key, keys.custody),
        (
            *accounts.custody_doves_price_account.key,
            keys.custody_doves_price_account,
        ),
        (
            *accounts.custody_pythnet_price_account.key,
            keys.custody_pythnet_price_account,
        ),
        (
            *accounts.custody_token_account.key,
            keys.custody_token_account,
        ),
        (*accounts.lp_token_mint.key, keys.lp_token_mint),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn perps_v2_remove_liquidity_verify_writable_privileges<'me, 'info>(
    accounts: PerpsV2RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.funding_or_receiving_account,
        accounts.lp_token_account,
        accounts.pool,
        accounts.custody,
        accounts.custody_token_account,
        accounts.lp_token_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn perps_v2_remove_liquidity_verify_account_privileges<'me, 'info>(
    accounts: PerpsV2RemoveLiquidityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    perps_v2_remove_liquidity_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct MoonshotWrappedBuyAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
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
    pub user_wsol_token_account: &'me AccountInfo<'info>,
    pub temp_wsol_token_account: &'me AccountInfo<'info>,
    pub wsol_mint: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MoonshotWrappedBuyKeys {
    pub swap_program: Pubkey,
    pub sender: Pubkey,
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
    pub user_wsol_token_account: Pubkey,
    pub temp_wsol_token_account: Pubkey,
    pub wsol_mint: Pubkey,
}
impl From<MoonshotWrappedBuyAccounts<'_, '_>> for MoonshotWrappedBuyKeys {
    fn from(accounts: MoonshotWrappedBuyAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            sender: *accounts.sender.key,
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
            user_wsol_token_account: *accounts.user_wsol_token_account.key,
            temp_wsol_token_account: *accounts.temp_wsol_token_account.key,
            wsol_mint: *accounts.wsol_mint.key,
        }
    }
}
impl From<MoonshotWrappedBuyKeys> for [AccountMeta; MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN] {
    fn from(keys: MoonshotWrappedBuyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: false,
                is_writable: true,
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
            AccountMeta {
                pubkey: keys.user_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.temp_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.wsol_mint,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN]> for MoonshotWrappedBuyKeys {
    fn from(pubkeys: [Pubkey; MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            sender: pubkeys[1],
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
            user_wsol_token_account: pubkeys[12],
            temp_wsol_token_account: pubkeys[13],
            wsol_mint: pubkeys[14],
        }
    }
}
impl<'info> From<MoonshotWrappedBuyAccounts<'_, 'info>>
    for [AccountInfo<'info>; MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MoonshotWrappedBuyAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.sender.clone(),
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
            accounts.user_wsol_token_account.clone(),
            accounts.temp_wsol_token_account.clone(),
            accounts.wsol_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN]>
    for MoonshotWrappedBuyAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            sender: &arr[1],
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
            user_wsol_token_account: &arr[12],
            temp_wsol_token_account: &arr[13],
            wsol_mint: &arr[14],
        }
    }
}
pub const MOONSHOT_WRAPPED_BUY_IX_DISCM: [u8; 8] = [207, 150, 213, 156, 138, 104, 238, 142];
#[derive(Clone, Debug, PartialEq)]
pub struct MoonshotWrappedBuyIxData;
impl MoonshotWrappedBuyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MOONSHOT_WRAPPED_BUY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MOONSHOT_WRAPPED_BUY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MOONSHOT_WRAPPED_BUY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn moonshot_wrapped_buy_ix_with_program_id(
    program_id: Pubkey,
    keys: MoonshotWrappedBuyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MOONSHOT_WRAPPED_BUY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MoonshotWrappedBuyIxData.try_to_vec()?,
    })
}
pub fn moonshot_wrapped_buy_ix(keys: MoonshotWrappedBuyKeys) -> std::io::Result<Instruction> {
    moonshot_wrapped_buy_ix_with_program_id(JUPITER_ID, keys)
}
pub fn moonshot_wrapped_buy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MoonshotWrappedBuyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MoonshotWrappedBuyKeys = accounts.into();
    let ix = moonshot_wrapped_buy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn moonshot_wrapped_buy_invoke(accounts: MoonshotWrappedBuyAccounts<'_, '_>) -> ProgramResult {
    moonshot_wrapped_buy_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn moonshot_wrapped_buy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MoonshotWrappedBuyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MoonshotWrappedBuyKeys = accounts.into();
    let ix = moonshot_wrapped_buy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn moonshot_wrapped_buy_invoke_signed(
    accounts: MoonshotWrappedBuyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    moonshot_wrapped_buy_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn moonshot_wrapped_buy_verify_account_keys(
    accounts: MoonshotWrappedBuyAccounts<'_, '_>,
    keys: MoonshotWrappedBuyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.sender.key, keys.sender),
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
        (
            *accounts.user_wsol_token_account.key,
            keys.user_wsol_token_account,
        ),
        (
            *accounts.temp_wsol_token_account.key,
            keys.temp_wsol_token_account,
        ),
        (*accounts.wsol_mint.key, keys.wsol_mint),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn moonshot_wrapped_buy_verify_writable_privileges<'me, 'info>(
    accounts: MoonshotWrappedBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.sender,
        accounts.sender_token_account,
        accounts.curve_account,
        accounts.curve_token_account,
        accounts.dex_fee,
        accounts.helio_fee,
        accounts.user_wsol_token_account,
        accounts.temp_wsol_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn moonshot_wrapped_buy_verify_account_privileges<'me, 'info>(
    accounts: MoonshotWrappedBuyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    moonshot_wrapped_buy_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct MoonshotWrappedSellAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub sender: &'me AccountInfo<'info>,
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
    pub user_wsol_token_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MoonshotWrappedSellKeys {
    pub swap_program: Pubkey,
    pub sender: Pubkey,
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
    pub user_wsol_token_account: Pubkey,
}
impl From<MoonshotWrappedSellAccounts<'_, '_>> for MoonshotWrappedSellKeys {
    fn from(accounts: MoonshotWrappedSellAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            sender: *accounts.sender.key,
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
            user_wsol_token_account: *accounts.user_wsol_token_account.key,
        }
    }
}
impl From<MoonshotWrappedSellKeys> for [AccountMeta; MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN] {
    fn from(keys: MoonshotWrappedSellKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.sender,
                is_signer: false,
                is_writable: true,
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
            AccountMeta {
                pubkey: keys.user_wsol_token_account,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN]> for MoonshotWrappedSellKeys {
    fn from(pubkeys: [Pubkey; MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            sender: pubkeys[1],
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
            user_wsol_token_account: pubkeys[12],
        }
    }
}
impl<'info> From<MoonshotWrappedSellAccounts<'_, 'info>>
    for [AccountInfo<'info>; MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: MoonshotWrappedSellAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.sender.clone(),
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
            accounts.user_wsol_token_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN]>
    for MoonshotWrappedSellAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            sender: &arr[1],
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
            user_wsol_token_account: &arr[12],
        }
    }
}
pub const MOONSHOT_WRAPPED_SELL_IX_DISCM: [u8; 8] = [248, 2, 240, 253, 17, 184, 57, 8];
#[derive(Clone, Debug, PartialEq)]
pub struct MoonshotWrappedSellIxData;
impl MoonshotWrappedSellIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != MOONSHOT_WRAPPED_SELL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MOONSHOT_WRAPPED_SELL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&MOONSHOT_WRAPPED_SELL_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn moonshot_wrapped_sell_ix_with_program_id(
    program_id: Pubkey,
    keys: MoonshotWrappedSellKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; MOONSHOT_WRAPPED_SELL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: MoonshotWrappedSellIxData.try_to_vec()?,
    })
}
pub fn moonshot_wrapped_sell_ix(keys: MoonshotWrappedSellKeys) -> std::io::Result<Instruction> {
    moonshot_wrapped_sell_ix_with_program_id(JUPITER_ID, keys)
}
pub fn moonshot_wrapped_sell_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MoonshotWrappedSellAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MoonshotWrappedSellKeys = accounts.into();
    let ix = moonshot_wrapped_sell_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn moonshot_wrapped_sell_invoke(
    accounts: MoonshotWrappedSellAccounts<'_, '_>,
) -> ProgramResult {
    moonshot_wrapped_sell_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn moonshot_wrapped_sell_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MoonshotWrappedSellAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MoonshotWrappedSellKeys = accounts.into();
    let ix = moonshot_wrapped_sell_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn moonshot_wrapped_sell_invoke_signed(
    accounts: MoonshotWrappedSellAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    moonshot_wrapped_sell_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn moonshot_wrapped_sell_verify_account_keys(
    accounts: MoonshotWrappedSellAccounts<'_, '_>,
    keys: MoonshotWrappedSellKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.sender.key, keys.sender),
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
        (
            *accounts.user_wsol_token_account.key,
            keys.user_wsol_token_account,
        ),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn moonshot_wrapped_sell_verify_writable_privileges<'me, 'info>(
    accounts: MoonshotWrappedSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.sender,
        accounts.sender_token_account,
        accounts.curve_account,
        accounts.curve_token_account,
        accounts.dex_fee,
        accounts.helio_fee,
        accounts.user_wsol_token_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn moonshot_wrapped_sell_verify_account_privileges<'me, 'info>(
    accounts: MoonshotWrappedSellAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    moonshot_wrapped_sell_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct StabbleStableSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub vault_token_in: &'me AccountInfo<'info>,
    pub vault_token_out: &'me AccountInfo<'info>,
    pub beneficiary_token_out: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StabbleStableSwapKeys {
    pub swap_program: Pubkey,
    pub user: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub vault_token_in: Pubkey,
    pub vault_token_out: Pubkey,
    pub beneficiary_token_out: Pubkey,
    pub pool: Pubkey,
    pub withdraw_authority: Pubkey,
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<StabbleStableSwapAccounts<'_, '_>> for StabbleStableSwapKeys {
    fn from(accounts: StabbleStableSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            user: *accounts.user.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            vault_token_in: *accounts.vault_token_in.key,
            vault_token_out: *accounts.vault_token_out.key,
            beneficiary_token_out: *accounts.beneficiary_token_out.key,
            pool: *accounts.pool.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            vault: *accounts.vault.key,
            vault_authority: *accounts.vault_authority.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<StabbleStableSwapKeys> for [AccountMeta; STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: StabbleStableSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.beneficiary_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN]> for StabbleStableSwapKeys {
    fn from(pubkeys: [Pubkey; STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            user: pubkeys[1],
            user_token_in: pubkeys[2],
            user_token_out: pubkeys[3],
            vault_token_in: pubkeys[4],
            vault_token_out: pubkeys[5],
            beneficiary_token_out: pubkeys[6],
            pool: pubkeys[7],
            withdraw_authority: pubkeys[8],
            vault: pubkeys[9],
            vault_authority: pubkeys[10],
            vault_program: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<StabbleStableSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: StabbleStableSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.user.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.vault_token_in.clone(),
            accounts.vault_token_out.clone(),
            accounts.beneficiary_token_out.clone(),
            accounts.pool.clone(),
            accounts.withdraw_authority.clone(),
            accounts.vault.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN]>
    for StabbleStableSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            user: &arr[1],
            user_token_in: &arr[2],
            user_token_out: &arr[3],
            vault_token_in: &arr[4],
            vault_token_out: &arr[5],
            beneficiary_token_out: &arr[6],
            pool: &arr[7],
            withdraw_authority: &arr[8],
            vault: &arr[9],
            vault_authority: &arr[10],
            vault_program: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const STABBLE_STABLE_SWAP_IX_DISCM: [u8; 8] = [144, 73, 163, 148, 143, 34, 40, 144];
#[derive(Clone, Debug, PartialEq)]
pub struct StabbleStableSwapIxData;
impl StabbleStableSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STABBLE_STABLE_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STABBLE_STABLE_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STABBLE_STABLE_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stabble_stable_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: StabbleStableSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STABBLE_STABLE_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: StabbleStableSwapIxData.try_to_vec()?,
    })
}
pub fn stabble_stable_swap_ix(keys: StabbleStableSwapKeys) -> std::io::Result<Instruction> {
    stabble_stable_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn stabble_stable_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StabbleStableSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: StabbleStableSwapKeys = accounts.into();
    let ix = stabble_stable_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn stabble_stable_swap_invoke(accounts: StabbleStableSwapAccounts<'_, '_>) -> ProgramResult {
    stabble_stable_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn stabble_stable_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StabbleStableSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StabbleStableSwapKeys = accounts.into();
    let ix = stabble_stable_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stabble_stable_swap_invoke_signed(
    accounts: StabbleStableSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stabble_stable_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn stabble_stable_swap_verify_account_keys(
    accounts: StabbleStableSwapAccounts<'_, '_>,
    keys: StabbleStableSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.user.key, keys.user),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.vault_token_in.key, keys.vault_token_in),
        (*accounts.vault_token_out.key, keys.vault_token_out),
        (
            *accounts.beneficiary_token_out.key,
            keys.beneficiary_token_out,
        ),
        (*accounts.pool.key, keys.pool),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stabble_stable_swap_verify_writable_privileges<'me, 'info>(
    accounts: StabbleStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.vault_token_in,
        accounts.vault_token_out,
        accounts.beneficiary_token_out,
        accounts.pool,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stabble_stable_swap_verify_account_privileges<'me, 'info>(
    accounts: StabbleStableSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stabble_stable_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct StabbleWeightedSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub user_token_in: &'me AccountInfo<'info>,
    pub user_token_out: &'me AccountInfo<'info>,
    pub vault_token_in: &'me AccountInfo<'info>,
    pub vault_token_out: &'me AccountInfo<'info>,
    pub beneficiary_token_out: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub withdraw_authority: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub vault_authority: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StabbleWeightedSwapKeys {
    pub swap_program: Pubkey,
    pub user: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub vault_token_in: Pubkey,
    pub vault_token_out: Pubkey,
    pub beneficiary_token_out: Pubkey,
    pub pool: Pubkey,
    pub withdraw_authority: Pubkey,
    pub vault: Pubkey,
    pub vault_authority: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<StabbleWeightedSwapAccounts<'_, '_>> for StabbleWeightedSwapKeys {
    fn from(accounts: StabbleWeightedSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            user: *accounts.user.key,
            user_token_in: *accounts.user_token_in.key,
            user_token_out: *accounts.user_token_out.key,
            vault_token_in: *accounts.vault_token_in.key,
            vault_token_out: *accounts.vault_token_out.key,
            beneficiary_token_out: *accounts.beneficiary_token_out.key,
            pool: *accounts.pool.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            vault: *accounts.vault.key,
            vault_authority: *accounts.vault_authority.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<StabbleWeightedSwapKeys> for [AccountMeta; STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: StabbleWeightedSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_in,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.beneficiary_token_out,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN]> for StabbleWeightedSwapKeys {
    fn from(pubkeys: [Pubkey; STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            user: pubkeys[1],
            user_token_in: pubkeys[2],
            user_token_out: pubkeys[3],
            vault_token_in: pubkeys[4],
            vault_token_out: pubkeys[5],
            beneficiary_token_out: pubkeys[6],
            pool: pubkeys[7],
            withdraw_authority: pubkeys[8],
            vault: pubkeys[9],
            vault_authority: pubkeys[10],
            vault_program: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<StabbleWeightedSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: StabbleWeightedSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.user.clone(),
            accounts.user_token_in.clone(),
            accounts.user_token_out.clone(),
            accounts.vault_token_in.clone(),
            accounts.vault_token_out.clone(),
            accounts.beneficiary_token_out.clone(),
            accounts.pool.clone(),
            accounts.withdraw_authority.clone(),
            accounts.vault.clone(),
            accounts.vault_authority.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN]>
    for StabbleWeightedSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            user: &arr[1],
            user_token_in: &arr[2],
            user_token_out: &arr[3],
            vault_token_in: &arr[4],
            vault_token_out: &arr[5],
            beneficiary_token_out: &arr[6],
            pool: &arr[7],
            withdraw_authority: &arr[8],
            vault: &arr[9],
            vault_authority: &arr[10],
            vault_program: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const STABBLE_WEIGHTED_SWAP_IX_DISCM: [u8; 8] = [94, 214, 232, 111, 142, 61, 123, 29];
#[derive(Clone, Debug, PartialEq)]
pub struct StabbleWeightedSwapIxData;
impl StabbleWeightedSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STABBLE_WEIGHTED_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STABBLE_WEIGHTED_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STABBLE_WEIGHTED_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stabble_weighted_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: StabbleWeightedSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STABBLE_WEIGHTED_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: StabbleWeightedSwapIxData.try_to_vec()?,
    })
}
pub fn stabble_weighted_swap_ix(keys: StabbleWeightedSwapKeys) -> std::io::Result<Instruction> {
    stabble_weighted_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn stabble_weighted_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StabbleWeightedSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: StabbleWeightedSwapKeys = accounts.into();
    let ix = stabble_weighted_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn stabble_weighted_swap_invoke(
    accounts: StabbleWeightedSwapAccounts<'_, '_>,
) -> ProgramResult {
    stabble_weighted_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn stabble_weighted_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StabbleWeightedSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StabbleWeightedSwapKeys = accounts.into();
    let ix = stabble_weighted_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stabble_weighted_swap_invoke_signed(
    accounts: StabbleWeightedSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stabble_weighted_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn stabble_weighted_swap_verify_account_keys(
    accounts: StabbleWeightedSwapAccounts<'_, '_>,
    keys: StabbleWeightedSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.user.key, keys.user),
        (*accounts.user_token_in.key, keys.user_token_in),
        (*accounts.user_token_out.key, keys.user_token_out),
        (*accounts.vault_token_in.key, keys.vault_token_in),
        (*accounts.vault_token_out.key, keys.vault_token_out),
        (
            *accounts.beneficiary_token_out.key,
            keys.beneficiary_token_out,
        ),
        (*accounts.pool.key, keys.pool),
        (*accounts.withdraw_authority.key, keys.withdraw_authority),
        (*accounts.vault.key, keys.vault),
        (*accounts.vault_authority.key, keys.vault_authority),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stabble_weighted_swap_verify_writable_privileges<'me, 'info>(
    accounts: StabbleWeightedSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.user_token_in,
        accounts.user_token_out,
        accounts.vault_token_in,
        accounts.vault_token_out,
        accounts.beneficiary_token_out,
        accounts.pool,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stabble_weighted_swap_verify_account_privileges<'me, 'info>(
    accounts: StabbleWeightedSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stabble_weighted_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const OBRIC_SWAP_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct ObricSwapAccounts<'me, 'info> {
    pub swap_program: &'me AccountInfo<'info>,
    pub trading_pair: &'me AccountInfo<'info>,
    pub mint_x: &'me AccountInfo<'info>,
    pub mint_y: &'me AccountInfo<'info>,
    pub reserve_x: &'me AccountInfo<'info>,
    pub reserve_y: &'me AccountInfo<'info>,
    pub user_token_account_x: &'me AccountInfo<'info>,
    pub user_token_account_y: &'me AccountInfo<'info>,
    pub protocol_fee: &'me AccountInfo<'info>,
    pub x_price_feed: &'me AccountInfo<'info>,
    pub y_price_feed: &'me AccountInfo<'info>,
    pub user: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ObricSwapKeys {
    pub swap_program: Pubkey,
    pub trading_pair: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub user_token_account_x: Pubkey,
    pub user_token_account_y: Pubkey,
    pub protocol_fee: Pubkey,
    pub x_price_feed: Pubkey,
    pub y_price_feed: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
}
impl From<ObricSwapAccounts<'_, '_>> for ObricSwapKeys {
    fn from(accounts: ObricSwapAccounts) -> Self {
        Self {
            swap_program: *accounts.swap_program.key,
            trading_pair: *accounts.trading_pair.key,
            mint_x: *accounts.mint_x.key,
            mint_y: *accounts.mint_y.key,
            reserve_x: *accounts.reserve_x.key,
            reserve_y: *accounts.reserve_y.key,
            user_token_account_x: *accounts.user_token_account_x.key,
            user_token_account_y: *accounts.user_token_account_y.key,
            protocol_fee: *accounts.protocol_fee.key,
            x_price_feed: *accounts.x_price_feed.key,
            y_price_feed: *accounts.y_price_feed.key,
            user: *accounts.user.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<ObricSwapKeys> for [AccountMeta; OBRIC_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: ObricSwapKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.swap_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.trading_pair,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.mint_x,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.mint_y,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account_x,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_token_account_y,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.protocol_fee,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.x_price_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.y_price_feed,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.user,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; OBRIC_SWAP_IX_ACCOUNTS_LEN]> for ObricSwapKeys {
    fn from(pubkeys: [Pubkey; OBRIC_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: pubkeys[0],
            trading_pair: pubkeys[1],
            mint_x: pubkeys[2],
            mint_y: pubkeys[3],
            reserve_x: pubkeys[4],
            reserve_y: pubkeys[5],
            user_token_account_x: pubkeys[6],
            user_token_account_y: pubkeys[7],
            protocol_fee: pubkeys[8],
            x_price_feed: pubkeys[9],
            y_price_feed: pubkeys[10],
            user: pubkeys[11],
            token_program: pubkeys[12],
        }
    }
}
impl<'info> From<ObricSwapAccounts<'_, 'info>>
    for [AccountInfo<'info>; OBRIC_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: ObricSwapAccounts<'_, 'info>) -> Self {
        [
            accounts.swap_program.clone(),
            accounts.trading_pair.clone(),
            accounts.mint_x.clone(),
            accounts.mint_y.clone(),
            accounts.reserve_x.clone(),
            accounts.reserve_y.clone(),
            accounts.user_token_account_x.clone(),
            accounts.user_token_account_y.clone(),
            accounts.protocol_fee.clone(),
            accounts.x_price_feed.clone(),
            accounts.y_price_feed.clone(),
            accounts.user.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; OBRIC_SWAP_IX_ACCOUNTS_LEN]>
    for ObricSwapAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; OBRIC_SWAP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            swap_program: &arr[0],
            trading_pair: &arr[1],
            mint_x: &arr[2],
            mint_y: &arr[3],
            reserve_x: &arr[4],
            reserve_y: &arr[5],
            user_token_account_x: &arr[6],
            user_token_account_y: &arr[7],
            protocol_fee: &arr[8],
            x_price_feed: &arr[9],
            y_price_feed: &arr[10],
            user: &arr[11],
            token_program: &arr[12],
        }
    }
}
pub const OBRIC_SWAP_IX_DISCM: [u8; 8] = [65, 93, 96, 169, 190, 214, 95, 3];
#[derive(Clone, Debug, PartialEq)]
pub struct ObricSwapIxData;
impl ObricSwapIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != OBRIC_SWAP_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    OBRIC_SWAP_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&OBRIC_SWAP_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn obric_swap_ix_with_program_id(
    program_id: Pubkey,
    keys: ObricSwapKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; OBRIC_SWAP_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ObricSwapIxData.try_to_vec()?,
    })
}
pub fn obric_swap_ix(keys: ObricSwapKeys) -> std::io::Result<Instruction> {
    obric_swap_ix_with_program_id(JUPITER_ID, keys)
}
pub fn obric_swap_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ObricSwapAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ObricSwapKeys = accounts.into();
    let ix = obric_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn obric_swap_invoke(accounts: ObricSwapAccounts<'_, '_>) -> ProgramResult {
    obric_swap_invoke_with_program_id(JUPITER_ID, accounts)
}
pub fn obric_swap_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ObricSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ObricSwapKeys = accounts.into();
    let ix = obric_swap_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn obric_swap_invoke_signed(
    accounts: ObricSwapAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    obric_swap_invoke_signed_with_program_id(JUPITER_ID, accounts, seeds)
}
pub fn obric_swap_verify_account_keys(
    accounts: ObricSwapAccounts<'_, '_>,
    keys: ObricSwapKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.swap_program.key, keys.swap_program),
        (*accounts.trading_pair.key, keys.trading_pair),
        (*accounts.mint_x.key, keys.mint_x),
        (*accounts.mint_y.key, keys.mint_y),
        (*accounts.reserve_x.key, keys.reserve_x),
        (*accounts.reserve_y.key, keys.reserve_y),
        (
            *accounts.user_token_account_x.key,
            keys.user_token_account_x,
        ),
        (
            *accounts.user_token_account_y.key,
            keys.user_token_account_y,
        ),
        (*accounts.protocol_fee.key, keys.protocol_fee),
        (*accounts.x_price_feed.key, keys.x_price_feed),
        (*accounts.y_price_feed.key, keys.y_price_feed),
        (*accounts.user.key, keys.user),
        (*accounts.token_program.key, keys.token_program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn obric_swap_verify_writable_privileges<'me, 'info>(
    accounts: ObricSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.trading_pair,
        accounts.reserve_x,
        accounts.reserve_y,
        accounts.user_token_account_x,
        accounts.user_token_account_y,
        accounts.protocol_fee,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn obric_swap_verify_account_privileges<'me, 'info>(
    accounts: ObricSwapAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    obric_swap_verify_writable_privileges(accounts)?;
    Ok(())
}
