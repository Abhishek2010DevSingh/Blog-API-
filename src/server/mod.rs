pub mod routes;

use anyhow::Context;

pub async fn setup_server() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .context("Failed to bind to port 3000")?;

    tracing::info!("Listening on http://localhost:3000");
    axum::serve(listener, routes::setup_routes())
        .await
        .context("Failed to start server")?;

    Ok(())
}
