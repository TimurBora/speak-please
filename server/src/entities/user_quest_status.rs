use sea_orm::{ActiveValue::Set, entity::prelude::*, sqlx::types::chrono};
use serde::{Deserialize, Serialize};
use shared::models::user_quest_status_dto::QuestStatus;
use specta::Type;
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize, Type)]
#[sea_orm(table_name = "user_quest_status")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: String,
    #[sea_orm(belongs_to, from = "user_id", to = "ulid")]
    #[serde(skip)]
    pub user: Option<super::users::Entity>,

    #[sea_orm(primary_key)]
    pub quest_id: String,

    #[sea_orm(belongs_to, from = "quest_id", to = "ulid")]
    #[serde(skip)]
    pub daily_quest: Option<super::quests::Entity>,

    pub is_completed: bool,
    pub current_value: u32,
    pub quest_status: QuestStatus,

    #[sea_orm(primary_key)]
    pub assigned_at: Date,
    pub updated_at: DateTimeUtc,
}

impl ActiveModel {
    pub fn new_user_quest_status(user_id: Ulid, quest_id: Ulid, date: Date) -> Self {
        Self {
            user_id: Set(user_id.to_string()),
            quest_id: Set(quest_id.to_string()),
            is_completed: Set(false),
            updated_at: Set(chrono::Utc::now()),
            assigned_at: Set(date),
            current_value: Set(0),
            quest_status: Set(QuestStatus::NotStarted),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
