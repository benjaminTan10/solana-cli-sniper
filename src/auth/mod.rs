
use colored::Colorize;
use log::{error};
use mongodb::{Client, Collection};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::env::load_settings;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AccessList {
    pub wallets: Vec<String>,
}

pub fn get_database_uri() -> String {
    let db_url = "mongodb+srv://tamur:TjiMEHQ6eEAMJSKv@serverlessinstance0.rpbkjvq.mongodb.net/";
    db_url.to_string()
}

pub async fn get_user_collection() -> Result<Collection<AccessList>, mongodb::error::Error> {
    let client = Client::with_uri_str(&get_database_uri()).await?;
    let db = client.database("Mevarik");
    let users = db.collection("AccessList");
    Ok(users)
}

pub async fn get_user_addresses(wallet: Pubkey) -> Result<bool, Box<dyn std::error::Error>> {
    let users = get_user_collection().await?;
    let result: Option<AccessList> = users.find_one(None, None).await?;
    let user = result.ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "User not found",
        ))
    })?;

    Ok(user.wallets.contains(&wallet.to_string()))
}

pub async fn auth_verification() -> Result<(), Box<dyn std::error::Error>> {
    let args = match load_settings().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into()); // Return the error
        }
    };

    let auth_wallet = args.bot_auth;

    let keypair = match Keypair::from_bytes(match &bs58::decode(&auth_wallet).into_vec() {
        Ok(key) => key,
        Err(e) => {
            error!("Error: {:?}", e);
            return Ok(()); // Return the error
        }
    }) {
        Ok(keypair) => keypair,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into()); // Return the error
        }
    };

    let wallet = keypair.pubkey();

    let is_user = get_user_addresses(wallet).await?;

    if !is_user {
        error!("{}: {}", "Unauthorized User".bold().red(), wallet);
        std::process::exit(1); // Stop the bot and exit the process
    }

    Ok(())
}
