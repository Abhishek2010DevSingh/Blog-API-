use axum::{extract::Path, Json};
use axum_valid::Valid;

use crate::{database::extractor::DatabaseConnection, error::AppError, model::blog::BlogPost};

use super::create::BlogPostBody;

pub async fn update_by_id(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
    Valid(Json(value)): Valid<Json<BlogPostBody>>,
) -> Result<Json<BlogPost>, AppError> {
    let value = sqlx::query_as!(
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
        value.title,
        value.content,
        value.category,
        &value.tags,
        id
    )
    .fetch_optional(&mut *conn)
    .await?;
}
