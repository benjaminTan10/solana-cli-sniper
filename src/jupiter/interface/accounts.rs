use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const TOKEN_LEDGER_ACCOUNT_DISCM: [u8; 8] = [156, 247, 9, 188, 54, 108, 85, 77];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenLedger {
    pub token_account: Pubkey,
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TokenLedgerAccount(pub TokenLedger);
impl TokenLedgerAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOKEN_LEDGER_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TOKEN_LEDGER_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TokenLedger::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOKEN_LEDGER_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
