//! Structured syntax errors with source positions.

use std::fmt;

/// A lexing or parsing error, carrying the source position where it occurred.
///
/// The [`fmt::Display`] implementation renders the message together with the
/// line and column, so callers that only need a human-readable string can use
/// `error.to_string()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl SyntaxError {
    /// Create a new syntax error at the given position (1-based line/column).
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.line, self.column
        )
    }
}

impl std::error::Error for SyntaxError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_includes_position() {
        let error = SyntaxError::new("Unexpected token", 3, 14);
        assert_eq!(error.to_string(), "Unexpected token at line 3, column 14");
    }
}
