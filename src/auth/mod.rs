use std::env;

use mongodb::{Client, Collection};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AccessList {
    pub wallets: Vec<String>,
}

pub fn get_database_uri() -> Result<String, env::VarError> {
    env::var("DATABASE_URI")
}

pub async fn get_user_collection() -> Result<Collection<AccessList>, mongodb::error::Error> {
    match get_database_uri() {
        Ok(db_uri) => match Client::with_uri_str(&db_uri).await {
            Ok(client) => {
                let database = client.database("Mevarik");
                Ok(database.collection::<AccessList>("User_AccessList"))
            }
            Err(e) => Err(e),
        },
        Err(_) => {
            let io_error =
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "DATABASE_URI not found");
            Err(mongodb::error::Error::from(io_error))
        }
    }
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
