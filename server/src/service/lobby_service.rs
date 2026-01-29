use crate::entities::{
    lobbies::{self, LobbyToSelfMember},
    lobbies_members,
    prelude::Lobby,
};
use sea_orm::{ActiveValue::Set, TransactionTrait, entity::prelude::*};
use shared::{
    errors::{AppError, AppResult},
    models::lobby_dto::Role,
};
use ulid::Ulid;

pub struct LobbyService;

impl LobbyService {
    pub async fn create_lobby(
        db: &DatabaseConnection,
        owner_id: String,
        name: String,
        topic: String,
        description: Option<String>,
    ) -> AppResult<lobbies::Model> {
        let txn = db.begin().await?;

        let lobby_id = Ulid::new().to_string();

        let new_lobby = lobbies::ActiveModel {
            ulid: Set(lobby_id.clone()),
            name: Set(name),
            topic: Set(topic),
            description: Set(description),
            owner_id: Set(owner_id.clone()),
            created_at: Set(chrono::Utc::now()),
        };
        let lobby_model = new_lobby.insert(&txn).await?;

        let membership = lobbies_members::ActiveModel {
            lobby_id: Set(lobby_id),
            user_id: Set(owner_id),
            joined_at: Set(chrono::Utc::now()),
            role: Set(Role::Admin),
        };
        membership.insert(&txn).await?;

        txn.commit().await?;
        Ok(lobby_model)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> AppResult<lobbies::Model> {
        Lobby::find_by_id(id.to_string())
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn get_all_lobbies(db: &DatabaseConnection) -> AppResult<Vec<lobbies::Model>> {
        Lobby::find().all(db).await.map_err(AppError::from)
    }

    // Get all lobbies and a boolean indicating if the user is a member.
    pub async fn get_lobbies_with_membership_status(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> AppResult<Vec<(lobbies::Model, bool)>> {
        let lobbies_with_member = Lobby::find()
            .find_also_linked(LobbyToSelfMember(user_id.to_owned()))
            .all(db)
            .await?;

        let result = lobbies_with_member
            .into_iter()
            .map(|(lobby, member)| (lobby, member.is_some()))
            .collect();

        Ok(result)
    }
}
