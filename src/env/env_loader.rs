use rand::{rngs::StdRng, Rng, SeedableRng};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::plugins::jito_plugin::lib::generate_tip_accounts;

pub fn tip_program_id() -> Pubkey {
    let auth = Pubkey::from_str("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt").unwrap();

    return auth;
}

pub fn tip_account() -> Pubkey {
    let tip_accounts = generate_tip_accounts(&tip_program_id());
    let mut rng = StdRng::from_entropy();
    let tip_account = tip_accounts[rng.gen_range(0..tip_accounts.len())];

    return tip_account;
}
