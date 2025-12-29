use axum::{Json, Router, extract::State, routing::post};
use shared::{
    endpoints::{API, user_endpoints::UserEndpoints},
    errors::{AppResult, auth_errors::AuthError},
    models::user_dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
};
use ulid::Ulid;
use validator::Validate;

use crate::{
    AppState,
    service::{refresh_token_service::RefreshTokenService, user_service::UserService},
};

pub fn public_user_router() -> Router<AppState> {
    Router::new()
        .route(
            UserEndpoints::RegisterUserEndpoint.template(),
            post(register),
        )
        .route(UserEndpoints::LoginUserEndpoint.template(), post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    payload
        .validate()
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    let user_model = UserService::create_user(
        &state.connection,
        payload.username,
        payload.email,
        payload.password,
    )
    .await?;

    let refresh_token = Ulid::new().to_string();
    RefreshTokenService::create_refresh_token(&state.connection, &refresh_token, &user_model.ulid)
        .await?;

    let response = RegisterResponse {
        ulid: user_model.ulid,
        username: user_model.username,
        email: user_model.email,
        created_at: user_model.created_at,
        refresh_token,
    };

    Ok(Json(response))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    payload
        .validate()
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    let user_model = UserService::login_by_username_and_password(
        &state.connection,
        &payload.email,
        &payload.password,
    )
    .await?;

    let refresh_token = Ulid::new().to_string();
    RefreshTokenService::create_refresh_token(&state.connection, &refresh_token, &user_model.ulid)
        .await?;

    let response = LoginResponse {
        ulid: user_model.ulid,
        username: user_model.username,
        email: user_model.email,
        refresh_token,
    };

    Ok(Json(response))
}
