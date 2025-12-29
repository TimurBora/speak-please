pub fn validate_ulid(token: &str) -> Result<(), validator::ValidationError> {
    ulid::Ulid::from_string(token)
        .map(|_| ())
        .map_err(|_| validator::ValidationError::new("invalid_ulid"))
}
