# CLI-Befehle

<!-- markdownlint-disable MD024 MD040 -->

Die HypnoScript CLI (Rust Edition) bietet alle wesentlichen Befehle für Entwicklung, Testing und Analyse von HypnoScript-Programmen.

## Übersicht

```bash
hypnoscript <COMMAND> [OPTIONS]
```

**Verfügbare Befehle:**

| Befehl         | Beschreibung                                |
| -------------- | ------------------------------------------- |
| `run`          | Führt ein HypnoScript-Programm aus          |
| `lex`          | Tokenisiert eine HypnoScript-Datei          |
| `parse`        | Zeigt den AST einer Datei                   |
| `check`        | Führt Type Checking durch                   |
| `compile-wasm` | Kompiliert zu WebAssembly (.wat)            |
| `self-update`  | Prüft auf Updates und startet den Installer |
| `version`      | Zeigt Versionsinformationen                 |
| `builtins`     | Listet alle Builtin-Funktionen              |

## run - Programm ausführen

Führt ein HypnoScript-Programm aus. Dies ist der Hauptbefehl für die Ausführung von .hyp-Dateien.

### Syntax

```bash
hypnoscript run <FILE> [OPTIONS]
```

### Argumente

| Argument | Beschreibung        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-Datei | ✅ Ja        |

### Optionen

| Option      | Kurzform | Beschreibung           |
| ----------- | -------- | ---------------------- |
| `--debug`   | `-d`     | Debug-Modus aktivieren |
| `--verbose` | `-v`     | Ausführliche Ausgabe   |

### Verhalten

1. **Lexing**: Tokenisiert den Quellcode
2. **Parsing**: Erstellt den AST
3. **Type Checking**: Prüft Typen (Fehler werden als Warnung ausgegeben)
4. **Execution**: Führt das Programm aus

**Hinweis:** Type-Fehler führen nicht zum Abbruch - das Programm wird trotzdem ausgeführt.

### Beispiele

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

### Debug-Modus Ausgabe

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

Tokenisiert eine HypnoScript-Datei und zeigt alle Token an.

### Syntax

```bash
hypnoscript lex <FILE>
```

### Argumente

| Argument | Beschreibung        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-Datei | ✅ Ja        |

### Ausgabe

Listet alle Token mit Index und Typ:

```
=== Tokens ===
   0: Token { token_type: Focus, lexeme: "Focus", line: 1, column: 1 }
   1: Token { token_type: LBrace, lexeme: "{", line: 1, column: 7 }
   2: Token { token_type: Observe, lexeme: "observe", line: 2, column: 5 }
   ...

Total tokens: 42
```

### Verwendung

- **Syntax-Debugging**: Verstehen wie der Lexer Code interpretiert
- **Token-Analyse**: Prüfen ob Schlüsselwörter korrekt erkannt werden
- **Lernzwecke**: Verstehen wie HypnoScript-Code tokenisiert wird

### Beispiel

```bash
hypnoscript lex examples/01_hello_trance.hyp
```

## parse - AST anzeigen

Parst eine HypnoScript-Datei und zeigt den resultierenden Abstract Syntax Tree (AST).

### Syntax

```bash
hypnoscript parse <FILE>
```

### Argumente

| Argument | Beschreibung        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-Datei | ✅ Ja        |

### Ausgabe

Zeigt den AST in formatierter Form:

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

### Verwendung

- **Struktur-Analyse**: Verstehen wie Code geparst wird
- **Compiler-Debugging**: Probleme im Parser identifizieren
- **Entwicklung**: AST-Struktur für Compiler-Erweiterungen verstehen

### Beispiel

```bash
hypnoscript parse examples/02_variables_arithmetic.hyp
```

## check - Type Checking

Führt Type Checking auf einer HypnoScript-Datei durch und meldet Typ-Fehler.

### Syntax

```bash
hypnoscript check <FILE>
```

### Argumente

| Argument | Beschreibung        | Erforderlich |
| -------- | ------------------- | ------------ |
| `<FILE>` | Pfad zur .hyp-Datei | ✅ Ja        |

### Ausgabe

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

### Type Checking Regeln

Der Type Checker prüft:

- ✅ Variablendeklarationen
- ✅ Funktionsaufrufe und -signaturen
- ✅ Typ-Kompatibilität in Zuweisungen
- ✅ Array-Typen
- ✅ Session-Member-Zugriffe
- ✅ Return-Statement Typen

### Verwendung

- **Vor Deployment**: Typ-Fehler frühzeitig finden
- **Entwicklung**: Code-Qualität sicherstellen
- **CI/CD**: Als Teil der Build-Pipeline

### Beispiel

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

### Argumente

| Argument  | Beschreibung               | Erforderlich |
| --------- | -------------------------- | ------------ |
| `<INPUT>` | Pfad zur .hyp-Eingabedatei | ✅ Ja        |

### Optionen

