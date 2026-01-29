use crate::entities::prelude::Message;
use crate::{entities::messages, service::lobby_member_service::LobbyMemberService};
use sea_orm::{QueryOrder, QuerySelect, entity::prelude::*};
use shared::errors::{AppError, AppResult};

pub struct MessageService;

impl MessageService {
    pub async fn send_message(
        db: &DatabaseConnection,
        lobby_id: String,
        author_id: String,
        content: String,
    ) -> AppResult<messages::Model> {
        let is_member = LobbyMemberService::is_member(db, &lobby_id, &author_id).await;
        if !is_member {
            return Err(AppError::Custom(
                "You are not a member of this lobby".into(),
            ));
        }

        let new_message = messages::ActiveModel::new_message(lobby_id, author_id, content);
        let saved_message = new_message.insert(db).await.map_err(AppError::from)?;

        Ok(saved_message)
    }

    pub async fn get_lobby_messages(
        db: &DatabaseConnection,
        lobby_id: &str,
        user_id: &str,
        limit: u64,
        offset: u64,
    ) -> AppResult<Vec<messages::Model>> {
        if !LobbyMemberService::is_member(db, lobby_id, user_id).await {
            return Err(AppError::NotFound);
        }

        let messages = Message::find()
            .filter(messages::Column::LobbyId.eq(lobby_id))
            .order_by_desc(messages::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?;

        Ok(messages)
    }

    pub async fn delete_message(
        db: &DatabaseConnection,
        message_id: &str,
        user_id: &str,
    ) -> AppResult<()> {
        let message = Message::find_by_id(message_id.to_string())
            .one(db)
            .await?
            .ok_or(AppError::NotFound)?;

        let can_manage = LobbyMemberService::can_manage_lobby(db, &message.lobby_id, user_id).await;
        let is_author = message.author_id == user_id;

        if is_author || can_manage {
            message.delete(db).await?;
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        message_id: &str,
    ) -> AppResult<messages::Model> {
        Message::find_by_id(message_id.to_string())
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }
}
