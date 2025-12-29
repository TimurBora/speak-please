use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, post},
};
use shared::{
    endpoints::{API, refresh_token_endpoints::RefreshTokenEndpoints},
    errors::{AppError, AppResult, auth_errors::AuthError, jwt_errors::JwtError},
    models::refresh_token_dto::{
        CreateRefreshTokenRequest, CreateRefreshTokenResponse, DeleteRefreshTokenRequest,
    },
    utils::{jwt::create_access_token, time::is_expired, ulid_validation::validate_ulid},
};

use validator::Validate;

use crate::{AppState, service::refresh_token_service::RefreshTokenService};

pub fn refresh_token_router() -> Router<AppState> {
    Router::new()
        .route(
            RefreshTokenEndpoints::CreateRefreshToken.template(),
            post(create_refresh_access_token),
        )
        .route(
            RefreshTokenEndpoints::DeleteRefreshToken("0".to_string()).template(),
            delete(delete_refresh_token),
        )
    //.route("/login", post(login))
}

pub async fn create_refresh_access_token(
    State(state): State<AppState>,
    Json(payload): Json<CreateRefreshTokenRequest>,
) -> AppResult<Json<CreateRefreshTokenResponse>> {
    payload
        .validate()
        .map_err(|e| AuthError::ValidationError(e.to_string()))?;

    let refresh_token =
        RefreshTokenService::find_by_token(&state.connection, &payload.refresh_token)
            .await?
            .ok_or(JwtError::InvalidToken)?;

    if is_expired(refresh_token.expires_at) {
        RefreshTokenService::delete_by_id(&state.connection, &refresh_token.ulid).await?;
        return Err(JwtError::InvalidToken.into());
    }

    let access_token = create_access_token(refresh_token.user_id, payload.username)?;

    let (_, new_refresh_token) =
        RefreshTokenService::rotate_refresh_token(&state.connection, &payload.refresh_token)
            .await?;

    Ok(Json(CreateRefreshTokenResponse {
        access_token,
        new_refresh_token,
        expires_in_seconds: 3600,
    }))
}

async fn delete_refresh_token(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<DeleteRefreshTokenRequest>,
) -> AppResult<StatusCode> {
    payload.validate().map_err(|_| JwtError::InvalidToken)?;
    validate_ulid(&id)?;

    let delete_result = RefreshTokenService::delete_by_token(&state.connection, &id).await?;

    if delete_result.rows_affected == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
