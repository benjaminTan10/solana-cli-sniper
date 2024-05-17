//! Instruction types

#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]

// use crate::instruction::state::{AmmParams, Fees, LastOrderDistance, SimulateParams};
use arrayref::array_ref;
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar,
};
use solana_sdk::{
    commitment_config::CommitmentConfig, compute_budget::ComputeBudgetInstruction, pubkey,
};
use std::convert::TryInto;
use std::mem::size_of;

use crate::raydium::{
    swap::instructions::SOLC_MINT,
    utils::utils::{
        program_address, LIQUIDITY_STATE_LAYOUT_V4, MARKET_STATE_LAYOUT_V3, SPL_MINT_LAYOUT,
    },
};

use super::error::AmmError;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction2 {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
    /// init token pc amount
    pub init_pc_amount: u64,
    /// init token coin amount
    pub init_coin_amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PreInitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MonitorStepInstruction {
    /// max value of plan/new/cancel orders
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DepositInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WithdrawInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub amount: u64,
}

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Default, PartialEq)]
// pub struct SetParamsInstruction {
//     pub param: u8,
//     pub value: Option<u64>,
//     pub new_pubkey: Option<Pubkey>,
//     pub fees: Option<Fees>,
//     pub last_order_distance: Option<LastOrderDistance>,
// }

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WithdrawSrmInstruction {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseIn {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseOut {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub max_amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimulateInstruction {
    pub param: u8,
    pub swap_base_in_value: Option<SwapInstructionBaseIn>,
    pub swap_base_out_value: Option<SwapInstructionBaseOut>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AdminCancelOrdersInstruction {
    pub limit: u16,
}

/// Update config acccount params
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ConfigArgs {
    pub param: u8,
    pub owner: Option<Pubkey>,
    pub create_pool_fee: Option<u64>,
}

/// Instructions supported by the AmmInfo program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum AmmInstruction {
    ///   Initializes a new AmmInfo.
    ///
    ///   Not supported yet, please use `Initialize2` to new a AMM pool
    #[deprecated(note = "Not supported yet, please use `Initialize2` instead")]
    Initialize(InitializeInstruction),

    ///   Initializes a new AMM pool.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Associated Token program id
    ///   2. `[]` Sys program id
    ///   3. `[]` Rent program id
    ///   4. `[writable]` New AMM Account to create.
    ///   5. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   6. `[writable]` AMM open orders Account
    ///   7. `[writable]` AMM lp mint Account
    ///   8. `[]` AMM coin mint Account
    ///   9. `[]` AMM pc mint Account
    ///   10. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
    ///   11. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
    ///   12. `[writable]` AMM target orders Account. To store plan orders informations.
    ///   13. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
    ///   14. `[]` AMM create pool fee destination Account
    ///   15. `[]` Market program id
    ///   16. `[writable]` Market Account. Market program is the owner.
    ///   17. `[writable, singer]` User wallet Account
    ///   18. `[]` User token coin Account
    ///   19. '[]` User token pc Account
    ///   20. `[writable]` User destination lp token ATA Account
    Initialize2(InitializeInstruction2),

    ///   MonitorStep. To monitor place Amm order state machine turn around step by step.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Rent program id
    ///   2. `[]` Sys Clock id
    ///   3. `[writable]` AMM Account
    ///   4. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   5. `[writable]` AMM open orders Account
    ///   6. `[writable]` AMM target orders Account. To store plan orders infomations.
    ///   7. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
    ///   8. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
    ///   9. `[]` Market program id
    ///   10. `[writable]` Market Account. Market program is the owner.
    ///   11. `[writable]` Market coin vault Account
    ///   12. `[writable]` Market pc vault Account
    ///   13. '[]` Market vault signer Account
    ///   14. '[writable]` Market request queue Account
    ///   15. `[writable]` Market event queue Account
    ///   16. `[writable]` Market bids Account
    ///   17. `[writable]` Market asks Account
    ///   18. `[writable]` (optional) the (M)SRM account used for fee discounts
    ///   19. `[writable]` (optional) the referrer pc account used for settle back referrer
    MonitorStep(MonitorStepInstruction),

    ///   Deposit some tokens into the pool.  The output is a "pool" token representing ownership
    ///   into the pool. Inputs are converted to the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[]` AMM open_orders Account
    ///   4. `[writable]` AMM target orders Account. To store plan orders infomations.
    ///   5. `[writable]` AMM lp mint Account. Owned by $authority.
    ///   6. `[writable]` AMM coin vault $authority can transfer amount,
    ///   7. `[writable]` AMM pc vault $authority can transfer amount,
    ///   8. `[]` Market Account. Market program is the owner.
    ///   9. `[writable]` User coin token Account to deposit into.
    ///   10. `[writable]` User pc token Account to deposit into.
    ///   11. `[writable]` User lp token. To deposit the generated tokens, user is the owner.
    ///   12. '[signer]` User wallet Account
    ///   13. `[]` Market event queue Account.
    Deposit(DepositInstruction),

    ///   Withdraw the vault tokens from the pool at the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` AMM target orders Account
    ///   5. `[writable]` AMM lp mint Account. Owned by $authority.
    ///   6. `[writable]` AMM coin vault Account to withdraw FROM,
    ///   7. `[writable]` AMM pc vault Account to withdraw FROM,
    ///   8. `[]` Market program id
    ///   9. `[writable]` Market Account. Market program is the owner.
    ///   10. `[writable]` Market coin vault Account
    ///   11. `[writable]` Market pc vault Account
    ///   12. '[]` Market vault signer Account
    ///   13. `[writable]` User lp token Account.
    ///   14. `[writable]` User token coin Account. user Account to credit.
    ///   15. `[writable]` User token pc Account. user Account to credit.
    ///   16. `[singer]` User wallet Account
    ///   17. `[writable]` Market event queue Account
    ///   18. `[writable]` Market bids Account
    ///   19. `[writable]` Market asks Account
    Withdraw(WithdrawInstruction),

    ///   Migrate the associated market from Serum to OpenBook.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Sys program id
    ///   2. `[]` Rent program id
    ///   3. `[writable]` AMM Account
    ///   4. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   5. `[writable]` AMM open orders Account
    ///   6. `[writable]` AMM coin vault account owned by $authority,
    ///   7. `[writable]` AMM pc vault account owned by $authority,
    ///   8. `[writable]` AMM target orders Account
    ///   9. `[]` Market program id
    ///   10. `[writable]` Market Account. Market program is the owner.
    ///   11. `[writable]` Market bids Account
    ///   12. `[writable]` Market asks Account
    ///   13. `[writable]` Market event queue Account
    ///   14. `[writable]` Market coin vault Account
    ///   15. `[writable]` Market pc vault Account
    ///   16. '[]` Market vault signer Account
    ///   17. '[writable]` AMM new open orders Account
    ///   18. '[]` mew Market program id
    ///   19. '[]` new Market market Account
    ///   20. '[]` Admin Account
    MigrateToOpenBook,

    ///   Set AMM params
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account.
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` AMM target orders Account
    ///   5. `[writable]` AMM coin vault account owned by $authority,
    ///   6. `[writable]` AMM pc vault account owned by $authority,
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market coin vault Account
    ///   10. `[writable]` Market pc vault Account
    ///   11. '[]` Market vault signer Account
    ///   12. `[writable]` Market event queue Account
    ///   13. `[writable]` Market bids Account
    ///   14. `[writable]` Market asks Account
    ///   15. `[singer]` Admin Account
    ///   16. `[]` (optional) New AMM open orders Account to replace old AMM open orders Account
    // SetParams(SetParamsInstruction),

    ///   Withdraw Pnl from pool by protocol
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
    ///   3. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   4. `[writable]` AMM open orders Account
    ///   5. `[writable]` AMM coin vault account to withdraw FROM,
    ///   6. `[writable]` AMM pc vault account to withdraw FROM,
    ///   7. `[writable]` User coin token Account to withdraw to
    ///   8. `[writable]` User pc token Account to withdraw to
    ///   9. `[singer]` User wallet account
    ///   10. `[writable]` AMM target orders Account
    ///   11. `[]` Market program id
    ///   12. `[writable]` Market Account. Market program is the owner.
    ///   13. `[writable]` Market event queue Account
    ///   14. `[writable]` Market coin vault Account
    ///   15. `[writable]` Market pc vault Account
    ///   16. '[]` Market vault signer Account
    ///   17. `[]` (optional) the referrer pc account used for settle back referrer
    WithdrawPnl,

    ///   Withdraw (M)SRM from the (M)SRM Account used for fee discounts by admin
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` AMM Account.
    ///   2. `[singer]` Admin wallet Account
    ///   3. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   4. `[writable]` the (M)SRM Account withdraw from
    ///   5. `[writable]` the (M)SRM Account withdraw to
    WithdrawSrm(WithdrawSrmInstruction),

    /// Swap coin or pc from pool, base amount_in with a slippage of minimum_amount_out
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` (optional)AMM target orders Account, no longer used in the contract, recommended no need to add this Account.
    ///   5. `[writable]` AMM coin vault Account to swap FROM or To.
    ///   6. `[writable]` AMM pc vault Account to swap FROM or To.
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market bids Account
    ///   10. `[writable]` Market asks Account
    ///   11. `[writable]` Market event queue Account
    ///   12. `[writable]` Market coin vault Account
    ///   13. `[writable]` Market pc vault Account
    ///   14. '[]` Market vault signer Account
    ///   15. `[writable]` User source token Account.
    ///   16. `[writable]` User destination token Account.
    ///   17. `[singer]` User wallet Account
    SwapBaseIn(SwapInstructionBaseIn),

    ///   Continue Initializes a new Amm pool because of compute units limit.
    ///   Not supported yet, please use `Initialize2` to new a Amm pool
    #[deprecated(note = "Not supported yet, please use `Initialize2` instead")]
    PreInitialize(PreInitializeInstruction),

    /// Swap coin or pc from pool, base amount_out with a slippage of max_amount_in
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` (optional)AMM target orders Account, no longer used in the contract, recommended no need to add this Account.
    ///   5. `[writable]` AMM coin vault Account to swap FROM or To.
    ///   6. `[writable]` AMM pc vault Account to swap FROM or To.
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market bids Account
    ///   10. `[writable]` Market asks Account
    ///   11. `[writable]` Market event queue Account
    ///   12. `[writable]` Market coin vault Account
    ///   13. `[writable]` Market pc vault Account
    ///   14. '[]` Market vault signer Account
    ///   15. `[writable]` User source token Account.
    ///   16. `[writable]` User destination token Account.
    ///   17. `[singer]` User wallet Account
    SwapBaseOut(SwapInstructionBaseOut),

    AdminCancelOrders(AdminCancelOrdersInstruction),

    /// Create amm config account by admin
    CreateConfigAccount,

    /// Update amm config account by admin
    UpdateConfigAccount(ConfigArgs),
}

impl AmmInstruction {
    /// Unpacks a byte buffer into a [AmmInstruction](enum.AmmInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (nonce, rest) = Self::unpack_u8(rest)?;
                let (open_time, _reset) = Self::unpack_u64(rest)?;
                Self::Initialize(InitializeInstruction { nonce, open_time })
            }
            1 => {
                let (nonce, rest) = Self::unpack_u8(rest)?;
                let (open_time, rest) = Self::unpack_u64(rest)?;
                let (init_pc_amount, rest) = Self::unpack_u64(rest)?;
                let (init_coin_amount, _reset) = Self::unpack_u64(rest)?;
                Self::Initialize2(InitializeInstruction2 {
                    nonce,
                    open_time,
                    init_pc_amount,
                    init_coin_amount,
                })
            }
            2 => {
                let (plan_order_limit, rest) = Self::unpack_u16(rest)?;
                let (place_order_limit, rest) = Self::unpack_u16(rest)?;
                let (cancel_order_limit, _rest) = Self::unpack_u16(rest)?;
                Self::MonitorStep(MonitorStepInstruction {
                    plan_order_limit,
                    place_order_limit,
                    cancel_order_limit,
                })
            }
            3 => {
                let (max_coin_amount, rest) = Self::unpack_u64(rest)?;
                let (max_pc_amount, rest) = Self::unpack_u64(rest)?;
                let (base_side, _rest) = Self::unpack_u64(rest)?;
                Self::Deposit(DepositInstruction {
                    max_coin_amount,
                    max_pc_amount,
                    base_side,
                })
            }
            4 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::Withdraw(WithdrawInstruction { amount })
            }
            5 => Self::MigrateToOpenBook,

            7 => Self::WithdrawPnl,
            8 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawSrm(WithdrawSrmInstruction { amount })
            }
            9 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::SwapBaseIn(SwapInstructionBaseIn {
                    amount_in,
                    minimum_amount_out,
                })
            }
            10 => {
                let (nonce, _rest) = Self::unpack_u8(rest)?;
                Self::PreInitialize(PreInitializeInstruction { nonce })
            }
            11 => {
                let (max_amount_in, rest) = Self::unpack_u64(rest)?;
                let (amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::SwapBaseOut(SwapInstructionBaseOut {
                    max_amount_in,
                    amount_out,
                })
            }

            13 => {
                let (limit, _rest) = Self::unpack_u16(rest)?;
                Self::AdminCancelOrders(AdminCancelOrdersInstruction { limit })
            }
            14 => Self::CreateConfigAccount,
            15 => {
                let (param, rest) = Self::unpack_u8(rest)?;
                match param {
                    0 | 1 => {
                        let pubkey = array_ref![rest, 0, 32];
                        Self::UpdateConfigAccount(ConfigArgs {
                            param,
                            owner: Some(Pubkey::new_from_array(*pubkey)),
                            create_pool_fee: None,
                        })
                    }
                    2 => {
                        let (create_pool_fee, _rest) = Self::unpack_u64(rest)?;
                        Self::UpdateConfigAccount(ConfigArgs {
                            param,
                            owner: None,
                            create_pool_fee: Some(create_pool_fee),
                        })
                    }
                    _ => {
                        return Err(ProgramError::InvalidInstructionData.into());
                    }
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        })
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.len() >= 1 {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if input.len() >= 2 {
            let (amount, rest) = input.split_at(2);
            let amount = amount
                .get(..2)
                .and_then(|slice| slice.try_into().ok())
                .map(u16::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    /// Packs a [AmmInstruction](enum.AmmInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::Initialize(InitializeInstruction { nonce, open_time }) => {
                buf.push(0);
                buf.push(*nonce);
                buf.extend_from_slice(&open_time.to_le_bytes());
            }
            Self::Initialize2(InitializeInstruction2 {
                nonce,
                open_time,
                init_pc_amount,
                init_coin_amount,
            }) => {
                buf.push(1);
                buf.push(*nonce);
                buf.extend_from_slice(&open_time.to_le_bytes());
                buf.extend_from_slice(&init_pc_amount.to_le_bytes());
                buf.extend_from_slice(&init_coin_amount.to_le_bytes());
            }
            Self::MonitorStep(MonitorStepInstruction {
                plan_order_limit,
                place_order_limit,
                cancel_order_limit,
            }) => {
                buf.push(2);
                buf.extend_from_slice(&plan_order_limit.to_le_bytes());
                buf.extend_from_slice(&place_order_limit.to_le_bytes());
                buf.extend_from_slice(&cancel_order_limit.to_le_bytes());
            }
            Self::Deposit(DepositInstruction {
                max_coin_amount,
                max_pc_amount,
                base_side,
            }) => {
                buf.push(3);
                buf.extend_from_slice(&max_coin_amount.to_le_bytes());
                buf.extend_from_slice(&max_pc_amount.to_le_bytes());
                buf.extend_from_slice(&base_side.to_le_bytes());
            }
            Self::Withdraw(WithdrawInstruction { amount }) => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::MigrateToOpenBook => {
                buf.push(5);
            }

            Self::WithdrawPnl => {
                buf.push(7);
            }
            Self::WithdrawSrm(WithdrawSrmInstruction { amount }) => {
                buf.push(8);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::SwapBaseIn(SwapInstructionBaseIn {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(9);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::PreInitialize(PreInitializeInstruction { nonce }) => {
                buf.push(10);
                buf.push(*nonce);
            }
            Self::SwapBaseOut(SwapInstructionBaseOut {
                max_amount_in,
                amount_out,
            }) => {
                buf.push(11);
                buf.extend_from_slice(&max_amount_in.to_le_bytes());
                buf.extend_from_slice(&amount_out.to_le_bytes());
            }

            Self::AdminCancelOrders(AdminCancelOrdersInstruction { limit }) => {
                buf.push(13);
                buf.extend_from_slice(&limit.to_le_bytes());
            }
            Self::CreateConfigAccount => {
                buf.push(14);
            }
            Self::UpdateConfigAccount(ConfigArgs {
                param,
                owner,
                create_pool_fee,
            }) => {
                buf.push(15);
                buf.push(*param);
                match param {
                    0 | 1 => {
                        let owner = match owner {
                            Some(owner) => {
                                if *owner == Pubkey::default() {
                                    return Err(ProgramError::InvalidInstructionData.into());
                                } else {
                                    owner
                                }
                            }
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&owner.to_bytes());
                    }
                    2 => {
                        let create_pool_fee = match create_pool_fee {
                            Some(create_pool_fee) => create_pool_fee,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&create_pool_fee.to_le_bytes());
                    }
                    _ => return Err(ProgramError::InvalidInstructionData.into()),
                }
            }
        }
        Ok(buf)
    }
}

pub fn initialize_amm_pool(
    amm_program: &Pubkey,
    amm_keys: &AmmKeys,
    create_fee_detination: &Pubkey,
    user_owner: &Pubkey,
    user_coin: &Pubkey,
    user_pc: &Pubkey,
    user_lp: &Pubkey,
    open_time: u64,   // default is 0, or set a future time on the chain can start swap
    pc_amount: u64,   // transfer pc asset to the pool pc vault as pool init vault
    coin_amount: u64, // transfer coin asset to the pool coin vault as pool init vault
) -> eyre::Result<Instruction> {
    println!("Coin: {}\nPC: {}\nLP: {}", user_coin, user_pc, user_lp);
    let amm_pool_init_instruction = initialize2(
        &amm_program,
        &amm_keys.amm_pool,
        &amm_keys.amm_authority,
        &amm_keys.amm_open_order,
        &amm_keys.amm_lp_mint,
        &amm_keys.amm_coin_mint,
        &amm_keys.amm_pc_mint,
        &amm_keys.amm_coin_vault,
        &amm_keys.amm_pc_vault,
        &amm_keys.amm_target,
        &Pubkey::find_program_address(&[&AMM_CONFIG_SEED], &amm_program).0,
        create_fee_detination,
        &amm_keys.market_program,
        &amm_keys.market,
        &user_owner,
        &user_coin,
        &user_pc,
        &user_lp,
        amm_keys.nonce,
        open_time,
        pc_amount,
        coin_amount,
    )?;
    Ok(amm_pool_init_instruction)
}

/// Creates an 'initialize2' instruction.
pub fn initialize2(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_lp_mint: &Pubkey,
    amm_coin_mint: &Pubkey,
    amm_pc_mint: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_config: &Pubkey,
    create_fee_destination: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    user_wallet: &Pubkey,
    user_token_coin: &Pubkey,
    user_token_pc: &Pubkey,
    user_token_lp: &Pubkey,
    nonce: u8,
    open_time: u64,
    init_pc_amount: u64,
    init_coin_amount: u64,
) -> Result<Instruction, ProgramError> {
    let init_data = AmmInstruction::Initialize2(InitializeInstruction2 {
        nonce,
        open_time,
        init_pc_amount,
        init_coin_amount,
    });
    let data = init_data.pack()?;

    let accounts = vec![
        // spl & sys
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_lp_mint, false),
        AccountMeta::new_readonly(*amm_coin_mint, false),
        AccountMeta::new_readonly(*amm_pc_mint, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new_readonly(*amm_config, false),
        AccountMeta::new(*create_fee_destination, false),
        // market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new_readonly(*market, false),
        // user wallet
        AccountMeta::new(*user_wallet, true),
        AccountMeta::new(*user_token_coin, false),
        AccountMeta::new(*user_token_pc, false),
        AccountMeta::new(*user_token_lp, false),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'deposit' instruction.
pub fn deposit(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_lp_mint: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    market: &Pubkey,
    market_event_queue: &Pubkey,
    user_token_coin: &Pubkey,
    user_token_pc: &Pubkey,
    user_token_lp: &Pubkey,
    user_owner: &Pubkey,
    max_coin_amount: u64,
    max_pc_amount: u64,
    base_side: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Deposit(DepositInstruction {
        max_coin_amount,
        max_pc_amount,
        base_side,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new_readonly(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_lp_mint, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        // market
        AccountMeta::new_readonly(*market, false),
        // user
        AccountMeta::new(*user_token_coin, false),
        AccountMeta::new(*user_token_pc, false),
        AccountMeta::new(*user_token_lp, false),
        AccountMeta::new_readonly(*user_owner, true),
        AccountMeta::new_readonly(*market_event_queue, false),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'withdraw' instruction.
pub fn withdraw(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_lp_mint: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
    user_token_lp: &Pubkey,
    user_token_coin: &Pubkey,
    user_token_pc: &Pubkey,
    user_owner: &Pubkey,
    market_event_queue: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,

    referrer_pc_account: Option<&Pubkey>,

    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::Withdraw(WithdrawInstruction { amount }).pack()?;

    let mut accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_lp_mint, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        // market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
        // user
        AccountMeta::new(*user_token_lp, false),
        AccountMeta::new(*user_token_coin, false),
        AccountMeta::new(*user_token_pc, false),
        AccountMeta::new_readonly(*user_owner, true),
        AccountMeta::new(*market_event_queue, false),
        AccountMeta::new(*market_bids, false),
        AccountMeta::new(*market_asks, false),
    ];

    if let Some(referrer_pc_key) = referrer_pc_account {
        accounts.push(AccountMeta::new(*referrer_pc_key, false));
    }

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'swap base in' instruction.
pub fn swap_base_in(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,
    market_event_queue: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
    user_token_source: &Pubkey,
    user_token_destination: &Pubkey,
    user_source_owner: &Pubkey,

    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::SwapBaseIn(SwapInstructionBaseIn {
        amount_in,
        minimum_amount_out,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        // market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new(*market_bids, false),
        AccountMeta::new(*market_asks, false),
        AccountMeta::new(*market_event_queue, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
        // user
        AccountMeta::new(*user_token_source, false),
        AccountMeta::new(*user_token_destination, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'swap base out' instruction.
pub fn swap_base_out(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,
    market_event_queue: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
    user_token_source: &Pubkey,
    user_token_destination: &Pubkey,
    user_source_owner: &Pubkey,

    max_amount_in: u64,
    amount_out: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::SwapBaseOut(SwapInstructionBaseOut {
        max_amount_in,
        amount_out,
    })
    .pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        // AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        // market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new(*market_bids, false),
        AccountMeta::new(*market_asks, false),
        AccountMeta::new(*market_event_queue, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
        // user
        AccountMeta::new(*user_token_source, false),
        AccountMeta::new(*user_token_destination, false),
        AccountMeta::new_readonly(*user_source_owner, true),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'migrate_to_openbook' instruction.
pub fn migrate_to_openbook(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    amm_target_orders: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,
    market_event_queue: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,

    new_amm_open_orders: &Pubkey,
    new_market_program: &Pubkey,
    new_market: &Pubkey,

    admin: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::MigrateToOpenBook.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new(*amm_target_orders, false),
        // old market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new(*market_bids, false),
        AccountMeta::new(*market_asks, false),
        AccountMeta::new(*market_event_queue, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
        // new market
        AccountMeta::new(*new_amm_open_orders, false),
        AccountMeta::new_readonly(*new_market_program, false),
        AccountMeta::new_readonly(*new_market, false),
        // admin
        AccountMeta::new(*admin, true),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'withdrawpnl' instruction
pub fn withdrawpnl(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_config: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    user_token_coin: &Pubkey,
    user_token_pc: &Pubkey,
    user_owner: &Pubkey,
    amm_target_orders: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    market_event_queue: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::WithdrawPnl.pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_config, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new(*user_token_coin, false),
        AccountMeta::new(*user_token_pc, false),
        AccountMeta::new_readonly(*user_owner, true),
        AccountMeta::new(*amm_target_orders, false),
        // serum
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new_readonly(*market_event_queue, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'monitor_step' instruction.
pub fn monitor_step(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    amm_token_srm: Option<Pubkey>,
    market_program: &Pubkey,
    market: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
    market_request_queue: &Pubkey,
    market_event_queue: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,
    referrer_token_pc: Option<Pubkey>,

    plan_order_limit: u16,
    place_order_limit: u16,
    cancel_order_limit: u16,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::MonitorStep(MonitorStepInstruction {
        plan_order_limit,
        place_order_limit,
        cancel_order_limit,
    })
    .pack()?;

    let mut accounts = vec![
        // spl
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        // amm
        AccountMeta::new(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        // market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
        AccountMeta::new(*market_request_queue, false),
        AccountMeta::new(*market_event_queue, false),
        AccountMeta::new(*market_bids, false),
        AccountMeta::new(*market_asks, false),
    ];

    if let Some(token_srm) = amm_token_srm {
        accounts.push(AccountMeta::new(token_srm, false));
        if let Some(referrer_pc) = referrer_token_pc {
            accounts.push(AccountMeta::new(referrer_pc, false));
        }
    }

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates a 'withdrawsrm' instruction
pub fn withdrawsrm(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    admin: &Pubkey,
    token_srm: &Pubkey,
    dest_token_srm: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::WithdrawSrm(WithdrawSrmInstruction { amount }).pack()?;

    let accounts = vec![
        // spl token
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new_readonly(*amm_pool, false),
        AccountMeta::new_readonly(*admin, true),
        AccountMeta::new_readonly(*amm_authority, false),
        // market
        AccountMeta::new(*token_srm, false),
        AccountMeta::new(*dest_token_srm, false),
    ];

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

pub fn admin_cancel_orders(
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    amm_coin_vault: &Pubkey,
    amm_pc_vault: &Pubkey,
    amm_cancel_owner: &Pubkey,
    amm_config: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    market_coin_vault: &Pubkey,
    market_pc_vault: &Pubkey,
    market_vault_signer: &Pubkey,
    market_event_queue: &Pubkey,
    market_bids: &Pubkey,
    market_asks: &Pubkey,
    amm_token_srm: Option<Pubkey>,
    referrer_token_pc: Option<Pubkey>,
    cancel_order_limit: u16,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::AdminCancelOrders(AdminCancelOrdersInstruction {
        limit: cancel_order_limit,
    })
    .pack()?;

    let mut accounts = vec![
        // spl
        AccountMeta::new_readonly(spl_token::id(), false),
        // amm
        AccountMeta::new_readonly(*amm_pool, false),
        AccountMeta::new_readonly(*amm_authority, false),
        AccountMeta::new(*amm_open_orders, false),
        AccountMeta::new(*amm_target_orders, false),
        AccountMeta::new(*amm_coin_vault, false),
        AccountMeta::new(*amm_pc_vault, false),
        AccountMeta::new_readonly(*amm_cancel_owner, true),
        AccountMeta::new(*amm_config, false),
        // market
        AccountMeta::new_readonly(*market_program, false),
        AccountMeta::new(*market, false),
        AccountMeta::new(*market_coin_vault, false),
        AccountMeta::new(*market_pc_vault, false),
        AccountMeta::new_readonly(*market_vault_signer, false),
        AccountMeta::new(*market_event_queue, false),
        AccountMeta::new(*market_bids, false),
        AccountMeta::new(*market_asks, false),
    ];

    if let Some(token_srm) = amm_token_srm {
        accounts.push(AccountMeta::new(token_srm, false));
        if let Some(referrer_pc) = referrer_token_pc {
            accounts.push(AccountMeta::new(referrer_pc, false));
        }
    }

    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates an 'create_config_account' instruction.
pub fn create_config_account(
    amm_program: &Pubkey,
    admin: &Pubkey,
    amm_config: &Pubkey,
    pnl_owner: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::CreateConfigAccount.pack()?;
    let accounts = vec![
        AccountMeta::new(*admin, true),
        AccountMeta::new(*amm_config, false),
        AccountMeta::new_readonly(*pnl_owner, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

/// Creates an 'update_config_account' instruction.
pub fn update_config_account(
    amm_program: &Pubkey,
    admin: &Pubkey,
    amm_config: &Pubkey,
    config_args: ConfigArgs,
) -> Result<Instruction, ProgramError> {
    let data = AmmInstruction::UpdateConfigAccount(config_args).pack()?;
    let accounts = vec![
        AccountMeta::new_readonly(*admin, true),
        AccountMeta::new(*amm_config, false),
    ];
    Ok(Instruction {
        program_id: *amm_program,
        accounts,
        data,
    })
}

pub fn compute_ixs(priority_fee: u64, limit: u32) -> Result<Vec<Instruction>, ProgramError> {
    let mut ixs = vec![];

    let unit_limit = ComputeBudgetInstruction::set_compute_unit_limit(limit);
    let compute_price = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);

    ixs.push(unit_limit);
    ixs.push(compute_price);

    Ok(ixs)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoolKeysSniper {
    pub id: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub lp_decimals: u8,
    pub version: u8,
    pub program_id: Pubkey,
    pub authority: Pubkey,
    pub open_orders: Pubkey,
    pub target_orders: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub withdraw_queue: Pubkey,
    pub lp_vault: Pubkey,
    pub market_version: u8,
    pub market_program_id: Pubkey,
    pub market_id: Pubkey,
    pub market_authority: Pubkey,
    pub market_base_vault: Pubkey,
    pub market_quote_vault: Pubkey,
    pub market_bids: Pubkey,
    pub market_asks: Pubkey,
    pub market_event_queue: Pubkey,
    pub lookup_table_account: Pubkey,
}

// only use for initialize_amm_pool, because the keys of some amm pools are not used in this way.
pub fn get_amm_pda_keys(
    amm_program: &Pubkey,
    market_program: &Pubkey,
    market: &Pubkey,
    coin_mint: &Pubkey,
    pc_mint: &Pubkey,
) -> AmmKeys {
    let amm_pool = get_associated_address_and_bump_seed(
        &amm_program,
        &market,
        AMM_ASSOCIATED_SEED,
        &amm_program,
    )
    .0;
    let (amm_authority, nonce) = Pubkey::find_program_address(&[AUTHORITY_AMM], &amm_program);
    let amm_open_order = get_associated_address_and_bump_seed(
        &amm_program,
        &market,
        OPEN_ORDER_ASSOCIATED_SEED,
        &amm_program,
    )
    .0;
    let amm_lp_mint = get_associated_address_and_bump_seed(
        &amm_program,
        &market,
        LP_MINT_ASSOCIATED_SEED,
        &amm_program,
    )
    .0;
    let amm_coin_vault = get_associated_address_and_bump_seed(
        &amm_program,
        &market,
        COIN_VAULT_ASSOCIATED_SEED,
        &amm_program,
    )
    .0;
    let amm_pc_vault = get_associated_address_and_bump_seed(
        &amm_program,
        &market,
        PC_VAULT_ASSOCIATED_SEED,
        &amm_program,
    )
    .0;
    let amm_target = get_associated_address_and_bump_seed(
        &amm_program,
        &market,
        TARGET_ASSOCIATED_SEED,
        &amm_program,
    )
    .0;

    AmmKeys {
        amm_pool,
        amm_target,
        amm_coin_vault,
        amm_pc_vault,
        amm_lp_mint,
        amm_open_order,
        amm_coin_mint: *coin_mint,
        amm_pc_mint: *pc_mint,
        amm_authority,
        market: *market,
        market_program: *market_program,
        nonce,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AmmKeys {
    pub amm_pool: Pubkey,
    pub amm_coin_mint: Pubkey,
    pub amm_pc_mint: Pubkey,
    pub amm_authority: Pubkey,
    pub amm_target: Pubkey,
    pub amm_coin_vault: Pubkey,
    pub amm_pc_vault: Pubkey,
    pub amm_lp_mint: Pubkey,
    pub amm_open_order: Pubkey,
    pub market_program: Pubkey,
    pub market: Pubkey,
    pub nonce: u8,
}

/// Suffix for amm authority seed
pub const AUTHORITY_AMM: &'static [u8] = b"amm authority";
/// Suffix for amm associated seed
pub const AMM_ASSOCIATED_SEED: &'static [u8] = b"amm_associated_seed";
/// Suffix for target associated seed
pub const TARGET_ASSOCIATED_SEED: &'static [u8] = b"target_associated_seed";
/// Suffix for amm open order associated seed
pub const OPEN_ORDER_ASSOCIATED_SEED: &'static [u8] = b"open_order_associated_seed";
/// Suffix for coin vault associated seed
pub const COIN_VAULT_ASSOCIATED_SEED: &'static [u8] = b"coin_vault_associated_seed";
/// Suffix for pc vault associated seed
pub const PC_VAULT_ASSOCIATED_SEED: &'static [u8] = b"pc_vault_associated_seed";
/// Suffix for lp mint associated seed
pub const LP_MINT_ASSOCIATED_SEED: &'static [u8] = b"lp_mint_associated_seed";
/// Amm config seed
pub const AMM_CONFIG_SEED: &'static [u8] = b"amm_config_account_seed";

pub fn get_associated_address_and_bump_seed(
    info_id: &Pubkey,
    market_address: &Pubkey,
    associated_seed: &[u8],
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &info_id.to_bytes(),
            &market_address.to_bytes(),
            &associated_seed,
        ],
        program_id,
    )
}

pub const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPubkeys {
    pub market: Box<Pubkey>,
    pub req_q: Box<Pubkey>,
    pub event_q: Box<Pubkey>,
    pub bids: Box<Pubkey>,
    pub asks: Box<Pubkey>,
    pub coin_vault: Box<Pubkey>,
    pub pc_vault: Box<Pubkey>,
    pub vault_signer_key: Box<Pubkey>,
    pub coin_mint: Box<Pubkey>,
    pub pc_mint: Box<Pubkey>,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
}

pub fn swap(
    amm_program: &Pubkey,
    amm_keys: &AmmKeys,
    market_keys: &MarketPubkeys,
    user_owner: &Pubkey,
    user_source: &Pubkey,
    user_destination: &Pubkey,
    amount_specified: u64,
    other_amount_threshold: u64,
    out: bool,
) -> eyre::Result<Instruction> {
    let mut swap_instruction;
    if out {
        swap_instruction = swap_base_in(
            &amm_program,
            &amm_keys.amm_pool,
            &amm_keys.amm_authority,
            &amm_keys.amm_open_order,
            &amm_keys.amm_target,
            &amm_keys.amm_coin_vault,
            &amm_keys.amm_pc_vault,
            &amm_keys.market_program,
            &amm_keys.market,
            &market_keys.bids,
            &market_keys.asks,
            &market_keys.event_q,
            &market_keys.coin_vault,
            &market_keys.pc_vault,
            &market_keys.vault_signer_key,
            user_destination,
            user_source,
            user_owner,
            amount_specified,
            other_amount_threshold,
        )?;
    } else {
        swap_instruction = swap_base_in(
            &amm_program,
            &amm_keys.amm_pool,
            &amm_keys.amm_authority,
            &amm_keys.amm_open_order,
            &amm_keys.amm_target,
            &amm_keys.amm_coin_vault,
            &amm_keys.amm_pc_vault,
            &amm_keys.market_program,
            &amm_keys.market,
            &market_keys.bids,
            &market_keys.asks,
            &market_keys.event_q,
            &market_keys.coin_vault,
            &market_keys.pc_vault,
            &market_keys.vault_signer_key,
            user_source,
            user_destination,
            user_owner,
            amount_specified,
            other_amount_threshold,
        )?;
    }
    Ok(swap_instruction)
}

#[cfg(target_endian = "little")]
pub async fn get_keys_for_market<'a>(
    client: &'a RpcClient,
    program_id: &'a Pubkey,
    market: &'a Pubkey,
) -> eyre::Result<MarketPubkeys> {
    use std::{borrow::Cow, convert::identity};

    use safe_transmute::{transmute_one_pedantic, transmute_one_to_bytes, transmute_to_bytes};
    use serum_dex::state::{gen_vault_signer_key, AccountFlag, Market, MarketState, MarketStateV2};

    let account_data: Vec<u8> = client.get_account_data(&market).await?;
    let words: Cow<[u64]> = remove_dex_account_padding(&account_data)?;
    let market_state: MarketState = {
        let account_flags = Market::account_flags(&account_data)?;
        if account_flags.intersects(AccountFlag::Permissioned) {
            log::info!("MarketStateV2");
            let state = transmute_one_pedantic::<MarketStateV2>(transmute_to_bytes(&words))
                .map_err(|e| e.without_src())?;
            state.check_flags(true)?;
            state.inner
        } else {
            log::info!("MarketStateV");
            let state = transmute_one_pedantic::<MarketState>(transmute_to_bytes(&words))
                .map_err(|e| e.without_src())?;
            state.check_flags(true)?;
            state
        }
    };
    let vault_signer_key =
        gen_vault_signer_key(market_state.vault_signer_nonce, market, program_id)?;
    assert_eq!(
        transmute_to_bytes(&identity(market_state.own_address)),
        market.as_ref()
    );
    Ok(MarketPubkeys {
        market: Box::new(*market),
        req_q: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.req_q))).unwrap(),
        ),
        event_q: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.event_q))).unwrap(),
        ),
        bids: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.bids))).unwrap(),
        ),
        asks: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.asks))).unwrap(),
        ),
        coin_vault: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.coin_vault))).unwrap(),
        ),
        pc_vault: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.pc_vault))).unwrap(),
        ),
        vault_signer_key: Box::new(vault_signer_key),
        coin_mint: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.coin_mint))).unwrap(),
        ),
        pc_mint: Box::new(
            Pubkey::try_from(transmute_one_to_bytes(&identity(market_state.pc_mint))).unwrap(),
        ),
        coin_lot_size: market_state.coin_lot_size,
        pc_lot_size: market_state.pc_lot_size,
    })
}
use eyre::format_err;

