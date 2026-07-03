use axum::Json;
use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    healthy: bool,
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse { healthy: true }))
}

pub fn get_health_router() -> Router {
    Router::new()
        .route("/", get(health_handler))
        .route("/health", get(health_handler))
}
