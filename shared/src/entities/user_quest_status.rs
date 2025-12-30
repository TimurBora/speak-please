use sea_orm::{ActiveValue::Set, entity::prelude::*, sqlx::types::chrono};
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "user_quest_status")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: String,
    #[sea_orm(belongs_to, from = "user_id", to = "ulid")]
    pub user: Option<super::users::Entity>,

    #[sea_orm(primary_key)]
    pub quest_id: String,
    #[sea_orm(belongs_to, from = "quest_id", to = "ulid")]
    pub daily_quest: Option<super::daily_quests::Entity>,

    pub is_completed: bool,
    pub updated_at: DateTimeUtc,
}

impl ActiveModel {
    pub fn new_user_quest_status(user_id: Ulid, quest_id: Ulid) -> Self {
        Self {
            user_id: Set(user_id.to_string()),
            quest_id: Set(quest_id.to_string()),
            is_completed: Set(false),
            updated_at: Set(chrono::Utc::now()),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
