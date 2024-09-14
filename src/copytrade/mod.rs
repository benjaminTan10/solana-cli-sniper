use log::{error, info};
use subscription::copytrading_grpc;

use crate::{env::load_config, user_inputs::tokens::token_env};

pub mod copytrading_decoder;
pub mod subscription;

pub async fn copytrade() -> eyre::Result<()> {
    let mut args = match load_config().await {
        Ok(args) => args,
        Err(e) => {
            error!("Error: {:?}", e);
            return Err(e.into());
        }
    };

    if args.trading.copytrade_accounts.is_empty() {
        args.trading.copytrade_accounts = vec![token_env("Copytrade Account: ").await.to_string()];

        info!("Listening for the Launch...")
    }

    let addresses = args.trading.copytrade_accounts.clone();

    let _ = match copytrading_grpc(args, addresses.into()).await {
        Ok(_) => info!("Transaction Sent"),
        Err(e) => error!("{}", e),
    };

    Ok(())
}
