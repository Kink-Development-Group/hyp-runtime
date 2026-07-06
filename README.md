# HypnoScript

**HypnoScript** is a hypnotically-inspired programming language with its own syntax (`Focus { ... } Relax`).
The complete runtime environment, compiler, and command-line tools are exclusively developed in Rust from version 1.0 onwards.

---

## 🚀 Highlights

- 🦀 **Pure Rust Codebase** – fast builds, no .NET dependencies
- 🧠 **Complete Toolchain** – Lexer, Parser, Type Checker, Interpreter, and multiple compiler backends
- 🎯 **Multiple Targets** – Interpreter, WebAssembly (Text & Binary), Native Code (planned)
- ⚡ **Code Optimization** – Constant Folding, Dead Code Elimination, CSE, LICM, Inlining
- 🧰 **180+ Builtins** – Math, Strings, Arrays, Hypnosis, Files, Time, System, Statistics, Hashing, Validation, Cryptography
- 🌍 **Multilingual** – i18n support (EN, DE, FR, ES)
- 🔐 **Cryptography** – SHA-256, SHA-512, MD5, Base64, UUID
- 🧬 **Functional Programming** – map, filter, reduce, compose, pipe
- 🎭 **Hypnotic Operators** – 14 synonyms like `youAreFeelingVerySleepy`, `lookAtTheWatch`, `underMyControl`
- 🎯 **Pattern Matching** – `entrain`/`when`/`otherwise` with Destructuring, Guards, and Type Patterns
- 🔔 **Event-Driven** – `trigger` for Callbacks and Event Handlers
- 💎 **Nullish Operators** – `lucidFallback` (`??`) and `dreamReach` (`?.`) for safe null handling
- 🏛️ **OOP Support** – Sessions with `constructor`, `expose`/`conceal`, `dominant` (static)
- 🖥️ **Extended CLI** – `run`, `lex`, `parse`, `check`, `compile-wasm`, `compile-native`, `optimize`, `builtins`, `version`
- 🧵 **String Interpolation** – `"Hello, ${name}!"` with arbitrary expressions inside `${...}`
- 🔢 **Readable Numbers** – digit separators (`1_000_000`) and exponent notation (`2.5e3`)
- 🌫️ **Null Safety** – `null` literal, nullable types (`number?` / `lucid number`), enforced by the type checker
- 🪤 **Closures** – nested suggestions capture their lexical environment and can recurse
- 🛡️ **Sandboxing** – confine all file builtins to a directory via `--sandbox` / `HYPNO_SANDBOX`
- 🌀 **Safe Recursion** – graceful `RecursionLimitExceeded` errors instead of stack-overflow crashes (`--max-call-depth` / `HYPNO_MAX_CALL_DEPTH`)
- ⏳ **Promises** – `delayedValue`, `instantPromise`, `promiseAll`, `promiseRace`, `isPromiseResolved` with deterministic `await`
- 🔁 **Labeled Loops** – `outer: loop (...) { snap outer; }` for breaking/continuing outer loops
- ⏸️ **Timed Pauses** – `drift(ms);` statement; scale or skip all pauses via `HYPNO_TIME_SCALE`
- ✅ **Comprehensive Tests** – 300+ tests across all compiler modules, plus an end-to-end harness over the sample programs
- 📚 **Documentation** – VitePress + extensive architecture docs + complete Rustdoc
- 🚀 **Performance** – Zero-cost abstractions, no garbage collector, optimized native code

---

## 🏗️ Workspace Architecture

```text
hyp-runtime/
├── Cargo.toml                    # Workspace configuration
├── hypnoscript-core/             # Type system & symbols (100%)
├── hypnoscript-lexer-parser/     # Tokens, Lexer, AST, Parser (100%)
│   ├── lexer.rs                  # ✅ Tokenizer incl. string interpolation
│   ├── parser.rs                 # ✅ Recursive-descent parser
│   └── error.rs                  # ✅ Syntax errors with line/column info
├── hypnoscript-compiler/         # Compiler backend (100%)
│   ├── interpreter/              # ✅ Tree-walking interpreter (error/value/session/builtins)
│   ├── type_checker.rs           # ✅ Static type checking
│   ├── wasm_codegen.rs           # ✅ WASM Text Format (.wat)
│   ├── wasm_binary.rs            # ✅ WASM Binary Format (.wasm)
│   ├── optimizer.rs              # ✅ Code optimizations
│   └── native_codegen.rs         # 🚧 Native compilation (planned)
├── hypnoscript-runtime/          # 180+ builtin functions (100%)
└── hypnoscript-cli/              # Command-line interface (100%)
```

