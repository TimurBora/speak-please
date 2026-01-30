use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

pub fn hash(text: &str) -> Result<String, argon2::password_hash::Error> {
    let refresh_token_salt = env!("REFRESH_TOKEN_SALT");
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password_with_salt(text.as_bytes(), refresh_token_salt.as_bytes())?
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_success() {
        let password = "my_super_secret_password_123";

        let hashed_password = hash(password).expect("Hash generation should not fail");

        assert!(!hashed_password.is_empty());
        assert_ne!(hashed_password, password);

        let is_valid =
            verify_hash(password, &hashed_password).expect("Verification should not fail");
        assert!(is_valid, "Password should be valid against its own hash");
    }

    #[test]
    fn test_verify_wrong_password() {
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let hashed_password = hash(password).expect("Hash should be generated");

        let is_valid =
            verify_hash(wrong_password, &hashed_password).expect("Verification should run");
        assert!(!is_valid, "Wrong password should not be verified");
    }

    #[test]
    fn test_hash_consistency() {
        let text = "consistency_test";

        let hash1 = hash(text).unwrap();
        let hash2 = hash(text).unwrap();

        assert_eq!(
            hash1, hash2,
            "Hashes should be identical because of static salt"
        );
    }

    #[test]
    fn test_invalid_hash_format() {
        let result = verify_hash("password", "not_a_valid_phc_hash");

        assert!(
            result.is_err(),
            "Should return error for malformed hash string"
        );
    }
}
