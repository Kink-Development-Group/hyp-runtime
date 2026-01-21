# HypnoScript Copilot Instructions

## Project Overview

HypnoScript is a hypnotically-themed programming language with a pure Rust implementation. The codebase is a Cargo workspace with five interconnected crates following a clear compilation pipeline.

## Architecture & Data Flow

```
Source (.hyp) → Lexer → Tokens → Parser → AST → TypeChecker → Interpreter/Compiler
```

**Workspace crates (dependency order):**

1. **hypnoscript-core** - Type system (`HypnoType`, `HypnoBaseType`), symbols, symbol tables
2. **hypnoscript-lexer-parser** - Tokenization and AST generation (`AstNode` enum)
3. **hypnoscript-runtime** - 180+ builtin functions organized by category (Math, String, Array, etc.)
4. **hypnoscript-compiler** - Interpreter, TypeChecker, WASM/Native codegen, Optimizer
5. **hypnoscript-cli** - CLI entry point + package manager (`trance.json`)

## Language Syntax Quick Reference

HypnoScript uses hypnotic-themed keywords. Key mappings:

- `Focus { ... } Relax` → program boundaries
- `induce`/`implant`/`freeze` → `let`/`var`/`const`
- `suggestion` → function, `session` → class
- `observe`/`whisper`/`command` → print variants
- `awaken` → return, `entrain`/`when`/`otherwise` → pattern matching
- Operators: `yourEyesAreGettingHeavy` (>=), `fallUnderMySpell` (<), etc.

See [hypnoscript-lexer-parser/src/token.rs](hypnoscript-lexer-parser/src/token.rs) for all tokens.

## Developer Commands

```bash
# Build all crates
cargo build --all --release

# Run a .hyp file
cargo run -p hypnoscript-cli -- exec file.hyp

# Run all tests (185+ tests)
cargo test --all

# Test single crate with output
cargo test --package hypnoscript-compiler -- --nocapture

# Quality checks
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
```

## Code Conventions

### Adding Builtin Functions

1. Create module in `hypnoscript-runtime/src/` (e.g., `my_builtins.rs`)
2. Implement `BuiltinModule` trait from [builtin_trait.rs](hypnoscript-runtime/src/builtin_trait.rs)
3. Register in interpreter at [interpreter.rs](hypnoscript-compiler/src/interpreter.rs)
4. Include `#[cfg(test)] mod tests { ... }` with unit tests

### AST Node Pattern

All language constructs map to variants in `AstNode` enum ([ast.rs](hypnoscript-lexer-parser/src/ast.rs)):

```rust
AstNode::VariableDeclaration { name, type_annotation, initializer, is_constant, storage }
```

### Error Handling

- Use `thiserror` for error types (`#[derive(Error)]`)
- Support i18n via `LocalizedMessage` for user-facing errors
- Errors in interpreter: `InterpreterError` enum with `#[error(...)]`

### Testing Pattern

Tests are inline modules at file end:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_feature() { ... }
}
```

## Package Manager (trance.json)

HypnoScript projects use `trance.json` manifest with themed keys:

- `ritualName` → package name
- `mantra` → version
- `anchors`/`deepAnchors` → dependencies/devDependencies
- `suggestions` → npm-style scripts

## Key Files to Understand

| File                                                                               | Purpose                        |
| ---------------------------------------------------------------------------------- | ------------------------------ |
| [hypnoscript-compiler/src/interpreter.rs](hypnoscript-compiler/src/interpreter.rs) | Main runtime, 3400+ lines      |
| [hypnoscript-lexer-parser/src/ast.rs](hypnoscript-lexer-parser/src/ast.rs)         | All AST node types             |
| [hypnoscript-lexer-parser/src/token.rs](hypnoscript-lexer-parser/src/token.rs)     | Token types & keyword mappings |
| [hypnoscript-core/src/types.rs](hypnoscript-core/src/types.rs)                     | Type system implementation     |
| [hypnoscript-cli/src/package.rs](hypnoscript-cli/src/package.rs)                   | Package manager manifest types |
