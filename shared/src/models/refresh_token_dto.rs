use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

use crate::utils::ulid_validation::validate_ulid;

#[derive(Debug, Serialize, Deserialize, Validate, Type)]
pub struct CreateRefreshTokenRequest {
    #[validate(custom(function = "validate_ulid"))]
    pub refresh_token: String,

    #[validate(email(message = "Not valid email",))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct CreateRefreshTokenResponse {
    pub access_token: String,
    pub new_refresh_token: String,
    pub expires_in_seconds: i64,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DeleteRefreshTokenResponse {
    pub expires_in_seconds: i64,
}
