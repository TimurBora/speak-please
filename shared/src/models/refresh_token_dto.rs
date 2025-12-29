use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

use crate::utils::ulid_validation::validate_ulid;

#[derive(Serialize, Deserialize, Validate, Type)]
pub struct CreateRefreshTokenRequest {
    #[validate(custom(function = "validate_ulid"))]
    pub refresh_token: String,

    #[validate(length(
        min = 5,
        max = 20,
        message = "Username must be between 5 and 20 characters long",
    ))]
    pub email: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct CreateRefreshTokenResponse {
    pub access_token: String,
    pub new_refresh_token: String,
    pub expires_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Validate, Type)]
pub struct DeleteRefreshTokenRequest {
    #[validate(custom(function = "validate_ulid"))]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct DeleteRefreshTokenResponse {
    pub expires_in_seconds: i64,
}
