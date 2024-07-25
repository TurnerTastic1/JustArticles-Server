use axum::Json;
use axum::response::IntoResponse;
use crate::models::health_response::HealthResponse;

pub async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "UP",
    };

    Json(response)
}
