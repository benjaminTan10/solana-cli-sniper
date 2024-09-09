use chrono::Local;
use colored::Colorize;
use log::info;
use pretty_env_logger::env_logger::fmt::Color;
use std::io::Write;
use Mevarik::{
    app::{app, embeds::embed},
    env::utils::read_keys,
};
#[tokio::main]
async fn main() {
    pretty_env_logger::env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format(|f, record| {
            let level = record.level();
            let color = match level {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug => Color::Blue,
                log::Level::Trace => Color::Magenta,
            };

            let mut style = f.style();
            style.set_color(color).set_bold(true);

            let timestamp = Local::now().format("%I:%M:%S%.3f %p");

            writeln!(
                f,
                "{} {} {} {}",
                style.value(level),
                timestamp,
                "⮞ ".bold().bright_black(),
                record.args()
            )
        })
        .init();

    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", embed());
    // info!("Authenticating...");

    // let _ = match Mevarik::auth::auth_verification().await {
    //     Ok(_) => {
    //         println!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //         println!("{}", embed());
    //         println!("{}", "Authentication Successful".bold().green());
    //     }
    //     Err(e) => {
    //         log::error!("Error: {}", e);
    //         let _ = read_keys().await;
    //         return;
    //     }
    // };

    let _ = app(true).await;

    let _ = read_keys().await;
}
