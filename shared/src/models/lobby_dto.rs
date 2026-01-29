use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

use sea_orm::prelude::StringLen;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LobbyDto {
    pub ulid: String,
    pub name: String,
    pub topic: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LobbyFeedItem {
    pub lobby: LobbyDto,
    pub is_member: bool,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LobbyFeedResponse {
    pub items: Vec<LobbyFeedItem>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Type)]
pub struct CreateLobbyRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,

    #[validate(length(min = 2, max = 30))]
    pub topic: String,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LobbyMemberDto {
    pub lobby_id: String,
    pub user_id: String,
    pub role: Role,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct LobbyDetailsResponse {
    pub lobby: LobbyDto,
    pub members_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, Type)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(15))")]
#[serde(rename_all = "UPPERCASE")]
pub enum Role {
    #[sea_orm(string_value = "STRANGER")]
    Stranger,

    #[sea_orm(string_value = "MEMBER")]
    Member,

    #[sea_orm(string_value = "HELPER")]
    Helper,

    #[sea_orm(string_value = "MODERATOR")]
    Moderator,

    #[sea_orm(string_value = "ADMIN")]
    Admin,
}
