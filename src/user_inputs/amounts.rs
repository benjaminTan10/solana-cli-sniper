use demand::Input;
use solana_sdk::native_token::sol_to_lamports;

use crate::app::theme;

pub async fn amount_percentage() -> u64 {
    let theme = theme();
    let mut amount: u64;
    loop {
        let t = Input::new("Sell Amount Percentage:")
            .placeholder("50 (%)")
            .theme(&theme)
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

pub async fn bundle_priority_tip() -> u64 {
    let mut amount: u64;
    loop {
        let t = Input::new("Bundle Tip:")
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

pub async fn priority_fee() -> u64 {
    let theme = theme();
    let mut amount: u64;

    loop {
        let t = Input::new("Priority Fee:")
            .placeholder("0.0001")
            .theme(&theme)
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

pub async fn sol_amount() -> u64 {
    let theme = theme();
    let mut amount: u64;

    loop {
        let t = Input::new("Sol Amount:")
            .placeholder("0.01")
            .theme(&theme)
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
