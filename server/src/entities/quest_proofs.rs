use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "quest_proofs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ulid: String,

    pub user_id: String,
    #[sea_orm(belongs_to, from = "user_id", to = "ulid")]
    pub user: HasOne<super::users::Entity>,

    pub quest_id: String,
    #[sea_orm(belongs_to, from = "quest_id", to = "ulid")]
    pub quest: HasOne<super::quests::Entity>,

    #[sea_orm(column_type = "Text")]
    pub proof_text: String,
    pub photos: Option<Json>,
    pub voice_notes: Option<Json>,

    pub status: ProofStatus,
    pub votes_count: i32,
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, Type)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(10))")]
pub enum ProofStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "approved")]
    Approved,
    #[sea_orm(string_value = "rejected")]
    Rejected,
}

impl ActiveModelBehavior for ActiveModel {}
