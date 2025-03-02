use crate::{database::extractor::DatabaseConnection, error::AppError};
use axum::{extract::Path, http::StatusCode};

/// Deletes a blog post by its ID from the database.
///
/// # Arguments
///
/// * `DatabaseConnection(mut conn)`: A wrapper for the database connection.
/// * `Path(id)`: The ID of the blog post to delete.
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `StatusCode::NO_CONTENT` if the deletion is successful.
/// - `AppError::NotFound` if no blog post with the given ID exists.
/// - `AppError` if an error occurs during the database query.
///
/// # Errors
///
/// This function will return an `AppError` if:
/// - The database query fails.
/// - The specified blog post does not exist.
///
/// # Example
///
/// ```
/// DELETE /posts/1
/// ```
pub async fn delete_by_id(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let rows_affected = sqlx::query!("DELETE FROM blog_posts WHERE id = $1", id)
        .execute(&mut *conn)
        .await?;

    if rows_affected.rows_affected() == 0 {
        return Err(AppError::NotFound("Blog post not found".to_string()));
    }
    Ok(StatusCode::NO_CONTENT)
}
