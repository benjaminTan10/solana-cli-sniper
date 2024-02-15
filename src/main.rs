use firstx::{
    app::{
        app,
        embeds::{embed, license_checker},
    },
    raydium::subscribe::raydium_stream,
};
use log::info;
use serde_json::json;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let _ = app().await;
}
