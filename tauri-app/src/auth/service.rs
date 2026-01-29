use crate::auth::keyring::{get_refresh_token_internal, save_refresh_token_internal};
use crate::auth::session::UserSession;
use log::{debug, error, info, warn};
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use shared::endpoints::{refresh_token_endpoints::RefreshTokenEndpoints, API};
use shared::errors::{jwt_errors::JwtError, AppError, AppResult};
use shared::models::refresh_token_dto::{CreateRefreshTokenRequest, CreateRefreshTokenResponse};
use shared::models::Pagination;
use shared::utils::jwt::verify_access_token;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use tokio::sync::{Mutex, RwLock};

const API_URL: &str = env!("BACKEND_URL");

// TODO: DONT FORGET ABOUT SRR MATE, ITS IMPORTANT
pub struct AuthService {
    pub client: Client,
    pub session: RwLock<Option<UserSession>>,
    pub refresh_lock: Mutex<()>,
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

    pub async fn get_current_user_ulid(&self) -> AppResult<String> {
        let session_lock = self.session.read().await;
        let session = session_lock.as_ref().ok_or_else(|| {
            error!("Action failed: No active session found");
            AppError::NotFound
        })?;
        Ok(session.user_ulid.clone())
    }

    pub async fn perform_request<V, T>(
        &self,
        method: Method,
        body: Option<&V>,
        pagination: Option<&Pagination>,
        endpoint: impl API,
    ) -> AppResult<T>
    where
        V: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let url = endpoint.format_with_api_url(API_URL);
        debug!("Performing request: {} {}", method, url);

        let response = self
            .execute_raw(&url, method.clone(), body, pagination)
            .await?;

        if response.status() == StatusCode::UNAUTHORIZED && !endpoint.is_auth_endpoint() {
            warn!(
                "Unauthorized access to {}. Attempting token refresh...",
                url
            );
            self.refresh_access_token().await?;

            debug!("Retrying request: {} {}", method, url);
            let retry_res = self.execute_raw(&url, method, body, pagination).await?;
            return self.parse_response(retry_res).await;
        }

        self.parse_response(response).await
    }

    async fn execute_raw<V>(
        &self,
        url: &str,
        method: Method,
        body: Option<&V>,
        pagination: Option<&Pagination>,
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

        if let Some(q) = pagination {
            rb = rb.query(q);
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
        let response = self
            .execute_raw(&url, Method::POST, Some(&payload), None)
            .await?;

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
            if std::mem::size_of::<T>() == 0 {
                return serde_json::from_str("null").map_err(|e| AppError::Custom(e.to_string()));
            }
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

    #[expect(clippy::too_many_arguments)]
    pub async fn finalize_login(
        &self,
        app: &AppHandle,
        ulid: &str,
        refresh_token: &str,
        username: &str,
        email: &str,
        level: u32,
        avatar_url: Option<String>,
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
            username: username.to_string(),
            level,
            avatar_url: avatar_url.clone(),
        });

        if let Ok(store) = app.store("user.json") {
            store.set(
                "user_conf",
                serde_json::json!({
                    "email": email,
                    "user_ulid": ulid,
                    "username": username,
                    "level": level,
                    "avatar_url": avatar_url,
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
