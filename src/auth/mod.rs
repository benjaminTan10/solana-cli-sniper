use std::thread::sleep;

use colored::Colorize;
use log::{error, warn};
use mongodb::{Client, Collection};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::{app::embeds::embed, env::load_settings};

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

    let mut keyauthapp = keyauth::v1_2::KeyauthApi::new(
        "Mevarik",                                                          // Application Name
        "zblTZGeem8",                                                       // Owner ID
        "90aeddb3b5559a9f47d285bdd803f21c72d489cfc3e6aaaa956753ec4f2466e1", // Application Secret
        "1.0",                                                              // Application Version
        "https://keyauth.win/api/1.2/", // This is the API URL, change this to your custom domain if you have it enabled
    );

    keyauthapp.init(None).unwrap();

    let result = match keyauthapp.register(
        args.username.clone(),
        args.license_key.clone(),
        args.license_key.clone(),
        None,
    ) {
        Ok(result) => result,
        Err(e) => {}
    };

    let result = match keyauthapp.login(args.username, args.license_key.clone(), None) {
        Ok(result) => {
            println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", embed());
            result
        }
        Err(e) => {
            return Err(e.into());
        }
    };

    Ok(())
}
