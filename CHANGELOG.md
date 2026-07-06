# Changelog

All notable changes to this project will be documented in this file. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- **Closures with lexical scoping**: functions declared inside other functions now
  capture the enclosing local variables (by-value snapshot at declaration time) and can
  recurse. Variable lookups no longer leak across function-call boundaries (previously
  the interpreter used dynamic scoping, so a callee could accidentally read the caller's
  locals); accessing a caller-local from a callee is now an `UndefinedVariable` error.
- **Recursion depth limit**: deeply recursive programs abort with a graceful
  `RecursionLimitExceeded` error instead of crashing the host process with a stack
  overflow. Default limit is 1,000 calls; configurable via the `HYPNO_MAX_CALL_DEPTH`
  environment variable, the `--max-call-depth` CLI flag on `exec`, or
  `Interpreter::set_max_call_depth`. The interpreter additionally grows the native stack
  on demand (`stacker`), so the limit is reliable in debug and release builds alike.
- **Filesystem sandbox for file builtins**: setting the `HYPNO_SANDBOX` environment
  variable (or passing `--sandbox <dir>` to `exec`, or calling
  `hypnoscript_runtime::set_sandbox_root`) confines `ReadFile`, `WriteFile`,
  `DeleteFile`, `ListDirectory` and all other file builtins to that directory. Escapes
  via `..`, absolute paths or symlinks are rejected with `PermissionDenied`; relative
  paths resolve against the sandbox root. Without a configured root, behaviour is
  unchanged.
- **Working promise builtins**: `delayedValue(ms, value)` returns a real promise that
  resolves when awaited (delay honours `HYPNO_TIME_SCALE`), plus `instantPromise(value)`,
  `promiseAll(array)` (waits for the longest delay, simulating concurrency),
  `promiseRace(array)` (waits for the shortest) and `isPromiseResolved(p)`. `await` now
  resolves pending promises deterministically instead of sleeping a hard-coded 10 ms.
- **Central builtin registry** (`hypnoscript_compiler::builtin_registry`): all 142
  builtin signatures are declared once in a table that feeds both the type checker
  registrations and the CLI `builtins` command (which previously printed a hardcoded,
  outdated list and now renders every builtin with its full signature).
- **Value representation optimized**: arrays and records are shared via `Rc` (safe since
  both are immutable value types in the language), and function bodies are shared via
  `Rc` as well — cloning a value or looking up a function no longer deep-copies element
  vectors or the body AST.
- **`AsyncBuiltins` placeholders removed**: `delayed_value` (returned a fake string
  "promise"), `promise_all` and `promise_race` (returned wrong results) were replaced by
  the real interpreter builtins above.
- **`null` literal**: `null` is now a first-class literal (previously the nullish
  operators `lucidFallback`/`dreamReach` existed but `null` itself could not be written).
  It works in expressions, record fields and as an `entrain` pattern (`when null => ...`).
- **Nullable types**: `number?` (or the hypnotic spelling `lucid number`) declares a type
  that admits `null`. The type checker enforces this in both directions: assigning `null`
  to a non-nullable type is an error, as is assigning a nullable value to a non-nullable
  target. `lucidFallback` strips nullability from the result type.
- **Array type annotations**: `string[]`, `number[][]` and combinations like `number[]?`
  are accepted everywhere a type annotation is allowed, and are checked against array
  literal element types.
- **Labeled loops with labeled break/continue**: `outer: loop (...) { ... snap outer; }`
  breaks out of the named loop from any nesting depth; `sink outer;` continues its next
  iteration. Also supports the `label name:` keyword form. Unknown labels are runtime
  errors.
- **`drift(ms);` / `pauseReality(ms);` statements**: pause execution, with static type
  checking of the duration expression.
- **Standalone `deepFocus (cond) { ... }` statement**: the conditional block form that was
  already in the AST and interpreter is now parseable.
- **`imperative suggestion` two-word form** for function declarations (top-level and in
  sessions), alongside the existing one-word `imperativeSuggestion`.
- **`sharedTrance` shorthand**: `sharedTrance total: number = 0;` now works without an
  explicit `induce`/`implant`/`embed`/`freeze` keyword.
- **Source encoding tolerance**: `.hyp` files with UTF-8 BOMs or in UTF-16 (LE/BE, with or
  without BOM) are decoded transparently by the CLI and test harness
  (`hypnoscript_lexer_parser::decode_source`).
- **`HYPNO_TIME_SCALE` environment variable**: scales every themed pause (`drift`,
  `DeepTrance`, `HypnoticCountdown`, ...). `0` skips pauses entirely (used by the test
  harness), `0.5` halves them, unset means real time.
- **String Interpolation**: `"Hello, ${name}!"` embeds arbitrary expressions in string
  literals. Interpolations are desugared by the lexer into string concatenation, so they
  work uniformly across the interpreter, type checker and all compile targets.
  `\${` escapes a literal `${`; nested braces and strings inside `${...}` are supported.
- **Readable numeric literals**: digit separators (`1_000_000`) and exponent notation
  (`2.5e3`, `7e-2`) in number literals.
- **End-to-end sample-program harness** (`hypnoscript-compiler/tests/hyp_programs.rs`):
  every `.hyp` file in `hypnoscript-tests/` must be categorized as runnable or
  known-unsupported (with a reason); runnable programs are executed on every
  `cargo test` run instead of only one file in CI. With the language gaps above closed,
  **all 27 sample programs now run** (previously 16; legacy files with genuine bugs —
  a duplicated `Focus {`, a missing semicolon, `deeplyLess` used where `<` was meant —
  were fixed).
- **`check` command now exits non-zero when type errors are found**, so it can gate CI.

### Changed

- **Structured syntax errors**: the lexer and parser now return `SyntaxError` values with
  line/column information instead of plain strings. Parser errors additionally report the
  offending token (e.g. `Expected ';' after expression, found 'observe' at line 4, column 5`).
- **Interpreter modularized**: `hypnoscript-compiler/src/interpreter.rs` (3,800 lines) was
  split into focused submodules `error`, `value`, `session` and `builtins`; the public API
  (`Interpreter`, `InterpreterError`, `Value`) is unchanged.
- **Keyword table deduplicated**: keyword definitions in `token.rs` live in a single
  declarative table instead of ~570 lines of repetitive map insertions.
- **Lexer cleanup**: operator lexing extracted into a table-driven helper; comment skipping
  unified; multi-line strings now report the token's start line.

### Fixed

- `a..b` no longer silently swallows one dot (it is now a clear syntax error suggesting
  `.` or `...`).
- Number literals no longer accept multiple decimal points (`1.2.3` previously lexed as a
  single invalid number; `1.2.member` now lexes correctly as member access).
- Unterminated block comments are reported as errors instead of being silently accepted.
- Unterminated strings report the string's start position instead of the end of file.

## [1.2.0] - 2026-01-22

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
[1.2.0]: https://github.com/Kink-Development-Group/hyp-runtime/releases/tag/1.2.0
