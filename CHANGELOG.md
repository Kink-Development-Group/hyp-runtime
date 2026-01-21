# Changelog

All notable changes to this project will be documented in this file. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/).

## [1.2.0] - UNRELEASED

### Added

- **Complete Debugging Infrastructure** with interactive step-through execution:
  - New `hypnoscript-compiler/src/debug.rs` module with `DebugState`, `StepMode`, `CallFrame`, `WatchExpression`, and `PauseReason` types
  - Breakpoint management (set, remove, clear, list)
  - Step modes: Step Into, Step Over, Step Out, Continue
  - Call stack tracking with function name, line number, and local variables
  - Watch expressions for monitoring values during debugging
  - Source code display with breakpoint markers

- **Debug REPL** (`hypnoscript-cli/src/debug_repl.rs`) for interactive debugging sessions:
  - Commands: `break`, `delete`, `continue`, `step`, `next`, `out`, `print`, `locals`, `stack`, `watch`, `list`, `where`, `help`, `quit`
  - Expression evaluation in current scope
  - Variable inspection (locals, globals, all variables)
  - Configurable via `DebugConfig` with initial breakpoints and watches

- **Debug Builtins** (`hypnoscript-runtime/src/debug_builtins.rs`):
  - `inspect(value)` - Returns value with type information
  - `typeOf(value)` - Returns the type name
  - `stackTrace()` - Returns formatted call stack
  - `dump(value)` - Prints detailed value representation
  - `assertEqual()`, `assertNotEqual()`, `assertTruthy()`, `assertFalsy()`, `assertNull()`, `assertNotNull()` - Assertions with messages
  - `time(label)`, `timeEnd(label)`, `measureTime(fn)` - Performance timing
  - `log()`, `warn()`, `error()`, `trace()` - Debug output levels
  - `breakpoint()` - Programmatic breakpoint

- **Extended CLI Debug Options** for the `exec` command:
  - `--debug` / `-d` - Enable interactive debug mode
  - `--breakpoints <LINES>` - Set initial breakpoints (comma-separated line numbers)
  - `--watch <EXPRS>` - Set initial watch expressions (comma-separated)
  - `--trace-file <PATH>` - Output trace information to file

- **Interpreter Debug Integration**:
  - `enable_debug_mode()` / `disable_debug_mode()` methods
  - `set_breakpoint()`, `remove_breakpoint()`, `has_breakpoint()`, `clear_breakpoints()`
  - `set_step_mode()`, `step_mode()` for stepping control
  - `add_watch()`, `remove_watch()` for watch expressions
  - `debug_locals()`, `debug_globals()`, `debug_all_variables()` for variable inspection
  - `debug_call_stack()`, `debug_source_context()` for execution context

### Changed

- Extended `Interpreter` struct with optional `debug_state` field
- Updated CLI `exec` command to support debug mode with REPL integration

### Tests

- Added 9 new debug mode tests in `interpreter.rs`
- Added comprehensive unit tests for `DebugState`, `DebugCommand`, `CallFrame`
- Added unit tests for `DebugBuiltins` (assertions, timing, inspection)
- Added unit tests for `DebugSession` and `DebugConfig`

## [1.0.0] - 2025-11-15

### Added

- Initial release of the complete **HypnoScript** stack with compiler (`hypnoscript-compiler`), runtime (`hypnoscript-runtime`), and core library (`hypnoscript-core`).
- Integration of the Cranelift backend for native code generation including linker workflow and platform support for Linux, Windows, and macOS.
- Comprehensive CLI (`hypnoscript-cli`) with commands for running scripts, test runs, builtin listing, and version output.
- Asynchronous runtime ecosystem with Promise support, channel system, and extended builtins (strings, arrays, files, hashing, localization, and much more).
- Complete language documentation with VitePress, including getting-started guides, language reference, builtin catalog, and enterprise chapter.
- Automated build and release scripts for Linux, Windows (Winget), and macOS (Universal/x64/arm64, pkg & dmg).

### Changed

- Consolidated type checking, parser improvements, and iterator-based implementations to comply with strict `cargo clippy` warning guidelines.
- Unified handling of linker arguments, record types, and function signatures to ensure stable release builds across the entire workspace.

### Fixed

- Resolved borrow checker issues in the native code generator and stabilized channel synchronization in the async runtime module.
- Reduced error and warning messages in interpreter, optimizer, and parser through targeted refactorings.
- Added missing type system documentation and corrected unreachable documentation links (e.g., `language-reference/types.html`).

### Security & Compliance

- Updated `deny.toml`, including MPL-2.0 license exception for `webpki-roots` and ignoring the documented advisory `RUSTSEC-2020-0168`.
- Successfully completed `cargo deny check` with cleaned-up license and advisory checks.

[1.0.0]: https://github.com/Kink-Development-Group/hyp-runtime/releases/tag/1.0.0
