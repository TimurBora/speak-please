use sea_orm::{
    ActiveValue::Set, Order, QueryOrder, QuerySelect, TransactionTrait, entity::prelude::*,
    sqlx::types::chrono,
};
use shared::{
    errors::{AppError, AppResult},
    models::quest_dto::Complexity,
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

    pub async fn complete_quest(
        db: &DatabaseConnection,
        user_id: &str,
        quest_id: &str,
        date: Date,
    ) -> AppResult<user_quest_status::Model> {
        let quest = Quest::find_by_id(quest_id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        let status = UserQuestStatus::find_by_id((user_id.to_owned(), quest_id.to_owned(), date))
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
    ) -> AppResult<Vec<(user_quest_status::Model, Option<quests::Model>)>> {
        UserQuestStatus::find()
            .filter(user_quest_status::Column::UserId.eq(user_id))
            .find_also_related(Quest)
            .all(db)
            .await
            .map_err(AppError::from)
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

        // 1. Пытаемся получить квесты за сегодня
        let daily_status = UserQuestStatus::find()
            .filter(user_quest_status::Column::UserId.eq(user_id))
            .filter(user_quest_status::Column::AssignedAt.eq(today))
            .find_also_related(Quest)
            .all(db)
            .await?;

        if !daily_status.is_empty() {
            // Возвращаем только сами модели квестов
            return Ok(daily_status.into_iter().filter_map(|(_, q)| q).collect());
        }

        // 2. Если квестов нет, выбираем новые (2 Easy, 2 Medium, 1 Hard)
        // Можно добавить исключение последних выполненных квестов здесь
        let mut selected = Vec::new();

        selected.extend(Self::pick_random_by_complexity(db, Complexity::Easy, 2, &[]).await?);
        selected.extend(Self::pick_random_by_complexity(db, Complexity::Medium, 2, &[]).await?);
        selected.extend(Self::pick_random_by_complexity(db, Complexity::Hard, 1, &[]).await?);

        // 3. Сохраняем их в базу через твой assign_multiple_quests
        let ids: Vec<String> = selected.iter().map(|q| q.ulid.clone()).collect();
        Self::assign_multiple_quests(db, user_id, ids, today).await?;

        Ok(selected)
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

        // Используем ORDER BY RANDOM() (для Postgres/SQLite)
        query
            .order_by(Expr::cust("RANDOM()"), Order::Asc)
            .limit(limit)
            .all(db)
            .await
            .map_err(AppError::from)
    }
}
