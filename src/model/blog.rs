use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a blog post stored in the database.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BlogPost {
    /// Unique identifier for the blog post.
    pub id: i32,

    /// Title of the blog post.
    pub title: String,

    /// Content of the blog post.
    pub content: String,

    /// Category to which the blog post belongs.
    pub category: String,

    /// List of tags associated with the blog post.
    pub tags: Vec<String>,

    /// Timestamp when the blog post was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp when the blog post was last updated.
    pub updated_at: DateTime<Utc>,
}
