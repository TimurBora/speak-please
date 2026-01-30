use jwt_simple::prelude::*;

use crate::errors::jwt_errors::JwtError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomClaims {
    pub user_id: String,
    pub email: String,
}

fn jwt_key() -> HS256Key {
    let jwt_secret_key = std::env::var("JWT_SECRET_KEY").expect("JWT secret key must be provided");
    HS256Key::from_bytes(jwt_secret_key.as_bytes())
}

pub fn create_access_token(user_id: String, email: String) -> Result<String, JwtError> {
    let claims = Claims::with_custom_claims(
        CustomClaims { user_id, email },
        Duration::from_secs(15 * 60),
    );

    jwt_key()
        .authenticate(claims)
        .map_err(|_| JwtError::CreationFailed)
}

pub fn verify_access_token(token: &str) -> Result<CustomClaims, JwtError> {
    let claims = jwt_key()
        .verify_token::<CustomClaims>(token, None)
        .map_err(|_| JwtError::InvalidToken)?;

    Ok(claims.custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_access_token_success() {
        let user_id = "user_123".to_string();
        let email = "test@example.com".to_string();

        let token =
            create_access_token(user_id.clone(), email.clone()).expect("Token creation failed");

        let claims = verify_access_token(&token).expect("Token verification failed");

        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_verify_invalid_token() {
        let invalid_token = "not.a.valid.token";

        let result = verify_access_token(invalid_token);

        assert!(matches!(result, Err(JwtError::InvalidToken)));
    }

    #[test]
    fn test_verify_token_with_wrong_key() {
        let user_id = "user_123".to_string();
        let email = "test@example.com".to_string();

        let token = create_access_token(user_id, email).unwrap();

        let wrong_key = HS256Key::from_bytes(b"another_secret_key_that_is_long_enough");
        let result = wrong_key.verify_token::<CustomClaims>(&token, None);

        assert!(
            result.is_err(),
            "Token should not be valid for a different secret key"
        );
    }

    #[test]
    fn test_token_structure() {
        let token = create_access_token("id".into(), "email@test.com".into()).unwrap();

        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(
            parts.len(),
            3,
            "JWT should have header, payload, and signature"
        );
    }
}
