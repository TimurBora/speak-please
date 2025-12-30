use crate::endpoints::API;

pub enum RefreshTokenEndpoints {
    CreateRefreshToken,
    DeleteRefreshToken(String),
}

impl API for RefreshTokenEndpoints {
    fn path(&self) -> String {
        match self {
            Self::CreateRefreshToken => "/refresh_token".to_string(),
            Self::DeleteRefreshToken(id) => format!("/refresh_token/{}", id),
        }
    }

    fn template(&self) -> &'static str {
        match self {
            Self::CreateRefreshToken => "/refresh_token",
            Self::DeleteRefreshToken(_) => "/refresh_token/{id}",
        }
    }

    fn is_auth_endpoint(&self) -> bool {
        true
    }
}
