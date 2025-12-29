pub mod auth_errors;
pub mod jwt_errors;

use crate::errors::{auth_errors::AuthError, jwt_errors::JwtError};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;
use validator::ValidationError;

#[derive(Serialize, Debug, Error, Type)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Jwt(#[from] JwtError),

    #[error("Database error")]
    Database(
        #[from]
        #[specta(skip)]
        #[serde(skip)]
        sea_orm::DbErr,
    ),

    #[error("Keyring error")]
    Keyring(String),

    #[error("External API error: {0}")]
    Api(String),

    #[error("Validation error: {0}")]
    Validation(
        #[from]
        #[specta(skip)]
        ValidationError,
    ),

    #[error("Not found")]
    NotFound,
}

impl AppError {
    pub fn client_details(&self) -> (StatusCode, ErrorCode, String) {
        match self {
            Self::Auth(AuthError::InvalidCredentials) | Self::Jwt(JwtError::InvalidToken) => (
                StatusCode::UNAUTHORIZED,
                ErrorCode::AuthInvalid,
                self.to_string(),
            ),
            Self::Auth(AuthError::UserAlreadyExists) => (
                StatusCode::CONFLICT,
                ErrorCode::UserExists,
                self.to_string(),
            ),
            Self::Auth(AuthError::ValidationError(msg)) => (
                StatusCode::BAD_REQUEST,
                ErrorCode::ValidationError,
                msg.clone(),
            ),
            Self::Validation(e) => (
                StatusCode::BAD_REQUEST,
                ErrorCode::ValidationError,
                e.to_string(),
            ),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorCode::NotFound,
                "Resource not found".to_string(),
            ),
            Self::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorCode::DatabaseError,
                "A database error occurred".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorCode::DatabaseError,
                "An internal server error occurred".to_string(),
            ),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, _, _) = self.client_details();
        let body: ErrorBody = self.into();
        (status, Json(body)).into_response()
    }
}

impl From<sea_orm::TransactionError<AppError>> for AppError {
    fn from(err: sea_orm::TransactionError<AppError>) -> Self {
        match err {
            sea_orm::TransactionError::Connection(e) => AppError::Database(e),
            sea_orm::TransactionError::Transaction(e) => e,
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api(err.to_string())
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct ErrorBody {
    error_type: ErrorCode,
    message: String,
}

impl From<AppError> for ErrorBody {
    fn from(err: AppError) -> Self {
        let (_, error_type, message) = err.client_details();
        Self {
            error_type,
            message,
        }
    }
}

#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    AuthInvalid,
    UserExists,
    ValidationError,
    NotFound,
    DatabaseError,
    ServerError,
}

pub type AppResult<T> = Result<T, AppError>;
pub type FrontendRepresentation<T> = Result<T, ErrorBody>;
