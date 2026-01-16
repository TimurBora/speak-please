pub trait API {
    fn path(&self) -> String;
    fn template(&self) -> &'static str;
    fn format_with_api_url(&self, api_url: &str) -> String {
        format!("{}{}", api_url, self.path())
    }
    fn is_auth_endpoint(&self) -> bool;
}

pub mod refresh_token_endpoints;
pub mod user_endpoints;
pub mod user_quest_status_endpoints;
