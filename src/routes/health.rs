use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: &'static str,
}

pub async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse { status: "ok" })
}
