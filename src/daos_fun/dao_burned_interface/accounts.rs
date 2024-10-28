// use borsh::{BorshDeserialize, BorshSerialize};
// use solana_sdk::pubkey::Pubkey;
// pub const FUNDRAISE_STATE_ACCOUNT_DISCM: [u8; 8] = [191, 165, 0, 201, 61, 39, 110, 54];
// #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct FundraiseState {
//     pub admin: Pubkey,
//     pub token_mint: Pubkey,
//     pub funding_mint: Pubkey,
//     pub token_deposit: u64,
//     pub funding_goal: u64,
//     pub expiration_timestamp: i64,
//     pub funding_received: u64,
//     pub is_funded: bool,
//     pub is_finalized: bool,
//     pub recipient: Pubkey,
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct FundraiseStateAccount(pub FundraiseState);
// impl FundraiseStateAccount {
//     pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
//         use std::io::Read;
//         let mut reader = buf;
//         let mut maybe_discm = [0u8; 8];
//         reader.read_exact(&mut maybe_discm)?;
//         if maybe_discm != FUNDRAISE_STATE_ACCOUNT_DISCM {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!(
//                     "discm does not match. Expected: {:?}. Received: {:?}",
//                     FUNDRAISE_STATE_ACCOUNT_DISCM, maybe_discm
//                 ),
//             ));
//         }
//         Ok(Self(FundraiseState::deserialize(&mut reader)?))
//     }
//     pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         writer.write_all(&FUNDRAISE_STATE_ACCOUNT_DISCM)?;
//         self.0.serialize(&mut writer)
//     }
//     pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
//         let mut data = Vec::new();
//         self.serialize(&mut data)?;
//         Ok(data)
//     }
// }
// pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
// #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct State {
//     pub admin: Pubkey,
//     pub dao_mint: Pubkey,
//     pub funding_mint: Pubkey,
//     pub funding_goal: u64,
//     pub admin_closed_fund: bool,
//     pub redemption_started: bool,
//     pub expiration_timestamp: i64,
//     pub delegate_authorities: [Option<Pubkey>; 3],
//     pub carry_basis: u16,
//     pub fee_authority: Pubkey,
//     pub curve_initialized: bool,
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct StateAccount(pub State);
// impl StateAccount {
//     pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
//         use std::io::Read;
//         let mut reader = buf;
//         let mut maybe_discm = [0u8; 8];
//         reader.read_exact(&mut maybe_discm)?;
//         if maybe_discm != STATE_ACCOUNT_DISCM {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!(
//                     "discm does not match. Expected: {:?}. Received: {:?}",
//                     STATE_ACCOUNT_DISCM, maybe_discm
//                 ),
//             ));
//         }
//         Ok(Self(State::deserialize(&mut reader)?))
//     }
//     pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         writer.write_all(&STATE_ACCOUNT_DISCM)?;
//         self.0.serialize(&mut writer)
//     }
//     pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
//         let mut data = Vec::new();
//         self.serialize(&mut data)?;
//         Ok(data)
//     }
// }
// pub const TOKEN_ACCOUNT_REDEMPTION_ACCOUNT_DISCM: [u8; 8] = [138, 119, 164, 162, 129, 116, 235, 15];
// #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct TokenAccountRedemption {
//     pub total_amount: Option<u64>,
//     pub total_distributed: u64,
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct TokenAccountRedemptionAccount(pub TokenAccountRedemption);
// impl TokenAccountRedemptionAccount {
//     pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
//         use std::io::Read;
//         let mut reader = buf;
//         let mut maybe_discm = [0u8; 8];
//         reader.read_exact(&mut maybe_discm)?;
//         if maybe_discm != TOKEN_ACCOUNT_REDEMPTION_ACCOUNT_DISCM {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!(
//                     "discm does not match. Expected: {:?}. Received: {:?}",
//                     TOKEN_ACCOUNT_REDEMPTION_ACCOUNT_DISCM, maybe_discm
//                 ),
//             ));
//         }
//         Ok(Self(TokenAccountRedemption::deserialize(&mut reader)?))
//     }
//     pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         writer.write_all(&TOKEN_ACCOUNT_REDEMPTION_ACCOUNT_DISCM)?;
//         self.0.serialize(&mut writer)
//     }
//     pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
//         let mut data = Vec::new();
//         self.serialize(&mut data)?;
//         Ok(data)
//     }
// }
// pub const USER_DAO_BURN_REDEEMED_ACCOUNT_DISCM: [u8; 8] = [44, 6, 253, 38, 213, 105, 160, 17];
// #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct UserDaoBurnRedeemed {
//     pub dao_burn_redeemed: u64,
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct UserDaoBurnRedeemedAccount(pub UserDaoBurnRedeemed);
// impl UserDaoBurnRedeemedAccount {
//     pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
//         use std::io::Read;
//         let mut reader = buf;
//         let mut maybe_discm = [0u8; 8];
//         reader.read_exact(&mut maybe_discm)?;
//         if maybe_discm != USER_DAO_BURN_REDEEMED_ACCOUNT_DISCM {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!(
//                     "discm does not match. Expected: {:?}. Received: {:?}",
//                     USER_DAO_BURN_REDEEMED_ACCOUNT_DISCM, maybe_discm
//                 ),
//             ));
//         }
//         Ok(Self(UserDaoBurnRedeemed::deserialize(&mut reader)?))
//     }
//     pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         writer.write_all(&USER_DAO_BURN_REDEEMED_ACCOUNT_DISCM)?;
//         self.0.serialize(&mut writer)
//     }
//     pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
//         let mut data = Vec::new();
//         self.serialize(&mut data)?;
//         Ok(data)
//     }
// }
// pub const USER_DAO_BURNED_ACCOUNT_DISCM: [u8; 8] = [251, 92, 10, 58, 196, 17, 82, 182];
// #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct UserDaoBurned {
//     pub dao_burned: u64,
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct UserDaoBurnedAccount(pub UserDaoBurned);
// impl UserDaoBurnedAccount {
//     pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
//         use std::io::Read;
//         let mut reader = buf;
//         let mut maybe_discm = [0u8; 8];
//         reader.read_exact(&mut maybe_discm)?;
//         if maybe_discm != USER_DAO_BURNED_ACCOUNT_DISCM {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!(
//                     "discm does not match. Expected: {:?}. Received: {:?}",
//                     USER_DAO_BURNED_ACCOUNT_DISCM, maybe_discm
//                 ),
//             ));
//         }
//         Ok(Self(UserDaoBurned::deserialize(&mut reader)?))
//     }
//     pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         writer.write_all(&USER_DAO_BURNED_ACCOUNT_DISCM)?;
//         self.0.serialize(&mut writer)
//     }
//     pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
//         let mut data = Vec::new();
//         self.serialize(&mut data)?;
//         Ok(data)
//     }
// }
// pub const FUNDRAISE_STATE_ACCOUNT_DISCM: [u8; 8] = [191, 165, 0, 201, 61, 39, 110, 54];
// #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct FundraiseState {
//     pub admin: Pubkey,
//     pub token_mint: Pubkey,
//     pub funding_mint: Pubkey,
//     pub token_deposit: u64,
//     pub funding_goal: u64,
//     pub expiration_timestamp: i64,
//     pub funding_received: u64,
//     pub is_funded: bool,
//     pub is_finalized: bool,
//     pub recipient: Pubkey,
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct FundraiseStateAccount(pub FundraiseState);
// impl FundraiseStateAccount {
//     pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
//         use std::io::Read;
//         let mut reader = buf;
//         let mut maybe_discm = [0u8; 8];
//         reader.read_exact(&mut maybe_discm)?;
//         if maybe_discm != FUNDRAISE_STATE_ACCOUNT_DISCM {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!(
//                     "discm does not match. Expected: {:?}. Received: {:?}",
//                     FUNDRAISE_STATE_ACCOUNT_DISCM, maybe_discm
//                 ),
//             ));
//         }
//         Ok(Self(FundraiseState::deserialize(&mut reader)?))
//     }
//     pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         writer.write_all(&FUNDRAISE_STATE_ACCOUNT_DISCM)?;
//         self.0.serialize(&mut writer)
//     }
//     pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
//         let mut data = Vec::new();
//         self.serialize(&mut data)?;
//         Ok(data)
//     }
// }
// pub const STATE_ACCOUNT_DISCM: [u8; 8] = [216, 146, 107, 94, 104, 75, 182, 177];
// #[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct State {
//     pub admin: Pubkey,
//     pub dao_mint: Pubkey,
//     pub funding_mint: Pubkey,
//     pub funding_goal: u64,
//     pub admin_closed_fund: bool,
//     pub redemption_started: bool,
//     pub expiration_timestamp: i64,
//     pub delegate_authorities: [Option<Pubkey>; 3],
//     pub carry_basis: u16,
//     pub fee_authority: Pubkey,
//     pub curve_initialized: bool,
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct StateAccount(pub State);
// impl StateAccount {
//     pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
//         use std::io::Read;
//         let mut reader = buf;
//         let mut maybe_discm = [0u8; 8];
//         reader.read_exact(&mut maybe_discm)?;
//         if maybe_discm != STATE_ACCOUNT_DISCM {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 format!(
//                     "discm does not match. Expected: {:?}. Received: {:?}",
//                     STATE_ACCOUNT_DISCM, maybe_discm
//                 ),
//             ));
//         }
//         Ok(Self(State::deserialize(&mut reader)?))
//     }
//     pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
//         writer.write_all(&STATE_ACCOUNT_DISCM)?;
//         self.0.serialize(&mut writer)
//     }
//     pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
//         let mut data = Vec::new();
//         self.serialize(&mut data)?;
//         Ok(data)
//     }
// }