use std::borrow::Cow;

#[cfg(target_endian = "little")]
fn remove_dex_account_padding<'a>(data: &'a [u8]) -> eyre::Result<Cow<'a, [u64]>> {
    use safe_transmute::transmute_many_pedantic;
    use serum_dex::state::{ACCOUNT_HEAD_PADDING, ACCOUNT_TAIL_PADDING};
    let head = &data[..ACCOUNT_HEAD_PADDING.len()];
    if data.len() < ACCOUNT_HEAD_PADDING.len() + ACCOUNT_TAIL_PADDING.len() {
        return Err(format_err!(
            "dex account length {} is too small to contain valid padding",
            data.len()
        ));
    }
    if head != ACCOUNT_HEAD_PADDING {
        return Err(format_err!("dex account head padding mismatch"));
    }
    let tail = &data[data.len() - ACCOUNT_TAIL_PADDING.len()..];
    if tail != ACCOUNT_TAIL_PADDING {
        return Err(format_err!("dex account tail padding mismatch"));
    }
    let inner_data_range = ACCOUNT_HEAD_PADDING.len()..(data.len() - ACCOUNT_TAIL_PADDING.len());
    let inner: &'a [u8] = &data[inner_data_range];
    let words: Cow<'a, [u64]> = match transmute_many_pedantic::<u64>(inner) {
        Ok(word_slice) => Cow::Borrowed(word_slice),
        Err(transmute_error) => {
            let word_vec = transmute_error.copy().map_err(|e| e.without_src())?;
            Cow::Owned(word_vec)
        }
    };
    Ok(words)
}

