pub fn validate_ulid(token: &str) -> Result<(), validator::ValidationError> {
    ulid::Ulid::from_string(token)
        .map(|_| ())
        .map_err(|_| validator::ValidationError::new("invalid_ulid"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ulid::Ulid;

    #[test]
    fn test_validate_ulid_success() {
        let valid_ulid = Ulid::new().to_string();

        let result = validate_ulid(&valid_ulid);

        assert!(result.is_ok(), "Valid ULID should pass validation");
    }

    #[test]
    fn test_validate_ulid_invalid_characters() {
        let invalid_ulid = "01ARZ3NDEKTSV4RRFFQ6KH/###";

        let result = validate_ulid(invalid_ulid);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "invalid_ulid");
    }

    #[test]
    fn test_validate_ulid_wrong_length() {
        let short_ulid = "01ARZ3NDEKTSV4RRFFQ6";

        let result = validate_ulid(short_ulid);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "invalid_ulid");
    }

    #[test]
    fn test_validate_ulid_empty_string() {
        let result = validate_ulid("");

        assert!(result.is_err());
    }
}
