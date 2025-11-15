//! String manipulation builtin functions for HypnoScript.
//!
//! This module provides comprehensive string operations including:
//! - Basic operations (length, case conversion, trimming)
//! - Search and matching (index, contains, starts/ends with)
//! - Manipulation (replace, split, substring, repeat)
//! - Formatting (padding, truncation, wrapping)
//! - Advanced operations (slicing with negative indices, insertion, removal)
//!
//! All functions are designed to work with Unicode strings correctly,
//! handling multi-byte characters appropriately.

use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;

/// String manipulation functions.
///
/// This struct provides static methods for all string operations in HypnoScript.
/// All methods are Unicode-aware and handle multi-byte characters correctly.
pub struct StringBuiltins;

impl BuiltinModule for StringBuiltins {
    fn module_name() -> &'static str {
        "String"
    }

    fn description() -> &'static str {
        "String manipulation and analysis functions"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("String manipulation and analysis functions")
            .with_translation("de", "Zeichenketten-Manipulations- und Analysefunktionen")
            .with_translation("fr", "Fonctions de manipulation et d'analyse de chaînes")
            .with_translation("es", "Funciones de manipulación y análisis de cadenas");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "Length",
            "ToUpper",
            "ToLower",
            "Trim",
            "TrimStart",
            "TrimEnd",
            "IndexOf",
            "LastIndexOf",
            "Replace",
            "ReplaceFirst",
            "Reverse",
            "Capitalize",
            "StartsWith",
            "EndsWith",
            "Contains",
            "Split",
            "Substring",
            "Repeat",
            "PadLeft",
            "PadRight",
            "IsEmpty",
            "IsWhitespace",
            "CharAt",
            "Concat",
            "SliceWithNegative",
            "InsertAt",
            "RemoveAt",
            "CountSubstring",
            "Truncate",
            "WrapText",
        ]
    }
}

