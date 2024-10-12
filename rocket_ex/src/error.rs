use serde::{Deserialize, Serialize};
use sqlx::Error;
use thiserror::Error;

pub type CustomResult<T> = Result<T, CustomError>;

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
