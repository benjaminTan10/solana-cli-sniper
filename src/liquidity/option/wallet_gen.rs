use std::{error::Error, fs};

use demand::{Confirm, DemandOption, Input, Select};
use log::info;
use serde_json::Value;
use solana_sdk::signature::Keypair;

use crate::app::theme;

pub fn generate_wallets(count: i32) -> Vec<String> {
    let mut wallet: Vec<Keypair> = vec![];
    for _ in 0..count {
        wallet.push(Keypair::new());
    }

    wallet.iter().map(|x| x.to_base58_string()).collect()
}

pub async fn gen_wallet_save() -> eyre::Result<()> {
    let theme = theme();

    let amount: u64;
    let mut folder_name = String::new();
    loop {
        let deployment = new_deployment().await.unwrap();
        if deployment {
            let t = Input::new("Enter Folder Name:")
                .placeholder("Floki...")
                .theme(&theme)
                .prompt("Input: ");

            folder_name = t.run().expect("error running input");
        } else {
            (folder_name, _) = list_folders().await.unwrap();
        }

        let t = Input::new("Wallet Count:")
            .placeholder("27")
            .theme(&theme)
            .prompt("Input: ");

        let string = t.run().expect("error running input");

        match string.parse::<u64>() {
            Ok(val) => {
                amount = val;
                break;
            }
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    }

    let wallets = generate_wallets(amount as i32);

    info!("Generating {} wallets", wallets.len());

    fs::create_dir_all(folder_name.clone())?;

    for (i, wallet) in wallets.iter().enumerate() {
        let path = format!("{}/wallet_{}.json", folder_name, i + 1);
        let data = serde_json::to_string(&wallet)?;
        fs::write(path, data)?;
    }

    info!("{} Wallets saved to {} folder", wallets.len(), folder_name);

    Ok(())
}

pub async fn load_wallets() -> Result<Vec<Keypair>, Box<dyn Error>> {
    let mut wallets: Vec<Keypair> = vec![];

    let (folder_name, _) = list_folders().await?;

    let paths = fs::read_dir(folder_name)?;

    for path in paths {
        let path = path?.path();
        let data = fs::read_to_string(path)?;
        let wallet = Keypair::from_base58_string(&data);
        wallets.push(wallet);
    }

    Ok(wallets)
}

pub async fn new_deployment() -> Result<bool, Box<dyn Error>> {
    let confirm = Confirm::new("Deployment")
        .description("Generate a new directory for wallets to create new deployment?")
        .affirmative("No")
        .negative("Yes")
        .selected(true)
        .run()
        .unwrap();

    Ok(!confirm) // Flip the boolean value because we swapped the labels
}

pub async fn list_folders() -> Result<(String, Vec<Keypair>), Box<dyn Error>> {
    let paths = fs::read_dir(".")?;

    let mut dir_names = Vec::new();
    for path in paths {
        let path = path?.path();
        if path.is_dir() {
            dir_names.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }

    let mut select = Select::new("Wallets")
        .description("Select the Wallet Folder")
        .filterable(true);

    for dir_name in &dir_names {
        select = select.option(DemandOption::new(dir_name).label(dir_name));
    }

    let selected_option = select.run()?;

    println!("Selected: {}", selected_option);

    let mut json_values = Vec::new();

    let json_paths = fs::read_dir(selected_option)?;

    for json_path in json_paths {
        let json_path = json_path?.path();
        if json_path.extension().and_then(|s| s.to_str()) == Some("json") {
            let json_str = fs::read_to_string(json_path)?;
            let json_value: Value = serde_json::from_str(&json_str)?;
            json_values.push(json_value);
        }
    }

    let mut wallets = Vec::new();
    json_values.iter().for_each(|x| {
        let keypair = Keypair::from_base58_string(x.as_str().unwrap());
        wallets.push(keypair);
    });

    Ok((selected_option.clone(), wallets))
}
