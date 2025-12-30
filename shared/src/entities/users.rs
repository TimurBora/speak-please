use sea_orm::{ActiveValue::Set, entity::prelude::*, sqlx::types::chrono};
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub xp_balance: i32,
    pub total_xp_accumulated: i32,
    pub level: i32,
    pub created_at: DateTimeUtc,
    pub daily_quests_streak: i32,

    #[sea_orm(has_many)]
    pub refresh_token: HasMany<super::refresh_tokens::Entity>,
}

impl ActiveModel {
    pub fn new_user(username: String, email: String, password_hash: String) -> Self {
        Self {
            ulid: Set(Ulid::new().to_string()),
            username: Set(username),
            email: Set(email),
            password_hash: Set(password_hash),
            xp_balance: Set(0),
            total_xp_accumulated: Set(0),
            level: Set(1),
            created_at: Set(chrono::Utc::now()),
            daily_quests_streak: Set(0),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
