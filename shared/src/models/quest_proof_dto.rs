use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Type)]
pub struct SubmitProofRequest {
    pub proof_text: Option<String>,

    #[validate(range(min = 0, max = 5))]
    pub photo_count: u32,

    #[validate(range(min = 0, max = 3))]
    pub voice_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SubmitProofResponse {
    pub proof_ulid: String,
    pub status: String,
    pub photo_upload_urls: Vec<String>,
    pub voice_upload_urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ProofDetailsResponse {
    pub ulid: String,
    pub user_id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub quest_id: String,
    pub quest_title: String,
    pub quest_description: Option<String>,
    pub xp_reward: u32,
    pub proof_text: Option<String>,
    pub status: String,
    pub photo_urls: Vec<String>,
    pub voice_urls: Vec<String>,
    pub beliefs_count: u32,
    pub is_believed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct ProofFeedResponse {
    pub items: Vec<ProofDetailsResponse>,
    pub has_more: bool,
    pub next_offset: u32,
}

#[derive(Debug, Deserialize, Validate, Type)]
pub struct PaginationQuery {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
