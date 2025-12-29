use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use dotenvy_macro::dotenv;

const REFRESH_TOKEN_SALT: &str = dotenv!("REFRESH_TOKEN_SALT");

pub fn hash(text: &str) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password_with_salt(text.as_bytes(), REFRESH_TOKEN_SALT.as_bytes())?
        .to_string();

    Ok(password_hash)
}

pub fn verify_hash(
    text_to_verify: &str,
    hash: &str,
) -> Result<bool, argon2::password_hash::phc::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(text_to_verify.as_bytes(), &parsed_hash)
        .is_ok())
}
