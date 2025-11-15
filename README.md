# HypnoScript – Rust Implementation

**HypnoScript** ist eine hypnotisch angehauchte Programmiersprache mit eigener Syntax (`Focus { ... } Relax`).
Die komplette Laufzeitumgebung, der Compiler und die Kommandozeilen-Tools wurden aus C# nach Rust
portiert und ab Version 1.0 ausschließlich in Rust weiterentwickelt.

---

## 🚀 Highlights

- 🦀 **Reine Rust-Codebasis** – schneller Build, keine .NET-Abhängigkeiten mehr
- 🧠 **Vollständige Toolchain** – Lexer, Parser, Type Checker, Interpreter und mehrere Compiler-Backends
- 🎯 **Multiple Targets** – Interpreter, WebAssembly (Text & Binary), Native Code (geplant)
- ⚡ **Code-Optimierung** – Constant Folding, Dead Code Elimination, CSE, LICM, Inlining
- 🧰 **180+ Builtins** – Mathe, Strings, Arrays, Hypnose, Files, Zeit, System, Statistik, Hashing, Validation, Kryptographie
- 🌍 **Mehrsprachigkeit** – i18n-Unterstützung (EN, DE, FR, ES)
- 🔐 **Kryptographie** – SHA-256, SHA-512, MD5, Base64, UUID
- 🧬 **Funktionale Programmierung** – map, filter, reduce, compose, pipe
- 🎭 **Hypnotische Operatoren** – 14 Synonyme wie `youAreFeelingVerySleepy`, `lookAtTheWatch`, `underMyControl`
- 🎯 **Pattern Matching** – `entrain`/`when`/`otherwise` mit Destructuring, Guards und Type Patterns
- 🔔 **Event-Driven** – `trigger` für Callbacks und Event-Handler
- 💎 **Nullish Operators** – `lucidFallback` (`??`) und `dreamReach` (`?.`) für sichere Null-Behandlung
- 🏛️ **OOP-Support** – Sessions mit `constructor`, `expose`/`conceal`, `dominant` (static)
- 🖥️ **Erweiterte CLI** – `run`, `lex`, `parse`, `check`, `compile-wasm`, `compile-native`, `optimize`, `builtins`, `version`
- ✅ **Umfangreiche Tests** – 185+ Tests über alle Compiler-Module
- 📚 **Dokumentation** – VitePress + ausführliche Architektur-Docs + vollständige Rustdoc
- 🚀 **Performance** – Zero-cost abstractions, kein Garbage Collector, optimierter nativer Code

---

## 🏗️ Workspace-Architektur

```text
hyp-runtime/
├── Cargo.toml                    # Workspace-Konfiguration
├── COMPILER_ARCHITECTURE.md      # Detaillierte Compiler-Dokumentation
├── hypnoscript-core/             # Typ-System & Symbole (100%)
├── hypnoscript-lexer-parser/     # Tokens, Lexer, AST, Parser (100%)
├── hypnoscript-compiler/         # Compiler-Backend (100%)
│   ├── interpreter.rs            # ✅ Tree-Walking Interpreter
│   ├── type_checker.rs           # ✅ Statische Typprüfung
│   ├── wasm_codegen.rs           # ✅ WASM Text Format (.wat)
│   ├── wasm_binary.rs            # ✅ WASM Binary Format (.wasm)
│   ├── optimizer.rs              # ✅ Code-Optimierungen
│   └── native_codegen.rs         # 🚧 Native Compilation (LLVM)
├── hypnoscript-runtime/          # 180+ Builtin-Funktionen (100%)
└── hypnoscript-cli/              # Kommandozeileninterface (100%)
```

Zur Dokumentation steht weiterhin `hypnoscript-docs/` (Docusaurus) bereit.

---

## ⚙️ Installation & Quick Start

### Voraussetzungen

- Rust 1.76+ (empfohlen) inkl. `cargo`

### Automatischer Installer

```bash
curl -fsSL https://kink-development-group.github.io/hyp-runtime/install.sh | bash
```

