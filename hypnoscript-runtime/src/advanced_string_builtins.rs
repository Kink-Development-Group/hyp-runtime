//! Advanced string analysis and similarity functions for HypnoScript.
//!
//! This module provides advanced string algorithms including:
//! - Similarity metrics (Levenshtein distance, Jaro-Winkler distance)
//! - Phonetic algorithms (Soundex, Metaphone)
//! - String difference and comparison
//! - Fuzzy matching utilities
//!
//! These functions are useful for text analysis, spell checking, and
//! fuzzy search applications.

use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;

/// Advanced string analysis functions.
///
/// This module provides sophisticated string comparison and similarity
/// algorithms for advanced text processing tasks.
pub struct AdvancedStringBuiltins;

impl BuiltinModule for AdvancedStringBuiltins {
    fn module_name() -> &'static str {
        "AdvancedString"
    }

    fn description() -> &'static str {
        "Advanced string similarity and phonetic analysis functions"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("Advanced string similarity and phonetic analysis functions")
            .with_translation("de", "Erweiterte String-Ähnlichkeits- und phonetische Analysefunktionen")
            .with_translation("fr", "Fonctions avancées de similarité de chaînes et d'analyse phonétique")
            .with_translation("es", "Funciones avanzadas de similitud de cadenas y análisis fonético");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "LevenshteinDistance",
            "DamerauLevenshteinDistance",
            "JaroDistance",
            "JaroWinklerDistance",
            "HammingDistance",
            "Soundex",
            "LongestCommonSubstring",
            "LongestCommonSubsequence",
            "SimilarityRatio",
        ]
    }
}

