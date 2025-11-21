//! HypnoScript Compiler and Interpreter
//!
//! This module provides the complete compiler infrastructure and interpreter
//! for HypnoScript.
//!
//! ## Architecture
//!
//! The compiler supports multiple backends:
//!
//! ### 1. Interpreter (Runtime Execution)
//! - Direct execution of HypnoScript code
//! - Full language support including OOP (Sessions)
//! - Integrated built-in functions
//! - Ideal for development and debugging
//!
//! ### 2. Native Code Generator (Cranelift)
//! - Compiles to platform-specific machine code
//! - Supports Windows, macOS and Linux (x86_64, ARM64)
//! - Optimized binaries with Cranelift backend
//! - Faster alternative to LLVM
//!
//! ### 3. WASM Generator
//! - **Text Format (.wat)**: Human-readable WebAssembly
//! - **Binary Format (.wasm)**: Compact binary WebAssembly
//! - Browser and server support
//! - Sandboxed Execution
//!
//! ## Modules
//!
//! - **interpreter**: Interprets HypnoScript code directly
//! - **type_checker**: Static type checking before execution
//! - **optimizer**: Code optimizations (Constant Folding, Dead Code Elimination, etc.)
//! - **native_codegen**: Generates platform-specific native code with Cranelift
//! - **wasm_codegen**: Generates WebAssembly Text Format (.wat)
//! - **wasm_binary**: Generates WebAssembly Binary Format (.wasm)
//!
//! ## Design Principles
//!
//! ### OOP (Object-Oriented Programming)
//! - Sessions as class constructs
//! - Encapsulation and visibility modifiers
//! - Static and instance methods/fields
//!
//! ### DRY (Don't Repeat Yourself)
//! - Common infrastructure in `hypnoscript-core`
//! - Reusable type systems and symbol tables
//! - Shared traits for built-in functions
//!
//! ### Documentation
//! - Comprehensive Rustdoc comments
//! - Examples for each module
//! - Unit tests as living documentation
//!
//! ## Usage
//!
//! ### Example: Interpretation
//!
//! ```rust,no_run
//! use hypnoscript_compiler::Interpreter;
//! use hypnoscript_lexer_parser::{Lexer, Parser};
//!
//! let source = r#"
//! Focus {
//!     induce x: number = 42;
//!     observe x;
//! } Relax
//! "#;
//!
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.lex().unwrap();
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse_program().unwrap();
//!
//! let mut interpreter = Interpreter::new();
//! // interpreter.interpret(&ast).unwrap();
//! ```
//!
//! ### Example: Native Compilation
//!
//! ```rust,no_run
//! use hypnoscript_compiler::{NativeCodeGenerator, OptimizationLevel, TargetPlatform};
//! use hypnoscript_lexer_parser::{Lexer, Parser};
//!
//! let source = "Focus { induce x: number = 42; } Relax";
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.lex().unwrap();
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse_program().unwrap();
//!
//! let mut generator = NativeCodeGenerator::new();
//! generator.set_target_platform(TargetPlatform::LinuxX64);
//! generator.set_optimization_level(OptimizationLevel::Release);
//!
//! // let binary_path = generator.generate(&ast).unwrap();
//! ```
//!
//! ### Example: WASM Generation
//!
//! ```rust,no_run
//! use hypnoscript_compiler::{WasmCodeGenerator, WasmBinaryGenerator};
//! use hypnoscript_lexer_parser::{Lexer, Parser};
//! use std::fs;
//!
//! let source = "Focus { induce x: number = 42; } Relax";
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.lex().unwrap();
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse_program().unwrap();
//!
//! // Text Format (.wat)
//! let mut wat_gen = WasmCodeGenerator::new();
//! let wat_code = wat_gen.generate(&ast);
//! // fs::write("output.wat", wat_code).unwrap();
//!
//! // Binary Format (.wasm)
//! let mut wasm_gen = WasmBinaryGenerator::new();
//! // let wasm_bytes = wasm_gen.generate(&ast).unwrap();
//! // fs::write("output.wasm", wasm_bytes).unwrap();
//! ```
//!
//! ## Performance Comparison
//!
//! | Backend | Compile Time | Execution Time | Binary Size | Use Case |
//! |---------|--------------|----------------|-------------|----------|
//! | Interpreter | Instant | Slow | N/A | Development, Debugging |
//! | Native (Cranelift) | Fast | Very Fast | Small | Production, Server |
//! | WASM | Fast | Fast | Very Small | Web, Embedding |
//!
//! ## Security
//!
//! - Memory-safe through Rust
//! - Type-checked before execution
//! - WASM: Sandboxed Execution
//! - Native: Optimized, safe code generation

pub mod async_builtins;
pub mod async_promise;
pub mod async_runtime;
pub mod channel_system;
pub mod interpreter;
pub mod native_codegen;
pub mod optimizer;
pub mod type_checker;
pub mod wasm_binary;
pub mod wasm_codegen;

// Re-export commonly used types
pub use async_builtins::AsyncBuiltins;
pub use async_promise::{
    AsyncPromise, promise_all, promise_any, promise_delay, promise_from_async, promise_race,
};
pub use async_runtime::{
    AsyncRuntime, RuntimeEvent, TaskId, TaskResult, async_delay, async_timeout,
};
pub use channel_system::{
    BroadcastChannel, ChannelMessage, ChannelRegistry, ChannelType, MpscChannel, WatchChannel,
};
pub use interpreter::{Interpreter, InterpreterError, Value};
pub use native_codegen::{
    NativeCodeGenerator, NativeCodegenError, OptimizationLevel, TargetPlatform,
};
pub use optimizer::{OptimizationConfig, OptimizationError, OptimizationStats, Optimizer};
pub use type_checker::TypeChecker;
pub use wasm_binary::{WasmBinaryError, WasmBinaryGenerator};
pub use wasm_codegen::WasmCodeGenerator;
