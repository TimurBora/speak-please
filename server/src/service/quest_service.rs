use sea_orm::entity::prelude::*;
use shared::{
    errors::{AppError, AppResult},
    models::quest_dto::Complexity,
};

use crate::entities::{prelude::*, quests};

pub struct QuestService;

impl QuestService {
    pub async fn create_quest(
        db: &DatabaseConnection,
        title: String,
        description: Option<String>,
        xp_reward: u32,
        validation_type: String,
        target_value: u32,
        complexity: Complexity,
    ) -> AppResult<quests::Model> {
        let new_quest = quests::ActiveModel::new_daily_quest(
            &title,
            description,
            Some(xp_reward),
            &validation_type,
            Some(target_value),
            Some(complexity),
        );

        let model = new_quest.insert(db).await.map_err(AppError::from)?;
        Ok(model)
    }

    pub async fn get_all_quests(db: &DatabaseConnection) -> AppResult<Vec<quests::Model>> {
        Quest::find().all(db).await.map_err(AppError::from)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        quest_id: &str,
    ) -> AppResult<Option<quests::Model>> {
        Quest::find_by_id(quest_id)
            .one(db)
            .await
            .map_err(AppError::from)
    }
}
