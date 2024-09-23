pub mod config_init;
pub mod embeds;
pub mod wallets;

use solana_sdk::signature::Keypair;
use std::error::Error;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use demand::{Input, Theme};
use serde::Deserialize;
use termcolor::{Color, ColorSpec};

use crate::copytrade::copytrade;
use crate::pumpfun::pump::{pump_swap_in, pump_swap_out};
use crate::pumpfun::sniper::pumpfun_sniper;
use crate::raydium_amm::swap::swap_in::{swap_in, swap_out, PriorityTip};
use crate::raydium_amm::swap::trades::track_trades;
use crate::router::SniperRoute;
use crate::screen::dimensions::LogData;
use crate::user_inputs::mode::{automatic_snipe, unwrap_sol_call, wrap_sol_call};

use self::wallets::wallet_logger;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct UserData {
    pub module: String,
    pub platform: String,
    pub mode: String,
    pub wallet: String,
    #[serde(rename = "in")]
    pub tokenIn: String,
    #[serde(rename = "out")]
    pub tokenOut: String,
    pub amount_sol: f64,
    pub max_tx: f64,
    pub tx_delay: f64,
    pub priority_fee: f64,
    pub ms_before_drop: f64,
    pub autosell_take_profit: f64,
    pub autosell_stop_loss: f64,
    pub autosell_percent: f64,
    pub autosell_ms: f64,
}

#[derive(Debug)]
pub struct MevApe {
    pub sol_amount: u64,
    pub fee: PriorityTip,
    pub wallet: String,
}

pub fn theme() -> Theme {
    Theme {
        title: ColorSpec::new()
            .set_fg(Some(Color::Rgb(181, 228, 140)))
            .clone(),
        cursor: ColorSpec::new()
            .set_fg(Some(Color::Green))
            .set_bold(true)
            .clone(),

        selected_option: ColorSpec::new()
            .set_fg(Some(Color::Rgb(38, 70, 83)))
            .set_bold(true) // make the selected option bold
            .clone(),
        selected_prefix_fg: ColorSpec::new()
            .set_fg(Some(Color::Rgb(181, 228, 140)))
            .clone(),
        input_cursor: ColorSpec::new()
            .set_fg(Some(Color::Rgb(22, 138, 173)))
            .clone(),
        input_prompt: ColorSpec::new().set_fg(Some(Color::Blue)).clone(),
        ..Theme::default()
    }
}

impl MenuState {
    pub fn new(items: Vec<MenuItem>, parent: Option<Box<MenuState>>) -> Self {
        Self {
            items,
            selected: 0,
            parent,
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }

    pub fn selected_action(&self) -> MenuAction {
        self.items[self.selected].action.clone()
    }
}

pub async fn private_key_env(key: &str) -> Result<String, Box<dyn Error>> {
    loop {
        let t = Input::new(key)
            .placeholder("5eSB1...vYF49")
            .prompt("Input: ");

        let private_key = t.run().expect("error running input");

        // Check if the private key is valid
        if is_valid_private_key(&private_key) {
            return Ok(private_key);
        } else {
            println!("Invalid private key. Please enter a valid private key.");
        }
    }
}

fn is_valid_private_key(private_key: &str) -> bool {
    let decoded = bs58::decode(private_key)
        .into_vec()
        .unwrap_or_else(|_| vec![]);
    Keypair::from_bytes(&decoded).is_ok()
}
use eyre::Report;
#[derive(Clone)]
pub enum MenuAction {
    Function(
        Arc<
            dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), Report>> + Send + 'static>>
                + Send
                + Sync,
        >,
    ),
    Submenu(String),
}

pub struct MenuItem {
    pub label: String,
    pub action: MenuAction,
}

pub struct MenuState {
    pub items: Vec<MenuItem>,
    pub selected: usize,
    pub parent: Option<Box<MenuState>>,
}

pub struct AppState {
    pub current_menu: MenuState,
    pub log_data: LogData,
    pub menu_title: String,
}

