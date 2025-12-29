use sea_orm::sqlx::types::chrono;

pub fn is_expired(expires_at: chrono::DateTime<chrono::Utc>) -> bool {
    chrono::Utc::now() > expires_at
}
