#[cfg(test)]
mod tests {
    use crate::{entities::users, service::user_service::UserService};
    use chrono::Utc;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use shared::{
        errors::{AppError, auth_errors::AuthError},
        utils::hashing::hash,
    };

    fn mock_user_model(id: &str, username: &str, email: &str) -> users::Model {
        users::Model {
            ulid: id.to_owned(),
            username: username.to_owned(),
            email: email.to_owned(),
            password_hash: hash("password123").unwrap(),
            xp_balance: 0,
            total_xp_accumulated: 0,
            level: 1,
            created_at: Utc::now(),
            last_active_at: Utc::now(),
            avatar_url: None,
            bio: None,
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_user_model("U1", "alice", "alice@test.com")]])
            .into_connection();

        let result = UserService::create_user(
            &db,
            "alice".into(),
            "alice@test.com".into(),
            "secret_pass".into(),
        )
        .await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "alice");
        assert_eq!(user.level, 1);
    }

    #[tokio::test]
    async fn test_find_by_methods() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                vec![mock_user_model("U1", "alice", "alice@test.com")], // find_by_id
                vec![mock_user_model("U1", "alice", "alice@test.com")], // find_by_username
                vec![], // find_by_email (not found case)
            ])
            .into_connection();

        assert!(UserService::find_by_id(&db, "U1").await.unwrap().is_some());
        assert!(
            UserService::find_by_username(&db, "alice")
                .await
                .unwrap()
                .is_some()
        );
        assert!(
            UserService::find_by_email(&db, "unknown@test.com")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_login_success() {
        let email = "bob@test.com";
        let password = "password123";

        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_user_model("U2", "bob", email)]])
            .into_connection();

        let result = UserService::login_by_email_and_password(&db, email, password).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().username, "bob");
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_user_model("U2", "bob", "bob@test.com")]])
            .into_connection();

        let result = UserService::login_by_email_and_password(&db, "bob@test.com", "WRONG").await;

        assert!(matches!(
            result,
            Err(AppError::Auth(AuthError::InvalidCredentials))
        ));
    }

    #[tokio::test]
    async fn test_update_profile() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                vec![mock_user_model("U1", "old_name", "test@test.com")],
                vec![mock_user_model("U1", "new_name", "test@test.com")],
            ])
            .into_connection();

        let updated =
            UserService::update_profile(&db, "U1", Some("new_name".into()), Some("New Bio".into()))
                .await
                .unwrap();

        assert_eq!(updated.username, "new_name");
    }

    #[tokio::test]
    async fn test_confirm_avatar_update() {
        let user_id = "U1";
        let expected_url = format!("users/{}/avatar.jpg", user_id);

        let mut updated_user = mock_user_model(user_id, "user", "e@e.com");
        updated_user.avatar_url = Some(expected_url.clone());

        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                vec![mock_user_model(user_id, "user", "e@e.com")],
                vec![updated_user],
            ])
            .into_connection();

        let user = UserService::confirm_avatar_update(&db, user_id)
            .await
            .unwrap();

        assert_eq!(user.avatar_url, Some(expected_url));
    }
}
