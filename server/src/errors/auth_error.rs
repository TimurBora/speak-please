use axum::{Json, http::StatusCode, response::IntoResponse};
use sea_orm::{DbErr, SqlErr};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    DatabaseError(DbErr),
    HashError,
}

impl From<DbErr> for AuthError {
    fn from(err: DbErr) -> Self {
        if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
            return Self::UserAlreadyExists;
        }
        Self::DatabaseError(err)
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

impl AuthError {
    fn status(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthError::DatabaseError(_) | AuthError::HashError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            AuthError::InvalidCredentials => "Invalid username or password",
            AuthError::UserAlreadyExists => "User already exists",
            AuthError::DatabaseError(_) => "Internal database error",
            AuthError::HashError => "Error processing password",
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let (status, error_message) = (self.status(), self.message());
        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
