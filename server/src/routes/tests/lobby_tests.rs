#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectOptions, Database};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use ulid::Ulid;

    use crate::AppState;
    use crate::routes::lobby_routes::lobby_router;
    use shared::{
        endpoints::{API, LobbyUlid, UserUlid, lobby_endpoints::LobbyEndpoints},
        models::lobby_dto::{LobbyDetailsResponse, LobbyDto, LobbyFeedResponse, LobbyMemberDto},
    };

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

        let app = lobby_router().with_state(state);
        TestServer::new(app).expect("Failed to create test server")
    }

    #[tokio::test]
    async fn test_create_and_get_lobby_details() {
        let server = setup_test_server().await;
        let owner_id = Ulid::new().to_string();

        let create_payload = json!({
            "name": "Rust Enthusiasts",
            "topic": "Programming",
            "description": "A place for Rustaceans",
            "owner_id": owner_id
        });

        let response = server.post("/lobbies").json(&create_payload).await;
        response.assert_status_success();

        let created_lobby: LobbyDto = response.json();
        assert_eq!(created_lobby.name, "Rust Enthusiasts");
        assert_eq!(created_lobby.owner_id, owner_id);

        let details_url = format!("/lobbies/{}", created_lobby.ulid);
        let details_response = server.get(&details_url).await;
        details_response.assert_status_success();

        let details: LobbyDetailsResponse = details_response.json();
        assert_eq!(details.lobby.ulid, created_lobby.ulid);
        assert_eq!(details.members_count, 1);
    }

    #[tokio::test]
    async fn test_join_lobby_flow() {
        let server = setup_test_server().await;
        let owner_id = Ulid::new().to_string();
        let user_id = Ulid::new().to_string();

        let create_res = server
            .post("/lobbies")
            .json(&json!({
                "name": "Join Test",
                "topic": "Tests",
                "owner_id": owner_id
            }))
            .await;
        let lobby: LobbyDto = create_res.json();

        let join_url =
            LobbyEndpoints::Join(LobbyUlid(lobby.ulid.clone()), UserUlid(user_id.clone()));
        let join_res = server.post(&join_url.path()).await;
        join_res.assert_status_success();

        let membership: LobbyMemberDto = join_res.json();
        assert_eq!(membership.user_id, user_id);

        let count_url = LobbyEndpoints::GetMembersCount(LobbyUlid(lobby.ulid));
        let count_res = server.get(&count_url.path()).await;
        let count: u32 = count_res.json();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_lobby_feed_membership_status() {
        let server = setup_test_server().await;
        let user_a = Ulid::new().to_string();
        let user_b = Ulid::new().to_string();

        server
            .post("/lobbies")
            .json(&json!({
                "name": "Lobby A",
                "topic": "Topic A",
                "owner_id": user_a
            }))
            .await;

        server
            .post("/lobbies")
            .json(&json!({
                "name": "Lobby B",
                "topic": "Topic B",
                "owner_id": user_b
            }))
            .await;

        let feed_url = format!("/lobbies/feed/{}", user_a);
        let response = server.get(&feed_url).await;
        response.assert_status_success();

        let feed: LobbyFeedResponse = response.json();
        assert_eq!(feed.items.len(), 2);

        let lobby_a_item = feed
            .items
            .iter()
            .find(|i| i.lobby.name == "Lobby A")
            .unwrap();
        let lobby_b_item = feed
            .items
            .iter()
            .find(|i| i.lobby.name == "Lobby B")
            .unwrap();

        assert!(lobby_a_item.is_member);
        assert!(!lobby_b_item.is_member);
    }

    #[tokio::test]
    async fn test_create_lobby_validation() {
        let server = setup_test_server().await;

        let bad_payload = json!({
            "name": "lo",
            "topic": "valid_topic",
            "owner_id": Ulid::new().to_string()
        });

        let response = server.post("/lobbies").json(&bad_payload).await;
        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_join_non_existent_lobby() {
        let server = setup_test_server().await;
        let fake_lobby = Ulid::new().to_string();
        let user = Ulid::new().to_string();

        let response = server
            .post(&format!("/lobbies/{}/join/{}", fake_lobby, user))
            .await;
        response.assert_status_not_found();
    }
}
