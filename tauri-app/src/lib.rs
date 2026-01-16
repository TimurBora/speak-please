use keyring::Entry;
use log::{debug, error, info, warn};
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use shared::{
    endpoints::{
        refresh_token_endpoints::RefreshTokenEndpoints, user_endpoints::UserEndpoints,
        user_quest_status_endpoints::UserQuestEndpoints, API,
    },
    errors::{jwt_errors::JwtError, AppError, AppResult, FrontendRepresentation},
    models::{
        refresh_token_dto::{CreateRefreshTokenRequest, CreateRefreshTokenResponse},
        user_dto::RegisterResponse,
    },
    utils::jwt::verify_access_token,
};
use specta::Type;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_store::StoreExt;
use tauri_specta::{collect_commands, Builder};
use tokio::sync::{Mutex, RwLock};

const API_URL: &str = env!("BACKEND_URL");
const SERVICE_NAME: &str = "speak-please";

#[derive(Serialize, Deserialize, Clone, Type)]
pub struct UserSession {
    pub access_token: Option<String>,
    pub email: String,
    pub user_ulid: String,
}

pub struct AuthService {
    client: Client,
    session: RwLock<Option<UserSession>>,
    refresh_lock: Mutex<()>,
}

impl AuthService {
    pub fn new(initial_session: Option<UserSession>) -> Self {
        if let Some(ref session) = initial_session {
            info!(
                "AuthService initialized with existing session for: {}",
                session.email
            );
        } else {
            info!("AuthService initialized with no active session");
        }
        Self {
            client: Client::new(),
            session: RwLock::new(initial_session),
            refresh_lock: Mutex::new(()),
        }
    }

    pub async fn perform_request<V, T>(
        &self,
        method: Method,
        body: Option<&V>,
        endpoint: impl API,
    ) -> AppResult<T>
    where
        V: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let url = endpoint.format_with_api_url(API_URL);
        debug!("Performing request: {} {}", method, url);

        let response = self.execute_raw(&url, method.clone(), body).await?;

        if response.status() == StatusCode::UNAUTHORIZED && !endpoint.is_auth_endpoint() {
            warn!(
                "Unauthorized access to {}. Attempting token refresh...",
                url
            );
            self.refresh_access_token().await?;

            debug!("Retrying request: {} {}", method, url);
            let retry_res = self.execute_raw(&url, method, body).await?;
            return self.parse_response(retry_res).await;
        }

        self.parse_response(response).await
    }

    async fn execute_raw<V>(
        &self,
        url: &str,
        method: Method,
        body: Option<&V>,
    ) -> AppResult<reqwest::Response>
    where
        V: Serialize,
    {
        let session_guard = self.session.read().await;
        let mut rb = self.client.request(method, url);

        if let Some(token) = session_guard.as_ref().and_then(|s| s.access_token.as_ref()) {
            rb = rb.bearer_auth(token);
        }

        if let Some(b) = body {
            rb = rb.json(b);
        }

        rb.send().await.map_err(|e| {
            error!("Network request failed: {}", e);
            e.into()
        })
    }

    pub async fn refresh_access_token(&self) -> AppResult<String> {
        let _guard = self.refresh_lock.lock().await;
        {
            let s = self.session.read().await;
            if let Some(token) = s.as_ref().and_then(|s| s.access_token.as_ref()) {
                if verify_access_token(token).is_ok() {
                    return Ok(token.clone());
                }
            }
        }

        let (email, ulid) = {
            let session_lock = self.session.read().await;
            let session_lock = session_lock.as_ref().ok_or_else(|| {
                error!("Refresh failed: No active session found");
                AppError::NotFound
            })?;
            (session_lock.email.clone(), session_lock.user_ulid.clone())
        };

        info!("Refreshing access token for user: {}", email);
        let refresh_token = get_refresh_token_internal(&ulid)?;

        let payload = CreateRefreshTokenRequest {
            email: email.clone(),
            refresh_token,
        };

        let url = RefreshTokenEndpoints::CreateRefreshToken.format_with_api_url(API_URL);
        let response = self.execute_raw(&url, Method::POST, Some(&payload)).await?;

        if !response.status().is_success() {
            let status = response.status();
            error!("Token refresh failed with status: {}", status);

            if status == StatusCode::UNAUTHORIZED {
                return Err(JwtError::InvalidToken.into());
            } else {
                return Err(AppError::from_response(response).await);
            }
        }

        let data: CreateRefreshTokenResponse = response.json().await?;

        save_refresh_token_internal(&ulid, &data.new_refresh_token)?;

        let mut s = self.session.write().await;
        if let Some(ref mut session) = *s {
            session.access_token = Some(data.access_token.clone());
        }

        info!("Successfully refreshed access token for ULID: {}", ulid);
        Ok(data.access_token)
    }

    async fn parse_response<T>(&self, res: reqwest::Response) -> AppResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if res.status().is_success() {
            res.json::<T>().await.map_err(|e| {
                error!("Failed to deserialize response: {}", e);
                AppError::from(e)
            })
        } else {
            let err = AppError::from_response(res).await;
            warn!("API returned error: {:?}", err);
            Err(err)
        }
    }

    pub async fn finalize_login(
        &self,
        app: &AppHandle,
        ulid: &str,
        refresh_token: &str,
        username: &str,
        email: &str,
    ) -> AppResult<()> {
        info!("Finalizing login for user: {} (ULID: {})", email, ulid);

        save_refresh_token_internal(ulid, refresh_token)?;

        let access_token =
            shared::utils::jwt::create_access_token(ulid.to_string(), username.to_string()).ok();

        if access_token.is_none() {
            warn!("Could not create local access token for immediate use");
        }

        let mut session_guard = self.session.write().await;
        *session_guard = Some(UserSession {
            access_token,
            email: email.to_string(),
            user_ulid: ulid.to_string(),
        });

        if let Ok(store) = app.store("user.json") {
            store.set(
                "user_conf",
                serde_json::json!({
                    "email": email,
                    "user_ulid": ulid,
                }),
            );
            if let Err(e) = store.save() {
                error!("Failed to save user configuration to store: {:?}", e);
            } else {
                debug!("User configuration persisted to user.json");
            }
        }

        Ok(())
    }

    pub async fn delete_user_session(&self) {
        info!("Delete user session...");

        let mut session_guard = self.session.write().await;
        *session_guard = None;
    }
}

