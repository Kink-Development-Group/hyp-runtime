# CLI Commands

<!-- markdownlint-disable MD024 MD040 -->

The HypnoScript CLI provides all essential commands for development, testing, and analysis of HypnoScript programs.

## Overview

```bash
hypnoscript <COMMAND> [OPTIONS]
```

**Available Commands:**

| Command        | Description                                 |
| -------------- | ------------------------------------------- |
| `run`          | Executes a HypnoScript program              |
| `lex`          | Tokenizes a HypnoScript file                |
| `parse`        | Shows the AST of a file                     |
| `check`        | Performs type checking                      |
| `compile-wasm` | Compiles to WebAssembly (.wat)              |
| `self-update`  | Checks for updates and starts the installer |
| `version`      | Shows version information                   |
| `builtins`     | Lists all builtin functions                 |

## run - Run a program

Executes a HypnoScript program. This is the main command for running .hyp files.

### Syntax

```bash
hypnoscript run <FILE> [OPTIONS]
```

### Arguments

| Argument | Description           | Required |
| -------- | --------------------- | -------- |
| `<FILE>` | Path to the .hyp file | ✅ Yes   |

### Options

| Option      | Short | Description       |
| ----------- | ----- | ----------------- |
| `--debug`   | `-d`  | Enable debug mode |
| `--verbose` | `-v`  | Verbose output    |

### Behavior

1. **Lexing**: Tokenizes the source code
2. **Parsing**: Creates the AST
3. **Type checking**: Checks types (errors are output as warnings)
4. **Execution**: Executes the program

**Note:** Type errors do not cause termination - the program is executed anyway.

### Examples

```bash
# Einfache Ausführung
hypnoscript run hello.hyp

# Mit Debug-Modus
hypnoscript run script.hyp --debug

# Mit detaillierter Ausgabe
hypnoscript run complex.hyp --verbose

# Beide Optionen kombiniert
hypnoscript run test.hyp -d -v
```

### Debug-Modus Output

Im Debug-Modus werden zusätzliche Informationen ausgegeben:

```
Running file: script.hyp
Source code:
Focus { ... }

--- Lexing ---
Tokens: 42

--- Type Checking ---

--- Executing ---
<Programm-Ausgabe>

✅ Program executed successfully!
```

## lex - Tokenization

Tokenizes a HypnoScript file and displays all tokens.

### Syntax

```bash
hypnoscript lex <FILE>
```

### Arguments

| Argument | Description           | Required |
| -------- | --------------------- | -------- |
| `<FILE>` | Path to the .hyp file | ✅ Yes   |

### Output

Lists alle Token mit Index und Typ:

```
=== Tokens ===
   0: Token { token_type: Focus, lexeme: "Focus", line: 1, column: 1 }
   1: Token { token_type: LBrace, lexeme: "{", line: 1, column: 7 }
   2: Token { token_type: Observe, lexeme: "observe", line: 2, column: 5 }
   ...

Total tokens: 42
```

### Usage

- **Syntax-Debugging**: Verstehen wie der Lexer Code interpretiert
- **Token-Analyse**: Prüfen ob Schlüsselwörter korrekt erkannt werden
- **Lernzwecke**: Verstehen wie HypnoScript-Code tokenisiert wird

### Example

```bash
hypnoscript lex examples/01_hello_trance.hyp
```

## parse - Show AST

Parses a HypnoScript file and displays the resulting Abstract Syntax Tree (AST).

### Syntax

```bash
hypnoscript parse <FILE>
```

### Arguments

| Argument | Description           | Required |
| -------- | --------------------- | -------- |
| `<FILE>` | Path to the .hyp file | ✅ Yes   |

### Output

Shows den AST in formatierter Form:

```
=== AST ===
Program([
    FocusBlock([
        ObserveStatement(
            StringLiteral("Hallo Welt")
        ),
        VariableDeclaration {
            name: "x",
            type_annotation: Some("number"),
            initializer: Some(NumberLiteral(42.0)),
            is_constant: false
        }
    ])
])
```

### Usage

- **Struktur-Analyse**: Verstehen wie Code geparst wird
- **Compiler-Debugging**: Probleme im Parser identifizieren
- **Entwicklung**: AST-Struktur für Compiler-Erweiterungen verstehen

### Example

```bash
hypnoscript parse examples/02_variables_arithmetic.hyp
```

## check - Type checking

Performs type checking on a HypnoScript file and reports type errors.

### Syntax

```bash
hypnoscript check <FILE>
```

### Arguments

| Argument | Description           | Required |
| -------- | --------------------- | -------- |
| `<FILE>` | Path to the .hyp file | ✅ Yes   |

### Output

**Ohne Fehler:**

```
✅ No type errors found!
```