Documentation is available in `hypnoscript-docs/` (VitePress).

---

## ⚙️ Installation & Quick Start

### Prerequisites

- Rust 1.76+ (recommended) including `cargo`

### Automatic Installer

```bash
curl -fsSL https://kink-development-group.github.io/hyp-runtime/install.sh | bash
```

The script automatically detects Linux/macOS, downloads the appropriate runtime from the current release, and updates existing installations. Important options: `--prefix`, `--version`, `--check`, `--include-prerelease`, `--force`, `--uninstall`.

#### Updates & Uninstallation

- **Check for updates:** `hypnoscript self-update --check` displays available versions.
- **Update:** `hypnoscript self-update` pulls the latest release version including sudo handling.
- **Force reinstallation:** `hypnoscript self-update --force` runs the installer again.
- **Uninstall:** `curl -fsSL https://kink-development-group.github.io/hyp-runtime/install.sh | bash -s -- --uninstall` removes binary and metadata.

### Clone & Build Project

```bash
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime
cargo build --all --release
```

The CLI is created as two binaries: `hypnoscript` and `hyp` (short form). Both are identical and can be used interchangeably.

### Run Program

```bash
# Both variants work
./target/release/hypnoscript exec program.hyp
./target/release/hyp exec program.hyp
```

Or during development:

```bash
cargo run -p hypnoscript-cli -- exec test_simple.hyp
```

### Example Program

```hypnoscript
Focus {
    entrance {
        observe "Welcome to HypnoScript!";
    }

    induce x: number = 42;
    induce message: string = "Hello Trance";

    observe message;
    observe x;

    // String interpolation
    observe "The answer is ${x}, doubled it is ${x * 2}.";

    // Hypnotic operator synonym
    if (x yourEyesAreGettingHeavy 40) deepFocus {
        observe "X is greater than 40";
    }

    // Pattern matching with entrain
    induce result: string = entrain x {
        when 0 => "zero"
        when 42 => "answer to everything"
        when n if n > 100 => "large number"
        otherwise => "other"
    };
    observe result;

    // Nullish operators
    induce maybeNull: number? = null;
    induce defaulted: number = maybeNull lucidFallback 100;
    observe defaulted;  // 100

    // Trigger (event handler)
    trigger onComplete = suggestion() {
        observe "Task completed!";
    };
    onComplete();
} Relax
```

### Detailed CLI Commands

**Note:** All commands can be executed with either `hypnoscript` or `hyp`.

```bash
# Execute program (Interpreter)
hypnoscript exec program.hyp
# or short:
hyp exec program.hyp

# Analysis tools
hypnoscript lex program.hyp          # Tokenization
hypnoscript parse program.hyp        # Display AST
hypnoscript check program.hyp        # Type checking

# Compilation
hypnoscript compile-wasm program.hyp              # WASM Text Format (.wat)
hypnoscript compile-wasm -b program.hyp           # WASM Binary Format (.wasm)
hypnoscript compile-native program.hyp            # Native binary (planned)
hypnoscript compile-native -t linux-x64 \
  --opt-level release program.hyp                 # With target platform

# Code optimization
hypnoscript optimize program.hyp --stats          # With statistics

# Package Manager
hypnoscript init                         # Initialize new project
hypnoscript init --template cli          # CLI project template
hypnoscript add package --version "^1.0.0"       # Add dependency
hypnoscript add pkg --version "^1.0.0" --dev     # Add dev dependency
hypnoscript remove package               # Remove dependency
hypnoscript install                      # Install all dependencies
hypnoscript list                         # List dependencies
hypnoscript run <script>                 # Run script from trance.json
hypnoscript validate                     # Validate trance.json

# Utilities
hypnoscript builtins                 # Builtin functions
hypnoscript version                  # Version
hypnoscript self-update              # Self-update
```

#### WASM Compilation in Detail

