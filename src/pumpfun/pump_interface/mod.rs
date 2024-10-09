use solana_program::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const PUMPFUN_PROGRAM: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
pub mod accounts;
pub mod builder;
pub mod errors;
pub mod events;
pub mod instructions;
