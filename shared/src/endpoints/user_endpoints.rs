use crate::endpoints::API;

pub enum UserEndpoints {
    RegisterUserEndpoint,
    LoginUserEndpoint,
}

impl API for UserEndpoints {
    fn path(&self) -> String {
        match self {
            Self::RegisterUserEndpoint => "/register".to_string(),
            Self::LoginUserEndpoint => "/login".to_string(),
        }
    }

    fn template(&self) -> &'static str {
        match self {
            Self::RegisterUserEndpoint => "/register",
            Self::LoginUserEndpoint => "/login",
        }
    }

    fn is_auth_endpoint(&self) -> bool {
        true
    }
}
