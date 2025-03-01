use axum::extract::FromRef;

/// Represents the shared application state.
///
/// This struct is used to manage and share resources across the application,
/// such as the database connection pool.
#[derive(Clone, FromRef)]
pub struct AppState {
    /// A connection pool for interacting with the PostgreSQL database.
    pub pool: sqlx::PgPool,
}
