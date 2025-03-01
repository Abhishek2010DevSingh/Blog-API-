use anyhow::Context;

pub async fn connect() -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/axum")
        .await
        .context("Failed to connect to Postgres")?;

    Ok(pool)
}
