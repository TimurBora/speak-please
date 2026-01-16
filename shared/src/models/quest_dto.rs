use sea_orm::prelude::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, Type)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
#[serde(rename_all = "lowercase")]
pub enum Complexity {
    #[sea_orm(string_value = "E")]
    Easy,
    #[sea_orm(string_value = "M")]
    Medium,
    #[sea_orm(string_value = "H")]
    Hard,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct QuestDto {
    pub ulid: String,
    pub title: String,
    pub description: Option<String>,
    pub complexity: Complexity,
    pub xp_reward: u32,
    pub validation_type: String,
    pub target_value: u32,
}

#[derive(Debug, Serialize, Deserialize, Validate, Type)]
pub struct CreateQuestRequest {
    #[validate(length(min = 3, max = 100))]
    pub title: String,
    pub description: Option<String>,
    pub complexity: Complexity,
    pub xp_reward: u32,
    pub validation_type: String,
    pub target_value: u32,
}
