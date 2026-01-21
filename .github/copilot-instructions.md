# HypnoScript Copilot Instructions

## Architecture Overview

HypnoScript is a hypnotically-themed programming language with a pure Rust implementation (Rust 2024 edition). The compilation pipeline flows:

```
Source (.hyp) → Lexer → Tokens → Parser → AST → TypeChecker → Interpreter/Compiler
```

**Workspace crates (dependency order):**

| Crate                      | Purpose                     | Key Exports                                                              |
| -------------------------- | --------------------------- | ------------------------------------------------------------------------ |
| `hypnoscript-core`         | Type system foundation      | `HypnoType`, `HypnoBaseType`, `SymbolTable`                              |
| `hypnoscript-lexer-parser` | Tokenization & AST          | `Lexer`, `Parser`, `AstNode`, `Token`                                    |
| `hypnoscript-runtime`      | 180+ builtins (20+ modules) | `MathBuiltins`, `StringBuiltins`, `ArrayBuiltins`, etc.                  |
| `hypnoscript-compiler`     | All backends                | `Interpreter`, `TypeChecker`, `WasmCodeGenerator`, `NativeCodeGenerator` |
| `hypnoscript-cli`          | CLI + package manager       | Commands: `exec`, `lex`, `parse`, `check`, `compile-wasm`                |

## Essential Commands

```bash
# Development cycle
cargo build --all                           # Debug build
cargo run -p hypnoscript-cli -- exec file.hyp  # Run .hyp file
cargo run -p hypnoscript-cli -- exec file.hyp --debug  # Interactive debugger

# Testing (185+ tests, run on Windows/Linux/macOS in CI)
cargo test --all                            # All tests
cargo test --package hypnoscript-compiler -- --nocapture  # Single crate with output
cargo test --package hypnoscript-runtime test_math  # Specific test

# Pre-commit quality checks (enforced by CI)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Release build (LTO enabled, stripped)
cargo build --all --release
```

## Language Syntax Quick Reference

HypnoScript uses hypnotic-themed keywords. See [token.rs](hypnoscript-lexer-parser/src/token.rs) for complete list.

| HypnoScript                  | Equivalent | Example                                                          |
| ---------------------------- | ---------- | ---------------------------------------------------------------- |
| `Focus { } Relax`            | program    | `Focus { observe "hi"; } Relax`                                  |
| `induce`/`freeze`            | let/const  | `induce x: number = 42;`                                         |
| `suggestion`                 | function   | `suggestion add(a: number, b: number): number { awaken a + b; }` |
| `session`                    | class      | `session Counter { expose count: number = 0; }`                  |
| `observe`/`whisper`          | print      | `observe "Hello";` (with newline)                                |
| `awaken`                     | return     | `awaken result;`                                                 |
| `entrain`/`when`/`otherwise` | match      | `entrain x { when 0 => "zero" otherwise => "other" }`            |
| `yourEyesAreGettingHeavy`    | >=         | `if (x yourEyesAreGettingHeavy 10) deepFocus { }`                |
| `lucidFallback`              | ??         | `maybeNull lucidFallback 100`                                    |

## Adding New Builtin Functions

1. **Create module** in `hypnoscript-runtime/src/` (e.g., `my_builtins.rs`)
2. **Implement trait** `BuiltinModule` from [builtin_trait.rs](hypnoscript-runtime/src/builtin_trait.rs):
   ```rust
   impl BuiltinModule for MyBuiltins {
       fn module_name() -> &'static str { "My" }
       fn description() -> &'static str { "My builtin functions" }
       fn function_names() -> &'static [&'static str] { &["MyFunc1", "MyFunc2"] }
   }
   ```
3. **Export** in `hypnoscript-runtime/src/lib.rs`
4. **Register** in [interpreter.rs](hypnoscript-compiler/src/interpreter.rs) - add match arms in `call_builtin_function()`
5. **Add tests** as inline `#[cfg(test)] mod tests { }` at file end

## Code Conventions

- **Error handling**: Use `thiserror` with `#[derive(Error)]`. User-facing errors support i18n via `LocalizedMessage` (EN, DE, FR, ES)
- **AST nodes**: All constructs map to variants in `AstNode` enum ([ast.rs](hypnoscript-lexer-parser/src/ast.rs))
- **Tests**: Inline modules at file end with `#[cfg(test)]`. Test `.hyp` files in `hypnoscript-tests/`
- **Workspace deps**: Shared dependencies defined in root `Cargo.toml` under `[workspace.dependencies]`

## Package Manager (trance.json)

HypnoScript projects use themed manifest keys:

- `ritualName` → package name, `mantra` → version
- `anchors`/`deepAnchors` → dependencies/devDependencies
- `suggestions` → npm-style scripts

See [package.rs](hypnoscript-cli/src/package.rs) for schema.

## Key Files Reference

| File                                                         | Purpose                                      |
| ------------------------------------------------------------ | -------------------------------------------- |
| [interpreter.rs](hypnoscript-compiler/src/interpreter.rs)    | Main runtime (3400+ lines), builtin dispatch |
| [ast.rs](hypnoscript-lexer-parser/src/ast.rs)                | All 30+ AST node variants                    |
| [token.rs](hypnoscript-lexer-parser/src/token.rs)            | Token types & hypnotic keyword mappings      |
| [types.rs](hypnoscript-core/src/types.rs)                    | `HypnoType` enum, type inference             |
| [builtin_trait.rs](hypnoscript-runtime/src/builtin_trait.rs) | `BuiltinModule` trait for extending runtime  |
