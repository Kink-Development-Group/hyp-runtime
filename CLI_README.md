# HypnoScript CLI - Runtime Edition

Eine vollständige Command-Line-Interface für die HypnoScript-Programmiersprache mit drei Hauptmodi: Run, Compile und Analyze.

## Installation

```bash
# Projekt klonen und bauen
git clone <repository>
cd hyp-runtime
dotnet build
```

## Verwendung

### Grundlegende Syntax

```bash
dotnet run --project HypnoScript.CLI <befehl> <datei.hyp> [--debug]
```

### Verfügbare Befehle

#### 1. Run - Programm ausführen

Führt HypnoScript-Code direkt aus.

```bash
# Einfache Ausführung
dotnet run --project HypnoScript.CLI run test_simple.hyp

# Mit Debug-Ausgaben
dotnet run --project HypnoScript.CLI run test_simple.hyp --debug
```

**Features:**

- ✅ Lexikalische Analyse (Tokenisierung)
- ✅ Syntax-Analyse (Parsing)
- ✅ Typüberprüfung (TypeChecking)
- ✅ Interpreter-Ausführung
- ✅ Detaillierte Fehlerberichte

#### 2. Compile - Zu WASM kompilieren

Kompiliert HypnoScript-Code zu WebAssembly (WAT-Format).

```bash
# Kompilierung
dotnet run --project HypnoScript.CLI compile test_advanced.hyp

# Mit Debug-Ausgaben
dotnet run --project HypnoScript.CLI compile test_advanced.hyp --debug
```

**Features:**

- ✅ WASM Code Generation
- ✅ WAT-Format Ausgabe
- ✅ Automatische Datei-Erweiterung (.wat)
- ✅ Optimierte Code-Generierung

#### 3. Analyze - Statische Analyse

Führt eine umfassende statische Analyse durch.

```bash
# Analyse
dotnet run --project HypnoScript.CLI analyze test_advanced.hyp

# Mit Debug-Ausgaben
dotnet run --project HypnoScript.CLI analyze test_advanced.hyp --debug
```

**Features:**

- 📊 Token-Analyse (Häufigkeit, Typen)
- 🌳 AST-Analyse (Statement-Typen)
- 📈 Code-Metriken (Zeilen, Zeichen, Tokens)
- ✅ Typüberprüfung
- 📋 Detaillierte Berichte

## Beispiele

### Einfaches Programm (test_simple.hyp)

```hypno
Focus {
    observe "Hello World!";
} Relax
```

### Erweitertes Programm (test_advanced.hyp)

```hypno
Focus {
    entrance {
        observe "Willkommen in der erweiterten HypnoScript-Welt!";
        drift(1000);
    }

    induce x: number = 10;
    induce y: number = 5;

    if (x > 5) deepFocus {
        observe "x ist größer als 5";
    }

    while (y > 0) {
        observe "Countdown: " + y;
        y = y - 1;
    }
} Relax
```

## Ausgabe-Beispiele

### Run-Modus

```bash
=== RUN MODE ===
✓ Datei beginnt mit 'Focus' - Syntax OK
✓ Lexing erfolgreich!
✓ Parsing erfolgreich!
✓ TypeChecking erfolgreich!
✓ Ausführung erfolgreich!
🎉 HypnoScript-Programm erfolgreich ausgeführt!
```

### Compile-Modus

```bash
=== COMPILE MODE ===
✓ Lexing erfolgreich!
✓ Parsing erfolgreich!
✓ WASM Code Generation erfolgreich!
📁 WASM (WAT) Code gespeichert: test_advanced.wat
🎉 Kompilierung erfolgreich abgeschlossen!
```

### Analyze-Modus

```bash
=== ANALYZE MODE ===
✓ Lexing erfolgreich!

📊 TOKEN-ANALYSE:
  Identifier: 15x
  StringLiteral: 8x
  NumberLiteral: 6x
  LBrace: 5x
  RBrace: 5x
  ...

🌳 AST-ANALYSE:
  Top-Level Statements: 12
  ExpressionStatementNode: 8x
  VarDeclNode: 4x

📈 CODE-METRIKEN:
  Zeilen: 25
  Zeichen: 456
  Tokens: 67
  Statements: 12

🎉 Statische Analyse erfolgreich abgeschlossen!
```

## Fehlerbehandlung

Die CLI bietet umfassende Fehlerbehandlung:

- **Datei nicht gefunden**: Exit Code 2
- **Syntax-Fehler**: Detaillierte Fehlermeldungen mit Zeilen-/Spaltenangaben
- **Typ-Fehler**: Spezifische Typfehler mit Kontext
- **Runtime-Fehler**: Ausführungsfehler mit Stack-Trace (im Debug-Modus)

## Debug-Modus

Der `--debug` Flag aktiviert zusätzliche Ausgaben:

- Detaillierte Verarbeitungsschritte
- Token-Details
- AST-Struktur
- Stack-Traces bei Fehlern
- Performance-Metriken

## Exit Codes

- **0**: Erfolg
- **1**: Fehler (Syntax, Typ, Runtime)
- **2**: Datei nicht gefunden
- **99**: Fataler Fehler

## Erweiterte Features

### Unterstützte Sprachkonstrukte

- ✅ Variablen (`induce`)
- ✅ Kontrollstrukturen (`if`, `while`, `loop`)
- ✅ Funktionen (`suggestion`)
- ✅ Arrays und Listen
- ✅ Strings und Zahlen
- ✅ Hypnotische Operatoren
- ✅ Sessions und Tranceify
- ✅ Built-in Funktionen

### Performance-Optimierungen

- Effiziente Tokenisierung
- Optimierte AST-Erstellung
- Schnelle Typüberprüfung
- Minimaler Speicherverbrauch

## Entwicklung

### Projektstruktur

```bash
HypnoScript.CLI/
├── Program.cs          # Haupt-CLI-Logik
├── HypnoScript.CLI.csproj
└── ...

HypnoScript.LexerParser/
├── Lexer/              # Tokenisierung
├── Parser/             # Syntax-Analyse
└── AST/                # Abstract Syntax Tree

HypnoScript.Compiler/
├── Analysis/           # Typüberprüfung
├── Interpreter/        # Ausführung
└── CodeGen/            # WASM-Generierung
```

### Erweitern der CLI

Neue Befehle können einfach hinzugefügt werden:

1. Neuen Case im `Main` Switch hinzufügen
2. Neue Methode für den Befehl erstellen
3. `ShowUsage()` aktualisieren

## Lizenz

HypnoScript CLI - Runtime Edition
Copyright (c) 2024 HypnoScript Team
