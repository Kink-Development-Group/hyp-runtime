/// String builtin functions
pub struct StringBuiltins;

impl StringBuiltins {
    /// Get string length
    pub fn length(s: &str) -> usize {
        s.len()
    }

    /// Convert to uppercase
    pub fn to_upper(s: &str) -> String {
        s.to_uppercase()
    }

    /// Convert to lowercase
    pub fn to_lower(s: &str) -> String {
        s.to_lowercase()
    }

    /// Trim whitespace
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }

    /// Find index of substring
    pub fn index_of(s: &str, pattern: &str) -> i64 {
        s.find(pattern).map(|i| i as i64).unwrap_or(-1)
    }

    /// Replace substring
    pub fn replace(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }

    /// Reverse string
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// Capitalize first letter
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    /// Check if string starts with prefix
    pub fn starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }

    /// Check if string ends with suffix
    pub fn ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }

    /// Check if string contains substring
    pub fn contains(s: &str, pattern: &str) -> bool {
        s.contains(pattern)
    }

    /// Split string by delimiter
    pub fn split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|s| s.to_string()).collect()
    }

    /// Substring from start to end
    pub fn substring(s: &str, start: usize, end: usize) -> String {
        let chars: Vec<char> = s.chars().collect();
        let start = start.min(chars.len());
        let end = end.min(chars.len());
        if start >= end {
            String::new()
        } else {
            chars[start..end].iter().collect()
        }
    }

    /// Repeat string n times
    pub fn repeat(s: &str, times: usize) -> String {
        s.repeat(times)
    }

    /// Pad left with character
    pub fn pad_left(s: &str, total_width: usize, pad_char: char) -> String {
        let padding = total_width.saturating_sub(s.len());
        format!("{}{}", pad_char.to_string().repeat(padding), s)
    }

    /// Pad right with character
    pub fn pad_right(s: &str, total_width: usize, pad_char: char) -> String {
        let padding = total_width.saturating_sub(s.len());
        format!("{}{}", s, pad_char.to_string().repeat(padding))
    }

    /// Check if string is empty
    pub fn is_empty(s: &str) -> bool {
        s.is_empty()
    }

    /// Check if string is whitespace
    pub fn is_whitespace(s: &str) -> bool {
        s.chars().all(char::is_whitespace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(StringBuiltins::length("hello"), 5);
        assert_eq!(StringBuiltins::length(""), 0);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(StringBuiltins::reverse("hello"), "olleh");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(StringBuiltins::capitalize("hello"), "Hello");
        assert_eq!(StringBuiltins::capitalize(""), "");
    }

    #[test]
    fn test_index_of() {
        assert_eq!(StringBuiltins::index_of("hello world", "world"), 6);
        assert_eq!(StringBuiltins::index_of("hello", "xyz"), -1);
    }
}
