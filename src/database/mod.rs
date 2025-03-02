/// Database-related functionality.
///
/// This module contains utilities for establishing and managing database connections.
/// It provides connection pooling and request extractors to simplify database access.
pub mod connection;

/// Request extractors for database connections.
///
/// This module defines custom extractors that allow handlers to easily access database
/// connections from the application state. These extractors integrate seamlessly with
/// Axum's request handling system.
pub mod extractor;
/// Full-text search functionality.
pub mod full_text_search;
