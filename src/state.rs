use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}
