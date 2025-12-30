use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "daily_quests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,

    pub title: String,
    pub description: Option<String>,
    pub complexity: Complexity,

    pub xp_reward: u32,

    pub action_type: String,     // TODO: It's enum
    pub validation_type: String, // TODO: ENUM(AUTOMATIC, COMMUNITY, MODERATION AND OTHER)
    pub target_value: u32,
}

impl ActiveModel {
    pub fn new_daily_quest(
        title: &str,
        description: Option<String>,
        xp_reward: Option<u32>,
        action_type: &str,
        validation_type: &str,
        target_value: Option<u32>,
        complexity: Option<Complexity>,
    ) -> Self {
        let xp_reward = xp_reward.unwrap_or(10);
        let target_value = target_value.unwrap_or(1);
        let complexity = complexity.unwrap_or(Complexity::Easy);
        Self {
            ulid: Set(Ulid::new().to_string()),
            title: Set(title.to_string()),
            description: Set(description),
            xp_reward: Set(xp_reward),
            action_type: Set(action_type.to_string()),
            validation_type: Set(validation_type.to_string()),
            target_value: Set(target_value),
            complexity: Set(complexity),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum Complexity {
    #[sea_orm(string_value = "E")]
    Easy,
    #[sea_orm(string_value = "M")]
    Medium,
    #[sea_orm(string_value = "H")]
    Hard,
}
