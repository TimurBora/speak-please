#[cfg(test)]
mod tests {
    use crate::{entities::quests, service::quest_service::QuestService};
    use sea_orm::{DatabaseBackend, MockDatabase};
    use shared::models::quest_dto::Complexity;

    fn mock_quest_model(id: &str, title: &str) -> quests::Model {
        quests::Model {
            ulid: id.to_owned(),
            lobby_id: None,
            title: title.to_owned(),
            description: Some("Test Description".into()),
            complexity: Complexity::Easy,
            xp_reward: 100,
            validation_type: "AUTOMATIC".into(),
            target_value: 1,
        }
    }

    #[tokio::test]
    async fn test_create_quest_success() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                // Возвращаем модель, которую "вставили"
                vec![mock_quest_model("NEW_ULID", "Save the Kingdom")],
            ])
            .into_connection();

        let result = QuestService::create_quest(
            &db,
            "Save the Kingdom".into(),
            Some("Description".into()),
            100,
            "AUTOMATIC".into(),
            1,
            Complexity::Easy,
            None,
        )
        .await;

        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.title, "Save the Kingdom");
        assert_eq!(model.xp_reward, 100);
    }

    #[tokio::test]
    async fn test_get_all_quests() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![
                mock_quest_model("Q1", "Quest 1"),
                mock_quest_model("Q2", "Quest 2"),
            ]])
            .into_connection();

        let quests = QuestService::get_all_quests(&db).await.unwrap();

        assert_eq!(quests.len(), 2);
        assert_eq!(quests[0].title, "Quest 1");
    }

    #[tokio::test]
    async fn test_find_by_id_found() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![vec![mock_quest_model("Q1", "Title")]])
            .into_connection();

        let result = QuestService::find_by_id(&db, "Q1").await.unwrap();

        assert!(result.is_some());
        assert_eq!(result.unwrap().ulid, "Q1");
    }

    #[tokio::test]
    async fn test_find_by_id_none() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![Vec::<quests::Model>::new()])
            .into_connection();

        let result = QuestService::find_by_id(&db, "NON_EXISTENT").await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_seed_quests_logic() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results(vec![
                vec![mock_quest_model("EXISTING", "Daily Login")],
                vec![],
                vec![mock_quest_model("NEW", "World Explorer")],
            ])
            .into_connection();

        let data = include_str!("../../test_quests.json");
        let result = quests::seed_quests_internal(&db, data).await;

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_daily_quest_defaults() {
        use sea_orm::ActiveValue;

        let active_model = quests::ActiveModel::new_daily_quest(
            "New Quest",
            None,
            None, // xp_reward
            "COMMUNITY",
            None, // target_value
            None, // complexity
            None,
        );

        assert_eq!(active_model.xp_reward.unwrap(), 10);
        assert_eq!(active_model.target_value.unwrap(), 1);

        if let ActiveValue::Set(comp) = active_model.complexity {
            assert_eq!(comp, Complexity::Easy);
        } else {
            panic!("Complexity should be Set");
        }
    }
}
