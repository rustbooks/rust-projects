// Input validation utilities

pub fn is_valid_question_text(text: &str) -> bool {
    text.len() >= 10 && text.len() <= 5000
}

pub fn sanitize_input(input: &str) -> String {
    // Remove potentially dangerous characters
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?-()[]{}:;'\"".contains(*c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_question_text() {
        assert!(is_valid_question_text("This is a valid question text"));
        assert!(!is_valid_question_text("Too short"));
    }
}
