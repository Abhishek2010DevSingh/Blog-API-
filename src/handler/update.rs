use axum::{extract::Path, Json};
use axum_valid::Valid;

use crate::{database::extractor::DatabaseConnection, error::AppError, model::blog::BlogPost};

use super::create::BlogPostBody;

pub async fn update_by_id(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
    Valid(value): Valid<Json<BlogPostBody>>,
) -> Result<Json<BlogPost>, AppError> {
}
