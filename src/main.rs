mod routes;
mod model;

use axum::{routing::{get}, Router};
use log::{debug, warn};

#[tokio::main]
async fn main() {
    init_logging();

    debug!("Starting server...");
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(routes::health::health_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub fn init_logging() {
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        warn!("Failed to initialize logging: {:?}", e);
    }
}