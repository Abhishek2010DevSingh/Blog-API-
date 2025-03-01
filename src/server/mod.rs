use crate::{database::connection::db_connect, state::AppState};
use anyhow::Context;
use tokio::net::TcpListener;

/// Routes module for the Axum server.
mod routes;

/// Initializes and runs the Axum server.
///
/// This function performs the following steps:
/// 1. Binds a TCP listener to `localhost` on port `3000`.
/// 2. Establishes a connection to the PostgreSQL database.
/// 3. Constructs the application state with the database connection pool.
/// 4. Sets up the application routes.
/// 5. Starts the Axum server with the configured routes and state.
///
/// # Errors
///
/// Returns an error if:
/// - Binding to the specified address and port fails.
/// - Establishing a database connection fails.
/// - Starting the server encounters an issue.
///
/// # Example
///
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     setup_server().await
/// }
/// ```
///
/// Ensure that the `DATABASE_URL` environment variable is set before running the server.
pub async fn setup_server() -> anyhow::Result<()> {
    // Bind a TCP listener to localhost on port 3000
    let listener = TcpListener::bind("localhost:3000")
        .await
        .context("Failed to bind to port 3000")?;

    // Establish a connection to the PostgreSQL database
    let pool = db_connect().await?;

    // Construct the application state with the database connection pool
    let state = AppState { pool };

    // Log the server's listening address
    tracing::info!("Listening on http://localhost:3000");

    // Set up the application routes with the provided state

    // Start the Axum server with the configured routes and state
    axum::serve(listener, routes::setup_routes(state))
        .await
        .context("Failed to start server")?;
    Ok(())
}
