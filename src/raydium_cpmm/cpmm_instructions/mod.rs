use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const RAYDIUM_CPMM: Pubkey = pubkey!("TH1S1SNoTAVAL1DPUBKEYDoNoTUSE11111111111111");

pub mod accounts;
pub use accounts::*;
pub mod typedefs;
pub use typedefs::*;
pub mod instructions;
pub use instructions::*;
pub mod errors;
pub use errors::*;
pub mod events;
pub use events::*;