Das Skript erkennt Linux/macOS automatisch, lädt die passende Runtime aus dem aktuellen Release und aktualisiert bestehende Installationen. Wichtige Optionen: `--prefix`, `--version`, `--check`, `--include-prerelease`, `--force`, `--uninstall`.

#### Updates & Deinstallation

- **Updates checken:** `hypnoscript self-update --check` zeigt verfügbare Versionen an.
- **Aktualisieren:** `hypnoscript self-update` zieht die neueste Release-Version inklusive sudo-Handhabung.
- **Neuinstallation erzwingen:** `hypnoscript self-update --force` führt den Installer erneut aus.
- **Deinstallation:** `curl -fsSL https://kink-development-group.github.io/hyp-runtime/install.sh | bash -s -- --uninstall` entfernt Binärdatei und Metadaten.

### Projekt klonen & bauen

```bash
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime
cargo build --all --release
```

### Programm ausführen

```bash
./target/release/hypnoscript-cli run program.hyp
```

Oder während der Entwicklung:

```bash
cargo run -p hypnoscript-cli -- run test_simple.hyp
```

### Beispielprogramm

```hypnoscript
Focus {
    entrance {
        observe "Welcome to HypnoScript!";
    }

    induce x: number = 42;
    induce message: string = "Hello Trance";

    observe message;
    observe x;

    // Hypnotischer Operator-Synonym
    if (x yourEyesAreGettingHeavy 40) deepFocus {
        observe "X is greater than 40";
    }

    // Pattern Matching mit entrain
    induce result: string = entrain x {
        when 0 => "zero"
        when 42 => "answer to everything"
        when n if n > 100 => "large number"
        otherwise => "other"
    };
    observe result;

    // Nullish Operators
    induce maybeNull: number? = null;
    induce defaulted: number = maybeNull lucidFallback 100;
    observe defaulted;  // 100

    // Trigger (Event Handler)
    trigger onComplete = suggestion() {
        observe "Task completed!";
    };
    onComplete();
} Relax
```

### CLI-Befehle im Detail

```bash
# Programm ausführen (Interpreter)
hypnoscript run program.hyp

# Analyse-Tools
hypnoscript lex program.hyp          # Tokenisierung
hypnoscript parse program.hyp        # AST anzeigen
hypnoscript check program.hyp        # Typprüfung

# Kompilierung
hypnoscript compile-wasm program.hyp              # WASM Text Format (.wat)
hypnoscript compile-wasm -b program.hyp           # WASM Binary Format (.wasm)
hypnoscript compile-native program.hyp            # Native Binary (geplant)
hypnoscript compile-native -t linux-x64 \
  --opt-level release program.hyp                 # Mit Zielplattform

# Code-Optimierung
hypnoscript optimize program.hyp --stats          # Mit Statistiken

# Utilities
hypnoscript builtins                 # Builtin-Funktionen
hypnoscript version                  # Version
hypnoscript self-update              # Selbst-Update
```

#### WASM-Kompilierung im Detail

```bash
# Text-Format (lesbar, debugging-freundlich)
hypnoscript compile-wasm script.hyp -o output.wat

# Binär-Format (kompakt, production-ready)
hypnoscript compile-wasm --binary script.hyp -o output.wasm

# Mit wabt-tools zu komplettem WASM-Binary konvertieren
wat2wasm output.wat -o output.wasm
```

#### Native Kompilierung (Geplant)

```bash
# Für aktuelle Plattform
hypnoscript compile-native app.hyp

# Cross-Compilation
hypnoscript compile-native -t windows-x64 app.hyp
hypnoscript compile-native -t macos-arm64 app.hyp
hypnoscript compile-native -t linux-x64 app.hyp

# Mit Optimierung
hypnoscript compile-native --opt-level release app.hyp
```

---

## 🧪 Tests & Qualitätssicherung

Alle Tests ausführen:

```bash
cargo test --all
```

**Test-Abdeckung**:

