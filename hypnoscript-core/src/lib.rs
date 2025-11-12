//! HypnoScript Core Library
//! 
//! This module provides the core types and data structures for the HypnoScript language,
//! including the type system, symbols, and symbol tables.

pub mod types;
pub mod symbols;
pub mod symbol_table;

// Re-export commonly used types
pub use types::{HypnoBaseType, HypnoType};
pub use symbols::{Symbol, SymbolKind};
pub use symbol_table::SymbolTable;
