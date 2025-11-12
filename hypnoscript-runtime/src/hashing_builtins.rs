use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hashing and utility builtin functions
pub struct HashingBuiltins;

impl HashingBuiltins {
    /// Calculate simple hash of string
    pub fn hash_string(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    /// Calculate simple hash of number
    pub fn hash_number(n: f64) -> u64 {
        let mut hasher = DefaultHasher::new();
        n.to_bits().hash(&mut hasher);
        hasher.finish()
    }

    /// Generate a simple pseudo-random number (not cryptographically secure)
    pub fn simple_random(seed: u64) -> u64 {
        // Simple LCG (Linear Congruential Generator)
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;
        seed.wrapping_mul(A).wrapping_add(C)
    }

    /// Check if two strings are anagrams
    pub fn are_anagrams(s1: &str, s2: &str) -> bool {
        let mut chars1: Vec<char> = s1.chars().collect();
        let mut chars2: Vec<char> = s2.chars().collect();
        chars1.sort_unstable();
        chars2.sort_unstable();
        chars1 == chars2
    }

    /// Check if string is palindrome
    pub fn is_palindrome(s: &str) -> bool {
        let clean: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        let lower = clean.to_lowercase();
        lower == lower.chars().rev().collect::<String>()
    }

    /// Count occurrences of substring
    pub fn count_occurrences(text: &str, pattern: &str) -> usize {
        if pattern.is_empty() {
            return 0;
        }
        text.matches(pattern).count()
    }

    /// Remove duplicates from string
    pub fn remove_duplicates(s: &str) -> String {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        s.chars().filter(|c| seen.insert(*c)).collect()
    }

    /// Get unique characters in string
    pub fn unique_characters(s: &str) -> String {
        use std::collections::HashSet;
        let unique: HashSet<char> = s.chars().collect();
        unique.into_iter().collect()
    }

    /// Reverse words in string
    pub fn reverse_words(s: &str) -> String {
        s.split_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Title case (capitalize first letter of each word)
    pub fn title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let hash1 = HashingBuiltins::hash_string("hello");
        let hash2 = HashingBuiltins::hash_string("hello");
        let hash3 = HashingBuiltins::hash_string("world");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_are_anagrams() {
        assert!(HashingBuiltins::are_anagrams("listen", "silent"));
        assert!(HashingBuiltins::are_anagrams("evil", "vile"));
        assert!(!HashingBuiltins::are_anagrams("hello", "world"));
    }

    #[test]
    fn test_is_palindrome() {
        assert!(HashingBuiltins::is_palindrome("racecar"));
        assert!(HashingBuiltins::is_palindrome("A man a plan a canal Panama"));
        assert!(!HashingBuiltins::is_palindrome("hello"));
    }

    #[test]
    fn test_count_occurrences() {
        assert_eq!(HashingBuiltins::count_occurrences("hello world hello", "hello"), 2);
        assert_eq!(HashingBuiltins::count_occurrences("abcabc", "abc"), 2);
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(HashingBuiltins::reverse_words("hello world"), "world hello");
        assert_eq!(HashingBuiltins::reverse_words("one two three"), "three two one");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(HashingBuiltins::title_case("hello world"), "Hello World");
        assert_eq!(HashingBuiltins::title_case("the quick brown fox"), "The Quick Brown Fox");
    }
}
