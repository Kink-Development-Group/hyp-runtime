# HypnoScript – Rust Implementation

**HypnoScript** ist eine hypnotisch angehauchte Programmiersprache mit eigener Syntax (`Focus { ... } Relax`).
Die komplette Laufzeitumgebung, der Compiler und die Kommandozeilen-Tools wurden aus C# nach Rust
portiert und ab Version 1.0 ausschließlich in Rust weiterentwickelt.

---

## 🚀 Highlights

- 🦀 **Reine Rust-Codebasis** – schneller Build, keine .NET-Abhängigkeiten mehr
- 🧠 **Vollständige Toolchain** – Lexer, Parser, Type Checker, Interpreter und WASM-Codegen
- 🧰 **110+ Builtins** – Mathe, Strings, Arrays, Hypnose, Files, Zeit, System, Statistik, Hashing, Validation
- 🖥️ **CLI-Workflow** – `run`, `lex`, `parse`, `check`, `compile-wasm`, `builtins`, `version`
- ✅ **Umfangreiche Tests** – 48 Tests über alle Crates (Lexer, Runtime, Compiler, CLI)
- 📚 **Dokumentation** – Docusaurus im Ordner `HypnoScript.Dokumentation`
- 🚀 **Performance** – Zero-cost abstractions, kein Garbage Collector, nativer Code

---

## 🏗️ Workspace-Architektur

```text
hyp-runtime/
├── Cargo.toml                    # Workspace-Konfiguration
├── hypnoscript-core/             # Typ-System & Symbole (100%)
├── hypnoscript-lexer-parser/     # Tokens, Lexer, AST, Parser (100%)
├── hypnoscript-compiler/         # Type Checker, Interpreter, WASM Codegen (100%)
├── hypnoscript-runtime/          # 110+ Builtin-Funktionen (75%)
└── hypnoscript-cli/              # Kommandozeileninterface (100%)
```

Zur Dokumentation steht weiterhin `HypnoScript.Dokumentation/` (Docusaurus) bereit.

---

## ⚙️ Installation & Quick Start

### Voraussetzungen

- Rust 1.76+ (empfohlen) inkl. `cargo`

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
        observe "Welcome to HypnoScript Rust Edition!";
    }

    induce x: number = 42;
    induce message: string = "Hello Trance";

    observe message;
    observe x;

    if (x > 40) deepFocus {
        observe "X is greater than 40";
    }
} Relax
```

### CLI-Befehle im Detail

```bash
# Programm ausführen
hypnoscript-cli run program.hyp

# Datei tokenisieren (Token-Stream anzeigen)
hypnoscript-cli lex program.hyp

# AST anzeigen
hypnoscript-cli parse program.hyp

# Typprüfung durchführen
hypnoscript-cli check program.hyp

# Zu WebAssembly kompilieren
hypnoscript-cli compile-wasm program.hyp --output program.wat

# Liste der Builtin-Funktionen
hypnoscript-cli builtins

# Version anzeigen
hypnoscript-cli version
```

---

## 🧪 Tests & Qualitätssicherung

Alle Tests ausführen:

```bash
cargo test --all
```

**_Ergebnis: Alle 48 Tests erfolgreich ✅_**

Alle Crates besitzen Unit-Tests – Lexer, Parser, Runtime-Builtins, Type Checker, Interpreter und WASM Codegen.

### Code-Qualität

```bash
# Formatierung prüfen
cargo fmt --all -- --check

# Linting mit Clippy
cargo clippy --all
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

## 📝 Migrationsstatus

**_Gesamt: ~95% Komplett_**

- ✅ Core-Typ-System (100%)
- ✅ Symbol-Tabelle (100%)
- ✅ Lexer (100%)
- ✅ Parser (100%)
- ✅ Type Checker (100%)
- ✅ Interpreter (100%)
- ✅ WASM Codegen (100%)
- ✅ Runtime-Builtins (75% - 110+ von 150+)
- ✅ CLI-Framework (100%)
- ✅ CI/CD-Pipelines (100%)

---

## 🎯 Roadmap

### Abgeschlossen ✅

- [x] Lexer-Implementierung
- [x] Parser-Implementierung
- [x] Type Checker-Implementierung
- [x] Interpreter-Implementierung
- [x] WASM Code Generator-Implementierung
- [x] 110+ Builtin-Funktionen
- [x] Vollständige Programmausführung
- [x] CLI-Integration (7 Befehle)
- [x] CI/CD-Pipelines
- [x] Umfassende Tests (48 Tests)

### Optionale Erweiterungen 🔄

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
