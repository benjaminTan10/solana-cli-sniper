use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::Backend,
    Terminal,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::Instant;

use crate::app::{
    create_main_menu, create_pumpfun_menu, create_raydium_menu, with_app_state, AppState,
    MenuAction,
};

use super::canvas::draw;

pub struct LogData {
    pub left: Vec<String>,
    pub right_top: Vec<String>,
    pub right_bottom: Vec<String>,
}

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn run(terminal: &mut Terminal<impl Backend>) -> Result<(), DynError> {
    let mut last_redraw = Instant::now();
    let redraw_interval = Duration::from_millis(100); // Adjust this value as needed

    loop {
        // Check if it's time to redraw
        if last_redraw.elapsed() >= redraw_interval {
            terminal.draw(|f| draw(f))?;
            last_redraw = Instant::now();
        }

        // Poll for events with a timeout
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => {
                        with_app_state(|state| state.current_menu.next());
                    }
                    KeyCode::Up => {
                        with_app_state(|state| state.current_menu.previous());
                    }
                    KeyCode::Enter => {
                        let action = with_app_state(|state| state.current_menu.selected_action());

                        match action {
                            MenuAction::Function(func) => {
                                tokio::spawn(async move {
                                    let result = func().await;
                                    with_app_state(|state| match result {
                                        Ok(_) => {}
                                        Err(e) => {
                                            state.log_data.right_top.push(format!("Error: {}", e))
                                        }
                                    })
                                });
                            }
                            MenuAction::Submenu(menu_name) => {
                                with_app_state(|state| {
                                    let (new_menu, new_title) = match menu_name.as_str() {
                                        "main" => (create_main_menu(), "Main Menu".to_string()),
                                        "raydium" => {
                                            (create_raydium_menu(), "Raydium Menu".to_string())
                                        }
                                        "pumpfun" => {
                                            (create_pumpfun_menu(), "PumpFun Menu".to_string())
                                        }
                                        _ => (create_main_menu(), "Main Menu".to_string()),
                                    };
                                    state.current_menu = new_menu;
                                    state.menu_title = new_title;
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else {
            // If no event, yield to the executor to allow other tasks to run
            tokio::task::yield_now().await;
        }
    }
}
