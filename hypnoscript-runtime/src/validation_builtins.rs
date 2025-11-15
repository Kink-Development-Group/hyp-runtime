use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;
use regex::Regex;
use std::sync::OnceLock;

/// Validation builtin functions
///
/// Provides data validation functions for common formats like email,
/// URL, phone numbers, and pattern matching.
pub struct ValidationBuiltins;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static URL_REGEX: OnceLock<Regex> = OnceLock::new();
static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();

impl BuiltinModule for ValidationBuiltins {
    fn module_name() -> &'static str {
        "Validation"
    }

    fn description() -> &'static str {
        "Data validation functions for emails, URLs, phone numbers, and patterns"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("Data validation functions for emails, URLs, phone numbers, and patterns")
            .with_translation("de", "Datenvalidierungsfunktionen für E-Mails, URLs, Telefonnummern und Muster")
            .with_translation("fr", "Fonctions de validation de données pour e-mails, URL, numéros de téléphone et motifs")
            .with_translation("es", "Funciones de validación de datos para correos electrónicos, URLs, números de teléfono y patrones");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "IsValidEmail",
            "IsValidUrl",
            "IsValidPhoneNumber",
            "IsAlphanumeric",
            "IsAlphabetic",
            "IsNumeric",
            "IsLowercase",
            "IsUppercase",
            "IsInRange",
            "MatchesPattern",
        ]
    }
}

impl ValidationBuiltins {
    /// Check if string is valid email
    pub fn is_valid_email(email: &str) -> bool {
        let regex = EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        });
        regex.is_match(email)
    }

    /// Check if string is valid URL
    pub fn is_valid_url(url: &str) -> bool {
        let regex = URL_REGEX.get_or_init(|| Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap());
        regex.is_match(url)
    }

    /// Check if string is valid phone number (simple format)
    pub fn is_valid_phone_number(phone: &str) -> bool {
        let regex = PHONE_REGEX.get_or_init(|| Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap());
        regex.is_match(&phone.replace(&['-', ' ', '(', ')'][..], ""))
    }

    /// Check if string is alphanumeric
    pub fn is_alphanumeric(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_alphanumeric())
    }

    /// Check if string is alphabetic
    pub fn is_alphabetic(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
    }

    /// Check if string is numeric
    pub fn is_numeric(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_numeric())
    }

    /// Check if string is lowercase
    pub fn is_lowercase(s: &str) -> bool {
        !s.is_empty()
            && s.chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_lowercase())
    }

    /// Check if string is uppercase
    pub fn is_uppercase(s: &str) -> bool {
        !s.is_empty()
            && s.chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
    }

    /// Check if number is in range
    pub fn is_in_range(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    /// Check if string matches pattern (regex)
    pub fn matches_pattern(text: &str, pattern: &str) -> bool {
        Regex::new(pattern)
            .map(|r| r.is_match(text))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_email() {
        assert!(ValidationBuiltins::is_valid_email("test@example.com"));
        assert!(ValidationBuiltins::is_valid_email("user.name@domain.co.uk"));
        assert!(!ValidationBuiltins::is_valid_email("invalid.email"));
        assert!(!ValidationBuiltins::is_valid_email("@example.com"));
    }

    #[test]
    fn test_is_valid_url() {
        assert!(ValidationBuiltins::is_valid_url("http://example.com"));
        assert!(ValidationBuiltins::is_valid_url(
            "https://www.example.com/path"
        ));
        assert!(!ValidationBuiltins::is_valid_url("not a url"));
        assert!(!ValidationBuiltins::is_valid_url("ftp://example.com"));
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(ValidationBuiltins::is_alphanumeric("abc123"));
        assert!(ValidationBuiltins::is_alphanumeric("ABC"));
        assert!(!ValidationBuiltins::is_alphanumeric("abc 123"));
        assert!(!ValidationBuiltins::is_alphanumeric(""));
    }

    #[test]
    fn test_is_in_range() {
        assert!(ValidationBuiltins::is_in_range(5.0, 1.0, 10.0));
        assert!(!ValidationBuiltins::is_in_range(15.0, 1.0, 10.0));
        assert!(ValidationBuiltins::is_in_range(1.0, 1.0, 10.0));
    }
}
