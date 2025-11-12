//! HypnoScript Compiler and Interpreter
//! 
//! This module provides the compiler infrastructure and interpreter for HypnoScript.

pub mod interpreter;
pub mod type_checker;
pub mod wasm_codegen;

// Re-export commonly used types
pub use interpreter::{Interpreter, Value, InterpreterError};
pub use type_checker::TypeChecker;
pub use wasm_codegen::WasmCodeGenerator;
