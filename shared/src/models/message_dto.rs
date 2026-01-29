use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type", content = "data")]
pub enum LobbyEvent {
    MessageCreated(MessageDto),
    MessageDeleted { id: String },
    UserJoined { user_id: String }, // TODO: Make UserDTO for this
    UserLeft { user_id: String },
    Typing { user_id: String },
}

#[derive(Serialize, Clone, Debug, Type)]
pub struct MessageDto {
    pub ulid: String,
    pub author_id: String,
    pub author_name: String,
    pub content: String,
    pub created_at: DateTimeUtc,
}
