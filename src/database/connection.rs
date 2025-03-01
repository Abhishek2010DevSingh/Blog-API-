use anyhow::{Context, Result};
use sqlx::{Pool, Postgres};
use std::env;

/// Establishes a connection pool to the PostgreSQL database and runs any pending migrations.
///
/// # Errors
///
/// Returns an error if:
/// - The `DATABASE_URL` environment variable is not set.
/// - The connection to the database cannot be established.
/// - Migrations fail to run.
///
/// # Panics
///
/// This function will panic if the migrations directory is not found.
pub async fn db_connect() -> Result<Pool<Postgres>> {
    // Retrieve the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;

    // Create a connection pool with a maximum of 5 connections
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    // Run pending migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    Ok(pool)
}
