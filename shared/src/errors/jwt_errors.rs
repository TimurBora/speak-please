use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, Debug, Error, Type)]
pub enum JwtError {
    #[error("Invalid or expired token")]
    InvalidToken,
    #[error("JWT configuration error")]
    InvalidKey,
    #[error("Token creation failed")]
    CreationFailed,
}
