use keyring::Entry;
use log::{debug, error, info, warn};
use shared::errors::{AppError, AppResult};

const SERVICE_NAME: &str = "speak-please";

pub fn save_token(account: &str, token: &str) -> AppResult<()> {
    debug!("Saving refresh token to keyring for: {}", account);
    Entry::new(SERVICE_NAME, account)
        .map_err(|e| AppError::Keyring(e.to_string()))?
        .set_password(token)
        .map_err(|e| AppError::Keyring(e.to_string()))
}

pub fn get_token(account: &str) -> AppResult<String> {
    Entry::new(SERVICE_NAME, account)
        .map_err(|e| AppError::Keyring(e.to_string()))?
        .get_password()
        .map_err(|e| AppError::Keyring(e.to_string()))
}

pub fn delete_token(account: &str) -> AppResult<()> {
    debug!("Deleting refresh token to keyring for: {}", account);
    Entry::new(SERVICE_NAME, account)
        .map_err(|e| AppError::Keyring(e.to_string()))?
        .delete_credential()
        .map_err(|e| AppError::Keyring(e.to_string()))
}

pub fn save_refresh_token_internal(account: &str, token: &str) -> AppResult<()> {
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

pub fn get_refresh_token_internal(account: &str) -> AppResult<String> {
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

pub fn delete_refresh_token_internal(account: &str) -> AppResult<()> {
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
