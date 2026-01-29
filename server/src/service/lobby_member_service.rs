use crate::entities::{
    lobbies_members::{self},
    prelude::{Lobby, LobbyMembers},
};
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use shared::{
    errors::{AppError, AppResult},
    models::lobby_dto::Role,
};

pub struct LobbyMemberService;

impl LobbyMemberService {
    pub async fn join_lobby(
        db: &DatabaseConnection,
        lobby_id: String,
        user_id: String,
    ) -> AppResult<lobbies_members::Model> {
        let lobby_exists = Lobby::find_by_id(lobby_id.clone()).one(db).await?.is_some();
        if !lobby_exists {
            return Err(AppError::NotFound);
        }

        let existing = LobbyMembers::find_by_id((lobby_id.clone(), user_id.clone()))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(AppError::Custom("Already a member".into()));
        }

        let new_member = lobbies_members::ActiveModel {
            lobby_id: Set(lobby_id),
            user_id: Set(user_id),
            joined_at: Set(chrono::Utc::now()),
            role: Set(Role::Member),
        };

        Ok(new_member.insert(db).await?)
    }

    pub async fn is_member(db: &DatabaseConnection, lobby_id: &str, user_id: &str) -> bool {
        LobbyMembers::find_by_id((lobby_id.to_string(), user_id.to_string()))
            .one(db)
            .await
            .map(|m| m.is_some())
            .unwrap_or(false)
    }

    // I haven't figured out yet how to make a role system, I don't know which approach is the best
    pub async fn can_manage_lobby(db: &DatabaseConnection, lobby_id: &str, user_id: &str) -> bool {
        if let Ok(Some(member)) =
            LobbyMembers::find_by_id((lobby_id.to_string(), user_id.to_string()))
                .one(db)
                .await
        {
            return member.role == Role::Admin || member.role == Role::Moderator;
        }
        false
    }

    pub async fn get_lobby_members_count(
        db: &DatabaseConnection,
        lobby_id: &str,
    ) -> AppResult<u64> {
        LobbyMembers::find()
            .filter(lobbies_members::Column::LobbyId.eq(lobby_id))
            .count(db)
            .await
            .map_err(AppError::from)
    }
}
