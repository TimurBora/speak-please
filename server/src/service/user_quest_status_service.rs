use sea_orm::{ActiveValue::Set, TransactionTrait, entity::prelude::*, sqlx::types::chrono};
use shared::{
    entities::{daily_quests, prelude::*, user_quest_status},
    errors::{AppError, AppResult},
};

pub struct UserQuestService;

impl UserQuestService {
    pub async fn get_status(
        db: &DatabaseConnection,
        user_id: &str,
        quest_id: &str,
    ) -> AppResult<user_quest_status::Model> {
        UserQuestStatus::find_by_id((user_id.to_owned(), quest_id.to_owned()))
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn complete_quest(
        db: &DatabaseConnection,
        user_id: &str,
        quest_id: &str,
    ) -> AppResult<user_quest_status::Model> {
        let quest = DailyQuest::find_by_id(quest_id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        let status = UserQuestStatus::find_by_id((user_id.to_owned(), quest_id.to_owned()))
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        if status.is_completed {
            return Err(AppError::Validation(validator::ValidationError::new(
                "Quest already completed",
            )));
        }

        let mut active_status: user_quest_status::ActiveModel = status.into();
        active_status.is_completed = Set(true);
        active_status.updated_at = Set(chrono::Utc::now());

        let txn = db.begin().await?;

        let updated_model = active_status.update(&txn).await?;

        //        UserService::add_xp(&txn, user_id, quest.xp_reward as i32)
        //           .await
        //          .map_err(AppError::Api)?;

        txn.commit().await?;

        Ok(updated_model)
    }

    pub async fn get_user_journal(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> AppResult<Vec<(user_quest_status::Model, Option<daily_quests::Model>)>> {
        UserQuestStatus::find()
            .filter(user_quest_status::Column::UserId.eq(user_id))
            .find_also_related(DailyQuest)
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn assign_multiple_quests(
        db: &DatabaseConnection,
        user_id: &str,
        quest_ids: Vec<String>,
    ) -> AppResult<()> {
        let now = chrono::Utc::now();
        let records = quest_ids
            .into_iter()
            .map(|q_id| user_quest_status::ActiveModel {
                user_id: Set(user_id.to_owned()),
                quest_id: Set(q_id),
                is_completed: Set(false),
                updated_at: Set(now),
            })
            .collect::<Vec<_>>();

        if !records.is_empty() {
            UserQuestStatus::insert_many(records).exec(db).await?;
        }

        Ok(())
    }

    pub async fn clear_user_quests(db: &DatabaseConnection, user_id: &str) -> AppResult<u64> {
        let res = UserQuestStatus::delete_many()
            .filter(user_quest_status::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        Ok(res.rows_affected)
    }
}
