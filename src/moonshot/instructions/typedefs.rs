use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenMintParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub collateral_currency: u8,
    pub amount: u64,
    pub curve_type: u8,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TradeParams {
    pub amount: u64,
    pub collateral_amount: u64,
    pub slippage_bps: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigParams {
    pub migration_authority: Option<Pubkey>,
    pub backend_authority: Option<Pubkey>,
    pub config_authority: Option<Pubkey>,
    pub helio_fee: Option<Pubkey>,
    pub dex_fee: Option<Pubkey>,
    pub fee_bps: Option<u16>,
    pub dex_fee_share: Option<u8>,
    pub migration_fee: Option<u64>,
    pub marketcap_threshold: Option<u64>,
    pub marketcap_currency: Option<u8>,
    pub min_supported_decimal_places: Option<u8>,
    pub max_supported_decimal_places: Option<u8>,
    pub min_supported_token_supply: Option<u64>,
    pub max_supported_token_supply: Option<u64>,
    pub coef_b: Option<u32>,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Currency {
    Sol,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CurveType {
    LinearV1,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TradeType {
    Buy,
    Sell,
}