pub async fn load_amm_keys(
    client: &RpcClient,
    amm_program: &Pubkey,
    amm_pool: &Pubkey,
) -> eyre::Result<AmmKeys> {
    let mut retries = 0;
    let max_retries = 1000;
    let mut account = None;

    while account.is_none() && retries < max_retries {
        match client.get_account(&amm_pool).await {
            Ok(acc) => account = Some(acc),
            Err(_) => {
                retries += 1;
                continue;
            }
        }
    }

    let account = match account {
        Some(acc) => acc,
        None => return Err(eyre::eyre!("Account not found after maximum retries")),
    };

    let data = account.clone().data;
    let mut info = LIQUIDITY_STATE_LAYOUT_V4::decode(&mut &data[..])?;

    if info.baseMint == SOLC_MINT {
        info.baseMint = info.quoteMint;
        info.quoteMint = SOLC_MINT;
    }

    Ok(AmmKeys {
        amm_pool: *amm_pool,
        amm_target: info.targetOrders,
        amm_coin_vault: info.baseVault,
        amm_pc_vault: info.quoteVault,
        amm_lp_mint: info.lpMint,
        amm_open_order: info.openOrders,
        amm_coin_mint: info.baseMint,
        amm_pc_mint: info.quoteMint,
        amm_authority: program_address(&account.owner).await?,
        market: info.marketId,
        market_program: info.marketProgramId,
        nonce: info.nonce as u8,
    })
}

