use actix_web::http::StatusCode;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_code: u16,
    pub error_message: String,
    pub time_stamp: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The requested item was not found.")]
    NotFound,
    #[error("You are forbidden to access this resource.")]
    Forbidden,
    // This allows wrapping other errors
    #[error("An unexpected error occurred.")]
    InternalError(String),
}

impl AppError {
    // Helper function to get the status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    // Helper function to get a machine-readable error type
    pub fn error_type(&self) -> String {
        match self {
            AppError::NotFound => "not_found".to_string(),
            AppError::Forbidden => "forbidden".to_string(),
            AppError::InternalError(_) => "internal_server_error".to_string(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

