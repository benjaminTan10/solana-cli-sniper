pub mod embeds;
use std::thread::sleep;

use colorize::AnsiColor;
use demand::{DemandOption, Input, Select, Theme};
use log::{error, info};

use self::embeds::{embed, license_checker};

pub async fn app() -> Result<(), Box<dyn std::error::Error>> {
    let _ = println!("{}", embed().blue());
    let check = match license_checker().await {
        Ok(_) => info!("License Verified"),
        Err(e) => {
            error!("{}", e);
            sleep(std::time::Duration::from_secs(10));
            std::process::exit(1);
        }
    };

    let ms = Select::new("FirstTx")
        .description("Welcome, please select the mode (1-3)")
        .filterable(true)
        .option(DemandOption::new("[1] Start Tasks"))
        .option(DemandOption::new("[2] View Wallets"))
        .option(DemandOption::new("[3] Join Discord"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "[1] Start Tasks" => {
            info!("Starting tasks...");
        }
        "[2] View Wallets" => {
            // Call the function for viewing wallets here
        }
        "[3] Join Discord" => {
            // Call the function for joining Discord here
        }
        _ => {
            // Handle unexpected option here
        }
    }

    Ok(())
}
