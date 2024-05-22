use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::utils::CustomAppError;

use serde::Serialize;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(CustomAppError))]
pub struct CustomAppJson<T>(pub T);

impl<T> IntoResponse for CustomAppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
    pub status_code: u16,
    pub user_id: Option<uuid::Uuid>,
}

impl Default for SuccessResponse {
    fn default() -> Self {
        Self {
            message: "Success".to_string(),
            status_code: StatusCode::OK.as_u16(),
            user_id: None,
        }
    }
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::OK);
        let json_body = axum::Json(self);

        // Convert Json to Response
        let mut response = json_body.into_response();

        // Set the correct status code
        *response.status_mut() = status;

        response
    }
}
