use axum::Router;

use crate::state::AppState;

pub fn setup_routes(state: AppState) -> Router {
    Router::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}
