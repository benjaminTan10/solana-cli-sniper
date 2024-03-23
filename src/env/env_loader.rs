use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn tip_program_id() -> Pubkey {
    let auth = Pubkey::from_str("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt").unwrap();

    return auth;
}
