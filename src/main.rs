use chrono::Local;
use colored::Colorize;
use log::info;
use pretty_env_logger::env_logger::fmt::Color;
use std::io::Write;
use Mevarik::app::{app, embeds::embed};

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

            let timestamp = Local::now().format("%I:%M:%S %p");

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

    info!("{}", embed());

    let _ = app(true).await;
}
