pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;
    use sea_orm_migration::prelude::*;

    use crate::m20220101_000001_create_table::Migration;

    #[tokio::test]
    async fn test_migrations_run_successfully() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        let schema_manager = SchemaManager::new(&db);

        let migration = Migration;
        let up_result = migration.up(&schema_manager).await;
        assert!(
            up_result.is_ok(),
            "Migration UP failed: {:?}",
            up_result.err()
        );

        let tables = vec![
            "users",
            "quests",
            "lobbies",
            "user_quest_status",
            "refresh_tokens",
            "quest_proofs",
            "quest_proof_beliefs",
            "lobby_members",
        ];

        for table in tables {
            assert!(
                schema_manager.has_table(table).await.unwrap(),
                "Table '{}' was not created",
                table
            );
        }

        let down_result = migration.down(&schema_manager).await;
        assert!(
            down_result.is_ok(),
            "Migration DOWN failed: {:?}",
            down_result.err()
        );

        for table in ["users", "lobbies", "quests"] {
            assert!(
                !schema_manager.has_table(table).await.unwrap(),
                "Table '{}' still exists after down",
                table
            );
        }
    }
}
