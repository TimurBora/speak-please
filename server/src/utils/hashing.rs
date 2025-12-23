use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

pub fn hash(text: &str) -> anyhow::Result<String> {
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(text.as_bytes())?.to_string();

    Ok(password_hash)
}

pub fn verify_hash(text_to_verify: &str, hash: &str) -> anyhow::Result<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(text_to_verify.as_bytes(), &parsed_hash)
        .is_ok())
}
