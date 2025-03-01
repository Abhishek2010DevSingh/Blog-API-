use anyhow::Context;

pub async fn db_connect() -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}
