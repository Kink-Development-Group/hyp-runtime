# HypnoScript v1.0.0 – Die hypnotische Programmiersprache

**HypnoScript** ist eine moderne, esoterische Programmiersprache mit TypeScript-inspirierter Syntax und hypnotischem Flair. Sie ist Turing-vollständig, bietet eine umfangreiche Standardbibliothek und richtet sich an Entwickler, die Spaß an innovativen Sprachkonzepten haben.

---

## 🚀 Features (v1.0.0)

- **TypeScript-ähnliche Syntax**: `Focus { ... } Relax`, `induce`, `suggestion`, `session`, `tranceify`
- **150+ Builtins**: Mathe, Strings, Arrays, System, Zeit, Statistik, Hypnose, Netzwerk, Machine Learning
- **Objektorientierung**: Sessions (Klassen), Methoden, Konstruktoren
- **Funktionen & Kontrollstrukturen**: if, while, loop, suggestion, imperative suggestion
- **Erweiterte Features**: Pattern Matching, Time Dilation, Memory Enhancement, Creativity Boost
- **CLI mit 18 Befehlen**: run, compile, analyze, web, api, deploy, monitor, test, docs, benchmark, profile, lint, optimize, ...
- **WASM-Codegenerator**: Kompilierung zu WebAssembly (WAT)
- **Self-contained Binaries**: Für Windows (winget) & Linux (APT)
- **Automatisierte Tests & Doku**: Umfangreiche Testprogramme, Docusaurus-Dokumentation

---

## 🏗️ Architektur

- **HypnoScript.Core**: Typen, Symboltabellen
- **HypnoScript.LexerParser**: Lexer, Parser, AST
- **HypnoScript.Compiler**: TypeChecker, Interpreter, WASM-Codegen
- **HypnoScript.Runtime**: Builtins, Systemfunktionen
- **HypnoScript.CLI**: Kommandozeilen-Interface
- **HypnoScript.Dokumentation**: Docusaurus-Doku

**Beispielprogramme:** im Ordner `examples/` (empfohlen) oder als `test_*.hyp` in der Wurzel

---

## 🛠️ Installation & Quick Start

### Voraussetzungen

- .NET 8.0 SDK oder höher (siehe [Installationsanleitung](HypnoScript.Dokumentation/docs/getting-started/installation.md))

### Installation (Repository)

```bash
git clone <repository-url>
cd hyp-runtime
dotnet build
```

### Quick Start

```bash
dotnet run --project HypnoScript.CLI -- run test_enterprise_v3.hyp
```

### Windows (winget)

```powershell
winget install HypnoScript.HypnoScript
```

### Linux (APT)

```bash
sudo apt update
sudo apt install hypnoscript
```

Weitere Details: [Installationsanleitung](HypnoScript.Dokumentation/docs/getting-started/installation.md)

---

## 📝 CLI-Überblick (Details: [CLI_README.md](CLI_README.md))

```bash
# Programm ausführen
dotnet run -- run <file.hyp> [--debug] [--verbose]
# Zu WASM kompilieren
dotnet run -- compile <file.hyp>
# Statische Analyse
dotnet run -- analyze <file.hyp>
# Web/API/Deploy/Monitor
dotnet run -- web <file.hyp>
# Tests
dotnet run -- test <file.hyp>
# Dokumentation
dotnet run -- docs <file.hyp>
# Hilfe
dotnet run -- help
```

**Alle Befehle und Optionen:** Siehe [CLI_README.md](CLI_README.md)

---

## 📚 Beispiele

### Grundlegendes HypnoScript-Programm

```hypnoscript
Focus {
    entrance {
        observe "Willkommen in HypnoScript!";
    }
    induce greeting: string = "Hello Trance!";
    observe greeting;
    if (true) deepFocus {
        observe "You are feeling very relaxed...";
    }
} Relax
```

### Erweiterte Features, OOP, Machine Learning, Netzwerk

