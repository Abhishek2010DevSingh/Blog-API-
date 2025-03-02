use sqlx::PgPool;

/// Creates a full-text search (FTS) index on the `blog_posts` table.
///
/// This index uses a GIN index with `to_tsvector` for efficient text searching
/// across the `title`, `content`, and `tags` columns.
///
/// # Arguments
/// * `pool` - A reference to a `PgPool` for executing the query.
///
/// # Returns
/// * `anyhow::Result<()>` - Returns `Ok(())` if successful, otherwise an error.
///
/// # Notes
/// * The `IF NOT EXISTS` clause ensures the index is created only if it doesn't already exist.
/// # Errors
/// * If the query fails to execute, an error is returned.
///
/// * This index enhances full-text search performance in PostgreSQL.
pub async fn create_index(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        CREATE INDEX IF NOT EXISTS idx_fts 
        ON blog_posts USING GIN (
            to_tsvector('english', title || ' ' || content || ' ' || tags)
        );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
