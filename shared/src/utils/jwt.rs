use jwt_simple::prelude::*;

use dotenvy_macro::dotenv;

use crate::errors::jwt_errors::JwtError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomClaims {
    pub user_id: String,
    pub email: String,
}

const JWT_SECRET_KEY: &str = dotenv!("JWT_SECRET_KEY");

fn jwt_key() -> HS256Key {
    HS256Key::from_bytes(JWT_SECRET_KEY.as_bytes())
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
