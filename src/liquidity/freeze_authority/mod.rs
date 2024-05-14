use std::str::FromStr;

use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_token::instruction::freeze_account;

use crate::env::minter::load_minter_settings;

pub async fn freeze_sells() {
    let settings = load_minter_settings().await.unwrap();

    let deployer_key = Keypair::from_base58_string(&settings.deployer_key);
    let mint = match Pubkey::from_str(&settings.token_mint) {
        Ok(mint) => mint,
        Err(e) => {
            eprintln!("Error parsing token mint: {}", e);
            return;
        }
    };

    let authority = match freeze_account(
        &spl_token::id(),
        &deployer_key.pubkey(),
        &mint,
        &deployer_key.pubkey(),
        &[&deployer_key.pubkey()],
    ) {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error freezing account: {}", e);
            return;
        }
    };
}
