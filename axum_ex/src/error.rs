use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use thiserror::Error;

pub type CustomResult<T> = Result<T, CustomError>;

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CustomError::InternalError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            CustomError::NotFound(e) => (StatusCode::NOT_FOUND, e).into_response(),
        }
    }
}

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CustomError {
    #[error("Endpoint is not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for CustomError {
    fn from(e: Error) -> Self {
        Self::InternalError(e.to_string())
    }
}
