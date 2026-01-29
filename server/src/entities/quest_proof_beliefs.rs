use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize, Type)]
#[sea_orm(table_name = "quest_proof_beliefs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: String,

    #[sea_orm(belongs_to, from = "user_id", to = "ulid")]
    #[serde(skip)]
    pub user: Option<super::users::Entity>,

    #[sea_orm(primary_key)]
    pub proof_id: String,

    #[sea_orm(belongs_to, from = "proof_id", to = "ulid")]
    #[serde(skip)]
    pub proof: Option<super::quest_proofs::Entity>,

    pub created_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
