use serde::Serialize;
use validator::Validate;

#[derive(Serialize)]
pub struct ValidationResponse {
    pub success: bool,
    pub errors: Option<serde_json::Value>,
}

pub fn check_anything<T>(data: T) -> ValidationResponse
where
    T: Validate,
{
    match data.validate() {
        Ok(_) => ValidationResponse {
            success: true,
            errors: None,
        },
        Err(e) => ValidationResponse {
            success: false,
            errors: Some(serde_json::to_value(e).unwrap()),
        },
    }
}
