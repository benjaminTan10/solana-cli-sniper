use firstx::app::app;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let _ = app().await;
}
