use anyhow::Context;
use blog_api::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().context("Failed to load .env file")?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    server::setup_server().await
}
