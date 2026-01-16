use sea_orm::{ActiveValue::Set, entity::prelude::*};
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "refresh_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,
    pub user_id: String,
    #[sea_orm(belongs_to, from = "user_id", to = "ulid")]
    pub user: HasOne<super::users::Entity>,

    #[sea_orm(not_null, unique)]
    pub token_hash: String,
    #[sea_orm(not_null)]
    pub expires_at: DateTimeUtc,
}

impl ActiveModel {
    pub fn new_refresh_token(user_id: &str, token_hash: &str) -> Self {
        let expires_at = chrono::Utc::now() + chrono::TimeDelta::try_days(7).unwrap();
        // TODO:
        // FIX THIS SHIT
        Self {
            ulid: Set(Ulid::new().to_string()),
            user_id: Set(user_id.to_owned()),
            token_hash: Set(token_hash.to_owned()),
            expires_at: Set(expires_at),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
