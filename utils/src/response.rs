use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppError;

/// Standard error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

/// Standard success response structure for data
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSuccess<T> {
    pub data: T,
    pub message: Option<String>,
}

/// Helper functions for standard API responses

/// Return a 200 OK response with data
pub fn ok<T>(data: T) -> impl IntoResponse
where
    T: Serialize,
{
    (StatusCode::OK, Json(data))
}

/// Return a 201 Created response with data
pub fn created<T>(data: T) -> (StatusCode, Json<T>)
where
    T: Serialize,
{
    (StatusCode::CREATED, Json(data))
}

/// Return a 204 No Content response
pub fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}

/// A wrapper for ApiResult that implements IntoResponse
/// This allows us to use ApiResult as a return type in axum handlers
pub struct ApiResponse<T>(pub Result<T, AppError>);

impl<T> ApiResponse<T> {
    pub fn new(result: Result<T, AppError>) -> Self {
        Self(result)
    }
}

impl<T: IntoResponse> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self.0 {
            Ok(value) => value.into_response(),
            Err(err) => err.into_response(),
        }
    }
}

// Implement From for Result to allow .into() to work directly
impl<T, E> From<Result<T, E>> for ApiResponse<T>
where
    E: Into<AppError>,
{
    fn from(result: Result<T, E>) -> Self {
        Self(result.map_err(Into::into))
    }
}
