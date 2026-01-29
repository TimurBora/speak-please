#[cfg(test)]
mod tests {

    use crate::entities::{lobbies, lobbies_members};
    use crate::service::lobby_member_service::LobbyMemberService;
    use chrono::Utc;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use shared::errors::AppError;
    use shared::models::lobby_dto::Role;

    // Помогатор для создания дефолтной модели лобби
    fn mock_lobby_model(id: &str) -> lobbies::Model {
        lobbies::Model {
            ulid: id.to_owned(),
            name: "Test Lobby".into(),
            description: None,
            topic: "Rust".into(),
            owner_id: "OWNER".into(),
            created_at: Utc::now(),
        }
    }

    // Помогатор для создания дефолтной модели участника
    fn mock_member_model(lobby_id: &str, user_id: &str, role: Role) -> lobbies_members::Model {
        lobbies_members::Model {
            lobby_id: lobby_id.to_owned(),
            user_id: user_id.to_owned(),
            joined_at: Utc::now(),
            role,
        }
    }

    // --- ТЕСТЫ join_lobby ---

    #[tokio::test]
    async fn test_join_lobby_success() {
        let lobby_id = "L_1";
        let user_id = "U_1";

        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_lobby_model(lobby_id)]])
            .append_query_results(vec![Vec::<lobbies::Model>::new()])
            .append_query_results(vec![vec![mock_member_model(
                lobby_id,
                user_id,
                Role::Member,
            )]])
            .into_connection();

        let result = LobbyMemberService::join_lobby(&db, lobby_id.into(), user_id.into()).await;

        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.user_id, user_id);
        assert_eq!(model.role, Role::Member);
    }

    #[tokio::test]
    async fn test_join_lobby_not_found() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                // 1. Лобби не найдено
                Vec::<lobbies::Model>::new(),
            ])
            .into_connection();

        let result = LobbyMemberService::join_lobby(&db, "FAKE".into(), "USER".into()).await;

        match result {
            Err(AppError::NotFound) => (),
            _ => panic!("Should have returned NotFound error"),
        }
    }

    #[tokio::test]
    async fn test_join_lobby_already_member() {
        let lobby_id = "L_1";
        let user_id = "U_1";

        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                // 1. Лобби существует
                vec![mock_lobby_model(lobby_id)],
                // 2. Юзер уже есть (возвращаем модель)
            ])
            .append_query_results(vec![vec![mock_member_model(
                lobby_id,
                user_id,
                Role::Member,
            )]])
            .into_connection();

        let result = LobbyMemberService::join_lobby(&db, lobby_id.into(), user_id.into()).await;

        match result {
            Err(AppError::Custom(msg)) => assert_eq!(msg, "Already a member"),
            _ => panic!("Should have returned 'Already a member' error"),
        }
    }

    // --- ТЕСТЫ is_member ---

    #[tokio::test]
    async fn test_is_member_logic() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                vec![mock_member_model("L1", "U1", Role::Member)], // Первый вызов: True
                vec![],                                            // Второй вызов: False
            ])
            .into_connection();

        assert!(LobbyMemberService::is_member(&db, "L1", "U1").await);
        assert!(!LobbyMemberService::is_member(&db, "L1", "U2").await);
    }

    // --- ТЕСТЫ can_manage_lobby ---

    #[tokio::test]
    async fn test_can_manage_lobby_roles() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                vec![mock_member_model("L1", "ADMIN", Role::Admin)],
                vec![mock_member_model("L1", "MOD", Role::Moderator)],
                vec![mock_member_model("L1", "USER", Role::Member)],
                vec![], // Юзера нет
            ])
            .into_connection();

        // Админ может
        assert!(LobbyMemberService::can_manage_lobby(&db, "L1", "ADMIN").await);
        // Модератор может
        assert!(LobbyMemberService::can_manage_lobby(&db, "L1", "MOD").await);
        // Обычный участник не может
        assert!(!LobbyMemberService::can_manage_lobby(&db, "L1", "USER").await);
        // Левый человек не может
        assert!(!LobbyMemberService::can_manage_lobby(&db, "L1", "STRANGER").await);
    }

    #[tokio::test]
    async fn test_get_lobby_members_count() {
        let mut row = std::collections::BTreeMap::new();

        row.insert("num_items".to_owned(), sea_orm::Value::Int(Some(10i32)));

        let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![row]])
            .into_connection();

        let count = LobbyMemberService::get_lobby_members_count(&db, "L1")
            .await
            .expect("Теперь декодирование должно пройти успешно");

        assert_eq!(count, 10);
    }
}
