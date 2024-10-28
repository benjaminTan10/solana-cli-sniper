pub const VIRTUAL_DAOS: Pubkey = pubkey!("5jnapfrAN47UYkLkEf7HnprPPBCQLvkYWGZDeKkaP5hv");
pub mod accounts;
pub use accounts::*;
pub mod instructions;
pub use instructions::*;
pub mod errors;
pub use errors::*;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
