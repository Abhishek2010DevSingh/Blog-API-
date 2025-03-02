use crate::{
    handler::{
        create::create_post, delete::delete_by_id, list::find_all, read::find_by_id,
        search::search_posts, update::update_by_id,
    },
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

/// Configures the application's routes and middleware.
///
/// This function sets up the routing for the application and applies necessary middleware.
/// It initializes a new `Router`, adds a tracing layer for logging HTTP requests and responses,
/// and assigns the provided application state.
///
/// # Arguments
///
/// * `state` - An instance of `AppState` containing shared application state.
///
/// # Returns
///
/// A configured `Router` instance ready to be used in the Axum server.
///
/// # Example
///
/// ```rust
/// use axum::{Router, routing::get};
/// use tower_http::trace::TraceLayer;
/// use crate::state::AppState;
///
/// let state = AppState { /* initialize state */ };
/// let app = setup_routes(state);
/// ```
///
/// # Notes
///
/// - The `TraceLayer` from `tower_http` is used to log high-level information about incoming
///   HTTP requests and responses. This is useful for monitoring and debugging purposes.
/// - The application state is shared across all routes using Axum's state management.
pub fn setup_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/posts", post(create_post).get(find_all))
        .route("/posts/search", get(search_posts))
        .route(
            "/posts/{id}",
            get(find_by_id).put(update_by_id).delete(delete_by_id),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
