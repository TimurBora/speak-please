use sea_orm::sqlx::types::chrono;

pub fn is_expired(expires_at: chrono::DateTime<chrono::Utc>) -> bool {
    chrono::Utc::now() >= expires_at
}

#[cfg(test)]
mod tests {
    use chrono::Days;

    use crate::utils::time::is_expired;

    #[test]
    fn test_is_expired() {
        let not_expired_time = chrono::Utc::now().checked_add_days(Days::new(1)).unwrap();
        let expired_time = chrono::Utc::now().checked_sub_days(Days::new(1)).unwrap();

        assert!(is_expired(expired_time));
        assert!(!is_expired(not_expired_time));
    }
}
