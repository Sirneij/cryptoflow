use crate::utils::SuccessResponse;
use axum::{http::StatusCode, response::IntoResponse};

#[tracing::instrument]
pub async fn health_check() -> impl IntoResponse {
    SuccessResponse {
        message: "Rust(Axum) and SvelteKit application is healthy!".to_string(),
        status_code: StatusCode::OK.as_u16(),
    }
    .into_response()
}
