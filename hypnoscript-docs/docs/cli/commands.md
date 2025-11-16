# CLI-Commande

<!-- markdownlint-disable MD024 MD040 -->

Die HypnoScript CLI bietet alle wesentlichen Commande für Entwicklung, Testing und Analyse von HypnoScript-Programmen.

## Overview

```bash
hypnoscript <COMMAND> [OPTIONS]
```

**Verfügbare Commande:**

| Command         | Description                                |
| -------------- | ------------------------------------------- |
| `run`          | Executes ein HypnoScript-Programm aus          |
| `lex`          | Tokenizes eine HypnoScript-File          |
| `parse`        | Shows den AST einer File                   |
| `check`        | Executes Type checking durch                   |
| `compile-wasm` | Kompiliert zu WebAssembly (.wat)            |
| `self-update`  | Checks auf Updates und startet den Installer |
| `version`      | Shows Versionsinformationen                 |
| `builtins`     | Lists alle Builtin Functions              |

## run - Run a program

Executes ein HypnoScript-Programm aus. Dies ist der Hauptbefehl für die Ausführung von .hyp-Fileen.

### Syntax

```bash
hypnoscript run <FILE> [OPTIONS]
```

### Arguments

| Argument | Description        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-File | ✅ Ja        |

### Optionen

| Option      | Kurzform | Description           |
| ----------- | -------- | ---------------------- |
| `--debug`   | `-d`     | Debug-Modus enable |
| `--verbose` | `-v`     | Ausführliche Output   |

### Verhalten

1. **Lexing**: Tokenizes den Quellcode
2. **Parsing**: Creates den AST
3. **Type checking**: Checks Typen (Fehler werden als Warnung ausgegeben)
4. **Execution**: Executes das Programm aus

**Hinweis:** Type-Fehler führen nicht zum Abbruch - das Programm wird trotzdem ausgeführt.

### Examplee

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

## lex - Tokenisierung

Tokenizes eine HypnoScript-File und zeigt alle Token an.

### Syntax

```bash
hypnoscript lex <FILE>
```

### Arguments

| Argument | Description        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-File | ✅ Ja        |

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

## parse - AST anzeigen

Parst eine HypnoScript-File und zeigt den resultierenden Abstract Syntax Tree (AST).

### Syntax

```bash
hypnoscript parse <FILE>
```

### Arguments

| Argument | Description        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-File | ✅ Ja        |

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

Executes Type checking auf einer HypnoScript-File durch und meldet Typ-Fehler.

### Syntax

```bash
hypnoscript check <FILE>
```

### Arguments

| Argument | Description        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-File | ✅ Ja        |

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

## compile-wasm - WebAssembly Generierung

Kompiliert ein HypnoScript-Programm zu WebAssembly Text Format (.wat).

### Syntax

```bash
hypnoscript compile-wasm <INPUT> [OPTIONS]
```

### Arguments

| Argument  | Description               | Erforderlich |
| --------- | -------------------------- | ------------ |
| `<INPUT>` | Pfad zur .hyp-Inputdatei | ✅ Ja        |

### Optionen

| Option     | Kurzform | Description       | Standard      |
| ---------- | -------- | ------------------ | ------------- |
| `--output` | `-o`     | Output-.wat-File | `<input>.wat` |

### Verhalten

1. **Parsing**: Creates AST aus Quellcode
2. **Code Generation**: Generates WASM-Text-Format
3. **Output**: Writes .wat-File

**Hinweis:** Die generierte .wat-File kann mit Tools wie `wat2wasm` zu binärem WASM kompiliert werden.

### Output

```
✅ WASM code written to: output.wat
```

### Examplee

```bash
# Standard-Ausgabe (script.wat)
hypnoscript compile-wasm script.hyp

# Custom Ausgabedatei
hypnoscript compile-wasm script.hyp --output program.wat
hypnoscript compile-wasm script.hyp -o program.wat

# Komplett zu binärem WASM (benötigt wabt)
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

## self-update - Installer aus der CLI starten

Steuert das neue Installationsskript direkt aus der CLI. Die CLI lädt bei Bedarf das `install.sh` from the Release-Assets und führt es mit den gewünschten Optionen aus.

### Syntax

```bash
hypnoscript self-update [OPTIONS]
```

### Optionen

| Option                 | Description                                                             |
| ---------------------- | ------------------------------------------------------------------------ |
| `--check`              | Nur nach Updates suchen (Exit-Code `0` = aktuell, `2` = Update gefunden) |
| `--include-prerelease` | Vorabversionen berücksichtigen                                           |
| `--force`              | Installation erzwingen, selbst wenn Version bereits vorhanden ist        |
| `--quiet`              | Output minimieren (nur Fehler)                                          |
| `--no-sudo`            | Unterdrückt automatische `sudo`-Aufrufe für Systeme ohne Root-Zugriff    |

### Verhalten

1. **Versionen vergleichen:** Aktuelle CLI-Version vs. neueste Release-Tags (inkl. optionaler Prereleases)
2. **Installer finden:** Verwendet vorhandene `installation.json`-Metadaten oder das lokale Release-Archiv (`share/hypnoscript/install.sh`)
3. **Download-Fallback:** Lädt das Installer-Skript aus der Documentation, falls lokal keines gefunden wird
4. **Ausführen:** Startet `install.sh` mit übergebenen Parametersn und übergibt dem Benutzer die Output des Skripts

> **Hinweis:** Auf Windows steht derzeit nur `--check` zur Verfügung. Für die eigentliche Installation nutze weiterhin das Release-Archiv.

### Examplee

```bash
# Nur prüfen, ob Updates verfügbar sind
hypnoscript self-update --check

