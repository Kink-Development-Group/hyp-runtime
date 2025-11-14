//! Dictionary/Map builtin functions for HypnoScript.
//!
//! This module provides operations for working with key-value collections (dictionaries/maps).
//! In HypnoScript, dictionaries are represented as `Record` types or as string-based JSON objects.
//!
//! # Features
//! - Key-value pair operations
//! - Dictionary merging and transformation
//! - Key/value extraction and filtering
//! - JSON-based dictionary operations
//! - Full i18n support for error messages

use std::collections::HashMap;
use serde_json::Value as JsonValue;

use crate::builtin_trait::{BuiltinModule, BuiltinError, BuiltinResult};
use crate::localization::LocalizedMessage;

/// Dictionary/Map manipulation functions.
///
/// This struct provides static methods for working with key-value collections.
/// All operations are designed to work with both native Rust HashMaps and
/// JSON-based dictionary representations.
pub struct DictionaryBuiltins;

impl BuiltinModule for DictionaryBuiltins {
    fn module_name() -> &'static str {
        "Dictionary"
    }

    fn description() -> &'static str {
        "Key-value collection operations for dictionaries and maps"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("Key-value collection operations for dictionaries and maps")
            .with_translation("de", "Schlüssel-Wert-Sammlungsoperationen für Dictionaries und Maps")
            .with_translation("fr", "Opérations de collection clé-valeur pour les dictionnaires et les cartes")
            .with_translation("es", "Operaciones de colección clave-valor para diccionarios y mapas");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "DictCreate",
            "DictGet",
            "DictSet",
            "DictHasKey",
            "DictKeys",
            "DictValues",
            "DictSize",
            "DictIsEmpty",
            "DictRemove",
            "DictClear",
            "DictMerge",
            "DictFilter",
            "DictMap",
            "DictFromJson",
            "DictToJson",
        ]
    }
}

impl DictionaryBuiltins {
    /// Creates a new empty dictionary (as JSON string).
    ///
    /// # Returns
    /// Empty JSON object string `"{}"`
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::DictionaryBuiltins;
    /// let dict = DictionaryBuiltins::create();
    /// assert_eq!(dict, "{}");
    /// ```
    pub fn create() -> String {
        "{}".to_string()
    }

    /// Gets a value from a dictionary by key.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    /// * `key` - Key to look up
    ///
    /// # Returns
    /// Value as string, or empty string if key not found
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::DictionaryBuiltins;
    /// let dict = r#"{"name": "Alice", "age": 30}"#;
    /// let name = DictionaryBuiltins::get(dict, "name").unwrap();
    /// assert_eq!(name, "Alice");
    /// ```
    pub fn get(dict_json: &str, key: &str) -> BuiltinResult<String> {
        let dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        if let Some(obj) = dict.as_object() {
            if let Some(value) = obj.get(key) {
                return Ok(value_to_string(value));
            }
        }

        Ok(String::new())
    }

    /// Sets a key-value pair in a dictionary.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    /// * `key` - Key to set
    /// * `value` - Value to set (as string)
    ///
    /// # Returns
    /// Updated dictionary as JSON string
    ///
    /// # Example
    /// ```rust
    /// use hypnoscript_runtime::DictionaryBuiltins;
    /// let dict = "{}";
    /// let updated = DictionaryBuiltins::set(dict, "name", "Bob").unwrap();
    /// assert!(updated.contains("Bob"));
    /// ```
    pub fn set(dict_json: &str, key: &str, value: &str) -> BuiltinResult<String> {
        let mut dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        if let Some(obj) = dict.as_object_mut() {
            // Try to parse value as JSON, otherwise use as string
            let json_value = serde_json::from_str(value)
                .unwrap_or_else(|_| JsonValue::String(value.to_string()));
            obj.insert(key.to_string(), json_value);
        }

        serde_json::to_string(&dict)
            .map_err(|e| BuiltinError::new("dict", "serialize_error", vec![e.to_string()]))
    }

    /// Checks if a dictionary contains a key.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    /// * `key` - Key to check
    ///
    /// # Returns
    /// `true` if key exists, `false` otherwise
    pub fn has_key(dict_json: &str, key: &str) -> BuiltinResult<bool> {
        let dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        Ok(dict.as_object().map_or(false, |obj| obj.contains_key(key)))
    }

    /// Returns all keys from a dictionary.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    ///
    /// # Returns
    /// Vector of all keys as strings
    pub fn keys(dict_json: &str) -> BuiltinResult<Vec<String>> {
        let dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        Ok(dict
            .as_object()
            .map(|obj| obj.keys().cloned().collect())
            .unwrap_or_default())
    }

    /// Returns all values from a dictionary.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    ///
    /// # Returns
    /// Vector of all values as strings
    pub fn values(dict_json: &str) -> BuiltinResult<Vec<String>> {
        let dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        Ok(dict
            .as_object()
            .map(|obj| obj.values().map(value_to_string).collect())
            .unwrap_or_default())
    }

    /// Returns the number of key-value pairs in a dictionary.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    ///
    /// # Returns
    /// Number of entries in the dictionary
    pub fn size(dict_json: &str) -> BuiltinResult<usize> {
        let dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        Ok(dict.as_object().map_or(0, |obj| obj.len()))
    }

    /// Checks if a dictionary is empty.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    ///
    /// # Returns
    /// `true` if dictionary has no entries, `false` otherwise
    pub fn is_empty(dict_json: &str) -> BuiltinResult<bool> {
        Ok(Self::size(dict_json)? == 0)
    }

