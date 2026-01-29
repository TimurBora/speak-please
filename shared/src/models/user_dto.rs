use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Type)]
pub struct RegisterRequest {
    #[validate(length(
        min = 5,
        max = 20,
        message = "Username must be between 5 and 20 characters long",
    ))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct RegisterResponse {
    pub ulid: String,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: String,
    pub level: u32,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Type)]
pub struct LoginRequest {
    pub password: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LoginResponse {
    pub ulid: String,
    pub username: String,
    pub email: String,
    pub refresh_token: String,
    pub level: u32,
    pub avatar_url: Option<String>,
}
