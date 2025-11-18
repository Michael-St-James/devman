// Validation helper functions

/// Validates an email address (basic validation)
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

/// Validates a username (alphanumeric, 3-20 characters)
pub fn is_valid_username(username: &str) -> bool {
    let len = username.len();
    len >= 3 && len <= 20 && username.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Validates a password strength (minimum 8 characters)
pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 8
}

/// Validates that a string is not empty or whitespace only
pub fn is_not_empty(text: &str) -> bool {
    !text.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("user@example.com"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@.com"));
    }

    #[test]
    fn test_username_validation() {
        assert!(is_valid_username("john_doe"));
        assert!(is_valid_username("user123"));
        assert!(!is_valid_username("ab")); // too short
        assert!(!is_valid_username("user@name")); // invalid char
    }

    #[test]
    fn test_password_validation() {
        assert!(is_valid_password("password123"));
        assert!(!is_valid_password("short"));
    }
}
