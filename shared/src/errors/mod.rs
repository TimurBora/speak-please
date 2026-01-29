pub mod auth_errors;
pub mod jwt_errors;

use crate::errors::{auth_errors::AuthError, jwt_errors::JwtError};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::{DbErr, SqlErr};
use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors};

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

    #[error("Server error: {0}")]
    Server(String),

    #[error("Validation error: {0}")]
    Validation(
        #[from]
        #[specta(skip)]
        ValidationErrors,
    ),

    #[error("Custom error: {0}")]
    Custom(String),

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
            Self::Custom(msg) => (StatusCode::BAD_REQUEST, ErrorCode::CustomError, msg.clone()),
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

    pub async fn from_response(res: reqwest::Response) -> Self {
        let status = res.status();

        if let Ok(error_body) = res.json::<ErrorBody>().await {
            return error_body.into();
        }

        Self::Api(format!("HTTP Error: {}", status))
    }
}

impl From<ErrorBody> for AppError {
    fn from(error_body: ErrorBody) -> Self {
        match error_body.error_type {
            ErrorCode::AuthInvalid => Self::Auth(AuthError::InvalidCredentials),
            ErrorCode::UserExists => Self::Auth(AuthError::UserAlreadyExists),
            ErrorCode::ValidationError => {
                Self::Auth(AuthError::ValidationError(error_body.message))
            }
            ErrorCode::CustomError => Self::Custom(error_body.message),
            ErrorCode::NotFound => Self::NotFound,
            ErrorCode::DatabaseError => Self::Database(sea_orm::DbErr::Custom(error_body.message)),
            ErrorCode::ServerError => Self::Server(error_body.message),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::Api(err.to_string())
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

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        let mut errors = ValidationErrors::new();
        errors.add("default", err);
        Self::Validation(errors)
    }
}

pub trait DbResultExt<T> {
    fn map_db_error(self) -> AppResult<T>;
}

impl<T> DbResultExt<T> for Result<T, DbErr> {
    fn map_db_error(self) -> AppResult<T> {
        self.map_err(|err| {
            if let Some(SqlErr::UniqueConstraintViolation(details)) = err.sql_err() {
                tracing::warn!(target: "db", violation = %details, "Unique constraint violation");
                return AppError::Auth(AuthError::UserAlreadyExists);
            }

            tracing::error!(target: "db", error = %err, "Database operation failed");
            AppError::Database(err)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
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

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    AuthInvalid,
    UserExists,
    ValidationError,
    NotFound,
    DatabaseError,
    ServerError,
    CustomError,
}

pub type AppResult<T> = Result<T, AppError>;
pub type FrontendRepresentation<T> = Result<T, ErrorBody>;
