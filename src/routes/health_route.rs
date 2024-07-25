use axum::Router;
use axum::routing::get;
use crate::handlers::health_handler;

pub(crate) async fn health_status() -> Router {
    Router::new()
        .route("/health-status", get(health_handler::health_check))
}