lazy_static::lazy_static! {
    static ref GLOBAL_APP_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState {
        current_menu: create_main_menu(),
        menu_title: "Main Menu".to_string(),
        log_data: LogData {
            left: Vec::new(),
            right_top: Vec::new(),
            right_bottom: Vec::new(),
        },
    }));
}

pub async fn log_and_update(tx: &mpsc::Sender<String>, message: &str) {
    tx.send(message.to_string()).await.unwrap();
}

// Helper function to modify the AppState
pub fn with_app_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut AppState) -> R,
{
    let mut state = GLOBAL_APP_STATE.lock().unwrap();
    f(&mut state)
}

// Helper function to get a reference to the global AppState
pub fn get_app_state() -> Arc<Mutex<AppState>> {
    GLOBAL_APP_STATE.clone()
}

pub fn create_main_menu() -> MenuState {
    MenuState::new(
        vec![
            MenuItem {
                label: "▪ Raydium AMM Mode".to_string(),
                action: MenuAction::Submenu("raydium".to_string()),
            },
            MenuItem {
                label: "▪ PumpFun Mode".to_string(),
                action: MenuAction::Submenu("pumpfun".to_string()),
            },
            MenuItem {
                label: "▪ CopyTrade Mode".to_string(),
                action: MenuAction::Function(Arc::new(|| Box::pin(async { copytrade().await }))),
            },
            MenuItem {
                label: "📦 Wrap SOL".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { wrap_sol_call().await })
                })),
            },
            MenuItem {
                label: "🪤 Unwrap SOL".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { unwrap_sol_call().await })
                })),
            },
            MenuItem {
                label: "🍄 Wallet Details".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { wallet_logger().await })
                })),
            },
        ],
        None,
    )
}

pub fn create_raydium_menu() -> MenuState {
    MenuState::new(
        vec![
            MenuItem {
                label: "▪ Snipe Incoming Pools".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { automatic_snipe(false).await })
                })),
            },
            MenuItem {
                label: "▪ Manual Sniper".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { automatic_snipe(true).await })
                })),
            },
            MenuItem {
                label: "▪ Swap SOL to Tokens".to_string(),
                action: MenuAction::Function(Arc::new(|| Box::pin(async { swap_in().await }))),
            },
            MenuItem {
                label: "▪ Swap Tokens to SOL".to_string(),
                action: MenuAction::Function(Arc::new(|| Box::pin(async { swap_out().await }))),
            },
            MenuItem {
                label: "🎯 Track Token Gains".to_string(),
                action: MenuAction::Function(Arc::new(|| Box::pin(async { track_trades().await }))),
            },
            MenuItem {
                label: " ↪  Main Menu".to_string(),
                action: MenuAction::Submenu("main".to_string()),
            },
        ],
        Some(Box::new(create_main_menu())),
    )
}

pub fn create_pumpfun_menu() -> MenuState {
    MenuState::new(
        vec![
            MenuItem {
                label: "💠 PumpFun Migration Manual Sniper".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { pumpfun_sniper(true, SniperRoute::PumpFunMigration).await })
                })),
            },
            MenuItem {
                label: "💠 PumpFun Migration Auto Sniper".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { pumpfun_sniper(false, SniperRoute::PumpFunMigration).await })
                })),
            },
            MenuItem {
                label: "▪ PumpFun Coin Auto Sniper".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { pumpfun_sniper(false, SniperRoute::PumpFun).await })
                })),
            },
            MenuItem {
                label: "▪ PumpFun Coin Manual Sniper".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { pumpfun_sniper(true, SniperRoute::PumpFun).await })
                })),
            },
            MenuItem {
                label: "▪ Pump Swap-In".to_string(),
                action: MenuAction::Function(Arc::new(|| Box::pin(async { pump_swap_in().await }))),
            },
            MenuItem {
                label: "▪ Pump Swap-Out".to_string(),
                action: MenuAction::Function(Arc::new(|| {
                    Box::pin(async { pump_swap_out().await })
                })),
            },
            MenuItem {
                label: " ↪  Main Menu".to_string(),
                action: MenuAction::Submenu("main".to_string()),
            },
        ],
        Some(Box::new(create_main_menu())),
    )
}
