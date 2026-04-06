use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Invalid Input, cannot be processed: {field} - {message}")]
    UnProcessableEntity { field: String, message: String },
    #[error("Environment variable is missing: {0}")]
    MissingEnvironmentVarible(String),
    #[error("Failed to parse: {0}")]
    ParsingError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": message })),
            )
                .into_response(),
            AppError::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": message })),
            )
                .into_response(),
            AppError::UnProcessableEntity { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "status": "error",
                    "field": field,
                    "message": message
                })),
            )
                .into_response(),
            AppError::MissingEnvironmentVarible(message) | AppError::ParsingError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": message })),
            )
                .into_response(),
        }
    }
}
