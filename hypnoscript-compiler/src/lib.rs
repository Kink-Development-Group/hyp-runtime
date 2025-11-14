//! HypnoScript Compiler und Interpreter
//!
//! Dieses Modul stellt die vollständige Compiler-Infrastruktur und den Interpreter
//! für HypnoScript bereit.
//!
//! ## Architektur
//!
//! Der Compiler unterstützt mehrere Backends:
//!
//! ### 1. Interpreter (Runtime-Ausführung)
//! - Direktes Ausführen von HypnoScript-Code
//! - Vollständige Sprachunterstützung inkl. OOP (Sessions)
//! - Integrierte Built-in-Funktionen
//! - Ideal für Entwicklung und Debugging
//!
//! ### 2. Native Code-Generator (Cranelift)
//! - Kompiliert zu plattformspezifischem Maschinencode
//! - Unterstützt Windows, macOS und Linux (x86_64, ARM64)
//! - Optimierte Binaries mit Cranelift-Backend
//! - Schnellere Alternative zu LLVM
//!
//! ### 3. WASM-Generator
//! - **Text Format (.wat)**: Menschenlesbares WebAssembly
//! - **Binary Format (.wasm)**: Kompaktes binäres WebAssembly
//! - Browser- und Server-Unterstützung
//! - Sandboxed Execution
//!
//! ## Module
//!
//! - **interpreter**: Interpretiert HypnoScript-Code direkt
//! - **type_checker**: Statische Typprüfung vor der Ausführung
//! - **optimizer**: Code-Optimierungen (Constant Folding, Dead Code Elimination, etc.)
//! - **native_codegen**: Generiert plattformspezifischen nativen Code mit Cranelift
//! - **wasm_codegen**: Generiert WebAssembly Text Format (.wat)
//! - **wasm_binary**: Generiert WebAssembly Binary Format (.wasm)
//!
//! ## Design-Prinzipien
//!
//! ### OOP (Object-Oriented Programming)
//! - Sessions als Klassen-Konstrukte
//! - Kapselung und Sichtbarkeitsmodifikatoren
//! - Statische und Instanz-Methoden/Felder
//!
//! ### DRY (Don't Repeat Yourself)
//! - Gemeinsame Infrastruktur in `hypnoscript-core`
//! - Wiederverwendbare Typsysteme und Symbol-Tables
//! - Shared Traits für Built-in-Funktionen
//!
//! ### Dokumentation
//! - Umfassende Rustdoc-Kommentare
//! - Beispiele für jedes Modul
//! - Unit-Tests als lebende Dokumentation
//!
//! ## Verwendung
//!
//! ### Beispiel: Interpretation
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
//! ### Beispiel: Native Kompilierung
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
//! ### Beispiel: WASM-Generierung
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
//! ## Performance-Vergleich
//!
//! | Backend | Kompilierzeit | Ausführungszeit | Binary-Größe | Use Case |
//! |---------|--------------|-----------------|--------------|----------|
//! | Interpreter | Sofort | Langsam | N/A | Entwicklung, Debugging |
//! | Native (Cranelift) | Schnell | Sehr schnell | Klein | Produktion, Server |
//! | WASM | Schnell | Schnell | Sehr klein | Web, Embedding |
//!
//! ## Sicherheit
//!
//! - Memory-safe durch Rust
//! - Type-checked vor Ausführung
//! - WASM: Sandboxed Execution
//! - Native: Optimierte, sichere Code-Generierung

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
