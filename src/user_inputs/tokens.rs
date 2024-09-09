use std::str::FromStr;

use demand::Input;
use log::error;
use solana_sdk::pubkey::Pubkey;

pub async fn token_env(token_identifier: &str) -> Pubkey {
    let token_pubkey: Pubkey;

    loop {
        let t = Input::new(token_identifier)
            .placeholder("5eSB1...vYF49")
            .prompt("Input: ");

        let mint_address = t.run().expect("error running input");

        match Pubkey::from_str(&mint_address) {
            Ok(pubkey) => {
                token_pubkey = pubkey;
                break;
            }
            Err(_) => {
                error!("Invalid pubkey. Please try again.");
            }
        }
    }

    token_pubkey
}
