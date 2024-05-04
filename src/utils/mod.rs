pub mod rand;

use jito_protos::bundle::BundleResult;
use log::info;
use solana_sdk::signature::Keypair;
use std::{error::Error, sync::Arc};
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn read_single_key(stop_tx: &mut tokio::sync::mpsc::Sender<()>) {
    // let mut stdin = std::io::stdin();
    // let stdout = match std::io::stdout().into_raw_mode() {
    //     Ok(stdout) => stdout,
    //     Err(e) => {
    //         info!("Error: {}", e);
    //         return;
    //     }
    // };

    // let mut stdout = stdout.lock();
    // match write!(stdout, "{}", termion::cursor::Hide) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         info!("Error: {}", e);
    //     }
    // };

    // let mut stdin_lock = stdin.lock();
    // let _key = match read_single_key_impl(&mut stdin_lock, stop_tx).await {
    //     Ok(_) => {}
    //     Err(e) => {
    //         info!("Error: {}", e);
    //     }
    // };

    // match write!(
    //     stdout,
    //     "{}{}",
    //     termion::cursor::Show,
    //     termion::cursor::BlinkingBlock
    // ) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         info!("Error: {}", e);
    //     }
    // };
}

use console::{Key, Term};

use crate::{
    env::EngineSettings,
    raydium::{
        subscribe::PoolKeysSniper,
        swap::{
            raydium_swap_in::sell_tokens, raydium_swap_out::raydium_txn_backrun,
            swap_in::PriorityTip,
        },
    },
};

pub async fn read_single_key_impl(
    stop_tx: &mut Sender<()>,
    pool_keys: PoolKeysSniper,
    args: EngineSettings,
    fees: PriorityTip,
    wallet: &Arc<Keypair>,
    mut bundle_results_receiver: Receiver<BundleResult>,
) -> Result<(), Box<dyn Error + Send>> {
    let term = Term::stdout();

    loop {
        match term.read_key().unwrap() {
            Key::Char('1') => {
                let _ = stop_tx.send(()).await;
                info!("Selling 100% of tokens");
                let _ = match raydium_txn_backrun(
                    wallet,
                    pool_keys,
                    100,
                    fees,
                    args,
                    bundle_results_receiver,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        info!("Error: {}", e);
                    }
                };
                return Ok(());
            }
            Key::Char('2') => {
                let _ = stop_tx.send(()).await;
                info!("Selling 75% of tokens");
                let _ = match sell_tokens(
                    75,
                    pool_keys,
                    wallet.clone(),
                    fees,
                    args,
                    bundle_results_receiver,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        info!("Error: {}", e);
                    }
                };
                return Ok(());
            }
            Key::Char('3') => {
                let _ = stop_tx.send(()).await;
                info!("Selling 50% of tokens");
                let _ = match sell_tokens(
                    50,
                    pool_keys,
                    wallet.clone(),
                    fees,
                    args,
                    bundle_results_receiver,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        info!("Error: {}", e);
                    }
                };
                return Ok(());
            }
            Key::Char('4') => {
                let _ = stop_tx.send(()).await;
                info!("Selling 25% of tokens");
                let _ = match sell_tokens(
                    25,
                    pool_keys,
                    wallet.clone(),
                    fees,
                    args,
                    bundle_results_receiver,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        info!("Error: {}", e);
                    }
                };
                return Ok(());
            }
            Key::Escape => {
                // Handle Escape key
            }
            Key::Enter => {
                // Handle Enter key
            }
            _ => {
                // Handle other keys
            }
        }
    }
}
