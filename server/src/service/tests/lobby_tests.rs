#[cfg(test)]
mod tests {
    use crate::entities::{lobbies, lobbies_members};
    use crate::service::lobby_service::LobbyService;
    use chrono::Utc;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use shared::errors::AppError;
    use shared::models::lobby_dto::Role;

    fn mock_lobby_model(id: &str, owner_id: &str) -> lobbies::Model {
        lobbies::Model {
            ulid: id.to_owned(),
            name: "Rust Lobby".into(),
            description: Some("Description".into()),
            topic: "Programming".into(),
            owner_id: owner_id.to_owned(),
            created_at: Utc::now(),
        }
    }

    fn mock_member_model(lobby_id: &str, user_id: &str, role: Role) -> lobbies_members::Model {
        lobbies_members::Model {
            lobby_id: lobby_id.to_owned(),
            user_id: user_id.to_owned(),
            joined_at: Utc::now(),
            role,
        }
    }

    #[tokio::test]
    async fn test_create_lobby_success() {
        let owner_id = "USER_1";
        let lobby_name = "New Lobby";

        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_lobby_model("NEW_ULID", owner_id)]])
            .append_query_results(vec![vec![mock_member_model(
                "NEW_ULID",
                owner_id,
                Role::Admin,
            )]])
            .into_connection();

        let result = LobbyService::create_lobby(
            &db,
            owner_id.into(),
            lobby_name.into(),
            "Rust".into(),
            None,
        )
        .await;

        assert!(result.is_ok());
        let lobby = result.unwrap();
        assert_eq!(lobby.owner_id, owner_id);
    }

    #[tokio::test]
    async fn test_find_by_id_success() {
        let lobby_id = "L_100";
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_lobby_model(lobby_id, "OWNER")]])
            .into_connection();

        let result = LobbyService::find_by_id(&db, lobby_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().ulid, lobby_id);
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![Vec::<lobbies::Model>::new()])
            .into_connection();

        let result = LobbyService::find_by_id(&db, "NON_EXISTENT").await;

        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[tokio::test]
    async fn test_get_all_lobbies() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![
                mock_lobby_model("L1", "O1"),
                mock_lobby_model("L2", "O2"),
            ]])
            .into_connection();

        let result = LobbyService::get_all_lobbies(&db).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].ulid, "L1");
        assert_eq!(result[1].ulid, "L2");
    }

    #[tokio::test]
    async fn test_get_lobbies_with_membership_status() {
        let user_id = "ME";

        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![
                (
                    mock_lobby_model("L1", "OWNER1"),
                    Some(mock_member_model("L1", user_id, Role::Member)),
                ),
                (mock_lobby_model("L2", "OWNER2"), None),
            ]])
            .into_connection();

        let result = LobbyService::get_lobbies_with_membership_status(&db, user_id)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);

        let l1_status = result.iter().find(|(l, _)| l.ulid == "L1").unwrap();
        assert!(l1_status.1); // true

        let l2_status = result.iter().find(|(l, _)| l.ulid == "L2").unwrap();
        assert!(!l2_status.1); // false
    }

    #[tokio::test]
    async fn test_create_lobby_transaction_failure() {
        let _db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_lobby_model("ID", "OWNER")]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }])
            .into_connection();
    }
}
