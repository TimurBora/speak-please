pub mod arena_battles;
pub mod arena_participants;
pub mod arena_votes;
pub mod journal_entries;
pub mod lobbies;
pub mod quest_proofs;
pub mod quests;
pub mod refresh_tokens;
pub mod shop_items;
pub mod skill_prerequisites;
pub mod skills;
pub mod user_inventory;
pub mod user_quest_status;
pub mod user_skills;
pub mod users;

pub mod prelude {
    pub use super::quests::Entity as Quest;
    pub use super::refresh_tokens::Entity as RefreshToken;
    pub use super::user_quest_status::Entity as UserQuestStatus;
    pub use super::users::Entity as User;
}
