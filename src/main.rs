use chrono::Local;
use colored::Colorize;
use log::{error, info};
use pretty_env_logger::env_logger::fmt::Color;
use std::io::Write;
use Mevarik::{
    app::{app, embeds::embed},
    auth::auth_verification,
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
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", embed());
    info!("Authenticating...");
    let _auth = match auth_verification().await {
        Ok(_) => {}
        Err(e) => {
            error!("Error: {}", e);
            return;
        }
    };
    //clear previous line
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", embed());
    println!("{}", "Authentication successful!".bold().green());
    let _ = app(true).await;
}

// use self_update::cargo_crate_version;

// pub async fn self_update() -> Result<(), Box<dyn std::error::Error>> {
//     let status = self_update::backends::s3::Update::configure()
//         .bucket_name("my-bucket")
//         .asset_prefix("my-app/new-version")
//         .region("us-east-1")
//         .bin_name("my-app")
//         .show_download_progress(true)
//         .current_version(cargo_crate_version!())
//         .build()?
//         .update()?;
//     println!("S3 Update status: `{}`!", status.version());
//     Ok(())
// }
