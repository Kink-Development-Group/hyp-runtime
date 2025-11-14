use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a locale identifier (e.g., `en`, `de-DE`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locale(String);

impl Locale {
    /// Creates a new locale from any string.
    pub fn new<S: Into<String>>(code: S) -> Self {
        let mut code = code.into();
        if code.is_empty() {
            code = "en".to_string();
        }
        Self(code)
    }

    /// Returns the normalized (lowercase) locale code.
    pub fn code(&self) -> &str {
        &self.0
    }

    /// Returns the primary language portion (before `-`).
    pub fn language(&self) -> &str {
        self.0
            .split(|c| c == '-' || c == '_')
            .next()
            .unwrap_or("en")
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::new("en")
    }
}

impl<S: Into<String>> From<S> for Locale {
    fn from(value: S) -> Self {
        Self::new(value)
    }
}

/// Lightweight localized message helper storing translations per locale code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedMessage {
    fallback: String,
    translations: HashMap<String, String>,
}

impl LocalizedMessage {
    /// Creates a message with the provided fallback text.
    pub fn new<S: Into<String>>(fallback: S) -> Self {
        Self {
            fallback: fallback.into(),
            translations: HashMap::new(),
        }
    }

    /// Adds/overrides a translation for the locale code.
    pub fn with_translation<L: Into<String>, T: Into<String>>(
        mut self,
        locale: L,
        text: T,
    ) -> Self {
        self.translations
            .insert(locale.into().to_lowercase(), text.into());
        self
    }

    /// Resolves the best translation for the requested locale.
    pub fn resolve<'a>(&'a self, locale: &'a Locale) -> Cow<'a, str> {
        if let Some(value) = self
            .translations
            .get(&locale.code().to_lowercase())
            .or_else(|| self.translations.get(locale.language()))
        {
            Cow::Borrowed(value)
        } else {
            Cow::Borrowed(&self.fallback)
        }
    }
}

/// Utility to convert an optional locale string into a [`Locale`].
pub fn detect_locale(code: Option<&str>) -> Locale {
    code.map(Locale::from).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_defaults_to_en() {
        let locale = detect_locale(None);
        assert_eq!(locale.code(), "en");
        assert_eq!(locale.language(), "en");
    }

    #[test]
    fn localized_message_resolves_translation() {
        let locale = Locale::from("de-DE");
        let message = LocalizedMessage::new("Continue?")
            .with_translation("de", "Weiter?")
            .with_translation("en", "Continue?");

        assert_eq!(message.resolve(&locale), Cow::Borrowed("Weiter?"));
    }
}
