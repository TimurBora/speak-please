use sea_orm::entity::prelude::*;
use shared::{
    entities::{daily_quests, prelude::*},
    errors::{AppError, AppResult},
};

pub struct DailyQuestService;

impl DailyQuestService {
    #[allow(clippy::too_many_arguments)]
    pub async fn create_quest(
        db: &DatabaseConnection,
        title: String,
        description: Option<String>,
        xp_reward: u32,
        action_type: String,
        validation_type: String,
        target_value: u32,
        complexity: daily_quests::Complexity,
    ) -> AppResult<daily_quests::Model> {
        let new_quest = daily_quests::ActiveModel::new_daily_quest(
            &title,
            description,
            Some(xp_reward),
            &action_type,
            &validation_type,
            Some(target_value),
            Some(complexity),
        );

        let model = new_quest.insert(db).await.map_err(AppError::from)?;
        Ok(model)
    }

    pub async fn get_all_quests(db: &DatabaseConnection) -> AppResult<Vec<daily_quests::Model>> {
        DailyQuest::find().all(db).await.map_err(AppError::from)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        quest_id: &str,
    ) -> AppResult<Option<daily_quests::Model>> {
        DailyQuest::find_by_id(quest_id)
            .one(db)
            .await
            .map_err(AppError::from)
    }
}
