use sea_orm::{
    ActiveValue::Set, Order, QueryOrder, QuerySelect, TransactionTrait, entity::prelude::*,
    sqlx::types::chrono,
};
use shared::{
    errors::{AppError, AppResult},
    models::{quest_dto::Complexity, user_quest_status_dto::QuestStatus},
};
use ulid::Ulid;

use crate::entities::{
    prelude::{Quest, UserQuestStatus},
    quests::{self},
    user_quest_status,
};

pub struct UserQuestService;

impl UserQuestService {
    pub async fn get_status(
        db: &DatabaseConnection,
        user_id: &str,
        quest_id: &str,
        date: Date,
    ) -> AppResult<user_quest_status::Model> {
        UserQuestStatus::find_by_id((user_id.to_owned(), quest_id.to_owned(), date))
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn get_user_journal(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> AppResult<Vec<(user_quest_status::Model, quests::Model)>> {
        let data = UserQuestStatus::find()
            .filter(user_quest_status::Column::UserId.eq(user_id))
            .find_also_related(Quest)
            .all(db)
            .await?;

        Ok(data
            .into_iter()
            .filter_map(|(status, quest_opt)| quest_opt.map(|q| (status, q)))
            .collect())
    }

    pub async fn assign_multiple_quests(
        db: &DatabaseConnection,
        user_id: &str,
        quest_ids: Vec<String>,
        date: Date,
    ) -> AppResult<()> {
        let user_ulid = Ulid::from_string(user_id).unwrap();
        let records = quest_ids
            .into_iter()
            .map(|quest_id| {
                let quest_id = Ulid::from_string(&quest_id).unwrap();
                user_quest_status::ActiveModel::new_user_quest_status(user_ulid, quest_id, date)
            })
            .collect::<Vec<_>>();

        if !records.is_empty() {
            UserQuestStatus::insert_many(records).exec(db).await?;
        }
        Ok(())
    }

    pub async fn get_or_assign_quests(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> AppResult<Vec<quests::Model>> {
        let today = chrono::Utc::now().date_naive();
        let daily_status = UserQuestStatus::find()
            .filter(user_quest_status::Column::UserId.eq(user_id))
            .filter(user_quest_status::Column::AssignedAt.eq(today))
            .find_also_related(Quest)
            .all(db)
            .await?;

        if !daily_status.is_empty() {
            return Ok(daily_status.into_iter().filter_map(|(_, q)| q).collect());
        }

        let mut selected = Vec::new();
        selected.extend(Self::pick_random_by_complexity(db, Complexity::Easy, 2, &[]).await?);
        selected.extend(Self::pick_random_by_complexity(db, Complexity::Medium, 2, &[]).await?);
        selected.extend(Self::pick_random_by_complexity(db, Complexity::Hard, 1, &[]).await?);

        let ids: Vec<String> = selected.iter().map(|q| q.ulid.clone()).collect();
        Self::assign_multiple_quests(db, user_id, ids, today).await?;

        Ok(selected)
    }

    pub async fn get_daily_quests_with_status(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> AppResult<Vec<(user_quest_status::Model, quests::Model)>> {
        let today = chrono::Utc::now().date_naive();

        let daily_data = UserQuestStatus::find()
            .filter(user_quest_status::Column::UserId.eq(user_id))
            .filter(user_quest_status::Column::AssignedAt.eq(today))
            .find_also_related(Quest)
            .all(db)
            .await?;

        if !daily_data.is_empty() {
            return Ok(daily_data
                .into_iter()
                .filter_map(|(s, q)| q.map(|quest| (s, quest)))
                .collect());
        }

        let quests = Self::get_or_assign_quests(db, user_id).await?;
        let result = quests
            .into_iter()
            .map(|q| {
                let status = user_quest_status::Model {
                    user_id: user_id.to_string(),
                    quest_id: q.ulid.clone(),
                    assigned_at: today,
                    is_completed: false,
                    current_value: 0,
                    quest_status: QuestStatus::InProgress,
                    updated_at: chrono::Utc::now(),
                };
                (status, q)
            })
            .collect();

        Ok(result)
    }

    async fn pick_random_by_complexity(
        db: &DatabaseConnection,
        complexity: Complexity,
        limit: u64,
        exclude_ids: &[String],
    ) -> AppResult<Vec<quests::Model>> {
        let mut query = Quest::find().filter(quests::Column::Complexity.eq(complexity));
        if !exclude_ids.is_empty() {
            query = query.filter(quests::Column::Ulid.is_not_in(exclude_ids));
        }
        query
            .order_by(Expr::cust("RANDOM()"), Order::Asc)
            .limit(limit)
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn complete_quest_internal<C>(
        db: &C,
        user_id: &str,
        quest_id: &str,
        date: Date,
    ) -> AppResult<user_quest_status::Model>
    where
        C: ConnectionTrait,
    {
        let status = UserQuestStatus::find_by_id((user_id.to_owned(), quest_id.to_owned(), date))
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        if status.is_completed {
            return Ok(status);
        }

        let mut active_status: user_quest_status::ActiveModel = status.into();
        active_status.is_completed = Set(true);
        active_status.quest_status = Set(QuestStatus::Completed);
        active_status.updated_at = Set(chrono::Utc::now());

        Ok(active_status.update(db).await?)
    }

    // 8. Публичный метод с транзакцией
    pub async fn complete_quest(
        db: &DatabaseConnection,
        user_id: &str,
        quest_id: &str,
        date: Date,
    ) -> AppResult<user_quest_status::Model> {
        let txn = db.begin().await?;
        let res = Self::complete_quest_internal(&txn, user_id, quest_id, date).await?;
        txn.commit().await?;
        Ok(res)
    }
}
