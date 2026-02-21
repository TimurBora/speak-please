use crate::endpoints::LobbyUlid;

use super::API;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub enum MessageEndpoints {
    /// GET /lobbies/{lobby_id}/messages?page=X&per_page=Y
    GetLobbyMessages(LobbyUlid),
    /// POST /lobbies/{lobby_id}/messages
    SendMessage(LobbyUlid),
    /// DELETE /lobbies/{lobby_id}/messages/{message_id}
    DeleteMessage(LobbyUlid, String),
}

impl API for MessageEndpoints {
    fn path(&self) -> String {
        match self {
            MessageEndpoints::GetLobbyMessages(lobby_id) => {
                format!("/lobbies/{}/messages", lobby_id)
            }
            MessageEndpoints::SendMessage(lobby_id) => {
                format!("/lobbies/{}/messages", lobby_id)
            }
            MessageEndpoints::DeleteMessage(lobby_id, message_id) => {
                format!("/lobbies/{}/messages/{}", lobby_id, message_id)
            }
        }
    }

    fn template(&self) -> &'static str {
        match self {
            MessageEndpoints::GetLobbyMessages(_) => "/lobbies/{lobby_id}/messages",
            MessageEndpoints::SendMessage(_) => "/lobbies/{lobby_id}/messages",
            MessageEndpoints::DeleteMessage(_, _) => "/lobbies/{lobby_id}/messages/{message_id}",
        }
    }

    fn is_auth_endpoint(&self) -> bool {
        false
    }
}
