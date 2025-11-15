//! Base trait and utilities for HypnoScript builtin functions.
//!
//! This module provides a common interface and shared utilities for all builtin
//! function modules in HypnoScript, promoting DRY (Don't Repeat Yourself) principles
//! and consistent error handling across the runtime.

use crate::localization::LocalizedMessage;

/// Common trait for all builtin function modules.
///
/// This trait provides metadata and common functionality that all builtin
/// modules should implement for consistency and discoverability.
pub trait BuiltinModule {
    /// Returns the name of this builtin module (e.g., "String", "Math", "Array").
    fn module_name() -> &'static str;

    /// Returns a brief description of this module's purpose.
    ///
    /// # Returns
    /// A description in English (default). Localized versions can be provided
    /// via the `description_localized` method.
    fn description() -> &'static str;

    /// Returns a localized description of this module.
    ///
    /// # Arguments
    /// * `locale` - Optional locale hint. If `None`, uses system default.
    ///
    /// # Returns
    /// Localized module description.
    fn description_localized(locale: Option<&str>) -> String {
        let _ = locale;
        Self::description().to_string()
    }

    /// Returns the list of function names provided by this module.
    fn function_names() -> &'static [&'static str];

    /// Returns the version of this module (for documentation/compatibility).
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

/// Common error type for builtin operations.
///
/// This error type provides i18n support and can be used across all builtin modules.
#[derive(Debug, Clone)]
pub struct BuiltinError {
    /// The error category (e.g., "validation", "io", "math").
    pub category: &'static str,
    /// The error message key for localization.
    pub message_key: String,
    /// Additional context for error formatting.
    pub context: Vec<String>,
}

impl BuiltinError {
    /// Creates a new builtin error.
    ///
    /// # Arguments
    /// * `category` - Error category (e.g., "validation", "io").
    /// * `message_key` - Message key for localization.
    /// * `context` - Additional context values for message formatting.
    pub fn new(
        category: &'static str,
        message_key: impl Into<String>,
        context: Vec<String>,
    ) -> Self {
        Self {
            category,
            message_key: message_key.into(),
            context,
        }
    }

    /// Returns a localized error message.
    ///
    /// # Arguments
    /// * `locale` - Optional locale for message localization.
    ///
    /// # Returns
    /// Formatted, localized error message.
    pub fn to_localized_string(&self, locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);

        // Build localized message based on category and key
        let base_msg = match (self.category, self.message_key.as_str()) {
            ("validation", "invalid_email") => LocalizedMessage::new("Invalid email address")
                .with_translation("de", "Ungültige E-Mail-Adresse")
                .with_translation("fr", "Adresse e-mail invalide")
                .with_translation("es", "Dirección de correo electrónico no válida"),
            ("validation", "invalid_url") => LocalizedMessage::new("Invalid URL")
                .with_translation("de", "Ungültige URL")
                .with_translation("fr", "URL invalide")
                .with_translation("es", "URL no válida"),
            ("io", "file_not_found") => LocalizedMessage::new("File not found: {}")
                .with_translation("de", "Datei nicht gefunden: {}")
                .with_translation("fr", "Fichier introuvable : {}")
                .with_translation("es", "Archivo no encontrado: {}"),
            ("math", "division_by_zero") => LocalizedMessage::new("Division by zero")
                .with_translation("de", "Division durch Null")
                .with_translation("fr", "Division par zéro")
                .with_translation("es", "División por cero"),
            ("array", "index_out_of_bounds") => LocalizedMessage::new("Index out of bounds: {}")
                .with_translation("de", "Index außerhalb des gültigen Bereichs: {}")
                .with_translation("fr", "Index hors limites : {}")
                .with_translation("es", "Índice fuera de límites: {}"),
            _ => LocalizedMessage::new(format!("Error in {}: {}", self.category, self.message_key)),
        };

        let mut msg = base_msg.resolve(&locale).to_string();

        // Replace placeholders with context values
        for (i, ctx) in self.context.iter().enumerate() {
            msg = msg.replace("{}", ctx);
            if i == 0 {
                break; // Only replace first occurrence for simplicity
            }
        }

        msg
    }
}

impl std::fmt::Display for BuiltinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_localized_string(None))
    }
}

impl std::error::Error for BuiltinError {}

/// Result type commonly used in builtin operations.
pub type BuiltinResult<T> = Result<T, BuiltinError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_error_localization() {
        let err = BuiltinError::new("validation", "invalid_email", vec![]);

        let en_msg = err.to_localized_string(Some("en"));
        assert_eq!(en_msg, "Invalid email address");

        let de_msg = err.to_localized_string(Some("de"));
        assert_eq!(de_msg, "Ungültige E-Mail-Adresse");
    }

    #[test]
    fn test_builtin_error_with_context() {
        let err = BuiltinError::new("io", "file_not_found", vec!["test.txt".to_string()]);

        let en_msg = err.to_localized_string(Some("en"));
        assert_eq!(en_msg, "File not found: test.txt");

        let de_msg = err.to_localized_string(Some("de"));
        assert_eq!(de_msg, "Datei nicht gefunden: test.txt");
    }
}
