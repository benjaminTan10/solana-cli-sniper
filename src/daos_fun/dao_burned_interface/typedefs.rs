use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FundraiseState {
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub token_deposit: u64,
    pub funding_goal: u64,
    pub expiration_timestamp: i64,
    pub funding_received: u64,
    pub is_funded: bool,
    pub is_finalized: bool,
    pub recipient: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State {
    pub admin: Pubkey,
    pub dao_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub funding_goal: u64,
    pub admin_closed_fund: bool,
    pub redemption_started: bool,
    pub expiration_timestamp: i64,
    pub delegate_authorities: [Option<Pubkey>; 3],
    pub carry_basis: u16,
    pub fee_authority: Pubkey,
    pub curve_initialized: bool,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenAccountRedemption {
    pub total_amount: Option<u64>,
    pub total_distributed: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserDaoBurnRedeemed {
    pub dao_burn_redeemed: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserDaoBurned {
    pub dao_burned: u64,
}