```bash
# Text format (readable, debugging-friendly)
hypnoscript compile-wasm script.hyp -o output.wat

# Binary format (compact, production-ready)
hypnoscript compile-wasm --binary script.hyp -o output.wasm

# Convert to complete WASM binary with wabt-tools
wat2wasm output.wat -o output.wasm
```

#### Native Compilation (Planned)

```bash
# For current platform
hypnoscript compile-native app.hyp

# Cross-compilation
hypnoscript compile-native -t windows-x64 app.hyp
hypnoscript compile-native -t macos-arm64 app.hyp
hypnoscript compile-native -t linux-x64 app.hyp

# With optimization
hypnoscript compile-native --opt-level release app.hyp
```

---

## 📦 Package Manager

HypnoScript has a built-in package manager, similar to npm for JavaScript or Cargo for Rust. It uses `trance.json` as its manifest file.

### Quick Start

```bash
# Initialize new project
hypnoscript init --name my-project --template cli
# Add dependencies
hypnoscript add hypnoscript-runtime --version “^1.0.0”
hypnoscript add @hypno/testing-lab --version “^0.3.0” --dev
# Install dependencies
hypnoscript install
# List dependencies
hypnoscript list
# Run script
hypnoscript run test
# Validate manifest
hypnoscript validate
```

### trance.json Manifest

The manifest uses hypnotic terminology:

- **ritualName**: Package name (corresponds to `name` in npm)
- **mantra**: Version (corresponds to `version` in npm)
- **intent**: Project type (cli, library)
- **anchors**: Production dependencies (corresponds to `dependencies`)
- **deepAnchors**: Development dependencies (corresponds to `devDependencies`)
- **suggestions**: Executable scripts (corresponds to `scripts`)
- **channels**: Binary/CLI configuration
- **triggers**: Lifecycle hooks

Example `trance.json`:

```json
{
  "ritualName": "my-hypno-app",
  "mantra": "1.0.0",
  "intent": "cli",
  "suggestions": {
    "focus": "hypnoscript exec src/main.hyp",
    "test": "hypnoscript exec tests/test.hyp"
  },
  "anchors": {
    "hypnoscript-runtime": "^1.0.0"
  }
}
```

Complete documentation: see [PACKAGE_MANAGER.md](PACKAGE_MANAGER.md)

---

## 🧪 Tests & Quality Assurance

Run all tests:

```bash
cargo test --all
```

**Test Coverage**:

- ✅ Lexer: tokenization, escapes, interpolation, numeric literals
- ✅ Parser: all statements/expressions, positions in error messages
- ✅ Type Checker, Interpreter, Optimizer, WASM & Native Generators
- ✅ Runtime Builtins (math, string, array, collection, dict, file, ...)
- ✅ Pattern Matching, Triggers, Nullish Operators: full coverage
- ✅ End-to-end harness: every sample program in `hypnoscript-tests/` is
  categorized and the supported ones are executed on every test run
  (`hypnoscript-compiler/tests/hyp_programs.rs`)

### Total: 300+ tests (all passing)

### Compiler Tests

```bash
# Compiler tests only
cargo test --package hypnoscript-compiler

# With detailed output
cargo test --package hypnoscript-compiler -- --nocapture
```

### Code Quality

```bash
# Check formatting
cargo fmt --all -- --check

# Linting with Clippy
cargo clippy --all-targets --all-features
```

---

## 📦 Builtin Functions (110+)

### Mathematics (20+)

