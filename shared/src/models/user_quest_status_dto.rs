use crate::{models::quest_dto::QuestDto, utils::ulid_validation::validate_ulid};
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

use sea_orm::prelude::StringLen;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, Type)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuestStatus {
    #[sea_orm(rename = "SCREAMING_SNAKE_CASE")]
    NotStarted,

    #[sea_orm(rename = "SCREAMING_SNAKE_CASE")]
    InProgress,

    #[sea_orm(rename = "SCREAMING_SNAKE_CASE")]
    Completed,

    #[sea_orm(rename = "SCREAMING_SNAKE_CASE")]
    InPending,

    #[sea_orm(rename = "SCREAMING_SNAKE_CASE")]
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct UserQuestStatusResponse {
    pub user_ulid: String,
    pub quest: QuestDto,
    pub status: QuestStatus,
    pub current_value: u32,
    pub is_completed: bool,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Type)]
pub struct CompleteQuestRequest {
    #[validate(custom(function = "validate_ulid"))]
    pub user_ulid: String,

    #[validate(custom(function = "validate_ulid"))]
    pub quest_ulid: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DailyQuestsResponse {
    pub date: chrono::NaiveDate,
    pub quests: Vec<UserQuestStatusResponse>,
}
