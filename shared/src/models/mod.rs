use serde::{Deserialize, Serialize};
use specta::Type;

pub mod lobby_dto;
pub mod message_dto;
pub mod quest_dto;
pub mod quest_proof_dto;
pub mod refresh_token_dto;
pub mod user_dto;
pub mod user_quest_status_dto;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}
