use sea_orm::{ActiveValue::Set, entity::prelude::*, sqlx::types::chrono};
use serde::{Deserialize, Serialize};
use specta::Type;
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Type)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,

    pub xp_balance: i32,
    pub total_xp_accumulated: i32,
    pub level: u32,

    pub created_at: DateTimeUtc,
    pub last_active_at: DateTimeUtc,

    pub avatar_url: Option<String>,
    pub bio: Option<String>,

    #[sea_orm(has_many)]
    #[serde(skip)]
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
            last_active_at: Set(chrono::Utc::now()),

            avatar_url: Set(None),
            bio: Set(None),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
