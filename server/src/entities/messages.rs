use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use shared::models::message_dto::MessageDto;
use specta::Type;
use ulid::Ulid;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize, Type)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,

    pub lobby_id: String,

    #[sea_orm(belongs_to, from = "lobby_id", to = "ulid")]
    #[serde(skip)]
    pub lobby: HasOne<super::lobbies::Entity>,

    pub author_id: String,

    #[sea_orm(belongs_to, from = "author_id", to = "ulid")]
    #[serde(skip)]
    pub user: Option<super::users::Entity>,

    #[sea_orm(column_type = "Text")]
    pub content: String,

    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new_message(lobby_id: String, author_id: String, content: String) -> Self {
        Self {
            ulid: Set(Ulid::new().to_string()),
            lobby_id: Set(lobby_id),
            author_id: Set(author_id),
            content: Set(content),
            created_at: Set(chrono::Utc::now()),
        }
    }
}
