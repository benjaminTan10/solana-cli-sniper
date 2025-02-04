use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
pub const CURVE_ACCOUNT_DISCM: [u8; 8] = [191, 180, 249, 66, 180, 71, 51, 182];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Curve {
    pub token_amount: u64,
    pub funding_amount: u64,
    pub virtual_funding_amount: u64,
    pub token_mint: Pubkey,
    pub funding_mint: Pubkey,
    pub total_fee_amount: u64,
    pub total_fee_distributed: u64,
    pub fee_authority: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CurveAccount(pub Curve);
impl CurveAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CURVE_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CURVE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(Curve::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CURVE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_SHARE_STATE_ACCOUNT_DISCM: [u8; 8] = [38, 195, 84, 207, 192, 143, 5, 68];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeShareState {
    pub total_fees: u64,
    pub funding_mint: Pubkey,
    pub creator: Pubkey,
    pub referrer: Pubkey,
    pub platform: Pubkey,
    pub creator_distributed: u64,
    pub referrer_distributed: u64,
    pub platform_distributed: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeShareStateAccount(pub FeeShareState);
impl FeeShareStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_SHARE_STATE_ACCOUNT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FEE_SHARE_STATE_ACCOUNT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(FeeShareState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_SHARE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
