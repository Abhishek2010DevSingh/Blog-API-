use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

/// Represents different types of application errors.
///
/// This enum defines common errors that can occur within the application.
/// It implements `IntoResponse` to convert errors into HTTP responses
/// for Axum-based APIs.
///
/// # Variants
///
/// * `DatabaseError` - Represents an error occurring in database operations.
/// * `NotFound` - Indicates that the requested resource was not found.
/// * `BadRequest` - Represents a client-side request error.
/// * `InternalServerError` - Covers unexpected server-side errors.
///
/// # Example
///
/// ```rust
/// use crate::error::AppError;
///
/// let error = AppError::NotFound("User not found".into());
/// println!("{}", error); // Outputs: "Not Found: User not found"
/// ```
#[derive(Debug, Error)]
pub enum AppError {
    /// Represents an SQL database error.
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    /// Represents a "Not Found" error, typically for missing resources.
    #[error("Not Found: {0}")]
    NotFound(String),

    /// Represents a "Bad Request" error, typically due to invalid input.
    #[error("Bad Request: {0}")]
    BadRequest(String),

    /// Represents an internal server error.
    #[error("Internal Server Error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    /// Converts the `AppError` into an HTTP response.
    ///
    /// Logs the error and maps it to an appropriate HTTP status code and JSON body.
    ///
    /// # Response Format
    /// The response body is a JSON object with an `"error"` field describing the issue.
    ///
    /// | Error Variant         | HTTP Status Code        | Message                         |
    /// |-----------------------|------------------------|---------------------------------|
    /// | `DatabaseError`       | `500 Internal Server Error` | "A database error occurred."  |
    /// | `NotFound`            | `404 Not Found`        | Custom message                 |
    /// | `BadRequest`          | `400 Bad Request`      | Custom message                 |
    /// | `InternalServerError` | `500 Internal Server Error` | "An internal server error occurred." |
    ///
    /// # Example Usage
    ///
    /// ```rust
    /// use axum::response::IntoResponse;
    /// use crate::error::AppError;
    ///
    /// let error = AppError::NotFound("User not found".into());
    /// let response = error.into_response();
    /// ```
    fn into_response(self) -> Response {
        error!("{}", self);

        let (status, error_message) = match &self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database error occurred.",
            ),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message.as_str()),
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.as_str()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred.",
            ),
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}