`Sin`, `Cos`, `Tan`, `Sqrt`, `Pow`, `Log`, `Abs`, `Floor`, `Ceil`, `Round`, `Min`, `Max`, `Factorial`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci`, `Clamp`

### Strings (15+)

`ToUpper`, `ToLower`, `Capitalize`, `TitleCase`, `IndexOf`, `Replace`, `Reverse`, `Split`, `Substring`, `Trim`, `Repeat`, `PadLeft`, `PadRight`, `StartsWith`, `EndsWith`, `Contains`, `Length`, `IsWhitespace`

### Arrays (15+)

`ArrayLength`, `ArraySum`, `ArrayAverage`, `ArrayMin`, `ArrayMax`, `ArraySort`, `ArrayReverse`, `ArrayDistinct`, `ArrayFirst`, `ArrayLast`, `ArrayTake`, `ArraySkip`, `ArraySlice`, `ArrayJoin`, `ArrayCount`, `ArrayIndexOf`, `ArrayContains`, `ArrayIsEmpty`, `ArrayGet`

### Time/Date (15)

`GetCurrentTime`, `GetCurrentDate`, `GetCurrentDateTime`, `FormatDateTime`, `GetYear`, `GetMonth`, `GetDay`, `GetHour`, `GetMinute`, `GetSecond`, `GetDayOfWeek`, `GetDayOfYear`, `IsLeapYear`, `GetDaysInMonth`, `CurrentDate`, `DaysInMonth`

### Validation (10)

`IsValidEmail`, `IsValidUrl`, `IsValidPhoneNumber`, `IsAlphanumeric`, `IsAlphabetic`, `IsNumeric`, `IsLowercase`, `IsUppercase`, `IsInRange`, `MatchesPattern`

### File I/O (14)

`ReadFile`, `WriteFile`, `AppendFile`, `FileExists`, `IsFile`, `IsDirectory`, `DeleteFile`, `CreateDirectory`, `ListDirectory`, `GetFileSize`, `CopyFile`, `RenameFile`, `GetFileExtension`, `GetFileName`

### Statistics (9)

`CalculateMean`, `CalculateMedian`, `CalculateMode`, `CalculateStandardDeviation`, `CalculateVariance`, `CalculateRange`, `CalculatePercentile`, `CalculateCorrelation`, `LinearRegression`, `Mean`, `Variance`

### Hashing/Utilities (10)

`HashString`, `HashNumber`, `AreAnagrams`, `IsPalindrome`, `CountOccurrences`, `RemoveDuplicates`, `UniqueCharacters`, `ReverseWords`, `TitleCase`, `SimpleRandom`

### System (12)

`GetOperatingSystem`, `GetArchitecture`, `GetCpuCount`, `GetHostname`, `GetCurrentDirectory`, `GetHomeDirectory`, `GetTempDirectory`, `GetEnvVar`, `SetEnvVar`, `GetUsername`, `GetArgs`, `Exit`

### Hypnosis/Core (6)

`Observe`, `Drift`, `DeepTrance`, `HypnoticCountdown`, `TranceInduction`, `HypnoticVisualization`

### Conversions (4)

`ToInt`, `ToDouble`, `ToString`, `ToBoolean`

A complete list is available via `hypnoscript builtins` and in the Docusaurus documentation.

---

## 🎯 Advanced Language Features

### 🎭 Hypnotic Operator Synonyms

HypnoScript offers 14 hypnotic aliases for standard operators:

| Standard | Hypnotic                  | Description        |
| -------- | ------------------------- | ------------------ |
| `==`     | `youAreFeelingVerySleepy` | Equality           |
| `!=`     | `youCannotResist`         | Inequality         |
| `>`      | `lookAtTheWatch`          | Greater than       |
| `>=`     | `yourEyesAreGettingHeavy` | Greater or equal   |
| `<`      | `fallUnderMySpell`        | Less than          |
| `<=`     | `goingDeeper`             | Less or equal      |
| `&&`     | `underMyControl`          | Logical AND        |
| `\|\|`   | `resistanceIsFutile`      | Logical OR         |
| `!`      | `snapOutOfIt`             | Logical NOT        |
| `??`     | `lucidFallback`           | Nullish coalescing |
| `?.`     | `dreamReach`              | Optional chaining  |

> ⚠️ **String Concatenation:** When one of the operands of the `+` operator is a string, all other values are automatically converted to strings. Examples: `null + "text"` yields `"nulltext"`, `42 + "px"` yields `"42px"`. Check the type before concatenating if you want to avoid such implicit conversions.

**Example:**

```hypnoscript
induce age: number = 25;

if (age yourEyesAreGettingHeavy 18 underMyControl age fallUnderMySpell 65) {
    observe "Adult of working age";
}
```

📚 **Complete Documentation:** [`docs/language-reference/operator-synonyms.md`](hypnoscript-docs/docs/language-reference/operator-synonyms.md)

### 🎯 Pattern Matching (`entrain`/`when`/`otherwise`)

Powerful pattern matching with:

- **Literal Patterns:** Direct value comparison
- **Type Patterns:** Type-based matching with binding
- **Array Destructuring:** Spread operator, nested patterns
- **Record Patterns:** Field-based matching
- **Guards:** Conditional patterns with `if`
- **Identifier Binding:** Variable binding in patterns

**Example:**

```hypnoscript
induce status: number = 404;

