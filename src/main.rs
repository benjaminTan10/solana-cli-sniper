use chrono::Local;
use colored::Colorize;
use pretty_env_logger::env_logger::fmt::Color;
use std::io::Write;
use Mevarik::app::app;

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

            let timestamp = Local::now().format("%d-%m-%Y %H:%M:%S%.3f");

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

    // let t = Input::new("Bot Password: ")
    //     .placeholder("5eSB1...vYF49")
    //     .prompt("Input: ");

    // let input_password = t.run().expect("error running input");

    // let correct_password = "MoonISDW@1234";

    // if input_password != correct_password {
    //     error!("Invalid password");
    //     return;
    // }

    let _ = app().await;
}