pub fn authority_id(program_id: &Pubkey, amm_seed: &[u8], nonce: u8) -> Result<Pubkey, AmmError> {
    Pubkey::create_program_address(&[amm_seed, &[nonce]], program_id)
        .map_err(|_| AmmError::InvalidProgramAddress.into())
}

pub async fn get_account<T>(client: &RpcClient, addr: &Pubkey) -> eyre::Result<Option<T>>
where
    T: Clone,
{
    if let Some(account) = client
        .get_account_with_commitment(addr, CommitmentConfig::processed())
        .await?
        .value
    {
        let account_data = account.data.as_slice();
        let ret = unsafe { &*(&account_data[0] as *const u8 as *const T) };
        Ok(Some(ret.clone()))
    } else {
        Ok(None)
    }
}

#[cfg_attr(feature = "client", derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct AmmInfo {
    /// Initialized status.
    pub status: u64,
    /// Nonce used in program address.
    /// The program address is created deterministically with the nonce,
    /// amm program id, and amm account pubkey.  This program address has
    /// authority over the amm's token coin account, token pc account, and pool
    /// token mint.
    pub nonce: u64,
    /// max order count
    pub order_num: u64,
    /// within this range, 5 => 5% range
    pub depth: u64,
    /// coin decimal
    pub coin_decimals: u64,
    /// pc decimal
    pub pc_decimals: u64,
    /// amm machine state
    pub state: u64,
    /// amm reset_flag
    pub reset_flag: u64,
    /// min size 1->0.000001
    pub min_size: u64,
    /// vol_max_cut_ratio numerator, sys_decimal_value as denominator
    pub vol_max_cut_ratio: u64,
    /// amount wave numerator, sys_decimal_value as denominator
    pub amount_wave: u64,
    /// coinLotSize 1 -> 0.000001
    pub coin_lot_size: u64,
    /// pcLotSize 1 -> 0.000001
    pub pc_lot_size: u64,
    /// min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub min_price_multiplier: u64,
    /// max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
    pub max_price_multiplier: u64,
    /// system decimal value, used to normalize the value of coin and pc amount
    pub sys_decimal_value: u64,
    /// All fee information
    pub fees: Fees,
    /// Statistical data
    pub state_data: StateData,
    /// Coin vault
    pub coin_vault: Pubkey,
    /// Pc vault
    pub pc_vault: Pubkey,
    /// Coin vault mint
    pub coin_vault_mint: Pubkey,
    /// Pc vault mint
    pub pc_vault_mint: Pubkey,
    /// lp mint
    pub lp_mint: Pubkey,
    /// open_orders key
    pub open_orders: Pubkey,
    /// market key
    pub market: Pubkey,
    /// market program key
    pub market_program: Pubkey,
    /// target_orders key
    pub target_orders: Pubkey,
    /// padding
    pub padding1: [u64; 8],
    /// amm owner key
    pub amm_owner: Pubkey,
    /// pool lp amount
    pub lp_amount: u64,
    /// client order id
    pub client_order_id: u64,
    /// padding
    pub padding2: [u64; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StateData {
    /// delay to take pnl coin
    pub need_take_pnl_coin: u64,
    /// delay to take pnl pc
    pub need_take_pnl_pc: u64,
    /// total pnl pc
    pub total_pnl_pc: u64,
    /// total pnl coin
    pub total_pnl_coin: u64,
    /// ido pool open time
    pub pool_open_time: u64,
    /// padding for future updates
    pub padding: [u64; 2],
    /// switch from orderbookonly to init
    pub orderbook_to_init_time: u64,

    /// swap coin in amount
    pub swap_coin_in_amount: u128,
    /// swap pc out amount
    pub swap_pc_out_amount: u128,
    /// charge pc as swap fee while swap pc to coin
    pub swap_acc_pc_fee: u64,

    /// swap pc in amount
    pub swap_pc_in_amount: u128,
    /// swap coin out amount
    pub swap_coin_out_amount: u128,
    /// charge coin as swap fee while swap coin to pc
    pub swap_acc_coin_fee: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fees {
    /// numerator of the min_separate
    pub min_separate_numerator: u64,
    /// denominator of the min_separate
    pub min_separate_denominator: u64,

    /// numerator of the fee
    pub trade_fee_numerator: u64,
    /// denominator of the fee
    /// and 'trade_fee_denominator' must be equal to 'min_separate_denominator'
    pub trade_fee_denominator: u64,

    /// numerator of the pnl
    pub pnl_numerator: u64,
    /// denominator of the pnl
    pub pnl_denominator: u64,

    /// numerator of the swap_fee
    pub swap_fee_numerator: u64,
    /// denominator of the swap_fee
    pub swap_fee_denominator: u64,
}