induce message: string = entrain status {
    when 200 => "OK"
    when 404 => "Not Found"
    when 500 => "Server Error"
    when s if s yourEyesAreGettingHeavy 400 underMyControl s fallUnderMySpell 500 => "Client Error"
    otherwise => "Unknown"
};

// Array destructuring
induce coords: array = [10, 20, 30];
entrain coords {
    when [x, y, z] => observe "3D Point: " + x + ", " + y + ", " + z
    when [x, y] => observe "2D Point: " + x + ", " + y
    otherwise => observe "Invalid coordinates"
}
```

📚 **Complete Documentation:** [`docs/language-reference/pattern-matching.md`](hypnoscript-docs/docs/language-reference/pattern-matching.md)

### 🔔 Triggers (Event-Driven Callbacks)

Triggers are top-level event handlers that respond to events:

**Syntax:**

```hypnoscript
trigger triggerName = suggestion(parameters) {
    // Handler code
};
```

**Example:**

```hypnoscript
trigger onStartup = suggestion() {
    observe "Application initialized";
};

trigger onError = suggestion(code: number, message: string) {
    observe "Error " + code + ": " + message;
};

trigger onCleanup = suggestion() {
    observe "Cleaning up resources...";
};

entrance {
    onStartup();

    if (someCondition) {
        onError(404, "Resource not found");
    }

    onCleanup();
}
```

**Use Cases:**

- Event handlers (Click, Load, Error)
- Lifecycle hooks (Setup, Teardown)
- Callbacks for async operations
- Observers for state changes

📚 **Complete Documentation:** [`docs/language-reference/triggers.md`](hypnoscript-docs/docs/language-reference/triggers.md)

### 💎 Nullish Operators

**Nullish Coalescing (`lucidFallback` / `??`):**

Returns the right value only when the left value is `null` or `undefined` (not for `0`, `false`, `""`):

```hypnoscript
induce value: number? = null;
induce result: number = value lucidFallback 100;  // 100

induce zero: number = 0;
induce result2: number = zero lucidFallback 100;  // 0 (not 100!)
```

**Optional Chaining (`dreamReach` / `?.`):**

Safe navigation through nested structures:

```hypnoscript
session User {
    expose profile: Profile?;
}

session Profile {
    expose name: string;
}

