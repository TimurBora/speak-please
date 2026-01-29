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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[derive(Validate)]
    struct TestData {
        #[validate(length(min = 3))]
        name: String,
        #[validate(email)]
        email: String,
    }

    #[test]
    fn test_check_anything_success() {
        let data = TestData {
            name: "Alice".into(),
            email: "alice@test.com".into(),
        };

        let response = check_anything(data);

        assert!(response.success);
        assert!(response.errors.is_none());
    }

    #[test]
    fn test_check_anything_error() {
        let data = TestData {
            name: "Al".into(),
            email: "not-an-email".into(),
        };

        let response = check_anything(data);

        assert!(!response.success);
        assert!(response.errors.is_some());

        let errors_json = response.errors.unwrap();
        assert!(errors_json.get("name").is_some());
        assert!(errors_json.get("email").is_some());
    }
}
