use crate::types::HypnoType;
use serde::{Deserialize, Serialize};

/// Kind of symbol in the symbol table
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    Variable,
    Function,
    Session,
    Record,
    Parameter,
    Label,
    Builtin,
    Module,
}

/// Represents a symbol in the HypnoScript symbol table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub type_name: Option<String>,
    pub kind: SymbolKind,
    pub hypno_type: Option<HypnoType>,
    pub is_constant: bool,
    pub is_exported: bool,
    pub documentation: Option<String>,
    pub line_number: usize,
    pub column_number: usize,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(name: String, type_name: Option<String>, kind: SymbolKind) -> Self {
        Self {
            name,
            type_name,
            kind,
            hypno_type: None,
            is_constant: false,
            is_exported: false,
            documentation: None,
            line_number: 0,
            column_number: 0,
        }
    }

    /// Create a new symbol with type
    pub fn with_type(name: String, hypno_type: HypnoType, kind: SymbolKind) -> Self {
        Self {
            name,
            type_name: None,
            kind,
            hypno_type: Some(hypno_type),
            is_constant: false,
            is_exported: false,
            documentation: None,
            line_number: 0,
            column_number: 0,
        }
    }

    /// Factory method for creating a variable
    pub fn create_variable(name: String, type_name: String) -> Self {
        Self::new(name, Some(type_name), SymbolKind::Variable)
    }

    /// Factory method for creating a function
    pub fn create_function(
        name: String,
        return_type: String,
        documentation: Option<String>,
    ) -> Self {
        let mut sym = Self::new(name, Some(return_type), SymbolKind::Function);
        sym.documentation = documentation;
        sym
    }

    /// Factory method for creating a session
    pub fn create_session(name: String, documentation: Option<String>) -> Self {
        let mut sym = Self::new(name, Some("session".to_string()), SymbolKind::Session);
        sym.documentation = documentation;
        sym
    }

    /// Factory method for creating a record
    pub fn create_record(name: String, documentation: Option<String>) -> Self {
        let mut sym = Self::new(name, Some("record".to_string()), SymbolKind::Record);
        sym.documentation = documentation;
        sym
    }

    /// Factory method for creating a builtin
    pub fn create_builtin(
        name: String,
        return_type: String,
        documentation: Option<String>,
    ) -> Self {
        let mut sym = Self::new(name, Some(return_type), SymbolKind::Builtin);
        sym.documentation = documentation;
        sym
    }

    /// Factory method for creating a label
    pub fn create_label(name: String) -> Self {
        Self::new(name, None, SymbolKind::Label)
    }

    /// Check if symbol is a function
    pub fn is_function(&self) -> bool {
        matches!(self.kind, SymbolKind::Function | SymbolKind::Builtin)
    }

    /// Check if symbol is a type
    pub fn is_type(&self) -> bool {
        matches!(self.kind, SymbolKind::Session | SymbolKind::Record)
    }

    /// Check if symbol is a variable
    pub fn is_variable(&self) -> bool {
        matches!(self.kind, SymbolKind::Variable | SymbolKind::Parameter)
    }

    /// Get full description of the symbol
    pub fn get_full_description(&self) -> String {
        let mut result = format!("{:?} '{}'", self.kind, self.name);

        if let Some(ref t) = self.hypno_type {
            result.push_str(&format!(" of type {}", t));
        } else if let Some(ref tn) = self.type_name {
            result.push_str(&format!(" of type {}", tn));
        }

        if self.is_constant {
            result.push_str(" (constant)");
        }
        if self.is_exported {
            result.push_str(" (exported)");
        }
        if let Some(ref doc) = self.documentation {
            result.push_str(&format!(" - {}", doc));
        }

        result
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_info = self
            .hypno_type
            .as_ref()
            .map(|t| t.to_string())
            .or_else(|| self.type_name.clone())
            .unwrap_or_else(|| "unknown".to_string());
        let kind_info = format!("{:?}", self.kind).to_lowercase();
        write!(f, "{} {}: {}", kind_info, self.name, type_info)
    }
}
