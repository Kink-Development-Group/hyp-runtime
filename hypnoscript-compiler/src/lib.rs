//! HypnoScript Compiler and Interpreter
//!
//! Dieses Modul stellt die vollständige Compiler-Infrastruktur und den Interpreter
//! für HypnoScript bereit.
//!
//! ## Module
//!
//! - **interpreter**: Interpretiert HypnoScript-Code direkt
//! - **type_checker**: Statische Typprüfung
//! - **wasm_codegen**: Generiert WebAssembly Text Format (.wat)
//! - **wasm_binary**: Generiert WebAssembly Binary Format (.wasm)
//! - **native_codegen**: Generiert plattformspezifischen nativen Code
//! - **optimizer**: Code-Optimierungen (Constant Folding, Dead Code Elimination, etc.)
//!
//! ## Verwendung
//!
//! ```rust,no_run
//! use hypnoscript_compiler::{Interpreter, TypeChecker, WasmCodeGenerator};
//! use hypnoscript_lexer_parser::{Lexer, Parser};
//!
//! let source = "Focus { observe 42; } Relax";
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.lex().unwrap();
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse_program().unwrap();
//!
//! // Interpretation
//! let mut interpreter = Interpreter::new();
//! // interpreter.interpret(&ast).unwrap();
//!
//! // Type Checking
//! let mut checker = TypeChecker::new();
//! // checker.check(&ast);
//!
//! // WASM Generation
//! let mut wasm_gen = WasmCodeGenerator::new();
//! // let wasm = wasm_gen.generate(&ast);
//! ```

pub mod interpreter;
pub mod native_codegen;
pub mod optimizer;
pub mod type_checker;
pub mod wasm_binary;
pub mod wasm_codegen;

// Re-export commonly used types
pub use interpreter::{Interpreter, InterpreterError, Value};
pub use native_codegen::{NativeCodeGenerator, NativeCodegenError, OptimizationLevel, TargetPlatform};
pub use optimizer::{Optimizer, OptimizationConfig, OptimizationError, OptimizationStats};
pub use type_checker::TypeChecker;
pub use wasm_binary::{WasmBinaryGenerator, WasmBinaryError};
pub use wasm_codegen::WasmCodeGenerator;
