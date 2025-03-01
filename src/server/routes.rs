use axum::{Router, routing::get};

use crate::{database::extractor::DatabaseConnection, error::AppError, state::AppState};

pub fn setup_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}

async fn hello(DatabaseConnection(mut conn): DatabaseConnection) -> Result<String, AppError> {
    let result: Option<i32> = sqlx::query_scalar!("SELECT 1 + 1")
        .fetch_one(&mut *conn)
        .await?;
    return Ok(format!("Hello, world! {}", result.unwrap()));
}
