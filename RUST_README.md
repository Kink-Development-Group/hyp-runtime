# HypnoScript Rust Implementation

This directory contains the Rust implementation of the HypnoScript programming language runtime, migrated from C# for improved performance.

## 🦀 Architecture

The Rust implementation is organized as a Cargo workspace with the following crates:

```
hyp-runtime/
├── Cargo.toml                    # Workspace configuration
├── hypnoscript-core/             # Core type system and symbols
├── hypnoscript-lexer-parser/     # Lexer, Parser, and AST
├── hypnoscript-compiler/         # Compiler and interpreter
├── hypnoscript-runtime/          # Builtin functions and runtime
└── hypnoscript-cli/              # Command-line interface
```

## 🚀 Building

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Build All Crates
```bash
cargo build --all --release
```

### Build Specific Crate
```bash
cargo build -p hypnoscript-cli --release
```

## 🧪 Testing

Run all tests:
```bash
cargo test --all
```

Run tests for a specific crate:
```bash
cargo test -p hypnoscript-runtime
```

## 📦 Components

### hypnoscript-core
Core data structures and type system:
- `HypnoType`: Type system (primitives, arrays, records, functions)
- `Symbol`: Symbol definitions
- `SymbolTable`: Scope management with nested scopes

### hypnoscript-lexer-parser
Lexical analysis and parsing:
- `Token`: Token representation
- `TokenType`: 110+ token types
- `Lexer`: Tokenizer for HypnoScript code
- `AstNode`: Abstract syntax tree nodes

### hypnoscript-runtime
Runtime environment and builtin functions (50+ implemented):

**Math (20+):**
- Trigonometry: `sin`, `cos`, `tan`
- Basic: `sqrt`, `pow`, `log`, `abs`, `floor`, `ceil`, `round`
- Advanced: `factorial`, `gcd`, `lcm`, `is_prime`, `fibonacci`

**String (15+):**
- `length`, `to_upper`, `to_lower`, `trim`, `reverse`
- `index_of`, `replace`, `capitalize`, `split`, `substring`

**Array (15+):**
- `length`, `sum`, `average`, `min`, `max`, `sort`
- `reverse`, `distinct`, `first`, `last`, `take`, `skip`

**Hypnotic:**
- `observe` (output)
- `drift` (sleep)
- `deep_trance`, `hypnotic_countdown`, `trance_induction`

### hypnoscript-cli
Command-line interface:

```bash
# Show version
hypnoscript-cli version

# List builtin functions
hypnoscript-cli builtins

# Tokenize a file
hypnoscript-cli lex program.hyp

# Run a program (when interpreter is complete)
hypnoscript-cli run program.hyp
```

## 📊 Performance Benefits

Rust provides several advantages over C#:

1. **Zero-cost abstractions**: Compile-time optimizations with no runtime overhead
2. **No garbage collection**: Deterministic memory management
3. **Memory safety**: Compile-time prevention of common bugs
4. **Smaller binaries**: Self-contained executables without runtime dependency
5. **Better parallelization**: Safe concurrent access via ownership model

## 🔧 Development

### Adding New Builtins

1. Add function to appropriate module in `hypnoscript-runtime/src/`
2. Add tests in the same file
3. Update the builtins list in the CLI

Example:
```rust
// In math_builtins.rs
pub fn new_function(x: f64) -> f64 {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_function() {
        assert_eq!(new_function(5.0), expected_result);
    }
}
```

### Code Style
- Follow Rust standard style (use `cargo fmt`)
- Run clippy for linting: `cargo clippy`
- Keep functions focused and well-documented
- Write tests for new functionality

## 📝 Migration Status

- ✅ Core type system (100%)
- ✅ Symbol table (100%)
- ✅ Lexer (100%)
- ✅ Runtime builtins (50+ of 150+, ~35%)
- ✅ CLI framework (60%)
- ⏳ Parser (pending)
- ⏳ Interpreter (pending)
- ⏳ Compiler (pending)

## 🎯 Roadmap

1. Complete parser implementation
2. Implement interpreter
3. Port remaining builtin functions (100+ more)
4. Add WASM code generation
5. Performance benchmarking vs C# version
6. Comprehensive integration tests

## 🐛 Known Issues

- Lexer doesn't handle UTF-16 encoded files (use UTF-8)
- Parser not yet implemented
- Interpreter not yet implemented

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Original C# Implementation](../HypnoScript.CLI/)

## 🤝 Contributing

When contributing to the Rust implementation:
1. Maintain API compatibility with the C# version where possible
2. Follow DRY principles (Don't Repeat Yourself)
3. Write comprehensive tests
4. Document public APIs
5. Run `cargo fmt` and `cargo clippy` before committing

## 📄 License

MIT License (same as original project)
