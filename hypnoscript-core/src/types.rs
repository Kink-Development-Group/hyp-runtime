use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Base types in HypnoScript language
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HypnoBaseType {
    Number,
    String,
    Boolean,
    Trance,
    Array,
    Object,
    Function,
    Session,
    Record,
    /// The type of the `null` literal. Only compatible with nullable types.
    Null,
    Unknown,
}

/// Represents a type in the HypnoScript type system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HypnoType {
    pub base_type: HypnoBaseType,
    pub name: Option<String>,
    pub element_type: Option<Box<HypnoType>>,
    pub fields: Option<HashMap<String, HypnoType>>,
    pub parameter_types: Option<Vec<HypnoType>>,
    pub return_type: Option<Box<HypnoType>>,
    /// Whether the type admits `null` (declared with a `?` suffix or the
    /// `lucid` modifier, e.g. `number?` / `lucid number`).
    #[serde(default)]
    pub nullable: bool,
}

impl HypnoType {
    /// Create a new simple type
    pub fn new(base_type: HypnoBaseType, name: Option<String>) -> Self {
        Self {
            base_type,
            name,
            element_type: None,
            fields: None,
            parameter_types: None,
            return_type: None,
            nullable: false,
        }
    }

    /// Create an array type
    pub fn create_array(element_type: HypnoType) -> Self {
        Self {
            element_type: Some(Box::new(element_type)),
            ..Self::new(HypnoBaseType::Array, None)
        }
    }

    /// Create a record type
    pub fn create_record(name: String, fields: HashMap<String, HypnoType>) -> Self {
        Self {
            fields: Some(fields),
            ..Self::new(HypnoBaseType::Record, Some(name))
        }
    }

    /// Create a function type
    pub fn create_function(parameter_types: Vec<HypnoType>, return_type: HypnoType) -> Self {
        Self {
            parameter_types: Some(parameter_types),
            return_type: Some(Box::new(return_type)),
            ..Self::new(HypnoBaseType::Function, None)
        }
    }

    /// Predefined type constants
    pub fn number() -> Self {
        Self::new(HypnoBaseType::Number, None)
    }

    pub fn string() -> Self {
        Self::new(HypnoBaseType::String, None)
    }

    pub fn boolean() -> Self {
        Self::new(HypnoBaseType::Boolean, None)
    }

    pub fn unknown() -> Self {
        Self::new(HypnoBaseType::Unknown, None)
    }

    /// The type of the `null` literal.
    pub fn null() -> Self {
        Self {
            nullable: true,
            ..Self::new(HypnoBaseType::Null, None)
        }
    }

    /// Return this type marked as nullable (`T?` / `lucid T`).
    pub fn into_nullable(mut self) -> Self {
        self.nullable = true;
        self
    }

    /// Return this type with the nullable marker removed. Used when an
    /// operation (e.g. `lucidFallback`) guarantees a non-null result.
    pub fn into_non_nullable(mut self) -> Self {
        self.nullable = false;
        self
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable || self.base_type == HypnoBaseType::Null
    }

    /// Type checking predicates
    pub fn is_array(&self) -> bool {
        self.base_type == HypnoBaseType::Array
    }

    pub fn is_record(&self) -> bool {
        self.base_type == HypnoBaseType::Record
    }

    pub fn is_function(&self) -> bool {
        self.base_type == HypnoBaseType::Function
    }

    pub fn is_primitive(&self) -> bool {
        matches!(
            self.base_type,
            HypnoBaseType::Number | HypnoBaseType::String | HypnoBaseType::Boolean
        )
    }

