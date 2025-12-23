use axum::{Json, extract::State, routing::post, Router};
use crate::AppState;
use shared::models::user_dto::{RegisterRequest, UserResponse};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // логика...
}

async fn login(...) { ... }

// TODO: THINK ABOUT IT
