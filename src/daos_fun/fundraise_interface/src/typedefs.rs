use borsh::{BorshDeserialize, BorshSerialize};
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FundraiseState {
    pub admin: pubkey,
    pub token_mint: pubkey,
    pub funding_mint: pubkey,
    pub token_deposit: u64,
    pub funding_goal: u64,
    pub expiration_timestamp: i64,
    pub funding_received: u64,
    pub is_funded: bool,
    pub is_finalized: bool,
    pub recipient: pubkey,
}
