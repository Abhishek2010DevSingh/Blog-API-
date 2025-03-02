use axum::{Json, extract::Query};

use crate::{database::extractor::DatabaseConnection, error::AppError, model::blog::BlogPost};

/// Represents the query parameters for searching blog posts.
///
/// # Fields
///
/// * `term` - A string representing the search term used to filter blog posts.
///
/// # Example
///
/// ```
/// GET /posts?term=rust
/// ```
#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub term: String,
}

/// Searches for blog posts in the database based on a search term.
///
/// # Arguments
///
/// * `DatabaseConnection(mut conn)`: A wrapper for the database connection.
/// * `Query(SearchQuery { term })`: The search query containing the term to filter blog posts.
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Json(Vec<BlogPost>)` if matching blog posts are found.
/// - `AppError::NotFound` if no blog posts match the search criteria.
/// - `AppError` if an error occurs during the database query.
///
/// # Errors
///
/// This function will return an `AppError` if:
/// - The database query fails.
/// - No matching blog posts are found.
///
/// # Example
///
/// ```
/// GET /posts?term=rust
/// ```
pub async fn search_posts(
    DatabaseConnection(mut conn): DatabaseConnection,
    Query(SearchQuery { term }): Query<SearchQuery>,
) -> Result<Json<Vec<BlogPost>>, AppError> {
    let blog_posts = sqlx::query_as!(
        BlogPost,
        r#"
        SELECT * FROM blog_posts
        WHERE 
            title ILIKE '%' || $1 || '%' OR
            content ILIKE '%' || $1 || '%' OR
            array_to_string(tags, ',') ILIKE '%' || $1 || '%';
        "#,
        term
    )
    .fetch_all(&mut *conn)
    .await?;

    if blog_posts.is_empty() {
        return Err(AppError::NotFound("No blog posts found".to_string()));
    }

    Ok(Json(blog_posts))
}