| Option     | Kurzform | Beschreibung       | Standard      |
| ---------- | -------- | ------------------ | ------------- |
| `--output` | `-o`     | Ausgabe-.wat-Datei | `<input>.wat` |

### Verhalten

1. **Parsing**: Erstellt AST aus Quellcode
2. **Code Generation**: Generiert WASM-Text-Format
3. **Ausgabe**: Schreibt .wat-Datei

**Hinweis:** Die generierte .wat-Datei kann mit Tools wie `wat2wasm` zu binärem WASM kompiliert werden.

### Ausgabe

```
✅ WASM code written to: output.wat
```

### Beispiele

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

Steuert das neue Installationsskript direkt aus der CLI. Die CLI lädt bei Bedarf das `install.sh` aus den Release-Assets und führt es mit den gewünschten Optionen aus.

### Syntax

```bash
hypnoscript self-update [OPTIONS]
```

### Optionen

| Option                 | Beschreibung                                                             |
| ---------------------- | ------------------------------------------------------------------------ |
| `--check`              | Nur nach Updates suchen (Exit-Code `0` = aktuell, `2` = Update gefunden) |
| `--include-prerelease` | Vorabversionen berücksichtigen                                           |
| `--force`              | Installation erzwingen, selbst wenn Version bereits vorhanden ist        |
| `--quiet`              | Ausgabe minimieren (nur Fehler)                                          |
| `--no-sudo`            | Unterdrückt automatische `sudo`-Aufrufe für Systeme ohne Root-Zugriff    |

### Verhalten

1. **Versionen vergleichen:** Aktuelle CLI-Version vs. neueste Release-Tags (inkl. optionaler Prereleases)
2. **Installer finden:** Verwendet vorhandene `installation.json`-Metadaten oder das lokale Release-Archiv (`share/hypnoscript/install.sh`)
3. **Download-Fallback:** Lädt das Installer-Skript aus der Dokumentation, falls lokal keines gefunden wird
4. **Ausführen:** Startet `install.sh` mit übergebenen Parametern und übergibt dem Benutzer die Ausgabe des Skripts

> **Hinweis:** Auf Windows steht derzeit nur `--check` zur Verfügung. Für die eigentliche Installation nutze weiterhin das Release-Archiv.

### Beispiele

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

Zeigt Versionsinformationen und Features der HypnoScript CLI.

### Syntax

```bash
hypnoscript version
```

### Ausgabe

```
HypnoScript v1.0.0 (Rust Edition)
The Hypnotic Programming Language

Migrated from C# to Rust for improved performance

Features:
  - Full parser and interpreter
  - Type checker
  - WASM code generation
  - 110+ builtin functions
```

### Verwendung

- **Version prüfen**: Aktuell installierte Version feststellen
- **Feature-Überblick**: Verfügbare Funktionalität anzeigen
- **Debugging**: Version in Bug-Reports angeben

### Beispiel

```bash
hypnoscript version
```

## builtins - Builtin-Funktionen auflisten

Listet alle verfügbaren Builtin-Funktionen der HypnoScript Standard-Bibliothek.

### Syntax

```bash
hypnoscript builtins
```

### Ausgabe

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

### Verwendung

- **Referenz**: Schnell nachschlagen welche Funktionen verfügbar sind
- **Entwicklung**: Entdecken neuer Funktionalität
- **Dokumentation**: Liste für eigene Referenzen

### Beispiel

```bash
# Auflisten
hypnoscript builtins

# Ausgabe in Datei umleiten
hypnoscript builtins > builtin-reference.txt

# Filtern mit grep
hypnoscript builtins | grep "Array"
```

## Globale Optionen

Diese Optionen funktionieren mit allen Befehlen:

| Option      | Kurzform | Beschreibung         |
| ----------- | -------- | -------------------- |
| `--help`    | `-h`     | Zeigt Hilfe          |
| `--version` | `-V`     | Zeigt Version (kurz) |

### Beispiele

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

### Verwendung in Scripts

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

1. **Schreiben**: Code in .hyp-Datei schreiben
2. **Prüfen**: `hypnoscript check script.hyp`
3. **Testen**: `hypnoscript run script.hyp --debug`
4. **Optimieren**: Bei Bedarf Code anpassen
5. **Deployen**: Final mit `hypnoscript run script.hyp`

### Debugging-Workflow

1. **Lexing prüfen**: `hypnoscript lex script.hyp`
2. **AST prüfen**: `hypnoscript parse script.hyp`
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

Vereinfache häufige Befehle:

```bash
# In ~/.bashrc oder ~/.zshrc
alias hyp='hypnoscript'
alias hyp-run='hypnoscript run'
alias hyp-check='hypnoscript check'
alias hyp-wasm='hypnoscript compile-wasm'
```

Verwendung:

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

## Siehe auch

- [Quick Start](../getting-started/quick-start) - Erste Schritte
- [Debugging](./debugging) - Erweiterte Debugging-Techniken
- [Configuration](./configuration) - CLI-Konfiguration
- [Builtin Functions](../builtins/overview) - Referenz aller Funktionen
