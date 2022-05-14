use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Your password is invalid: {0}")]
    InvalidPassword(#[from] bcrypt::BcryptError),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Internal server error")]
    InternalServerError,
    #[error("No cookie found in the cookie storage.")]
    NoSessionCookieFound,
}

#[derive(Serialize)]
pub struct ErrorBody {
    error: String,
    status_code: u16,
}
impl IntoResponse for ErrorBody {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg) = match self {
            ApiError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong in the server.",),
            ),
            ApiError::InvalidPassword(_) => (
                StatusCode::BAD_REQUEST,
                "Your password is invalid".to_string(),
            ),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong in the server.".to_string(),
            ),
            ApiError::NoSessionCookieFound => (
                StatusCode::OK,
                "No cookie found in the cookie storage.".to_string(),
            ),
        };

        let body = ErrorBody {
            error: error_msg,
            status_code: status.as_u16(),
        };

        (status, body).into_response()
    }
}