**Mit Fehlern:**

```
❌ Type errors found:
  - Variable 'x' used before declaration at line 5
  - Type mismatch: expected number, got string at line 8
  - Function 'unknown' not defined at line 12
```

### Type checking Regeln

Der Type Checker prüft:

- ✅ Variablendeklarationen
- ✅ Functionsaufrufe und -signaturen
- ✅ Typ-Kompatibilität in Zuweisungen
- ✅ Array-Typen
- ✅ Session-Member-Zugriffe
- ✅ Return-Statement Typen

### Usage

- **Vor Deployment**: Typ-Fehler frühzeitig finden
- **Entwicklung**: Code-Qualität sicherstellen
- **CI/CD**: Als Teil der Build-Pipeline

### Example

```bash
hypnoscript check src/main.hyp

# In CI/CD Pipeline
hypnoscript check **/*.hyp
if [ $? -eq 0 ]; then
    echo "Type check passed"
else
    echo "Type check failed"
    exit 1
fi
```

## compile-wasm - WebAssembly Generation

Compiles a HypnoScript program to WebAssembly Text Format (.wat).

### Syntax

```bash
hypnoscript compile-wasm <INPUT> [OPTIONS]
```

### Arguments

| Argument  | Description                 | Required |
| --------- | --------------------------- | -------- |
| `<INPUT>` | Path to the .hyp input file | ✅ Yes   |

### Options

| Option     | Short | Description      | Default       |
| ---------- | ----- | ---------------- | ------------- |
| `--output` | `-o`  | Output .wat file | `<input>.wat` |

### Behavior

1. **Parsing**: Creates AST from source code
2. **Code Generation**: Generates WASM text format
3. **Output**: Writes .wat file

**Note:** The generated .wat file can be compiled to binary WASM with tools like `wat2wasm`.

### Output

```
✅ WASM code written to: output.wat
```

### Examples

```bash
# Standard output (script.wat)
hypnoscript compile-wasm script.hyp

# Custom output file
hypnoscript compile-wasm script.hyp --output program.wat
hypnoscript compile-wasm script.hyp -o program.wat

# Complete to binary WASM (requires wabt)
hypnoscript compile-wasm script.hyp
wat2wasm script.wat -o script.wasm
```

### WASM-Integration

Nach Kompilierung kann das WASM-Modul in verschiedenen Umgebungen verwendet werden:

**Web (JavaScript):**

```javascript
WebAssembly.instantiateStreaming(fetch('script.wasm')).then((module) => {
  // Nutze exportierte Funktionen
});
```

**Node.js:**

```javascript
const fs = require('fs');
const bytes = fs.readFileSync('script.wasm');
const module = await WebAssembly.instantiate(bytes);
```

## self-update - Start installer from the CLI

Controls the new installation script directly from the CLI. The CLI loads the `install.sh` from the release assets if needed and runs it with the desired options.

### Syntax

```bash
hypnoscript self-update [OPTIONS]
```

### Options

| Option                 | Description                                                          |
| ---------------------- | -------------------------------------------------------------------- |
| `--check`              | Only check for updates (Exit code `0` = current, `2` = update found) |
| `--include-prerelease` | Consider pre-releases                                                |
| `--force`              | Force installation even if version is already present                |
| `--quiet`              | Minimize output (only errors)                                        |
| `--no-sudo`            | Suppresses automatic `sudo` calls for systems without root access    |

### Behavior

1. **Compare versions:** Current CLI version vs. latest release tags (including optional prereleases)
2. **Find installer:** Uses existing `installation.json` metadata or the local release archive (`share/hypnoscript/install.sh`)
3. **Download fallback:** Downloads the installer script from the documentation if none is found locally
4. **Execute:** Starts `install.sh` with passed parameters and gives the user the script's output

> **Note:** On Windows, currently only `--check` is available. For the actual installation, continue to use the release archive.

### Examples

```bash
# Only check if updates are available
hypnoscript self-update --check

# Install prerelease version
hypnoscript self-update --include-prerelease

# Run update silently and without sudo (e.g., CI or restricted shell)
hypnoscript self-update --quiet --no-sudo

# Force reinstallation (e.g., repair corrupted installation)
hypnoscript self-update --force
```

## version - Version information

Shows version information and features of the HypnoScript CLI.

### Syntax

```bash
hypnoscript version
```

### Output

```
HypnoScript v1.0.0
The Hypnotic Programming Language

Migrated from C# to Rust for improved performance

Features:
  - Full parser and interpreter
  - Type checker
  - WASM code generation
  - 110+ builtin functions
```

### Usage

- **Check version**: Determine currently installed version
- **Feature overview**: Display available functionality
- **Debugging**: Specify version in bug reports

