use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use super::typedefs::{Currency, CurveType};
pub const CONFIG_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [189, 255, 97, 70, 186, 189, 24, 102];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConfigAccount {
    pub migration_authority: Pubkey,
    pub backend_authority: Pubkey,
    pub config_authority: Pubkey,
    pub helio_fee: Pubkey,
    pub dex_fee: Pubkey,
    pub fee_bps: u16,
    pub dex_fee_share: u8,
    pub migration_fee: u64,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub min_supported_decimal_places: u8,
    pub max_supported_decimal_places: u8,
    pub min_supported_token_supply: u64,
    pub max_supported_token_supply: u64,
    pub bump: u8,
    pub coef_b: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigAccountAccount(pub ConfigAccount);
impl ConfigAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CONFIG_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_ACCOUNT_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CONFIG_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CURVE_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [8, 91, 83, 28, 132, 216, 248, 22];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CurveAccount {
    pub total_supply: u64,
    pub curve_amount: u64,
    pub mint: Pubkey,
    pub decimals: u8,
    pub collateral_currency: Currency,
    pub curve_type: CurveType,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub migration_fee: u64,
    pub coef_b: u32,
    pub bump: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CurveAccountAccount(pub CurveAccount);
impl CurveAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CURVE_ACCOUNT_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CURVE_ACCOUNT_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CurveAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CURVE_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
