use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const SWAP_EVENT_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SwapEvent {
    amm: Pubkey,
    input_mint: Pubkey,
    input_amount: u64,
    output_mint: Pubkey,
    output_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEventEvent(pub SwapEvent);
impl BorshSerialize for SwapEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SWAP_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SwapEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SWAP_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapEvent::deserialize(buf)?))
    }
}
pub const FEE_EVENT_EVENT_DISCM: [u8; 8] = [73, 79, 78, 127, 184, 213, 13, 220];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FeeEvent {
    account: Pubkey,
    mint: Pubkey,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeEventEvent(pub FeeEvent);
impl BorshSerialize for FeeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        FEE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl FeeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != FEE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FEE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FeeEvent::deserialize(buf)?))
    }
}
