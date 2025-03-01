use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use sqlx::PgPool;

use crate::error::AppError;

/// A wrapper around a SQLx PostgreSQL connection.
///
/// This struct encapsulates a `sqlx::pool::PoolConnection<sqlx::Postgres>`,
/// providing a convenient way to manage individual database connections.
pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

/// Extractor implementation for `DatabaseConnection`.
///
/// This allows `DatabaseConnection` to be extracted from request parts in Axum handlers,
/// facilitating seamless database interactions within request processing.
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    /// Extracts a `DatabaseConnection` from the request parts.
    ///
    /// This method retrieves a connection pool from the application state,
    /// acquires a connection from the pool, and returns it wrapped in `DatabaseConnection`.
    ///
    /// # Errors
    ///
    /// Returns an `AppError` if acquiring a connection from the pool fails.
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Retrieve the connection pool from the application state
        let pool = PgPool::from_ref(state);
        // Acquire a connection from the pool
        let conn = pool.acquire().await?;
        // Return the connection wrapped in DatabaseConnection
        Ok(Self(conn))
    }
}