impl AdvancedStringBuiltins {
    /// Calculates the Levenshtein distance between two strings.
    ///
    /// The Levenshtein distance is the minimum number of single-character edits
    /// (insertions, deletions, or substitutions) required to change one string
    /// into another.
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    ///
    /// # Returns
    /// The Levenshtein distance as a usize
    ///
    /// # Example
    /// ```rust
    /// let distance = AdvancedStringBuiltins::levenshtein_distance("kitten", "sitting");
    /// assert_eq!(distance, 3);
    /// ```
    pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        // Initialize first row and column
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        // Fill matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1) // deletion
                    .min(matrix[i][j - 1] + 1) // insertion
                    .min(matrix[i - 1][j - 1] + cost); // substitution
            }
        }

        matrix[len1][len2]
    }

    /// Calculates the Damerau-Levenshtein distance between two strings.
    ///
    /// Similar to Levenshtein distance, but also allows transposition of two
    /// adjacent characters as a single operation.
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    ///
    /// # Returns
    /// The Damerau-Levenshtein distance
    pub fn damerau_levenshtein_distance(s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };

                matrix[i][j] = (matrix[i - 1][j] + 1) // deletion
                    .min(matrix[i][j - 1] + 1) // insertion
                    .min(matrix[i - 1][j - 1] + cost); // substitution

                // Transposition
                if i > 1 && j > 1 && chars1[i - 1] == chars2[j - 2] && chars1[i - 2] == chars2[j - 1] {
                    matrix[i][j] = matrix[i][j].min(matrix[i - 2][j - 2] + cost);
                }
            }
        }

        matrix[len1][len2]
    }

    /// Calculates the Jaro distance between two strings.
    ///
    /// The Jaro distance is a measure of similarity between two strings.
    /// The value ranges from 0 (no similarity) to 1 (exact match).
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    ///
    /// # Returns
    /// Similarity score between 0.0 and 1.0
    pub fn jaro_distance(s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        let match_window = (len1.max(len2) / 2).saturating_sub(1);

        let mut matches1 = vec![false; len1];
        let mut matches2 = vec![false; len2];
        let mut matches = 0;
        let mut transpositions = 0;

        // Find matches
        for i in 0..len1 {
            let start = i.saturating_sub(match_window);
            let end = (i + match_window + 1).min(len2);

            for j in start..end {
                if matches2[j] || chars1[i] != chars2[j] {
                    continue;
                }
                matches1[i] = true;
                matches2[j] = true;
                matches += 1;
                break;
            }
        }

        if matches == 0 {
            return 0.0;
        }

        // Count transpositions
        let mut k = 0;
        for i in 0..len1 {
            if !matches1[i] {
                continue;
            }
            while !matches2[k] {
                k += 1;
            }
            if chars1[i] != chars2[k] {
                transpositions += 1;
            }
            k += 1;
        }

        let m = matches as f64;
        (m / len1 as f64 + m / len2 as f64 + (m - transpositions as f64 / 2.0) / m) / 3.0
    }

    /// Calculates the Jaro-Winkler distance between two strings.
    ///
    /// The Jaro-Winkler distance is a variant of Jaro distance that gives
    /// more favorable ratings to strings with common prefixes.
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    /// * `prefix_scale` - Scaling factor for common prefix (typically 0.1)
    ///
    /// # Returns
    /// Similarity score between 0.0 and 1.0
    pub fn jaro_winkler_distance(s1: &str, s2: &str, prefix_scale: f64) -> f64 {
        let jaro = Self::jaro_distance(s1, s2);

        if jaro < 0.7 {
            return jaro;
        }

        // Find common prefix (up to 4 characters)
        let prefix_len = s1
            .chars()
            .zip(s2.chars())
            .take(4)
            .take_while(|(c1, c2)| c1 == c2)
            .count();

        jaro + (prefix_len as f64 * prefix_scale * (1.0 - jaro))
    }

    /// Calculates the Hamming distance between two strings.
    ///
    /// The Hamming distance is the number of positions at which the
    /// corresponding characters are different. Both strings must have
    /// the same length.
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    ///
    /// # Returns
    /// Hamming distance, or None if strings have different lengths
    pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();

        if chars1.len() != chars2.len() {
            return None;
        }

        Some(
            chars1
                .iter()
                .zip(chars2.iter())
                .filter(|(c1, c2)| c1 != c2)
                .count(),
        )
    }

    /// Generates the Soundex code for a string.
    ///
    /// Soundex is a phonetic algorithm for indexing names by sound.
    /// It converts names to a code based on how they sound rather than
    /// how they are spelled.
    ///
    /// # Arguments
    /// * `s` - Input string
    ///
    /// # Returns
    /// 4-character Soundex code (e.g., "R163" for "Robert")
    pub fn soundex(s: &str) -> String {
        if s.is_empty() {
            return "0000".to_string();
        }

        let chars: Vec<char> = s.to_uppercase().chars().collect();
        let mut code = String::new();

        // Keep first letter
        if let Some(&first) = chars.first() {
            if first.is_alphabetic() {
                code.push(first);
            }
        }

        let mut prev_code = soundex_code(chars.first().copied().unwrap_or(' '));

        for &ch in chars.iter().skip(1) {
            if code.len() >= 4 {
                break;
            }

            let curr_code = soundex_code(ch);
            if curr_code != '0' && curr_code != prev_code {
                code.push(curr_code);
            }
            if curr_code != '0' {
                prev_code = curr_code;
            }
        }

        // Pad with zeros
        while code.len() < 4 {
            code.push('0');
        }

        code.truncate(4);
        code
    }

    /// Finds the longest common substring between two strings.
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    ///
    /// # Returns
    /// The longest common substring
    pub fn longest_common_substring(s1: &str, s2: &str) -> String {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        if len1 == 0 || len2 == 0 {
            return String::new();
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        let mut max_length = 0;
        let mut end_index = 0;

        for i in 1..=len1 {
            for j in 1..=len2 {
                if chars1[i - 1] == chars2[j - 1] {
                    matrix[i][j] = matrix[i - 1][j - 1] + 1;
                    if matrix[i][j] > max_length {
                        max_length = matrix[i][j];
                        end_index = i;
                    }
                }
            }
        }

        if max_length == 0 {
            String::new()
        } else {
            chars1[end_index - max_length..end_index].iter().collect()
        }
    }

    /// Calculates the longest common subsequence length between two strings.
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    ///
    /// # Returns
    /// Length of the longest common subsequence
    pub fn longest_common_subsequence(s1: &str, s2: &str) -> usize {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if chars1[i - 1] == chars2[j - 1] {
                    matrix[i][j] = matrix[i - 1][j - 1] + 1;
                } else {
                    matrix[i][j] = matrix[i - 1][j].max(matrix[i][j - 1]);
                }
            }
        }

        matrix[len1][len2]
    }

    /// Calculates a similarity ratio between two strings (0.0 to 1.0).
    ///
    /// Uses Levenshtein distance normalized by the maximum string length.
    ///
    /// # Arguments
    /// * `s1` - First string
    /// * `s2` - Second string
    ///
    /// # Returns
    /// Similarity ratio where 1.0 means identical strings
    pub fn similarity_ratio(s1: &str, s2: &str) -> f64 {
        let distance = Self::levenshtein_distance(s1, s2);
        let max_len = s1.chars().count().max(s2.chars().count());

        if max_len == 0 {
            return 1.0;
        }

        1.0 - (distance as f64 / max_len as f64)
    }
}