### Example

```bash
hypnoscript version
```

## builtins - List builtin functions

Lists all available builtin functions of the HypnoScript standard library.

### Syntax

```bash
hypnoscript builtins
```

### Output

```
=== HypnoScript Builtin Functions ===

📊 Math Builtins:
  - Sin, Cos, Tan, Sqrt, Pow, Log, Log10
  - Abs, Floor, Ceil, Round, Min, Max
  - Factorial, Gcd, Lcm, IsPrime, Fibonacci
  - Clamp

📝 String Builtins:
  - Length, ToUpper, ToLower, Trim
  - IndexOf, Replace, Reverse, Capitalize
  - StartsWith, EndsWith, Contains
  - Split, Substring, Repeat
  - PadLeft, PadRight

📦 Array Builtins:
  - Length, IsEmpty, Get, IndexOf, Contains
  - Reverse, Sum, Average, Min, Max, Sort
  - First, Last, Take, Skip, Slice
  - Join, Count, Distinct

✨ Hypnotic Builtins:
  - observe (output)
  - drift (sleep)
  - DeepTrance
  - HypnoticCountdown
  - TranceInduction
  - HypnoticVisualization

🔄 Conversion Functions:
  - ToInt, ToDouble, ToString, ToBoolean

Total: 50+ builtin functions implemented
```

### Usage

- **Reference**: Quickly look up which functions are available
- **Development**: Discover new functionality
- **Documentation**: List for your own references

### Example

```bash
# List
hypnoscript builtins

# Redirect output to file
hypnoscript builtins > builtin-reference.txt

# Filter with grep
hypnoscript builtins | grep "Array"
```

## Global Options

These options work with all commands:

| Option      | Short | Description          |
| ----------- | ----- | -------------------- |
| `--help`    | `-h`  | Show help            |
| `--version` | `-V`  | Show version (short) |

### Examples

```bash
# Help for main command
hypnoscript --help

# Help for subcommand
hypnoscript run --help

# Short version
hypnoscript --version
```

## Exit Codes

The CLI uses standard exit codes:

| Code | Meaning                    |
| ---- | -------------------------- |
| `0`  | Success                    |
| `1`  | Error (Parse/Type/Runtime) |

### Usage in Scripts

```bash
#!/bin/bash

hypnoscript check script.hyp
if [ $? -eq 0 ]; then
    hypnoscript run script.hyp
else
    echo "Type check failed!"
    exit 1
fi
```

## Best Practices

### Development Workflow

1. **Write**: Write code in .hyp file
2. **Check**: `hypnoscript check script.hyp`
3. **Test**: `hypnoscript run script.hyp --debug`
4. **Optimize**: Adjust code if needed
5. **Deploy**: Final with `hypnoscript run script.hyp`

### Debugging Workflow

1. **Check lexing**: `hypnoscript lex script.hyp`
2. **Check AST**: `hypnoscript parse script.hyp`
3. **Check types**: `hypnoscript check script.hyp`
4. **Execute**: `hypnoscript run script.hyp --debug --verbose`

### CI/CD Integration

```yaml
# GitHub Actions Beispiel
steps:
  - name: Install HypnoScript
    run: cargo install --path hypnoscript-cli

  - name: Type Check
    run: hypnoscript check src/**/*.hyp

  - name: Run Tests
    run: |
      for file in tests/*.hyp; do
        hypnoscript run "$file"
      done

  - name: Build WASM
    run: hypnoscript compile-wasm src/main.hyp -o dist/app.wat
```

## Tips & Tricks

### Shell Aliases

Simplify frequent commands:

```bash
# In ~/.bashrc or ~/.zshrc
alias hyp='hypnoscript'
alias hyp-run='hypnoscript run'
alias hyp-check='hypnoscript check'
alias hyp-wasm='hypnoscript compile-wasm'
```

Usage:

```bash
hyp run script.hyp
hyp-check script.hyp
hyp-wasm script.hyp
```

### Batch Processing

```bash
# Check all .hyp files
for file in **/*.hyp; do
    echo "Checking $file..."
    hypnoscript check "$file"
done

# Run all tests
for file in tests/*.hyp; do
    echo "Running $file..."
    hypnoscript run "$file"
done
```

### Output Redirection

```bash
# Write errors to file
hypnoscript run script.hyp 2> errors.log

# Output AND errors
hypnoscript run script.hyp &> complete.log

# Show only errors
hypnoscript run script.hyp 2>&1 >/dev/null
```

## See Also

- [Quick Start](../getting-started/quick-start) - Getting started
- [Debugging](./debugging) - Advanced debugging techniques
- [Configuration](./configuration) - CLI configuration
- [Builtin Functions](../builtins/overview) - Reference of all functions
