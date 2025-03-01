use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use sqlx::PgPool;

use crate::error::AppError;

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        let conn = pool.acquire().await?;
        Ok(Self(conn))
    }
}
