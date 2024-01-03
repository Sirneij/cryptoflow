use crate::utils::CustomAppJson;
use argon2::password_hash::Error as ArgonError;
use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

pub enum ErrorContext {
    UnauthorizedAccess,
    InternalServerError,
    BadRequest,
    NotFound,
}

pub enum CustomAppError {
    JsonRejection(JsonRejection),
    DatabaseQueryError(sqlx::Error),
    PasswordHashError(ArgonError),
    RedisError(bb8_redis::redis::RedisError),
    UUIDError(uuid::Error),
    Unauthorized(String),
    InternalError(String),
    BadRequest(String),
    NotFound(String),
    ReqwestError(reqwest::Error),
}

impl IntoResponse for CustomAppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized

        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
            status_code: u16,
        }

        let (status, message) = match self {
            CustomAppError::JsonRejection(rejection) => {
                // This error is caused by bad user input so don't log it
                tracing::error!("Bad user input: {:?}", rejection);
                (rejection.status(), rejection.body_text())
            }
            CustomAppError::DatabaseQueryError(error) => {
                match &error {
                    sqlx::Error::RowNotFound => {
                        tracing::error!("Resource not found: {}", error);
                        (
                            StatusCode::NOT_FOUND,
                            "Resource not found or you are not allowed to perform this operation"
                                .to_string(),
                        )
                    }
                    sqlx::Error::Database(db_error) => {
                        if let Some(constraint) = db_error.constraint() {
                            match constraint {
                                "users_email_key" => {
                                    tracing::info!("Email already exists: {}", db_error);
                                    (
                                        StatusCode::BAD_REQUEST,
                                        "An account with this email already exists.".to_string(),
                                    )
                                }
                                // Handle other constraints if needed
                                _ => {
                                    tracing::error!("Database constraint error: {}", db_error);
                                    (
                                        StatusCode::BAD_REQUEST,
                                        format!("Database constraint error: {}", db_error),
                                    )
                                }
                            }
                        } else {
                            tracing::error!("Database error: {}", error);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "We're experiencing some technical difficulties. Please try again later.".to_string(),
                            )
                        }
                    }
                    _ => {
                        tracing::error!("Database error: {}", error);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "We're experiencing some technical difficulties. Please try again later.".to_string(),
                        )
                    }
                }
            }
            CustomAppError::PasswordHashError(error) => match error {
                ArgonError::Password => {
                    tracing::info!("Password mismatch error");
                    (
                        StatusCode::BAD_REQUEST,
                        "Email and Password combination does not match.".to_string(),
                    )
                }
                _ => {
                    // Handle other Argon2 errors
                    tracing::error!("Password hashing error: {}", error);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "An error occurred during password processing.".to_string(),
                    )
                }
            },
            CustomAppError::RedisError(error) => {
                // Check if the error is due to a nil response, which often means the key wasn't found
                if let bb8_redis::redis::ErrorKind::TypeError = error.kind() {
                    if error.to_string().contains("(response was nil)") {
                        tracing::info!("Redis key not found");
                        (
                            StatusCode::NOT_FOUND,
                            "Requested key or token is invalid or not found.".to_string(),
                        )
                    } else {
                        // Handle other TypeError cases
                        tracing::error!("Redis type error: {}", error);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Redis encountered a type error.".to_string(),
                        )
                    }
                } else {
                    // Handle general Redis errors
                    tracing::error!("Redis error: {}", error);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "We're experiencing some technical difficulties with Redis. Please try again later.".to_string(),
                    )
                }
            }
            CustomAppError::UUIDError(error) => {
                tracing::error!("UUID error: {}", error);
                (
                    StatusCode::BAD_REQUEST,
                    "Invalid UUID provided.".to_string(),
                )
            }
            CustomAppError::Unauthorized(error_message) => {
                tracing::warn!("Unauthorized access: {}", error_message);
                (StatusCode::UNAUTHORIZED, error_message)
            }
            CustomAppError::InternalError(error_message) => {
                tracing::error!("Internal server error: {}", error_message);
                (StatusCode::INTERNAL_SERVER_ERROR, error_message)
            }
            CustomAppError::BadRequest(error_message) => {
                tracing::warn!("Bad request: {}", error_message);
                (StatusCode::BAD_REQUEST, error_message)
            }
            CustomAppError::NotFound(error_message) => {
                tracing::warn!("Not found: {}", error_message);
                (StatusCode::NOT_FOUND, error_message)
            }
            CustomAppError::ReqwestError(error) => {
                tracing::error!("HTTP request error: {}", error);
                let status_inner = if error.is_timeout() {
                    StatusCode::REQUEST_TIMEOUT
                } else if error.is_connect() {
                    StatusCode::SERVICE_UNAVAILABLE
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                };
                (status_inner, "Failed to process the request".to_string())
            }
        };

        (
            status,
            CustomAppJson(ErrorResponse {
                message,
                status_code: status.as_u16(),
            }),
        )
            .into_response()
    }
}

impl From<JsonRejection> for CustomAppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<sqlx::Error> for CustomAppError {
    fn from(error: sqlx::Error) -> Self {
        Self::DatabaseQueryError(error)
    }
}

impl From<ArgonError> for CustomAppError {
    fn from(error: ArgonError) -> Self {
        Self::PasswordHashError(error)
    }
}

impl From<bb8_redis::redis::RedisError> for CustomAppError {
    fn from(error: bb8_redis::redis::RedisError) -> Self {
        Self::RedisError(error)
    }
}

impl From<uuid::Error> for CustomAppError {
    fn from(error: uuid::Error) -> Self {
        Self::UUIDError(error)
    }
}

impl From<reqwest::Error> for CustomAppError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<(String, ErrorContext)> for CustomAppError {
    fn from((message, context): (String, ErrorContext)) -> Self {
        match context {
            ErrorContext::UnauthorizedAccess => CustomAppError::Unauthorized(message),
            ErrorContext::InternalServerError => CustomAppError::InternalError(message),
            ErrorContext::BadRequest => CustomAppError::BadRequest(message),
            ErrorContext::NotFound => CustomAppError::NotFound(message),
        }
    }
}
