use crate::model::blog::BlogPost;
use crate::{database::extractor::DatabaseConnection, error::AppError};
use axum::{extract::Path, Json};

/// Retrieves a blog post by its ID from the database.
///
/// # Arguments
///
/// * `DatabaseConnection(mut conn)`: A wrapper for the database connection.
/// * `Path(id)`: The ID of the blog post to retrieve.
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Json(Some(BlogPost))` if the blog post is found.
/// - `Json(None)` if no blog post with the given ID exists.
/// - `AppError` if an error occurs during the database query.
///
/// # Errors
///
/// This function will return an `AppError` if:
/// - The database query fails.
///
/// # Example
///
/// ```
/// GET /posts/1
pub async fn find_by_id(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<Json<Option<BlogPost>>, AppError> {
    let value = sqlx::query_as!(BlogPost, "SELECT * FROM blog_posts WHERE id = $1", id)
        .fetch_optional(&mut *conn)
        .await?;

    match value {
        Some(post) => Ok(Json(Some(post))),
        None => Err(AppError::NotFound("Blog post not found".to_string())),
    }
}
