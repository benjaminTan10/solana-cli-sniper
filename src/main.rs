use firstx::app::app;

#[tokio::main]
async fn main() {
    pretty_env_logger::env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_module_path(false)
        .init();

    let _ = app().await;
}
