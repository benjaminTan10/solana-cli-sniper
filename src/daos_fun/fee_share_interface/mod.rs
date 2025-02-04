pub const FEE_SHARED: Pubkey = pubkey!("8qQwPgMtbjKUcPo8YVh8kbHSgK82P66bTjiGTZ6WPQBK");
pub mod accounts;
pub use accounts::*;
pub mod instructions;
pub use instructions::*;
pub mod errors;
pub use errors::*;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