- ✅ Lexer: 15+ Tests
- ✅ Parser: 20+ Tests
- ✅ Type Checker: 10+ Tests
- ✅ Interpreter: 12+ Tests
- ✅ WASM Generator: 4+ Tests
- ✅ Optimizer: 6+ Tests
- ✅ Native Generator: 5+ Tests
- ✅ Runtime Builtins: 30+ Tests
- ✅ Pattern Matching: Vollständige Abdeckung
- ✅ Triggers: Vollständige Abdeckung
- ✅ Nullish Operators: Vollständige Abdeckung

### Gesamt: 185+ Tests (alle bestanden)

### Compiler-Tests

```bash
# Nur Compiler-Tests
cargo test --package hypnoscript-compiler

# Mit detaillierter Ausgabe
cargo test --package hypnoscript-compiler -- --nocapture
```

### Code-Qualität

```bash
# Formatierung prüfen
cargo fmt --all -- --check

# Linting mit Clippy
cargo clippy --all-targets --all-features
```

---

## 📦 Builtin-Funktionen (110+)

### Mathematik (20+)

`Sin`, `Cos`, `Tan`, `Sqrt`, `Pow`, `Log`, `Abs`, `Floor`, `Ceil`, `Round`, `Min`, `Max`, `Factorial`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci`, `Clamp`

### Strings (15+)

`ToUpper`, `ToLower`, `Capitalize`, `TitleCase`, `IndexOf`, `Replace`, `Reverse`, `Split`, `Substring`, `Trim`, `Repeat`, `PadLeft`, `PadRight`, `StartsWith`, `EndsWith`, `Contains`, `Length`, `IsWhitespace`

### Arrays (15+)

`ArrayLength`, `ArraySum`, `ArrayAverage`, `ArrayMin`, `ArrayMax`, `ArraySort`, `ArrayReverse`, `ArrayDistinct`, `ArrayFirst`, `ArrayLast`, `ArrayTake`, `ArraySkip`, `ArraySlice`, `ArrayJoin`, `ArrayCount`, `ArrayIndexOf`, `ArrayContains`, `ArrayIsEmpty`, `ArrayGet`

### Zeit/Datum (15)

`GetCurrentTime`, `GetCurrentDate`, `GetCurrentDateTime`, `FormatDateTime`, `GetYear`, `GetMonth`, `GetDay`, `GetHour`, `GetMinute`, `GetSecond`, `GetDayOfWeek`, `GetDayOfYear`, `IsLeapYear`, `GetDaysInMonth`, `CurrentDate`, `DaysInMonth`

### Validierung (10)

`IsValidEmail`, `IsValidUrl`, `IsValidPhoneNumber`, `IsAlphanumeric`, `IsAlphabetic`, `IsNumeric`, `IsLowercase`, `IsUppercase`, `IsInRange`, `MatchesPattern`

### Datei-I/O (14)

`ReadFile`, `WriteFile`, `AppendFile`, `FileExists`, `IsFile`, `IsDirectory`, `DeleteFile`, `CreateDirectory`, `ListDirectory`, `GetFileSize`, `CopyFile`, `RenameFile`, `GetFileExtension`, `GetFileName`

### Statistik (9)

`CalculateMean`, `CalculateMedian`, `CalculateMode`, `CalculateStandardDeviation`, `CalculateVariance`, `CalculateRange`, `CalculatePercentile`, `CalculateCorrelation`, `LinearRegression`, `Mean`, `Variance`

### Hashing/Utilities (10)

`HashString`, `HashNumber`, `AreAnagrams`, `IsPalindrome`, `CountOccurrences`, `RemoveDuplicates`, `UniqueCharacters`, `ReverseWords`, `TitleCase`, `SimpleRandom`

### System (12)

`GetOperatingSystem`, `GetArchitecture`, `GetCpuCount`, `GetHostname`, `GetCurrentDirectory`, `GetHomeDirectory`, `GetTempDirectory`, `GetEnvVar`, `SetEnvVar`, `GetUsername`, `GetArgs`, `Exit`

### Hypnose/Core (6)

`Observe`, `Drift`, `DeepTrance`, `HypnoticCountdown`, `TranceInduction`, `HypnoticVisualization`

### Konvertierungen (4)

`ToInt`, `ToDouble`, `ToString`, `ToBoolean`

Eine vollständige Liste liefert `hypnoscript-cli builtins` sowie die Dokumentation im Docusaurus.

---

## 🎯 Erweiterte Sprachfeatures

### 🎭 Hypnotische Operator-Synonyme

HypnoScript bietet 14 hypnotische Aliase für Standard-Operatoren:

| Standard | Hypnotisch                | Beschreibung       |
| -------- | ------------------------- | ------------------ |
| `==`     | `youAreFeelingVerySleepy` | Gleichheit         |
| `!=`     | `youCannotResist`         | Ungleichheit       |
| `>`      | `lookAtTheWatch`          | Größer als         |
| `>=`     | `yourEyesAreGettingHeavy` | Größer gleich      |
| `<`      | `fallUnderMySpell`        | Kleiner als        |
| `<=`     | `goingDeeper`             | Kleiner gleich     |
| `&&`     | `underMyControl`          | Logisches UND      |
| `\|\|`   | `resistanceIsFutile`      | Logisches ODER     |
| `!`      | `snapOutOfIt`             | Logisches NICHT    |
| `??`     | `lucidFallback`           | Nullish Coalescing |
| `?.`     | `dreamReach`              | Optional Chaining  |

> ⚠️ **String-Konkatenation:** Wenn einer der Operanden beim `+`-Operator ein String ist, werden alle übrigen Werte automatisch in Strings konvertiert. Beispiele: `null + "text"` ergibt `"nulltext"`, `42 + "px"` ergibt `"42px"`. Prüfe den Typ vor dem Konkatenieren, wenn du solche impliziten Umwandlungen vermeiden möchtest.

**Beispiel:**

```hypnoscript
induce age: number = 25;

