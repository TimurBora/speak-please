use migration::IntoCondition;
use sea_orm::{ActiveValue::Set, ExprTrait, entity::prelude::*};
use serde::{Deserialize, Serialize};
use shared::models::lobby_dto::LobbyDto;
use specta::Type;
use ulid::Ulid;

use crate::entities::lobbies_members;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Type)]
#[sea_orm(table_name = "lobbies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,

    pub name: String,
    pub description: Option<String>,
    pub topic: String,

    pub owner_id: String,

    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new_lobby(
        name: String,
        topic: String,
        owner_id: String,
        description: Option<String>,
    ) -> Self {
        Self {
            ulid: Set(Ulid::new().to_string()),
            name: Set(name),
            topic: Set(topic),
            owner_id: Set(owner_id),
            description: Set(description),
            created_at: Set(chrono::Utc::now()),
        }
    }
}

impl From<Model> for LobbyDto {
    fn from(model: crate::entities::lobbies::Model) -> Self {
        Self {
            ulid: model.ulid,
            name: model.name,
            topic: model.topic,
            description: model.description,
            owner_id: model.owner_id,
            created_at: model.created_at,
        }
    }
}

pub struct LobbyToSelfMember(pub String);

impl Linked for LobbyToSelfMember {
    type FromEntity = Entity;
    type ToEntity = lobbies_members::Entity;

    fn link(&self) -> Vec<RelationDef> {
        let user_id = self.0.clone();
        vec![
            lobbies_members::Relation::Lobbies
                .def()
                .on_condition(move |_left, right| {
                    Expr::col((right, lobbies_members::Column::UserId))
                        .eq(user_id.clone())
                        .into_condition()
                })
                .rev(),
        ]
    }
}
