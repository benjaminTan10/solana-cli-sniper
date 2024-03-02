use std::env;

use mongodb::{Client, Collection};
use solana_sdk::pubkey::Pubkey;

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
                Ok(database.collection::<AccessList>("User_Settings"))
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

// pub async fn get_user_addresses(wallet: Pubkey) -> Result<bool, Box<dyn std::error::Error>> {
//     let users = get_user_collection().await?;
//     let user_id_str = format!("{}", user_id.clone());
//     let filter = doc! {"_id": user_id_str};
//     let result: Option<DiscordUser> = users.find_one(filter, None).await?;
//     let user = result.ok_or_else(|| {
//         Box::new(std::io::Error::new(
//             std::io::ErrorKind::NotFound,
//             "User not found",
//         ))
//     })?;

//     let user_clone = user.clone(); // Clone the user here

//     tokio::spawn(async move {
//         let mut cache = CACHE.lock().unwrap();
//         cache.put(user_id.to_string(), user_clone); // Use the cloned user here
//     });

//     Ok(user.keystores.contains(&wallet.to_string()))
// }
