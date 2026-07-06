//! HypnoScript Lexer and Parser Library
//!
//! This module provides the lexer and parser for the HypnoScript language.

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod source;
pub mod token;

// Re-export commonly used types
pub use error::SyntaxError;
pub use lexer::Lexer;
pub use parser::Parser;
pub use source::decode_source;
pub use token::{Token, TokenType};
