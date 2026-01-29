#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectOptions, Database};
    use serde_json::json;
    use shared::endpoints::API;
    use shared::endpoints::user_endpoints::UserEndpoints;
    use shared::models::user_dto::{LoginResponse, RegisterResponse};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    use crate::AppState;
    use crate::routes::user_routes::public_user_router;

    async fn setup_test_server() -> TestServer {
        let mut opt = ConnectOptions::new("sqlite::memory:");
        opt.sqlx_logging(false);
        let connection = Database::connect(opt)
            .await
            .expect("Failed to connect to test DB");

        Migrator::up(&connection, None)
            .await
            .expect("Failed to run migrations");

        let state = AppState {
            connection,
            s3_manager: crate::file_storage::s3_client::S3Manager::new(
                "test-bucket".into(),
                "mock-endpoint".to_string(),
                "mock-region".to_string(),
            )
            .await,
            lobby_channels: Arc::new(Mutex::new(HashMap::new())),
        };

        let app = public_user_router().with_state(state);

        TestServer::new(app).expect("Failed to create test server")
    }

    #[tokio::test]
    async fn test_full_auth_cycle() {
        let server = setup_test_server().await;

        let register_payload = json!({
            "username": "rust_ace",
            "email": "ace@example.com",
            "password": "super-secret-password"
        });

        let reg_response = server
            .post(&UserEndpoints::RegisterUserEndpoint.path())
            .json(&register_payload)
            .await;

        reg_response.assert_status_success();

        let reg_data: RegisterResponse = reg_response.json();
        assert_eq!(reg_data.username, "rust_ace");
        assert!(!reg_data.refresh_token.is_empty());

        let login_payload = json!({
            "email": "ace@example.com",
            "password": "super-secret-password"
        });

        let login_response = server
            .post(&UserEndpoints::LoginUserEndpoint.path())
            .json(&login_payload)
            .await;

        login_response.assert_status_success();

        let login_data: LoginResponse = login_response.json();
        assert_eq!(login_data.ulid, reg_data.ulid);
        assert_eq!(login_data.email, "ace@example.com");
    }

    #[tokio::test]
    async fn test_registration_validation() {
        let server = setup_test_server().await;

        let bad_payload = json!({
            "username": "user",
            "email": "not-an-email",
            "password": "123"
        });

        let response = server
            .post(&UserEndpoints::RegisterUserEndpoint.path())
            .json(&bad_payload)
            .await;

        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_login_wrong_credentials() {
        let server = setup_test_server().await;

        let login_payload = json!({
            "email": "ghost@example.com",
            "password": "some-password"
        });

        let response = server
            .post(&UserEndpoints::LoginUserEndpoint.path())
            .json(&login_payload)
            .await;

        response.assert_status_unauthorized();
    }

    #[tokio::test]
    async fn test_duplicate_email_registration() {
        let server = setup_test_server().await;

        let payload = json!({
            "username": "user1",
            "email": "same@example.com",
            "password": "password123"
        });

        server
            .post(&UserEndpoints::RegisterUserEndpoint.path())
            .json(&payload)
            .await
            .assert_status_success();

        let response = server
            .post(&UserEndpoints::RegisterUserEndpoint.path())
            .json(&payload)
            .await;

        response.assert_status_conflict();
    }

    #[tokio::test]
    async fn test_username_length_boundaries() {
        let server = setup_test_server().await;

        server
            .post(&UserEndpoints::RegisterUserEndpoint.path())
            .json(&json!({
                "username": "shrt",
                "email": "short@test.com",
                "password": "password123"
            }))
            .await
            .assert_status_bad_request();

        server
            .post(&UserEndpoints::RegisterUserEndpoint.path())
            .json(&json!({
                "username": "this_is_way_too_long_for_a_username",
                "email": "long@test.com",
                "password": "password123"
            }))
            .await
            .assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_login_non_existent_user() {
        let server = setup_test_server().await;

        let response = server
            .post(&UserEndpoints::LoginUserEndpoint.path())
            .json(&json!({
                "email": "nobody@example.com",
                "password": "some-password"
            }))
            .await;

        response.assert_status_unauthorized();
    }
}
