use reqwest::Method;
use shared::{
    endpoints::{refresh_token_endpoints::RefreshTokenEndpoints, user_endpoints::UserEndpoints},
    errors::{jwt_errors::JwtError, AppError, FrontendRepresentation},
    models::user_dto::RegisterResponse,
};
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

use crate::auth::{
    keyring::{delete_refresh_token_internal, get_refresh_token_internal},
    service::AppState,
    session::UserSession,
};

use log::{debug, info, warn};

#[tauri::command]
#[specta::specta]
pub fn check_access_token(access_token: String) -> bool {
    let result = shared::utils::jwt::verify_access_token(&access_token).is_ok();
    debug!("Access token verification result: {}", result);
    result
}

#[tauri::command]
#[specta::specta]
pub async fn get_current_session(
    state: State<'_, AppState>,
) -> FrontendRepresentation<Option<UserSession>> {
    let service = &state.0;

    let needs_refresh = {
        let session_lock = service.session.read().await;
        match session_lock.as_ref() {
            Some(session) => {
                let is_invalid = session
                    .access_token
                    .as_ref()
                    .map(|t| shared::utils::jwt::verify_access_token(t).is_err())
                    .unwrap_or(true);

                if is_invalid {
                    debug!("Current access token is missing or expired, requesting refresh");
                }
                is_invalid
            }
            None => {
                debug!("No current session found");
                return Ok(None);
            }
        }
    };

    if needs_refresh {
        match service.refresh_access_token().await {
            Ok(_) => {}
            Err(AppError::Jwt(JwtError::InvalidToken)) => {
                warn!("Refresh token invalid → clearing session");

                service.delete_user_session().await;
                return Ok(None);
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    let session = service.session.read().await.clone();
    Ok(session)
}

#[tauri::command]
#[specta::specta]
pub async fn register(
    app: AppHandle,
    state: State<'_, AppState>,
    payload: shared::models::user_dto::RegisterRequest,
) -> FrontendRepresentation<shared::models::user_dto::RegisterResponse> {
    info!("Registering new user: {}", payload.email);
    let service = &state.0;

    let response: RegisterResponse = service
        .perform_request(
            Method::POST,
            Some(&payload),
            None,
            UserEndpoints::RegisterUserEndpoint,
        )
        .await?;

    service
        .finalize_login(
            &app,
            &response.ulid,
            &response.refresh_token,
            &payload.username,
            &payload.email,
            response.level,
            response.avatar_url.clone(),
        )
        .await?;

    info!("Registration successful for: {}", payload.email);
    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn login(
    app: AppHandle,
    state: State<'_, AppState>,
    payload: shared::models::user_dto::LoginRequest,
) -> FrontendRepresentation<shared::models::user_dto::LoginResponse> {
    info!("Login attempt for user: {}", payload.email);
    let service = &state.0;

    let response: shared::models::user_dto::LoginResponse = service
        .perform_request(
            Method::POST,
            Some(&payload),
            None,
            UserEndpoints::LoginUserEndpoint,
        )
        .await?;

    service
        .finalize_login(
            &app,
            &response.ulid,
            &response.refresh_token,
            &response.username,
            &payload.email,
            response.level,
            response.avatar_url.clone(),
        )
        .await?;

    info!("Login successful for user: {}", payload.email);
    Ok(response)
}

#[tauri::command]
#[specta::specta]
pub async fn logout(app: AppHandle, state: State<'_, AppState>) -> FrontendRepresentation<()> {
    let service = &state.0;
    info!("Logging out current user...");

    let session_data = {
        let session_guard = service.session.read().await;
        session_guard.clone()
    };

    if let Some(session) = session_data {
        debug!("Cleaning up tokens for ULID: {}", session.user_ulid);
        if let Ok(refresh_token) = get_refresh_token_internal(&session.user_ulid) {
            let _ = service
                .perform_request::<(), ()>(
                    Method::DELETE,
                    None,
                    None,
                    RefreshTokenEndpoints::DeleteRefreshToken(refresh_token),
                )
                .await;
        }

        {
            let mut session_guard = service.session.write().await;
            *session_guard = None;
        }

        let _ = delete_refresh_token_internal(&session.user_ulid);

        if let Ok(store) = app.store("user.json") {
            store.delete("user_conf");
            let _ = store.save();
        }

        info!("Logout completed for user: {}", session.email);
    } else {
        warn!("Logout called but no active session was found");
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn delete_refresh_token(account: &str) -> FrontendRepresentation<()> {
    delete_refresh_token_internal(account).map_err(Into::into)
}
