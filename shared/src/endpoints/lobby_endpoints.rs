use crate::endpoints::{API, LobbyUlid, UserUlid};

pub enum LobbyEndpoints {
    Create,
    GetAll(UserUlid),
    GetDetails(LobbyUlid),
    Join(LobbyUlid, UserUlid),
    GetMembersCount(LobbyUlid),
}

impl API for LobbyEndpoints {
    fn path(&self) -> String {
        match self {
            Self::Create => "/lobbies".to_string(),
            Self::GetAll(user_ulid) => format!("/users/{user_ulid}/lobbies"),
            Self::GetDetails(id) => format!("/lobbies/{id}"),
            Self::Join(lobby_id, user_id) => format!("/lobbies/{lobby_id}/join/{user_id}"),
            Self::GetMembersCount(id) => format!("/lobbies/{id}/members/count"),
        }
    }

    fn template(&self) -> &'static str {
        match self {
            Self::Create => "/lobbies",
            Self::GetAll(_) => "/users/{user_ulid}/lobbies",
            Self::GetDetails(_) => "/lobbies/{id}",
            Self::Join(_, _) => "/lobbies/{lobby_id}/join/{user_id}",
            Self::GetMembersCount(_) => "/lobbies/{id}/members/count",
        }
    }

    fn is_auth_endpoint(&self) -> bool {
        true
    }
}