    /// Check if this type is compatible with another type.
    ///
    /// `self` is the expected type, `other` the actual one. A `null` value
    /// only fits nullable types; a nullable actual value only fits nullable
    /// expected types (it might be null at runtime).
    pub fn is_compatible_with(&self, other: &HypnoType) -> bool {
        if other.base_type == HypnoBaseType::Null {
            return self.is_nullable();
        }

        if other.nullable && !self.nullable {
            return false;
        }

        if self.base_type != other.base_type {
            return false;
        }

        match self.base_type {
            HypnoBaseType::Array => {
                if let (Some(elem1), Some(elem2)) = (&self.element_type, &other.element_type) {
                    elem1.is_compatible_with(elem2)
                } else {
                    false
                }
            }
            HypnoBaseType::Record => {
                if let (Some(fields1), Some(fields2)) = (&self.fields, &other.fields) {
                    if fields1.len() != fields2.len() {
                        return false;
                    }
                    fields1.iter().all(|(key, value)| {
                        fields2
                            .get(key)
                            .is_some_and(|v| value.is_compatible_with(v))
                    })
                } else {
                    false
                }
            }
            HypnoBaseType::Function => {
                if let (Some(params1), Some(params2)) =
                    (&self.parameter_types, &other.parameter_types)
                {
                    if params1.len() != params2.len() {
                        return false;
                    }
                    let params_match = params1
                        .iter()
                        .zip(params2.iter())
                        .all(|(p1, p2)| p1.is_compatible_with(p2));

                    let return_match = match (&self.return_type, &other.return_type) {
                        (Some(ret1), Some(ret2)) => ret1.is_compatible_with(ret2),
                        (None, None) => true,
                        _ => false,
                    };

                    params_match && return_match
                } else {
                    false
                }
            }
            _ => true,
        }
    }
}

impl fmt::Display for HypnoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_base(f)?;
        if self.nullable && self.base_type != HypnoBaseType::Null {
            write!(f, "?")?;
        }
        Ok(())
    }
}

impl HypnoType {
    fn fmt_base(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.base_type {
            HypnoBaseType::Null => write!(f, "null"),
            HypnoBaseType::Array => {
                if let Some(ref elem) = self.element_type {
                    write!(f, "[{}]", elem)
                } else {
                    write!(f, "Array")
                }
            }
            HypnoBaseType::Record => {
                if let Some(ref name) = self.name {
                    write!(f, "Record<{}>", name)
                } else {
                    write!(f, "Record")
                }
            }
            HypnoBaseType::Function => {
                let params = self
                    .parameter_types
                    .as_ref()
                    .map(|p| {
                        p.iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    })
                    .unwrap_or_default();
                let ret = self
                    .return_type
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "void".to_string());
                write!(f, "Function<{} -> {}>", params, ret)
            }
            _ => {
                if let Some(ref name) = self.name {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{:?}", self.base_type)
                }
            }
        }
    }
}

impl std::hash::Hash for HypnoType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base_type.hash(state);
        self.name.hash(state);
        // Note: We don't hash all fields for simplicity
        // This is a reasonable compromise for the type system
    }
}

impl Eq for HypnoType {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_only_fits_nullable_types() {
        let null_type = HypnoType::null();
        assert!(!HypnoType::number().is_compatible_with(&null_type));
        assert!(
            HypnoType::number()
                .into_nullable()
                .is_compatible_with(&null_type)
        );
    }

    #[test]
    fn test_nullable_actual_needs_nullable_expected() {
        let nullable_number = HypnoType::number().into_nullable();
        assert!(!HypnoType::number().is_compatible_with(&nullable_number));
        assert!(
            HypnoType::number()
                .into_nullable()
                .is_compatible_with(&nullable_number)
        );
        // Non-nullable values still fit nullable slots.
        assert!(
            HypnoType::number()
                .into_nullable()
                .is_compatible_with(&HypnoType::number())
        );
    }

    #[test]
    fn test_nullable_display() {
        assert_eq!(HypnoType::number().into_nullable().to_string(), "Number?");
        assert_eq!(HypnoType::null().to_string(), "null");
        assert_eq!(
            HypnoType::create_array(HypnoType::string()).to_string(),
            "[String]"
        );
    }

    #[test]
    fn test_non_nullable_strips_marker() {
        let t = HypnoType::string().into_nullable().into_non_nullable();
        assert!(!t.is_nullable());
    }
}
