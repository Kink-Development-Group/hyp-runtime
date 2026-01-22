# HypnoScript Compiler

The full compiler and interpreter for the HypnoScript programming language.

## Features

### 🎯 Multiple Backends

1. **Interpreter** - Direct execution of HypnoScript code
   - Full language support
   - OOP with sessions (classes)
   - Built-in functions included
   - Ideal for development and debugging

2. **Native Code Generator** (Cranelift)
   - Platform-specific machine code
   - Automatic linking to executable binaries
   - Supported platforms:
     - Windows (x86_64, ARM64) - requires Visual Studio Build Tools, GCC, or Clang
     - macOS (x86_64, ARM64/Apple Silicon) - requires Xcode Command Line Tools
     - Linux (x86_64, ARM64, RISC-V) - requires GCC or Clang
   - Optimized binaries with multiple optimization levels
   - Faster build process compared to LLVM
   - ✅ **Fully functional** - produces executable .exe/.bin files

3. **WebAssembly Generator**
   - Text format (.wat) - human-readable
   - Binary format (.wasm) - compact
   - Browser and server support
   - Sandboxed execution

### 🔧 Additional Features

- **Type Checker**: Static type checking
- **Optimizer**: Code optimizations
  - Constant Folding
  - Dead Code Elimination
  - Common Subexpression Elimination
  - Loop Invariant Code Motion
  - Function Inlining

## Usage

### Interpreter

```rust
use hypnoscript_compiler::Interpreter;
use hypnoscript_lexer_parser::{Lexer, Parser};

let source = r#"
Focus {
    induce x: number = 42;
    observe x;
} Relax
"#;

let mut lexer = Lexer::new(source);
let tokens = lexer.lex()?;
let mut parser = Parser::new(tokens);
let ast = parser.parse_program()?;

let mut interpreter = Interpreter::new();
interpreter.interpret(&ast)?;
```

### Native Compilation

```rust
use hypnoscript_compiler::{NativeCodeGenerator, OptimizationLevel, TargetPlatform};

let mut generator = NativeCodeGenerator::new();
generator.set_target_platform(TargetPlatform::LinuxX64);
generator.set_optimization_level(OptimizationLevel::Release);

let binary_path = generator.generate(&ast)?;
```

### WASM Generation

```rust
use hypnoscript_compiler::{WasmCodeGenerator, WasmBinaryGenerator};

// Text Format (.wat)
let mut wat_gen = WasmCodeGenerator::new();
let wat_code = wat_gen.generate(&ast);
std::fs::write("output.wat", wat_code)?;

// Binary Format (.wasm)
let mut wasm_gen = WasmBinaryGenerator::new();
let wasm_bytes = wasm_gen.generate(&ast)?;
std::fs::write("output.wasm", wasm_bytes)?;
```

## Architecture

### Design Principles

- **OOP First**: Sessions as full classes with encapsulation
- **DRY**: No code duplication, shared infrastructure
- **Type Safety**: Static type checking before execution
- **Memory Safety**: 100% Rust, no unsafe operations

### Module

```text
hypnoscript-compiler/
├── src/
│   ├── lib.rs              # Public API
│   ├── interpreter.rs      # Runtime interpreter (2392 lines)
│   ├── type_checker.rs     # Static type checking (1683 lines)
│   ├── optimizer.rs        # Code optimizations (421 lines)
│   ├── native_codegen.rs   # Cranelift backend with auto-linking
│   ├── wasm_codegen.rs     # WASM Text Generator
│   └── wasm_binary.rs      # WASM Binary Generator
└── Cargo.toml
```

## Performance Comparison

| Backend            | Compile Time | Runtime            | Binary Size | Use Case               |
| ------------------ | ------------ | ------------------ | ----------- | ---------------------- |
| Interpreter        | Instant      | ~10x slower        | N/A         | Development, debugging |
| Native (Cranelift) | ~1-2 seconds | Native (very fast) | 50-200 KB   | Production, server     |
| WASM               | ~50ms        | ~2x slower         | 10-50 KB    | Web, embedding         |

## System Requirements for Native Compilation

### Windows

- Visual Studio Build Tools (recommended) OR
- MinGW-w64/GCC OR
- LLVM/Clang

### macOS

- Xcode Command Line Tools (`xcode-select --install`)

### Linux

- GCC (`sudo apt install build-essential`) OR
- Clang (`sudo apt install clang`)

## Dependencies

- `cranelift`: Native code generation
- `hypnoscript-core`: Shared types and symbol tables
- `hypnoscript-lexer-parser`: AST and parser
- `hypnoscript-runtime`: Built-in functions

## Tests

```bash
cargo test --package hypnoscript-compiler
```

Current status: **34 tests, all passing** ✅

## Example: Native Compilation

```bash
# Compile HypnoScript to a native executable
hypnoscript compile-native my_program.hyp

# With optimization
hypnoscript compile-native my_program.hyp --opt-level release

# Specific output
hypnoscript compile-native my_program.hyp --output my_program.exe
```

## Roadmap

- [x] Interpreter with full language support
- [x] Type Checker
- [x] WASM Text Format (.wat)
- [x] WASM Binary Format (.wasm)
- [x] Native code generator with Cranelift
- [x] Automatic linking to executable binaries
- [x] Code optimizations
- [ ] Advanced Control Flow in WASM
- [ ] Full session support in Native/WASM
- [ ] Debugging information (DWARF)
- [ ] Cross-Compilation

## License

MIT
