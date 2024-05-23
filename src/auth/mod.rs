use std::{sync::Arc, thread::sleep};

use colored::Colorize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::{error, warn};
use mongodb::{Client, Collection};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

use crate::{
    app::embeds::embed,
    env::{env_loader::tip_account, load_settings},
    liquidity::utils::tip_txn,
    raydium::swap::{
        grpc_new_pairs::clear_previous_line, instructions::TAX_ACCOUNT, swapper::auth_keypair,
    },
};

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

    // Check if user is already registered
    let is_registered =
        match keyauthapp.login(args.username.clone(), args.license_key.clone(), None) {
            Ok(_) => true,
            Err(_) => false,
        };

    // If user is not registered, check balance and register
    if !is_registered {
        let rpc_client = RpcClient::new(args.rpc_url.clone());
        let wallet = Keypair::from_base58_string(&args.payer_keypair);
        let balance = rpc_client.get_balance(&wallet.pubkey()).await?;
        clear_previous_line()?;
        println!(
            "{} {}\n{}...",
            "Registering".green(),
            args.username.bold().white(),
            "Please wait".cyan()
        );
        // Check if balance is at least 1 SOL
        if balance < 1_000_000_000 {
            // 1 SOL is 1_000_000_000 lamports
            return Err("Insufficient balance for registration".into());
        }

        let recent_blockhash = rpc_client.get_latest_blockhash().await?;
        let register = tip_txn(wallet.pubkey(), TAX_ACCOUNT, sol_to_lamports(1.0));
        let tip = tip_txn(wallet.pubkey(), tip_account(), sol_to_lamports(0.001));

        let versioned_msg = VersionedMessage::V0(Message::try_compile(
            &wallet.pubkey(),
            &[register, tip],
            &[],
            recent_blockhash,
        )?);

        let txn = VersionedTransaction::try_new(versioned_msg, &[&wallet])?;

        let mut client = get_searcher_client(
            "https://ny.mainnet.block-engine.jito.wtf",
            &Arc::new(auth_keypair()),
        )
        .await?;

        let mut bundle_results_subscription = client
            .subscribe_bundle_results(SubscribeBundleResultsRequest {})
            .await
            .expect("subscribe to bundle results")
            .into_inner();

        // Register user
        match keyauthapp.register(
            args.username.clone(),
            args.license_key.clone(),
            args.license_key.clone(),
            None,
        ) {
            Ok(result) => {
                let bundle = match send_bundle_with_confirmation(
                    &[txn],
                    &Arc::new(rpc_client),
                    &mut client,
                    &mut bundle_results_subscription,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Distribution Error: {}", e);
                        panic!("Error: {}", e);
                    }
                };

                result
            }
            Err(e) => {
                return Err(e.into());
            }
        };
    }

    // Login user
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
