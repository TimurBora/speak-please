use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Clone, Type, Debug)]
pub struct UserSession {
    pub access_token: Option<String>,
    pub user_ulid: String,
    pub email: String,

    pub username: String,
    pub level: u32,
    pub avatar_url: Option<String>,
}
