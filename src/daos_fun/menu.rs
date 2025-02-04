use std::error::Error;

use demand::{DemandOption, Select};

use crate::app::{main_menu, theme};

use super::{
    daos_create_fund::create_daos_fund, daos_transaction::interact_daos_token,
    daosfun_snipe::daosfun_sniper,
};

pub async fn daosfun_menu() -> Result<(), Box<dyn Error + Send>> {
    let theme = theme();
    let ms = Select::new("Pump Mode")
        .description("Select the Mode")
        .theme(&theme)
        .filterable(true)
        .option(DemandOption::new("LaunchDaosFun").label("▪ Launch Fund"))
        .option(DemandOption::new("DaosSniperAuto").label("▪ DaosFun Coin Auto Sniper"))
        .option(DemandOption::new("DaosSniperManual").label("▪ DaosFun Coin Manual Sniper"))
        .option(DemandOption::new("DaosBuy").label("▪ DaosFun Swap-In"))
        .option(DemandOption::new("DaosSell").label("▪ DaosFun Swap-Out"))
        .option(DemandOption::new("Main Menu").label(" ↪  Main Menu"));

    let selected_option = ms.run().expect("error running select");

    match selected_option {
        "LaunchDaosFun" => {
            let _ = create_daos_fund().await;
        }
        "DaosSniperAuto" => {
            let _ = daosfun_sniper(false, crate::router::SniperRoute::DaosFun).await;
        }
        "DaosSniperManual" => {
            let _ = daosfun_sniper(true, crate::router::SniperRoute::DaosFun).await;
        }
        "DaosBuy" => {
            let _ = interact_daos_token(super::daos_transaction::DAOSTrade::Buy).await;
        }
        "DaosSell" => {
            let _ = interact_daos_token(super::daos_transaction::DAOSTrade::Sell).await;
        }
        "Main Menu" => {
            let _ = main_menu(false).await;
        }
        _ => {}
    }

    Ok(())
}
