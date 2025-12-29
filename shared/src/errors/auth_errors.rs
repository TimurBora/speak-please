use sea_orm::{DbErr, SqlErr};
use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, Debug, Error, Type)]
pub enum AuthError {
    #[error("Invalid username or password")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Password hashing failed")]
    HashError,
}

impl From<DbErr> for AuthError {
    fn from(err: DbErr) -> Self {
        if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
            return Self::UserAlreadyExists;
        }
        panic!("Database error in auth context: {:?}", err);
    }
}

impl From<argon2::password_hash::phc::Error> for AuthError {
    fn from(_: argon2::password_hash::phc::Error) -> Self {
        AuthError::HashError
    }
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(_: argon2::password_hash::Error) -> Self {
        AuthError::HashError
    }
}
