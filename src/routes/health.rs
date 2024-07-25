use axum::{Json, response::IntoResponse};
use crate::model::health::HealthResponse;

pub(crate) async fn health_status() -> impl IntoResponse {
    let response = HealthResponse {
        status: "UP",
    };

    Json(response)
}