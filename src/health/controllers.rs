use axum::{http::StatusCode, response::IntoResponse};

#[tracing::instrument()]
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
