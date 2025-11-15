use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hashing, cryptography and utility builtin functions
///
/// This module provides various hashing algorithms, encoding functions,
/// and string utilities for HypnoScript.
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
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
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

    // --- Cryptographic Hash Functions ---

    /// SHA-256 hash
    /// Returns hex-encoded SHA-256 hash of the input string
    pub fn sha256(s: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// SHA-512 hash
    /// Returns hex-encoded SHA-512 hash of the input string
    pub fn sha512(s: &str) -> String {
        use sha2::{Digest, Sha512};
        let mut hasher = Sha512::new();
        hasher.update(s.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// MD5 hash
    /// Returns hex-encoded MD5 hash of the input string
    /// Note: MD5 is NOT cryptographically secure, use for checksums only
    pub fn md5(s: &str) -> String {
        let digest = md5::compute(s.as_bytes());
        format!("{:x}", digest)
    }

    // --- Encoding Functions ---

    /// Base64 encode
    /// Encodes a string to Base64
    pub fn base64_encode(s: &str) -> String {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.encode(s.as_bytes())
    }

    /// Base64 decode
    /// Decodes a Base64 string, returns Result
    pub fn base64_decode(s: &str) -> Result<String, String> {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD
            .decode(s.as_bytes())
            .map_err(|e| format!("Base64 decode error: {}", e))
            .and_then(|bytes| {
                String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {}", e))
            })
    }

    /// URL encode (percent encoding)
    /// Encodes a string for use in URLs
    pub fn url_encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }

    /// URL decode (percent decoding)
    /// Decodes a URL-encoded string
    pub fn url_decode(s: &str) -> Result<String, String> {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '%' => {
                    let hex: String = chars.by_ref().take(2).collect();
                    if hex.len() != 2 {
                        return Err("Invalid URL encoding".to_string());
                    }
                    let byte = u8::from_str_radix(&hex, 16)
                        .map_err(|_| "Invalid hex in URL encoding".to_string())?;
                    result.push(byte as char);
                }
                '+' => result.push(' '),
                _ => result.push(c),
            }
        }

        Ok(result)
    }

    /// Hex encode
    /// Converts bytes to hexadecimal string
    pub fn hex_encode(s: &str) -> String {
        s.as_bytes().iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Hex decode
    /// Converts hexadecimal string to bytes/string
    pub fn hex_decode(s: &str) -> Result<String, String> {
        if !s.len().is_multiple_of(2) {
            return Err("Hex string must have even length".to_string());
        }

        let bytes: Result<Vec<u8>, _> = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect();

        bytes
            .map_err(|e| format!("Hex decode error: {}", e))
            .and_then(|b| String::from_utf8(b).map_err(|e| format!("UTF-8 error: {}", e)))
    }

    // --- UUID Generation ---

    /// Generate a random UUID (version 4)
    /// Returns a new random UUID string
    pub fn uuid_v4() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Generate a UUID with custom seed (deterministic)
    /// Useful for testing or reproducible UUIDs
    pub fn uuid_from_seed(seed: u64) -> String {
        // Create a deterministic UUID from seed
        let bytes = seed.to_le_bytes();
        let mut uuid_bytes = [0u8; 16];
        for i in 0..16 {
            uuid_bytes[i] = bytes[i % 8];
        }
        uuid::Uuid::from_bytes(uuid_bytes).to_string()
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
        assert!(HashingBuiltins::is_palindrome(
            "A man a plan a canal Panama"
        ));
        assert!(!HashingBuiltins::is_palindrome("hello"));
    }

    #[test]
    fn test_count_occurrences() {
        assert_eq!(
            HashingBuiltins::count_occurrences("hello world hello", "hello"),
            2
        );
        assert_eq!(HashingBuiltins::count_occurrences("abcabc", "abc"), 2);
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(HashingBuiltins::reverse_words("hello world"), "world hello");
        assert_eq!(
            HashingBuiltins::reverse_words("one two three"),
            "three two one"
        );
    }

    #[test]
    fn test_title_case() {
        assert_eq!(HashingBuiltins::title_case("hello world"), "Hello World");
        assert_eq!(
            HashingBuiltins::title_case("the quick brown fox"),
            "The Quick Brown Fox"
        );
    }

    #[test]
    fn test_sha256() {
        let hash = HashingBuiltins::sha256("hello");
        // SHA-256 of "hello"
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha512() {
        let hash = HashingBuiltins::sha512("test");
        assert_eq!(hash.len(), 128); // SHA-512 produces 128 hex characters
    }

    #[test]
    fn test_md5() {
        let hash = HashingBuiltins::md5("hello");
        // MD5 of "hello"
        assert_eq!(hash, "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_base64() {
        let encoded = HashingBuiltins::base64_encode("Hello, World!");
        assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");

        let decoded = HashingBuiltins::base64_decode(&encoded).unwrap();
        assert_eq!(decoded, "Hello, World!");
    }

    #[test]
    fn test_url_encoding() {
        let encoded = HashingBuiltins::url_encode("hello world!");
        assert!(encoded.contains("+") || encoded.contains("%20"));

        let decoded = HashingBuiltins::url_decode(&encoded).unwrap();
        assert_eq!(decoded, "hello world!");
    }

    #[test]
    fn test_hex_encoding() {
        let encoded = HashingBuiltins::hex_encode("ABC");
        assert_eq!(encoded, "414243");

        let decoded = HashingBuiltins::hex_decode(&encoded).unwrap();
        assert_eq!(decoded, "ABC");
    }

    #[test]
    fn test_uuid() {
        let uuid1 = HashingBuiltins::uuid_v4();
        let uuid2 = HashingBuiltins::uuid_v4();
        assert_ne!(uuid1, uuid2); // Random UUIDs should differ
        assert_eq!(uuid1.len(), 36); // Standard UUID format

        // Deterministic UUID from seed
        let uuid_seed1 = HashingBuiltins::uuid_from_seed(12345);
        let uuid_seed2 = HashingBuiltins::uuid_from_seed(12345);
        assert_eq!(uuid_seed1, uuid_seed2); // Same seed = same UUID
    }
}
