use std::{error::Error, str::FromStr};

use demand::Input;
use log::error;
use solana_sdk::{native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair};

pub async fn private_key_input(key: &str) -> Result<String, Box<dyn Error>> {
    loop {
        let t = Input::new(key)
            .placeholder("5eSB1...vYF49")
            .prompt("Input: ");

        let private_key = t.run().expect("error running input");

        // Check if the private key is valid
        if is_valid_private_key(&private_key) {
            return Ok(private_key);
        } else {
            println!("Invalid private key. Please enter a valid private key.");
        }
    }
}

fn is_valid_private_key(private_key: &str) -> bool {
    let decoded = bs58::decode(private_key)
        .into_vec()
        .unwrap_or_else(|_| vec![]);
    Keypair::from_bytes(&decoded).is_ok()
}

pub async fn amount_input(identifier: &str) -> u64 {
    let amount: u64;

    loop {
        let t = Input::new(identifier).placeholder("0.01").prompt("Input: ");

        let string = t.run().expect("error running input");

        match string.parse::<f64>() {
            Ok(val) => {
                amount = sol_to_lamports(val);
                break;
            }
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }

    amount
}

pub async fn gas_input(identifier: &str) -> u64 {
    let amount: u64;

    loop {
        let t = Input::new(identifier)
            .placeholder("0.0001")
            .prompt("Input: ");

        let string = t.run().expect("error running input");

        match string.parse::<f64>() {
            Ok(val) => {
                amount = sol_to_lamports(val);
                break;
            }
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }

    amount
}

pub async fn mint_input(token_identifier: &str) -> Pubkey {
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
            Err(e) => {
                error!("Invalid pubkey: {:?}", e);
            }
        }
    }

    token_pubkey
}

pub async fn percentage_input() -> f64 {
    let amount: f64;
    loop {
        let t = Input::new("Sell Amount Percentage:")
            .placeholder("50 (%)")
            .prompt("Input: ");

        let string = t.run().expect("error running input");

        match string.parse::<f64>() {
            Ok(_) => {
                amount = string.parse::<f64>().unwrap();
                break;
            }
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }
    amount
}
