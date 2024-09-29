pub mod rand;
pub mod terminal;
pub mod transaction;
pub mod transaction_history;

use log::info;
use std::error::Error;
use tokio::sync::mpsc::Sender;

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

use crate::raydium_amm::{
        subscribe::PoolKeysSniper,
        swap::{
            raydium_swap_in::sell_tokens, raydium_swap_out::raydium_txn_backrun,
        },
    };

pub async fn read_single_key_impl(
    stop_tx: &mut Sender<()>,
    pool_keys: PoolKeysSniper,
) -> Result<(), Box<dyn Error + Send>> {
    let term = Term::stdout();

    loop {
        match term.read_key().unwrap() {
            Key::Char('1') => {
                let _ = stop_tx.send(()).await;
                info!("Selling 100% of tokens");
                let _ = match raydium_txn_backrun(pool_keys, 100).await {
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
                let _ = match sell_tokens(75, pool_keys).await {
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
                let _ = match sell_tokens(50, pool_keys).await {
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
                let _ = match sell_tokens(25, pool_keys).await {
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
