use std::str::FromStr;

use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_instruction};

use crate::plugins::jito_plugin::lib::generate_tip_accounts;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct JitoPoolData {
    pub buyerPrivateKey: String,
    pub deployerPrivateKey: String,
    pub airdropChecked: bool,
    pub walletsNumbers: String,
    pub tokenMintAddress: String,
    pub tokenMarketID: String,
    pub tokenDecimals: String,
    pub totalSupply: String,
    pub tokenbuyAmount: String,
    pub tokenLiquidityAmount: String,
    pub tokenLiquidityAddPercent: String,
    pub BlockEngineSelection: String,
    pub BundleTip: String,
    pub TransactionTip: String,
}

pub fn tip_txn(source: Pubkey, destination: Pubkey, priority: u64) -> Instruction {
    let ix = system_instruction::transfer(&source, &destination, priority);
    ix
}

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
