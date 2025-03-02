use axum::{extract::Path, Json};
use axum_valid::Valid;
use sqlx::query_as;

use crate::{database::extractor::DatabaseConnection, error::AppError, model::blog::BlogPost};

use super::create::BlogPostBody;
/// Updates a blog post by its ID in the database.
///
/// # Arguments
///
/// * `DatabaseConnection(mut conn)`: A wrapper for the database connection.
/// * `Path(id)`: The ID of the blog post to update.
/// * `Valid(Json(payload))`: The validated JSON payload containing the updated blog post data.
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Json(BlogPost)` if the update is successful.
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
/// PUT /posts/1
/// {
///   "title": "Updated Title",
///   "content": "Updated content",
///   "category": "Updated category",
///   "tags": ["updated", "tags"]
/// }
/// ```
pub async fn update_by_id(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
    Valid(Json(payload)): Valid<Json<BlogPostBody>>,
) -> Result<Json<BlogPost>, AppError> {
    let updated_post = query_as!(
        BlogPost,
        r#"
        UPDATE blog_posts 
        SET title = $1,
            content = $2,
            category = $3,
            tags = $4
        WHERE id = $5
        RETURNING *;
        "#,
        payload.title,
        payload.content,
        payload.category,
        &payload.tags,
        id
    )
    .fetch_optional(&mut *conn)
    .await?;

    updated_post
        .map(Json)
        .ok_or_else(|| AppError::NotFound("Blog post not found".to_string()))
}