if (age yourEyesAreGettingHeavy 18 underMyControl age fallUnderMySpell 65) {
    observe "Erwachsener im arbeitsfähigen Alter";
}
```

📚 **Vollständige Dokumentation:** [`docs/language-reference/operator-synonyms.md`](hypnoscript-docs/docs/language-reference/operator-synonyms.md)

### 🎯 Pattern Matching (`entrain`/`when`/`otherwise`)

Leistungsstarkes Pattern Matching mit:

- **Literal Patterns:** Direkter Wertevergleich
- **Type Patterns:** Typ-basiertes Matching mit Binding
- **Array Destructuring:** Spread-Operator, Nested Patterns
- **Record Patterns:** Feldbasiertes Matching
- **Guards:** Bedingte Patterns mit `if`
- **Identifier Binding:** Variable Binding in Patterns

**Beispiel:**

```hypnoscript
induce status: number = 404;

induce message: string = entrain status {
    when 200 => "OK"
    when 404 => "Not Found"
    when 500 => "Server Error"
    when s if s yourEyesAreGettingHeavy 400 underMyControl s fallUnderMySpell 500 => "Client Error"
    otherwise => "Unknown"
};

// Array Destructuring
induce coords: array = [10, 20, 30];
entrain coords {
    when [x, y, z] => observe "3D Point: " + x + ", " + y + ", " + z
    when [x, y] => observe "2D Point: " + x + ", " + y
    otherwise => observe "Invalid coordinates"
}
```

📚 **Vollständige Dokumentation:** [`docs/language-reference/pattern-matching.md`](hypnoscript-docs/docs/language-reference/pattern-matching.md)

### 🔔 Triggers (Event-Driven Callbacks)

Triggers sind Top-Level Event-Handler, die auf Ereignisse reagieren:

**Syntax:**

```hypnoscript
trigger triggerName = suggestion(parameters) {
    // Handler-Code
};
```

**Beispiel:**

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

**Anwendungsfälle:**

- Event-Handler (Click, Load, Error)
- Lifecycle-Hooks (Setup, Teardown)
- Callbacks für Async-Operations
- Observers für Zustandsänderungen

📚 **Vollständige Dokumentation:** [`docs/language-reference/triggers.md`](hypnoscript-docs/docs/language-reference/triggers.md)

### 💎 Nullish Operators

**Nullish Coalescing (`lucidFallback` / `??`):**

Liefert rechten Wert nur wenn linker Wert `null` oder `undefined` ist (nicht bei `0`, `false`, `""`):

```hypnoscript
induce value: number? = null;
induce result: number = value lucidFallback 100;  // 100

