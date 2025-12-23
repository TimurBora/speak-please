use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "refresh_tokens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub user_id: i32,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::users::Entity>,

    #[sea_orm(not_null, unique)]
    pub token_hash: String,
    #[sea_orm(not_null)]
    pub expires_at: DateTimeUtc,
}

impl ActiveModel {}

impl ActiveModelBehavior for ActiveModel {}