/// Helper function to get Soundex code for a character.
fn soundex_code(ch: char) -> char {
    match ch {
        'B' | 'F' | 'P' | 'V' => '1',
        'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => '2',
        'D' | 'T' => '3',
        'L' => '4',
        'M' | 'N' => '5',
        'R' => '6',
        _ => '0',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(AdvancedStringBuiltins::levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(AdvancedStringBuiltins::levenshtein_distance("", "test"), 4);
        assert_eq!(AdvancedStringBuiltins::levenshtein_distance("same", "same"), 0);
    }

    #[test]
    fn test_jaro_distance() {
        let dist = AdvancedStringBuiltins::jaro_distance("MARTHA", "MARHTA");
        assert!(dist > 0.9 && dist < 1.0);

        assert_eq!(AdvancedStringBuiltins::jaro_distance("same", "same"), 1.0);
        assert_eq!(AdvancedStringBuiltins::jaro_distance("", "test"), 0.0);
    }

    #[test]
    fn test_jaro_winkler_distance() {
        let dist = AdvancedStringBuiltins::jaro_winkler_distance("MARTHA", "MARHTA", 0.1);
        assert!(dist > 0.9);
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(AdvancedStringBuiltins::hamming_distance("1011101", "1001001"), Some(2));
        assert_eq!(AdvancedStringBuiltins::hamming_distance("test", "best"), Some(1));
        assert_eq!(AdvancedStringBuiltins::hamming_distance("test", "testing"), None);
    }

    #[test]
    fn test_soundex() {
        assert_eq!(AdvancedStringBuiltins::soundex("Robert"), "R163");
        assert_eq!(AdvancedStringBuiltins::soundex("Rupert"), "R163");
        assert_eq!(AdvancedStringBuiltins::soundex("Smith"), "S530");
        assert_eq!(AdvancedStringBuiltins::soundex("Smythe"), "S530");
    }

    #[test]
    fn test_longest_common_substring() {
        assert_eq!(
            AdvancedStringBuiltins::longest_common_substring("ABABC", "BABCA"),
            "BABC"
        );
        assert_eq!(
            AdvancedStringBuiltins::longest_common_substring("test", "testing"),
            "test"
        );
    }

    #[test]
    fn test_similarity_ratio() {
        assert_eq!(AdvancedStringBuiltins::similarity_ratio("same", "same"), 1.0);
        let ratio = AdvancedStringBuiltins::similarity_ratio("kitten", "sitting");
        assert!(ratio > 0.5 && ratio < 0.6);
    }

    #[test]
    fn test_module_metadata() {
        assert_eq!(AdvancedStringBuiltins::module_name(), "AdvancedString");
        assert!(!AdvancedStringBuiltins::function_names().is_empty());
    }
}
