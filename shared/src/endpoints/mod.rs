use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Serialize, Deserialize, Clone, Type)]
pub struct UserUlid(pub String);

#[derive(Default, Debug, Serialize, Deserialize, Clone, Type)]
pub struct QuestProofUlid(pub String);

#[derive(Default, Debug, Serialize, Deserialize, Clone, Type)]
pub struct QuestUlid(pub String);

#[derive(Default, Debug, Serialize, Deserialize, Clone, Type)]
pub struct LobbyUlid(pub String);

macro_rules! impl_ulid_wrapper {
    ($($t:ty),*) => {
        $(
            impl std::fmt::Display for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl std::ops::Deref for $t {
                type Target = String;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        )*
    };
}

impl_ulid_wrapper!(UserUlid, QuestProofUlid, QuestUlid, LobbyUlid);

// I believe this is a solid approach for endpoints because we get compiler checks
// and unified interfaces for both frontend and backend.
pub trait API {
    fn path(&self) -> String;
    fn template(&self) -> &'static str;
    fn format_with_api_url(&self, api_url: &str) -> String {
        format!("{}{}", api_url, self.path())
    }
    fn is_auth_endpoint(&self) -> bool;
}

pub mod lobby_endpoints;
pub mod message_endpoints;
pub mod quest_proof_endpoints;
pub mod refresh_token_endpoints;
pub mod user_endpoints;
pub mod user_quest_status_endpoints;
