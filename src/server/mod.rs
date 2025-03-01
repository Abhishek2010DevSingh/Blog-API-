pub mod routes;

use anyhow::Context;

use crate::{database::connection::db_connect, state::AppState};

pub async fn setup_server() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .context("Failed to bind to port 3000")?;

    let pool = db_connect().await?;
    let state = AppState { pool };

    tracing::info!("Listening on http://localhost:3000");
    axum::serve(listener, routes::setup_routes(state))
        .await
        .context("Failed to start server")?;

    Ok(())
}
