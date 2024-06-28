use std::sync::Arc;

use colored::Colorize;
use jito_protos::searcher::SubscribeBundleResultsRequest;
use jito_searcher_client::{get_searcher_client, send_bundle_with_confirmation};
use log::error;
use mongodb::{Client, Collection};
use self_update::cargo_crate_version;
use semver::Version;
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
    env::load_settings,
    liquidity::utils::{tip_account, tip_txn},
    raydium::swap::{
        instructions::TAX_ACCOUNT, raydium_amm_sniper::clear_previous_line, swapper::auth_keypair,
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

    let result = keyauthapp.var("Mevarik-Version".to_string())?;

    let current_version = Version::parse(cargo_crate_version!())?;

    println!("Current Version: {}", current_version);

    let result_version = Version::parse(&result)?;

    println!("Latest Version: {}", result_version);

    if current_version < result_version {
        self_update().await?;
    } else {
        println!(
            "{}",
            format!("{}", "Already up to date".bold().bright_white())
        );
    }

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
        if balance < sol_to_lamports(0.65) {
            // 1 SOL is 1_000_000_000 lamports
            return Err("Insufficient balance for registration: 0.65 + 0.001 SOL Required".into());
        }

        let recent_blockhash = rpc_client.get_latest_blockhash().await?;
        let register = tip_txn(wallet.pubkey(), TAX_ACCOUNT, sol_to_lamports(0.65));
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
                std::mem::drop(bundle_results_subscription);
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

pub async fn self_update() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking for updates...");
    let releases = match self_update::backends::github::ReleaseList::configure()
        .repo_owner("taimurey")
        .repo_name("Mevarik")
        .build()
        .unwrap()
        .fetch()
    {
        Ok(releases) => releases,
        Err(e) => {
            return Err(e.into());
        }
    };

    let latest_release = releases.first().ok_or("No releases found")?;

    let target_os_suffix = if cfg!(target_os = "windows") {
        "-windows.exe"
    } else if cfg!(target_os = "linux") {
        "-linux.1"
    } else {
        return Err("Unsupported operating system".into());
    };

    let bin_name = latest_release
        .assets
        .iter()
        .find_map(|asset| {
            if asset.name.ends_with(target_os_suffix) {
                Some(asset.name.clone())
            } else {
                None
            }
        })
        .ok_or("No matching binary found")?;

    let status = self_update::backends::github::Update::configure()
        .repo_owner("taimurey") // replace with your GitHub username
        .repo_name("Mevarik") // replace with your repository name
        .bin_name(&bin_name)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    clear_previous_line()?;

    println!("Update status: {:?}", status.version());

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(())
}
