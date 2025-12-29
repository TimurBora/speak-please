use keyring::Entry;
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use shared::{
    endpoints::{
        refresh_token_endpoints::RefreshTokenEndpoints, user_endpoints::UserEndpoints, API,
    },
    errors::{jwt_errors::JwtError, AppError, AppResult, FrontendRepresentation},
    models::{
        refresh_token_dto::{CreateRefreshTokenRequest, CreateRefreshTokenResponse},
        user_dto::RegisterResponse,
    },
};
use specta::Type;
use std::sync::Arc;
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
        endoint: impl API,
    ) -> AppResult<T>
    where
        V: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let url = endoint.format_with_api_url(API_URL);

        let response = self.execute_raw(&url, method.clone(), body).await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            self.refresh_access_token().await?;
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

        rb.send().await.map_err(Into::into)
    }

    pub async fn refresh_access_token(&self) -> AppResult<String> {
        let _lock = self.refresh_lock.lock().await;

        let (email, ulid) = {
            let session_lock = self.session.read().await;
            let session_lock = session_lock.as_ref().ok_or(AppError::NotFound)?;
            (session_lock.email.clone(), session_lock.user_ulid.clone())
        };

        let refresh_token = get_refresh_token_internal(&ulid)?;

        let payload = CreateRefreshTokenRequest {
            email: email.clone(),
            refresh_token,
        };

        let url = RefreshTokenEndpoints::CreateRefreshToken.format_with_api_url(API_URL);
        let response = self.execute_raw(&url, Method::POST, Some(&payload)).await?;

        if !response.status().is_success() {
            return Err(JwtError::InvalidToken.into());
        }

        let data: CreateRefreshTokenResponse = response.json().await?;

        save_refresh_token_internal(ulid.clone(), data.new_refresh_token)?;

        let mut s = self.session.write().await;
        if let Some(ref mut session) = *s {
            session.access_token = Some(data.access_token.clone());
        }

        Ok(data.access_token)
    }

    async fn parse_response<T>(&self, res: reqwest::Response) -> AppResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if res.status().is_success() {
            res.json::<T>().await.map_err(AppError::from)
        } else {
            Err(AppError::Api(format!("Server error: {}", res.status())))
        }
    }
}

pub struct AppState(pub Arc<AuthService>);

#[tauri::command]
#[specta::specta]
fn check_access_token(access_token: String) -> bool {
    shared::utils::jwt::verify_access_token(&access_token).is_ok()
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
            Some(session) => session
                .access_token
                .as_ref()
                .map(|t| shared::utils::jwt::verify_access_token(t).is_err())
                .unwrap_or(true),
            None => return Ok(None),
        }
    };

    if needs_refresh {
        service.refresh_access_token().await?;
    }

    Ok(service.session.read().await.clone())
}

#[tauri::command]
#[specta::specta]
async fn register(
    app: AppHandle,
    state: State<'_, AppState>,
    payload: shared::models::user_dto::RegisterRequest,
) -> FrontendRepresentation<shared::models::user_dto::RegisterResponse> {
    let service = &state.0;

    let response: RegisterResponse = service
        .perform_request(
            Method::POST,
            Some(&payload),
            UserEndpoints::RegisterUserEndpoint,
        )
        .await?;

    save_refresh_token_internal(response.ulid.clone(), response.refresh_token.clone())?;

    let access_token =
        shared::utils::jwt::create_access_token(response.ulid.clone(), payload.username.clone())
            .ok();

    let mut session_guard = service.session.write().await;
    *session_guard = Some(UserSession {
        access_token,
        email: payload.email.clone(),
        user_ulid: response.ulid.clone(),
    });

    if let Ok(store) = app.store("user.json") {
        store.set(
            "user_conf",
            serde_json::json!({
                "email": payload.email,
                "user_ulid": response.ulid
            }),
        );
        let _ = store.save();
    }

    Ok(response)
}

#[tauri::command]
#[specta::specta]
async fn login(
    app: AppHandle,
    state: State<'_, AppState>,
    payload: shared::models::user_dto::LoginRequest,
) -> FrontendRepresentation<shared::models::user_dto::LoginResponse> {
    let service = &state.0;

    let response: shared::models::user_dto::LoginResponse = service
        .perform_request(
            Method::POST,
            Some(&payload),
            UserEndpoints::LoginUserEndpoint,
        )
        .await?;

    save_refresh_token_internal(response.ulid.clone(), response.refresh_token.clone())?;

    let access_token =
        shared::utils::jwt::create_access_token(response.ulid.clone(), payload.email.clone()).ok();

    let mut session_guard = service.session.write().await;
    *session_guard = Some(UserSession {
        access_token,
        email: payload.email.clone(),
        user_ulid: response.ulid.clone(),
    });

    if let Ok(store) = app.store("user.json") {
        store.set(
            "user_conf",
            serde_json::json!({
                "email": payload.email,
                "user_ulid": response.ulid
            }),
        );
        let _ = store.save();
    }

    Ok(response)
}

#[tauri::command]
#[specta::specta]
async fn logout(app: AppHandle, state: State<'_, AppState>) -> FrontendRepresentation<()> {
    let service = &state.0;
    let mut session_guard = service.session.write().await;

    if let Some(session) = session_guard.take() {
        let refresh_token = get_refresh_token_internal(&session.user_ulid)?;

        let _: shared::models::user_dto::RegisterResponse = service
            .perform_request::<(), _>(
                Method::DELETE,
                None,
                RefreshTokenEndpoints::DeleteRefreshToken(refresh_token),
            )
            .await?;

        delete_refresh_token_internal(session.user_ulid)?;

        if let Ok(store) = app.store("user.json") {
            store.delete("user_conf");
            let _ = store.save();
        }
    }

    Ok(())
}

fn save_refresh_token_internal(account: String, token: String) -> AppResult<()> {
    Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AppError::Keyring(e.to_string()))?
        .set_password(&token)
        .map_err(|e| AppError::Keyring(e.to_string()))
}

#[tauri::command]
#[specta::specta]
fn delete_refresh_token(account: String) -> FrontendRepresentation<()> {
    delete_refresh_token_internal(account).map_err(Into::into)
}

fn get_refresh_token_internal(account: &str) -> AppResult<String> {
    Entry::new(SERVICE_NAME, account)
        .map_err(|e| AppError::Keyring(e.to_string()))?
        .get_password()
        .map_err(|e| AppError::Keyring(e.to_string()))
}

fn delete_refresh_token_internal(account: String) -> AppResult<()> {
    Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AppError::Keyring(e.to_string()))?
        .delete_credential()
        .map_err(|e| AppError::Keyring(e.to_string()))
}

pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        register,
        login,
        logout,
        get_current_session,
        check_access_token,
        delete_refresh_token,
    ]);

    #[cfg(all(debug_assertions, not(mobile)))]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("Failed to export bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "android")]
            let _ = android_keyring::set_android_keyring_credential_builder();

            let initial_session = app
                .store("user.json")
                .ok()
                .and_then(|s| s.get("user_conf"))
                .map(|val| UserSession {
                    access_token: None,
                    email: val["email"].as_str().unwrap_or_default().to_string(),
                    user_ulid: val["user_ulid"].as_str().unwrap_or_default().to_string(),
                });

            app.manage(AppState(Arc::new(AuthService::new(initial_session))));
            Ok(())
        })
        .invoke_handler(specta_builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
