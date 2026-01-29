use sea_orm::EnumIter;
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use shared::models::lobby_dto::{LobbyMemberDto, Role};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "lobby_members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub lobby_id: String,

    #[sea_orm(belongs_to, from = "lobby_id", to = "ulid")]
    pub lobby: HasOne<super::lobbies::Entity>,

    #[sea_orm(primary_key)]
    pub user_id: String,

    #[sea_orm(belongs_to, from = "user_id", to = "ulid")]
    pub user: Option<super::users::Entity>,

    pub joined_at: DateTimeUtc,

    pub role: Role,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(lobby_id: String, user_id: String) -> Self {
        Self {
            lobby_id: Set(lobby_id),
            user_id: Set(user_id),
            joined_at: Set(chrono::Utc::now()),
            role: Set(Role::Member),
        }
    }
}

impl From<Model> for LobbyMemberDto {
    fn from(model: crate::entities::lobbies_members::Model) -> Self {
        Self {
            lobby_id: model.lobby_id,
            user_id: model.user_id,
            role: model.role,
            joined_at: model.joined_at,
        }
    }
}