impl StringBuiltins {
    /// Get the length of a string (number of Unicode characters).
    ///
    /// # Arguments
    /// * `s` - The string to measure
    ///
    /// # Returns
    /// Number of Unicode characters in the string
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::StringBuiltins;
    /// assert_eq!(StringBuiltins::length("hello"), 5);
    /// assert_eq!(StringBuiltins::length(""), 0);
    /// assert_eq!(StringBuiltins::length("🎯"), 1); // Unicode emoji
    /// ```
    pub fn length(s: &str) -> usize {
        s.chars().count()
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

    /// Trim whitespace from start only
    pub fn trim_start(s: &str) -> String {
        s.trim_start().to_string()
    }

    /// Trim whitespace from end only
    pub fn trim_end(s: &str) -> String {
        s.trim_end().to_string()
    }

    /// Find index of substring
    pub fn index_of(s: &str, pattern: &str) -> i64 {
        s.find(pattern).map(|i| i as i64).unwrap_or(-1)
    }

    /// Find last index of substring
    pub fn last_index_of(s: &str, pattern: &str) -> i64 {
        s.rfind(pattern).map(|i| i as i64).unwrap_or(-1)
    }

    /// Replace substring
    pub fn replace(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }

    /// Replace first occurrence only
    pub fn replace_first(s: &str, from: &str, to: &str) -> String {
        if let Some(pos) = s.find(from) {
            let mut result = String::with_capacity(s.len());
            result.push_str(&s[..pos]);
            result.push_str(to);
            result.push_str(&s[pos + from.len()..]);
            result
        } else {
            s.to_string()
        }
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

    /// Get character at index (as string)
    pub fn char_at(s: &str, index: usize) -> Option<String> {
        s.chars().nth(index).map(|c| c.to_string())
    }

    /// Concatenate multiple strings
    pub fn concat(strings: &[&str]) -> String {
        strings.concat()
    }

    /// Slice string with support for negative indices
    /// Negative indices count from the end (-1 is last character)
    pub fn slice_with_negative(s: &str, start: i64, end: i64) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len() as i64;

        let actual_start = if start < 0 {
            (len + start).max(0) as usize
        } else {
            (start.min(len)) as usize
        };

        let actual_end = if end < 0 {
            (len + end).max(0) as usize
        } else {
            (end.min(len)) as usize
        };

        if actual_start >= actual_end {
            String::new()
        } else {
            chars[actual_start..actual_end].iter().collect()
        }
    }

    /// Insert string at index
    pub fn insert_at(s: &str, index: usize, insert: &str) -> String {
        let chars: Vec<char> = s.chars().collect();
        if index >= chars.len() {
            format!("{}{}", s, insert)
        } else {
            let mut result = String::new();
            result.push_str(&chars[..index].iter().collect::<String>());
            result.push_str(insert);
            result.push_str(&chars[index..].iter().collect::<String>());
            result
        }
    }

    /// Remove character at index
    pub fn remove_at(s: &str, index: usize) -> String {
        let chars: Vec<char> = s.chars().collect();
        if index >= chars.len() {
            s.to_string()
        } else {
            chars
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, c)| c)
                .collect()
        }
    }

    /// Count substring occurrences
    pub fn count_substring(s: &str, pattern: &str) -> usize {
        if pattern.is_empty() {
            return 0;
        }
        s.matches(pattern).count()
    }

    /// Truncate string to max length with optional suffix
    pub fn truncate(s: &str, max_length: usize, suffix: &str) -> String {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() <= max_length {
            s.to_string()
        } else {
            let truncate_at = max_length.saturating_sub(suffix.len());
            let mut result: String = chars[..truncate_at].iter().collect();
            result.push_str(suffix);
            result
        }
    }

    /// Wrap text to specified line width
    pub fn wrap_text(s: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![s.to_string()];
        }

        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_len = 0;

        for word in s.split_whitespace() {
            let word_len = word.chars().count();
            if current_len + word_len + 1 > width && !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
                current_len = 0;
            }

            if !current_line.is_empty() {
                current_line.push(' ');
                current_len += 1;
            }

            current_line.push_str(word);
            current_len += word_len;
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
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

    #[test]
    fn test_last_index_of() {
        assert_eq!(StringBuiltins::last_index_of("hello hello", "hello"), 6);
        assert_eq!(StringBuiltins::last_index_of("test", "xyz"), -1);
    }

    #[test]
    fn test_trim_variants() {
        assert_eq!(StringBuiltins::trim_start("  hello  "), "hello  ");
        assert_eq!(StringBuiltins::trim_end("  hello  "), "  hello");
    }

    #[test]
    fn test_char_at() {
        assert_eq!(StringBuiltins::char_at("hello", 1), Some("e".to_string()));
        assert_eq!(StringBuiltins::char_at("hello", 10), None);
    }

    #[test]
    fn test_concat() {
        assert_eq!(
            StringBuiltins::concat(&["Hello", " ", "World"]),
            "Hello World"
        );
    }

    #[test]
    fn test_slice_with_negative() {
        assert_eq!(StringBuiltins::slice_with_negative("hello", 0, -1), "hell");
        assert_eq!(StringBuiltins::slice_with_negative("hello", -3, -1), "ll");
        assert_eq!(StringBuiltins::slice_with_negative("hello", 1, 4), "ell");
    }

    #[test]
    fn test_insert_at() {
        assert_eq!(
            StringBuiltins::insert_at("hello", 5, " world"),
            "hello world"
        );
        assert_eq!(StringBuiltins::insert_at("test", 2, "XX"), "teXXst");
    }

    #[test]
    fn test_remove_at() {
        assert_eq!(StringBuiltins::remove_at("hello", 1), "hllo");
        assert_eq!(StringBuiltins::remove_at("test", 0), "est");
    }

    #[test]
    fn test_count_substring() {
        assert_eq!(StringBuiltins::count_substring("banana", "na"), 2);
        assert_eq!(StringBuiltins::count_substring("test", "xyz"), 0);
    }

    #[test]
    fn test_truncate() {
        assert_eq!(
            StringBuiltins::truncate("Hello World", 8, "..."),
            "Hello..."
        );
        assert_eq!(StringBuiltins::truncate("Hi", 10, "..."), "Hi");
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a long line that needs to be wrapped";
        let lines = StringBuiltins::wrap_text(text, 20);
        assert!(lines.iter().all(|line| line.chars().count() <= 20));
    }
}
