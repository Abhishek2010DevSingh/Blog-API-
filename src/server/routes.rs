use axum::Router;

pub fn setup_routes() -> Router {
    Router::new().layer(tower_http::trace::TraceLayer::new_for_http())
}
