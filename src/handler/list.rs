use axum::Json;

use crate::{database::extractor::DatabaseConnection, error::AppError, model::blog::BlogPost};

/// Retrieves all blog posts from the database.
///
/// # Arguments
///
/// * `DatabaseConnection(mut conn)`: A wrapper for the database connection.
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Json(Vec<BlogPost>)` if the query is successful.
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
/// GET /posts
/// ```
pub async fn find_all(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<BlogPost>>, AppError> {
    let posts = sqlx::query_as!(BlogPost, "SELECT * FROM blog_posts")
        .fetch_all(&mut *conn)
        .await?;
    Ok(Json(posts))
}