pub struct AppState(pub AuthService);

#[tauri::command]
#[specta::specta]
fn check_access_token(access_token: String) -> bool {
    let result = shared::utils::jwt::verify_access_token(&access_token).is_ok();
    debug!("Access token verification result: {}", result);
    result
}

#[tauri::command]
#[specta::specta]
async fn get_user_quests(
    state: State<'_, AppState>,
) -> FrontendRepresentation<Vec<shared::models::user_quest_status_dto::UserQuestStatusResponse>> {
    let service = &state.0;

    let ulid = {
        let session_lock = service.session.read().await;
        let session_lock = session_lock.as_ref().ok_or_else(|| {
            error!("Refresh failed: No active session found");
            AppError::NotFound
        })?;
        session_lock.user_ulid.clone()
    };

    let response: Vec<shared::models::user_quest_status_dto::UserQuestStatusResponse> = service
        .perform_request(
            Method::GET,
            None::<&()>,
            UserQuestEndpoints::GetDailyQuests(ulid),
        )
        .await?;

    Ok(response)
}

#[tauri::command]
#[specta::specta]
async fn get_current_session(
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
async fn register(
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
        )
        .await?;

    info!("Registration successful for: {}", payload.email);
    Ok(response)
}

#[tauri::command]
#[specta::specta]
async fn login(
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
        )
        .await?;

    info!("Login successful for user: {}", payload.email);
    Ok(response)
}

#[tauri::command]
#[specta::specta]
async fn logout(app: AppHandle, state: State<'_, AppState>) -> FrontendRepresentation<()> {
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

fn save_refresh_token_internal(account: &str, token: &str) -> AppResult<()> {
    debug!("Saving refresh token to keyring for account: {}", account);
    Entry::new(SERVICE_NAME, account)
        .map_err(|e| {
            error!("Keyring entry creation failed: {}", e);
            AppError::Keyring(e.to_string())
        })?
        .set_password(token)
        .map_err(|e| {
            error!("Failed to write to keyring: {}", e);
            AppError::Keyring(e.to_string())
        })
}

#[tauri::command]
#[specta::specta]
fn delete_refresh_token(account: &str) -> FrontendRepresentation<()> {
    delete_refresh_token_internal(account).map_err(Into::into)
}

fn get_refresh_token_internal(account: &str) -> AppResult<String> {
    debug!(
        "Retrieving refresh token from keyring for account: {}",
        account
    );
    Entry::new(SERVICE_NAME, account)
        .map_err(|e| {
            error!("Keyring entry access failed: {}", e);
            AppError::Keyring(e.to_string())
        })?
        .get_password()
        .map_err(|e| {
            warn!(
                "Refresh token not found in keyring for account: {}",
                account
            );
            AppError::Keyring(e.to_string())
        })
}

fn delete_refresh_token_internal(account: &str) -> AppResult<()> {
    info!(
        "Deleting refresh token from keyring for account: {}",
        account
    );
    Entry::new(SERVICE_NAME, account)
        .map_err(|e| {
            error!("Keyring access failed during deletion: {}", e);
            AppError::Keyring(e.to_string())
        })?
        .delete_credential()
        .map_err(|e| {
            error!("Failed to delete credential from keyring: {}", e);
            AppError::Keyring(e.to_string())
        })
}

pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        register,
        login,
        logout,
        get_current_session,
        check_access_token,
        delete_refresh_token,
        get_user_quests,
    ]);

    #[cfg(all(debug_assertions, not(mobile)))]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("Failed to export bindings");

    info!("Starting Speak Please application...");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Warn)
                .level_for("speak_please_lib", log::LevelFilter::Info)
                .level_for("tauri", log::LevelFilter::Info)
                .level_for("zbus", log::LevelFilter::Off)
                .level_for("reqwest", log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "android")]
            {
                debug!("Configuring Android keyring...");
                let _ = android_keyring::set_android_keyring_credential_builder();
            }

            let initial_session = app
                .store("user.json")
                .ok()
                .and_then(|s| s.get("user_conf"))
                .map(|val| {
                    debug!("Loaded user configuration from store for: {}", val["email"]);
                    UserSession {
                        access_token: None,
                        email: val["email"].as_str().unwrap_or_default().to_string(),
                        user_ulid: val["user_ulid"].as_str().unwrap_or_default().to_string(),
                    }
                });

            app.manage(AppState(AuthService::new(initial_session)));
            Ok(())
        })
        .invoke_handler(specta_builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