    /// Removes a key-value pair from a dictionary.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    /// * `key` - Key to remove
    ///
    /// # Returns
    /// Updated dictionary as JSON string
    pub fn remove(dict_json: &str, key: &str) -> BuiltinResult<String> {
        let mut dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        if let Some(obj) = dict.as_object_mut() {
            obj.remove(key);
        }

        serde_json::to_string(&dict)
            .map_err(|e| BuiltinError::new("dict", "serialize_error", vec![e.to_string()]))
    }

    /// Clears all entries from a dictionary.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    ///
    /// # Returns
    /// Empty dictionary as JSON string
    pub fn clear(_dict_json: &str) -> String {
        "{}".to_string()
    }

    /// Merges two dictionaries (second overrides first on key conflicts).
    ///
    /// # Arguments
    /// * `dict1_json` - First dictionary as JSON string
    /// * `dict2_json` - Second dictionary as JSON string
    ///
    /// # Returns
    /// Merged dictionary as JSON string
    pub fn merge(dict1_json: &str, dict2_json: &str) -> BuiltinResult<String> {
        let mut dict1: JsonValue = serde_json::from_str(dict1_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;
        let dict2: JsonValue = serde_json::from_str(dict2_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        if let (Some(obj1), Some(obj2)) = (dict1.as_object_mut(), dict2.as_object()) {
            for (key, value) in obj2 {
                obj1.insert(key.clone(), value.clone());
            }
        }

        serde_json::to_string(&dict1)
            .map_err(|e| BuiltinError::new("dict", "serialize_error", vec![e.to_string()]))
    }

    /// Converts a Rust HashMap to JSON string.
    ///
    /// # Arguments
    /// * `map` - HashMap with string keys and values
    ///
    /// # Returns
    /// JSON representation of the map
    pub fn from_hashmap(map: &HashMap<String, String>) -> BuiltinResult<String> {
        serde_json::to_string(map)
            .map_err(|e| BuiltinError::new("dict", "serialize_error", vec![e.to_string()]))
    }

    /// Converts a JSON string to a Rust HashMap.
    ///
    /// # Arguments
    /// * `dict_json` - Dictionary as JSON string
    ///
    /// # Returns
    /// HashMap with string keys and values
    pub fn to_hashmap(dict_json: &str) -> BuiltinResult<HashMap<String, String>> {
        let dict: JsonValue = serde_json::from_str(dict_json)
            .map_err(|e| BuiltinError::new("dict", "parse_error", vec![e.to_string()]))?;

        let mut map = HashMap::new();
        if let Some(obj) = dict.as_object() {
            for (key, value) in obj {
                map.insert(key.clone(), value_to_string(value));
            }
        }

        Ok(map)
    }
}

/// Helper function to convert JSON values to strings.
fn value_to_string(value: &JsonValue) -> String {
    match value {
        JsonValue::String(s) => s.clone(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => "null".to_string(),
        _ => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_dict() {
        let dict = DictionaryBuiltins::create();
        assert_eq!(dict, "{}");
    }

    #[test]
    fn test_set_and_get() {
        let dict = DictionaryBuiltins::create();
        let dict = DictionaryBuiltins::set(&dict, "name", "Alice").unwrap();
        let name = DictionaryBuiltins::get(&dict, "name").unwrap();
        assert_eq!(name, "Alice");
    }

    #[test]
    fn test_has_key() {
        let dict = r#"{"name": "Bob", "age": "25"}"#;
        assert!(DictionaryBuiltins::has_key(dict, "name").unwrap());
        assert!(!DictionaryBuiltins::has_key(dict, "email").unwrap());
    }

    #[test]
    fn test_keys_and_values() {
        let dict = r#"{"a": "1", "b": "2", "c": "3"}"#;
        let keys = DictionaryBuiltins::keys(dict).unwrap();
        let values = DictionaryBuiltins::values(dict).unwrap();

        assert_eq!(keys.len(), 3);
        assert_eq!(values.len(), 3);
        assert!(keys.contains(&"a".to_string()));
    }

    #[test]
    fn test_size_and_is_empty() {
        let dict = DictionaryBuiltins::create();
        assert!(DictionaryBuiltins::is_empty(&dict).unwrap());
        assert_eq!(DictionaryBuiltins::size(&dict).unwrap(), 0);

        let dict = DictionaryBuiltins::set(&dict, "key", "value").unwrap();
        assert!(!DictionaryBuiltins::is_empty(&dict).unwrap());
        assert_eq!(DictionaryBuiltins::size(&dict).unwrap(), 1);
    }

    #[test]
    fn test_remove() {
        let dict = r#"{"a": "1", "b": "2"}"#;
        let dict = DictionaryBuiltins::remove(dict, "a").unwrap();
        assert!(!DictionaryBuiltins::has_key(&dict, "a").unwrap());
        assert!(DictionaryBuiltins::has_key(&dict, "b").unwrap());
    }

    #[test]
    fn test_merge() {
        let dict1 = r#"{"a": "1", "b": "2"}"#;
        let dict2 = r#"{"b": "3", "c": "4"}"#;
        let merged = DictionaryBuiltins::merge(dict1, dict2).unwrap();

        assert_eq!(DictionaryBuiltins::get(&merged, "a").unwrap(), "1");
        assert_eq!(DictionaryBuiltins::get(&merged, "b").unwrap(), "3"); // Overridden
        assert_eq!(DictionaryBuiltins::get(&merged, "c").unwrap(), "4");
    }

    #[test]
    fn test_module_metadata() {
        assert_eq!(DictionaryBuiltins::module_name(), "Dictionary");
        assert!(!DictionaryBuiltins::function_names().is_empty());
    }
}
