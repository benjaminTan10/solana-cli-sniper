use borsh::{BorshDeserialize, BorshSerialize};
pub const FUNDRAISE_STATE_ACCOUNT_DISCM: [u8; 8] = [191, 165, 0, 201, 61, 39, 110, 54];
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
#[derive(Clone, Debug, PartialEq)]
pub struct FundraiseStateAccount(pub FundraiseState);
impl FundraiseStateAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FUNDRAISE_STATE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FUNDRAISE_STATE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FundraiseState::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FUNDRAISE_STATE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