Siehe [Doku-Beispiele](HypnoScript.Dokumentation/docs/examples/basic-examples.md) und [test_*.hyp]

---

## 🔧 Builtin-Überblick

- **Mathematik**: Sin, Cos, Tan, Sqrt, Pow, Log, Random, Factorial, GCD, LCM, ...
- **Strings**: Length, ToUpper, ToLower, Trim, IndexOf, Replace, Reverse, Capitalize, ...
- **Arrays**: ArrayLength, ArrayGet, ArraySet, ArraySort, ArrayMap, ArrayReduce, ...
- **System**: GetCurrentDirectory, GetMachineName, GetUserName, ...
- **Zeit/Datum**: GetCurrentTime, FormatDateTime, IsLeapYear, ...
- **Statistik/ML**: CalculateMean, CalculateStandardDeviation, LinearRegression
- **Hypnose**: DeepTrance, HypnoticSuggestion, HypnoticPatternMatching, ...
- **Netzwerk**: HttpGet, HttpPost
- **Datenbank**: CreateRecord, GetRecordValue, ...
- **Validierung**: IsValidEmail, IsValidUrl, ...

**Vollständige Liste:** [Doku Builtins](HypnoScript.Dokumentation/docs/builtins/overview.md)

---

## 🏗️ Build & Distribution

- **Windows:** `dotnet publish HypnoScript.CLI -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o ./publish/win`
- **Linux:** `dotnet publish HypnoScript.CLI -c Release -r linux-x64 --self-contained true -p:PublishSingleFile=true -o ./publish/linux`
- **Paketierung:** Siehe [scripts/README.md](scripts/README.md)

---

## 💡 Best Practices & Roadmap

- **Projektstruktur:** Trenne Quellcode, Tests, Skripte, Doku, Beispiele
- **Automatisierung:** Nutze CI/CD für Build, Test, Release, Doku-Deployment
- **Erweiterbarkeit:** CLI und Builtins sind modular – eigene Erweiterungen möglich
- **Doku:** Halte Readmes und Builtin-Listen synchron (ggf. automatisiert)
- **Roadmap:**
  1. Web-Interface
  2. Package Manager
  3. IDE-Integration
  4. Cloud-Deployment
  5. Erweiterte ML/AI-Features

---

## 🛠️ Troubleshooting & Support

- **.NET nicht gefunden:** Prüfe mit `dotnet --version` (siehe [Installationsanleitung](HypnoScript.Dokumentation/docs/getting-started/installation.md))
- **Build-Fehler:** `dotnet restore`, `dotnet clean`, `dotnet build`
- **Pfade:** Achte auf plattformübergreifende Pfade in Skripten und Doku
- **Support:**
  - [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
  - [Discussions](https://github.com/Kink-Development-Group/hyp-runtime/discussions)
  - [Doku Troubleshooting](HypnoScript.Dokumentation/docs/development/debugging.md)

---

## 🔗 Weiterführende Links

- **Doku:** [HypnoScript.Dokumentation/README.md](HypnoScript.Dokumentation/README.md)
- **Online-Doku:** <https://hypnoscript.github.io/hyp-runtime/>
- **CLI-Details:** [CLI_README.md](CLI_README.md)
- **Build/Paketierung:** [scripts/README.md](scripts/README.md)
- **Lizenz:** MIT ([LICENSE](LICENSE))

---

## Automatisierte Dokumentation, CI/CD und Testabdeckung

- Die Builtin-Dokumentation wird automatisch aus dem Code generiert und mit der Doku synchronisiert.
- Die CI/CD-Pipeline (GitHub Actions) baut, testet und released automatisch für Windows und Linux.
- Testabdeckung und Monitoring werden kontinuierlich ausgebaut.
- Fehlerbehandlung und Logging folgen Best Practices für Zuverlässigkeit und Wartbarkeit.

---

**Bereit für die hypnotische Programmierung?**