induce user: User? = getUser();
induce name: string = user dreamReach profile dreamReach name lucidFallback "Anonymous";
```

**Benefits:**

- ✅ Avoids null pointer exceptions
- ✅ More readable than nested `if` checks
- ✅ Functional programming patterns
- ✅ Zero-cost abstraction (compiler-optimized)

📚 **Complete Documentation:** [`docs/language-reference/nullish-operators.md`](hypnoscript-docs/docs/language-reference/nullish-operators.md)

---

## 📊 Performance Benefits

Rust offers several advantages over C#:

1. **Zero-cost Abstractions**: Compile-time optimizations without runtime overhead
2. **No Garbage Collector**: Deterministic memory management
3. **Memory Safety**: Compile-time prevention of common bugs
4. **Smaller Binaries**: 5-10MB vs. 60+MB for C# with runtime
5. **Better Parallelization**: Safe concurrent access via ownership model
6. **Faster Execution**: Native code with LLVM optimizations

---

## 🔧 Development

### Adding New Builtins

1. Add function to the appropriate module in `hypnoscript-runtime/src/`
2. Add tests in the same file
3. Update builtins list in CLI
4. Export from `lib.rs`

Example:

```rust
// In math_builtins.rs
pub fn new_function(x: f64) -> f64 {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_function() {
        assert_eq!(MathBuiltins::new_function(5.0), expected_result);
    }
}
```

### Code Style

- Follow Rust standard style (use `cargo fmt`)
- Run Clippy for linting: `cargo clippy`
- Keep functions focused and well-documented
- Write tests for new functionality

---

## 📝 Migration Status & Features

### Compiler Backend

- ✅ **Interpreter** (100%) – Tree-walking interpreter with full builtin support
- ✅ **Type Checker** (100%) – Static type checking, OOP validation
- ✅ **WASM Text Generator** (100%) – WebAssembly Text Format (.wat)
- ✅ **WASM Binary Generator** (100%) – Direct binary generation (.wasm)
- ✅ **Code Optimizer** (100%) – Constant Folding, Dead Code Elimination, CSE, LICM, Inlining
- 🚧 **Native Code Generator** (20%) – LLVM backend planned

### Core System

- ✅ Core type system (100%)
- ✅ Symbol table (100%)
- ✅ Lexer (100%)
- ✅ Parser (100%)
- ✅ AST (100%)
- ✅ OOP/Sessions (100%)
- ✅ Pattern Matching (`entrain`/`when`/`otherwise`) (100%)
- ✅ Triggers (Event-Driven Callbacks) (100%)
- ✅ Nullish Operators (`lucidFallback`, `dreamReach`) (100%)
- ✅ Hypnotic Operator Synonyms (14 aliases) (100%)

### Runtime

- ✅ Runtime Builtins (180+ functions)
  - Math, String, Array, Collections
  - File I/O, Time/Date, System
  - Hashing, Validation, Statistics
  - Advanced String Operations
  - API/HTTP Helpers
- ✅ Localization (EN, DE, FR, ES)
- ✅ CLI Framework (100%)
- ✅ CI/CD Pipelines (100%)

---

## 🎯 Roadmap

### Completed ✅

- [x] Lexer implementation
- [x] Parser implementation
- [x] Type Checker implementation
- [x] Interpreter implementation
- [x] WASM Text Format Generator (.wat)
- [x] WASM Binary Format Generator (.wasm)
- [x] Code optimization framework
- [x] 180+ builtin functions
- [x] Session/OOP features
- [x] Complete program execution
- [x] CLI integration (10 commands)
- [x] CI/CD pipelines
- [x] Comprehensive tests (300+ tests)
- [x] String interpolation (`"${...}"`)
- [x] Multilingual documentation

### In Development 🚧

- [ ] **Native Code Generator** – LLVM backend for platform-specific binaries
  - Windows (x86_64, ARM64)
  - macOS (x86_64, ARM64/Apple Silicon)
  - Linux (x86_64, ARM64, RISC-V)
- [ ] **Advanced Optimizations** – Complete implementation of all optimization passes
- [ ] **Source Maps** – Debugging support for compiled code

### Planned 🔮

- [ ] JIT compilation
- [ ] Incremental compilation
- [ ] Profile-Guided Optimization (PGO)
- [ ] Link-Time Optimization (LTO)
- [ ] Language Server Protocol (LSP) for IDE integration
- [ ] Advanced WASM features (Threads, SIMD)
- [ ] Additional 40 specialized builtins (Network, ML)
- [ ] Session/OOP features
- [ ] Advanced error handling
- [ ] Performance benchmarking vs. C#
- [ ] Optimization passes

---

## 🐛 Known Limitations

- Some advanced C# builtins still pending (Network, ML features - optional)
- Session/OOP features are optional extensions

---

## 🧭 Migration & Project Status

- ✅ C# codebase removed (all former `.csproj` projects deleted)
- ✅ Rust workspace production-ready
- ✅ Complete port of core functionality
- ✅ All tests passing (300+)
- 🔄 Optional extensions (e.g., Network/ML builtins) possible as roadmap items

---

## 🔗 Links & Resources

- 📘 [Rust Book](https://doc.rust-lang.org/book/)
- 📦 [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- 🧾 Project Docs: `hypnoscript-docs/` (VitePress)
- 🐞 Issues & Discussions: <https://github.com/Kink-Development-Group/hyp-runtime>

---

## 🤝 Contributing

When contributing to the Rust implementation:

1. Maintain API compatibility with the C# version where possible
2. Follow DRY principles (Don't Repeat Yourself)
3. Write comprehensive tests
4. Document public APIs
5. Run `cargo fmt` and `cargo clippy` before committing

---

## 📄 License

MIT License (same as the original project)

---

**_The Rust runtime is production-ready for HypnoScript core programming! 🚀_**

**Enjoy hypnotic programming with Rust!**
