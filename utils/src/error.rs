use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

/// Main error type for the application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Rate limit exceeded")]
    RateLimited,
}

/// JSON response for errors
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl AppError {
    /// Maps application errors to appropriate HTTP status codes
    pub fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::Authorization(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ExternalService(_) => StatusCode::BAD_GATEWAY,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RateLimited => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    /// Helper to create a not found error with custom message
    pub fn not_found(resource: &str) -> Self {
        AppError::NotFound(format!("Resource not found: {}", resource))
    }

    /// Helper to create an invalid input error
    pub fn invalid_input(message: &str) -> Self {
        AppError::InvalidInput(message.to_string())
    }

    /// Helper to create an internal error
    pub fn internal(message: &str) -> Self {
        AppError::Internal(message.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.to_string();

        tracing::error!(%status, %message, "API error occurred");

        let body = Json(ErrorResponse {
            status: status.to_string(),
            message,
        });

        (status, body).into_response()
    }
}

// Implementation of From for common error types
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<sea_orm::TransactionError<AppError>> for AppError {
    fn from(err: sea_orm::TransactionError<AppError>) -> Self {
        match err {
            sea_orm::TransactionError::Connection(db_err) => {
                // Convert the underlying DbErr
                AppError::from(db_err)
            }
            sea_orm::TransactionError::Transaction(app_err) => {
                // The error from the transaction closure is already an AppError
                app_err
            }
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(format!("IO error: {}", err))
    }
}

/// Type alias for Results using AppError
pub type Result<T> = std::result::Result<T, AppError>;
