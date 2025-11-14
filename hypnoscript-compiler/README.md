# HypnoScript Compiler

Der vollständige Compiler und Interpreter für die HypnoScript-Programmiersprache.

## Features

### 🎯 Mehrere Backends

1. **Interpreter** - Direktes Ausführen von HypnoScript-Code

   - Vollständige Sprachunterstützung
   - OOP mit Sessions (Klassen)
   - Integrierte Built-in-Funktionen
   - Ideal für Entwicklung und Debugging

2. **Native Code Generator** (Cranelift)

   - Plattformspezifischer Maschinencode
   - Automatisches Linking zu ausführbaren Binaries
   - Unterstützte Plattformen:
     - Windows (x86_64, ARM64) - benötigt Visual Studio Build Tools, GCC oder Clang
     - macOS (x86_64, ARM64/Apple Silicon) - benötigt Xcode Command Line Tools
     - Linux (x86_64, ARM64, RISC-V) - benötigt GCC oder Clang
   - Optimierte Binaries mit verschiedenen Optimierungsstufen
   - Schneller Build-Prozess im Vergleich zu LLVM
   - ✅ **Vollständig funktionsfähig** - erzeugt ausführbare .exe/.bin Dateien

3. **WebAssembly Generator**
   - Text Format (.wat) - menschenlesbar
   - Binary Format (.wasm) - kompakt
   - Browser- und Server-Unterstützung
   - Sandboxed Execution

### 🔧 Zusätzliche Features

- **Type Checker**: Statische Typprüfung
- **Optimizer**: Code-Optimierungen
  - Constant Folding
  - Dead Code Elimination
  - Common Subexpression Elimination
  - Loop Invariant Code Motion
  - Function Inlining

## Verwendung

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

### Native Kompilierung

```rust
use hypnoscript_compiler::{NativeCodeGenerator, OptimizationLevel, TargetPlatform};

let mut generator = NativeCodeGenerator::new();
generator.set_target_platform(TargetPlatform::LinuxX64);
generator.set_optimization_level(OptimizationLevel::Release);

let binary_path = generator.generate(&ast)?;
```

### WASM-Generierung

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

## Architektur

### Design-Prinzipien

- **OOP First**: Sessions als vollwertige Klassen mit Kapselung
- **DRY**: Keine Code-Duplizierung, gemeinsame Infrastruktur
- **Type Safety**: Statische Typprüfung vor der Ausführung
- **Memory Safety**: 100% Rust, keine unsicheren Operationen

### Module

```text
hypnoscript-compiler/
├── src/
│   ├── lib.rs              # Public API
│   ├── interpreter.rs      # Runtime-Interpreter (2392 Zeilen)
│   ├── type_checker.rs     # Statische Typprüfung (1683 Zeilen)
│   ├── optimizer.rs        # Code-Optimierungen (421 Zeilen)
│   ├── native_codegen.rs   # Cranelift-Backend mit Auto-Linking
│   ├── wasm_codegen.rs     # WASM Text Generator
│   └── wasm_binary.rs      # WASM Binary Generator
└── Cargo.toml
```

## Performance-Vergleich

| Backend            | Kompilierzeit | Ausführungszeit      | Binary-Größe | Use Case               |
| ------------------ | ------------- | -------------------- | ------------ | ---------------------- |
| Interpreter        | Sofort        | ~10x langsamer       | N/A          | Entwicklung, Debugging |
| Native (Cranelift) | ~1-2 Sekunden | Nativ (sehr schnell) | 50-200 KB    | Produktion, Server     |
| WASM               | ~50ms         | ~2x langsamer        | 10-50 KB     | Web, Embedding         |

## Systemvoraussetzungen für Native Kompilierung

### Windows

- Visual Studio Build Tools (empfohlen) ODER
- MinGW-w64/GCC ODER
- LLVM/Clang

### macOS

- Xcode Command Line Tools (`xcode-select --install`)

### Linux

- GCC (`sudo apt install build-essential`) ODER
- Clang (`sudo apt install clang`)

## Dependencies

- `cranelift`: Native Code-Generierung
- `hypnoscript-core`: Gemeinsame Typen und Symbol-Tables
- `hypnoscript-lexer-parser`: AST und Parser
- `hypnoscript-runtime`: Built-in-Funktionen

## Tests

```bash
cargo test --package hypnoscript-compiler
```

Aktueller Stand: **34 Tests, alle erfolgreich** ✅

## Beispiel: Native Kompilierung

```bash
# Kompiliere HypnoScript zu nativer ausführbarer Datei
hypnoscript compile-native mein_programm.hyp

# Mit Optimierung
hypnoscript compile-native mein_programm.hyp --opt-level release

# Spezifisches Output
hypnoscript compile-native mein_programm.hyp --output mein_programm.exe
```

## Roadmap

- [x] Interpreter mit vollständiger Sprachunterstützung
- [x] Type Checker
- [x] WASM Text Format (.wat)
- [x] WASM Binary Format (.wasm)
- [x] Native Code-Generator mit Cranelift
- [x] Automatisches Linking zu ausführbaren Binaries
- [x] Code-Optimierungen
- [ ] Advanced Control Flow in WASM
- [ ] Vollständige Session-Unterstützung in Native/WASM
- [ ] Debugging-Informationen (DWARF)
- [ ] Cross-Compilation

## Lizenz

MIT