# Prerelease-Version installieren
hypnoscript self-update --include-prerelease

# Update stumm und ohne sudo ausführen (z.B. CI oder eingeschränkte Shell)
hypnoscript self-update --quiet --no-sudo

# Installation neu erzwingen (z.B. beschädigte Installation reparieren)
hypnoscript self-update --force
```

## version - Versionsinformationen

Shows Versionsinformationen und Features der HypnoScript CLI.

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

- **Version prüfen**: Aktuell installierte Version feststellen
- **Feature-Überblick**: Verfügbare Functionalität anzeigen
- **Debugging**: Version in Bug-Reports angeben

### Example

```bash
hypnoscript version
```

## builtins - Builtin Functions auflisten

Lists alle availableen Builtin Functions der HypnoScript Standard-Bibliothek.

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

- **Referenz**: Schnell nachschlagen welche Functionen available sind
- **Entwicklung**: Entdecken neuer Functionalität
- **Documentation**: Liste für eigene Referenzen

### Example

```bash
# Auflisten
hypnoscript builtins

# Ausgabe in Datei umleiten
hypnoscript builtins > builtin-reference.txt

# Filtern mit grep
hypnoscript builtins | grep "Array"
```

## Globale Optionen

Diese Optionen funktionieren mit allen Commanden:

| Option      | Kurzform | Description         |
| ----------- | -------- | -------------------- |
| `--help`    | `-h`     | Shows Hilfe          |
| `--version` | `-V`     | Shows Version (kurz) |

### Examplee

```bash
# Hilfe für Hauptbefehl
hypnoscript --help

# Hilfe für Unterbefehl
hypnoscript run --help

# Kurzversion
hypnoscript --version
```

## Exit Codes

Die CLI verwendet Standard-Exit-Codes:

| Code | Bedeutung                   |
| ---- | --------------------------- |
| `0`  | Erfolg                      |
| `1`  | Fehler (Parse/Type/Runtime) |

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

### Entwicklungs-Workflow

1. **Schreiben**: Code in .hyp-File schreiben
2. **Prüfen**: `hypnoscript check script.hyp`
3. **Testen**: `hypnoscript run script.hyp --debug`
4. **Optimieren**: Bei Bedarf Code anpassen
5. **Deployen**: Final mit `hypnoscript run script.hyp`

### Debugging-Workflow

1. **Lexing prüfen**: `hypnoscript lex script.hyp`
2. **Check AST**: `hypnoscript parse script.hyp`
3. **Typen prüfen**: `hypnoscript check script.hyp`
4. **Ausführen**: `hypnoscript run script.hyp --debug --verbose`

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

## Tipps & Tricks

### Shell-Aliase

Vereinfache häufige Commande:

```bash
# In ~/.bashrc oder ~/.zshrc
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

### Batch-Verarbeitung

```bash
# Alle .hyp-Dateien prüfen
for file in **/*.hyp; do
    echo "Checking $file..."
    hypnoscript check "$file"
done

# Alle Tests ausführen
for file in tests/*.hyp; do
    echo "Running $file..."
    hypnoscript run "$file"
done
```

### Output Redirection

```bash
# Fehler in Datei schreiben
hypnoscript run script.hyp 2> errors.log

# Ausgabe UND Fehler
hypnoscript run script.hyp &> complete.log

# Nur Fehler anzeigen
hypnoscript run script.hyp 2>&1 >/dev/null
```

## See auch

- [Quick Start](../getting-started/quick-start) - Erste Schritte
- [Debugging](./debugging) - Advanced Debugging-Techniken
- [Configuration](./configuration) - CLI-Konfiguration
- [Builtin Functions](../builtins/overview) - Referenz aller Functionen
