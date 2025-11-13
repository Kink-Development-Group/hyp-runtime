//! HypnoScript Core Library
//!
//! This module provides the core types and data structures for the HypnoScript language,
//! including the type system, symbols, and symbol tables.

pub mod symbol_table;
pub mod symbols;
pub mod types;

// Re-export commonly used types
pub use symbol_table::SymbolTable;
pub use symbols::{Symbol, SymbolKind};
pub use types::{HypnoBaseType, HypnoType};