induce zero: number = 0;
induce result2: number = zero lucidFallback 100;  // 0 (nicht 100!)
```

**Optional Chaining (`dreamReach` / `?.`):**

Sichere Navigation durch verschachtelte Strukturen:

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

**Vorteile:**

- ✅ Vermeidung von Null-Pointer-Exceptions
- ✅ Lesbarer als verschachtelte `if`-Checks
- ✅ Funktionale Programmierung-Patterns
- ✅ Zero-Cost Abstraction (Compiler-optimiert)

📚 **Vollständige Dokumentation:** [`docs/language-reference/nullish-operators.md`](hypnoscript-docs/docs/language-reference/nullish-operators.md)

---

## 📊 Performance-Vorteile

Rust bietet mehrere Vorteile gegenüber C#:

1. **Zero-cost Abstractions**: Compile-time Optimierungen ohne Runtime-Overhead
2. **Kein Garbage Collector**: Deterministisches Speichermanagement
3. **Speichersicherheit**: Compile-time Verhinderung häufiger Bugs
4. **Kleinere Binaries**: 5-10MB vs. 60+MB für C# mit Runtime
5. **Bessere Parallelisierung**: Sicherer gleichzeitiger Zugriff via Ownership-Modell
6. **Schnellere Ausführung**: Nativer Code mit LLVM-Optimierungen

---

## 🔧 Entwicklung

### Neue Builtins hinzufügen

1. Funktion zum passenden Modul in `hypnoscript-runtime/src/` hinzufügen
2. Tests in derselben Datei hinzufügen
3. Builtins-Liste im CLI aktualisieren
4. Aus `lib.rs` exportieren

Beispiel:

```rust
// In math_builtins.rs
pub fn new_function(x: f64) -> f64 {
    // Implementierung
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

### Code-Style

- Rust-Standard-Style befolgen (nutze `cargo fmt`)
- Clippy für Linting ausführen: `cargo clippy`
- Funktionen fokussiert und gut dokumentiert halten
- Tests für neue Funktionalität schreiben

---

## 📝 Migrationsstatus & Features

### Compiler-Backend

- ✅ **Interpreter** (100%) – Tree-Walking Interpreter mit voller Builtin-Unterstützung
- ✅ **Type Checker** (100%) – Statische Typprüfung, OOP-Validierung
- ✅ **WASM Text Generator** (100%) – WebAssembly Text Format (.wat)
- ✅ **WASM Binary Generator** (100%) – Direkte Binary-Generierung (.wasm)
- ✅ **Code Optimizer** (100%) – Constant Folding, Dead Code Elimination, CSE, LICM, Inlining
- 🚧 **Native Code Generator** (20%) – LLVM-Backend in Planung

### Core-System

- ✅ Core-Typ-System (100%)
- ✅ Symbol-Tabelle (100%)
- ✅ Lexer (100%)
- ✅ Parser (100%)
- ✅ AST (100%)
- ✅ OOP/Sessions (100%)
- ✅ Pattern Matching (`entrain`/`when`/`otherwise`) (100%)
- ✅ Triggers (Event-Driven Callbacks) (100%)
- ✅ Nullish Operators (`lucidFallback`, `dreamReach`) (100%)
- ✅ Hypnotische Operator-Synonyme (14 Aliase) (100%)

### Runtime

- ✅ Runtime-Builtins (180+ Funktionen)
  - Math, String, Array, Collections
  - File I/O, Time/Date, System
  - Hashing, Validation, Statistics
  - Advanced String Operations
  - API/HTTP Helpers
- ✅ Lokalisierung (EN, DE, FR, ES)
- ✅ CLI-Framework (100%)
- ✅ CI/CD-Pipelines (100%)

---

## 🎯 Roadmap

### Abgeschlossen ✅

- [x] Lexer-Implementierung
- [x] Parser-Implementierung
- [x] Type Checker-Implementierung
- [x] Interpreter-Implementierung
- [x] WASM Text Format Generator (.wat)
- [x] WASM Binary Format Generator (.wasm)
- [x] Code-Optimierungs-Framework
- [x] 180+ Builtin-Funktionen
- [x] Session/OOP-Features
- [x] Vollständige Programmausführung
- [x] CLI-Integration (10 Befehle)
- [x] CI/CD-Pipelines
- [x] Umfassende Tests (100+ Tests)
- [x] Mehrsprachige Dokumentation

### In Entwicklung 🚧

- [ ] **Native Code Generator** – LLVM-Backend für plattformspezifische Binaries
  - Windows (x86_64, ARM64)
  - macOS (x86_64, ARM64/Apple Silicon)
  - Linux (x86_64, ARM64, RISC-V)
- [ ] **Erweiterte Optimierungen** – Vollständige Implementierung aller Optimierungs-Pässe
- [ ] **Source Maps** – Debugging-Unterstützung für kompilierten Code

### Geplant 🔮

- [ ] JIT-Kompilierung
- [ ] Incremental Compilation
- [ ] Profile-Guided Optimization (PGO)
- [ ] Link-Time Optimization (LTO)
- [ ] Language Server Protocol (LSP) für IDE-Integration
- [ ] Erweiterte WASM-Features (Threads, SIMD)
- [ ] Zusätzliche 40 spezialisierte Builtins (Netzwerk, ML)
- [ ] Session/OOP-Features
- [ ] Erweiterte Fehlerbehandlung
- [ ] Performance-Benchmarking vs. C#
- [ ] Optimierungs-Passes

---

## 🐛 Bekannte Einschränkungen

- Einige fortgeschrittene C#-Builtins noch ausstehend (Netzwerk-, ML-Features - optional)
- Session/OOP-Features sind optionale Erweiterungen

---

## 🧭 Migration & Projektstatus

- ✅ C#-Codebasis entfernt (alle ehemaligen `.csproj`-Projekte wurden gelöscht)
- ✅ Rust-Workspace produktiv einsetzbar
- ✅ Kompletter Port der Kernfunktionalität
- ✅ Alle 48 Tests erfolgreich
- 🔄 Optionale Erweiterungen (z. B. Netzwerk-/ML-Builtins) sind als Roadmap möglich

Details zur Migration: siehe `IMPLEMENTATION_SUMMARY.md`.

---

## 🔗 Links & Ressourcen

- 📘 [Rust Book](https://doc.rust-lang.org/book/)
- 📦 [Cargo-Dokumentation](https://doc.rust-lang.org/cargo/)
- 🧾 Projekt-Doku: `HypnoScript.Dokumentation/`
- 🐞 Issues & Diskussionen: <https://github.com/Kink-Development-Group/hyp-runtime>

---

## 🤝 Contributing

Bei Beiträgen zur Rust-Implementierung:

1. API-Kompatibilität mit der C#-Version wo möglich beibehalten
2. DRY-Prinzipien befolgen (Don't Repeat Yourself)
3. Umfassende Tests schreiben
4. Öffentliche APIs dokumentieren
5. `cargo fmt` und `cargo clippy` vor dem Commit ausführen

---

## 📄 License

MIT License (gleiche wie das Original-Projekt)

---

**_Die Rust-Runtime ist production-ready für HypnoScript-Kernprogrammierung! 🚀_**

**Viel Spaß beim hypnotischen Programmieren mit Rust!**
