---
sidebar_position: 2
---

# CLI-Befehle

Die HypnoScript CLI bietet umfangreiche Befehle für Entwicklung, Testing und Deployment.

## run - Programm ausführen

Führt ein HypnoScript-Programm aus.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- run <datei> [optionen]
```

### Optionen

| Option      | Kurzform | Beschreibung          |
| ----------- | -------- | --------------------- |
| `--verbose` | `-v`     | Detaillierte Ausgabe  |
| `--quiet`   | `-q`     | Minimale Ausgabe      |
| `--output`  | `-o`     | Ausgabedatei          |
| `--timeout` | `-t`     | Timeout in Sekunden   |
| `--args`    | `-a`     | Zusätzliche Argumente |

### Beispiele

```bash
# Einfaches Programm ausführen
dotnet run --project HypnoScript.CLI -- run hello.hyp

# Mit detaillierter Ausgabe
dotnet run --project HypnoScript.CLI -- run script.hyp --verbose

# Mit Timeout
dotnet run --project HypnoScript.CLI -- run long_script.hyp --timeout 30

# Ausgabe in Datei umleiten
dotnet run --project HypnoScript.CLI -- run script.hyp --output result.txt

# Mit zusätzlichen Argumenten
dotnet run --project HypnoScript.CLI -- run script.hyp --args "param1=value1" "param2=value2"
```

## test - Tests ausführen

Führt Tests für HypnoScript-Dateien aus.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- test <pattern> [optionen]
```

### Optionen

| Option      | Kurzform | Beschreibung                    |
| ----------- | -------- | ------------------------------- |
| `--verbose` | `-v`     | Detaillierte Test-Ausgabe       |
| `--quiet`   | `-q`     | Nur Zusammenfassung             |
| `--format`  | `-f`     | Ausgabeformat (text, json, xml) |
| `--output`  | `-o`     | Test-Report-Datei               |
| `--filter`  | `-F`     | Test-Filter                     |

### Beispiele

```bash
# Alle Tests im aktuellen Verzeichnis
dotnet run --project HypnoScript.CLI -- test *.hyp

# Spezifische Test-Datei
dotnet run --project HypnoScript.CLI -- test test_math.hyp

# Tests mit detaillierter Ausgabe
dotnet run --project HypnoScript.CLI -- test *.hyp --verbose

# JSON-Report generieren
dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-report.json

# Tests mit Filter
dotnet run --project HypnoScript.CLI -- test *.hyp --filter "math"
```

## build - Programm kompilieren

Kompiliert ein HypnoScript-Programm.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- build <datei> [optionen]
```

### Optionen

| Option       | Kurzform | Beschreibung             |
| ------------ | -------- | ------------------------ |
| `--output`   | `-o`     | Ausgabedatei             |
| `--optimize` | `-O`     | Optimierungen aktivieren |
| `--debug`    | `-d`     | Debug-Informationen      |
| `--target`   | `-t`     | Zielformat (il, wasm)    |

### Beispiele

```bash
# Programm kompilieren
dotnet run --project HypnoScript.CLI -- build script.hyp

# Mit Optimierungen
dotnet run --project HypnoScript.CLI -- build script.hyp --optimize

# Debug-Version
dotnet run --project HypnoScript.CLI -- build script.hyp --debug

# WebAssembly-Target
dotnet run --project HypnoScript.CLI -- build script.hyp --target wasm
```

## debug - Debug-Modus

Führt ein Programm im Debug-Modus aus.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- debug <datei> [optionen]
```

### Optionen

| Option          | Kurzform | Beschreibung                   |
| --------------- | -------- | ------------------------------ |
| `--breakpoints` | `-b`     | Breakpoint-Datei               |
| `--step`        | `-s`     | Schritt-für-Schritt-Ausführung |
| `--trace`       | `-t`     | Ausführungs-Trace              |
| `--variables`   | `-v`     | Variablen anzeigen             |

### Beispiele

```bash
# Debug-Modus starten
dotnet run --project HypnoScript.CLI -- debug script.hyp

# Mit Breakpoints
dotnet run --project HypnoScript.CLI -- debug script.hyp --breakpoints breakpoints.txt

# Schritt-für-Schritt
dotnet run --project HypnoScript.CLI -- debug script.hyp --step

# Mit Trace
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace

# Variablen anzeigen
dotnet run --project HypnoScript.CLI -- debug script.hyp --variables
```

## serve - Webserver starten

Startet einen Webserver für HypnoScript-Anwendungen.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- serve [optionen]
```

### Optionen

| Option     | Kurzform | Beschreibung        |
| ---------- | -------- | ------------------- |
| `--port`   | `-p`     | Port-Nummer         |
| `--host`   | `-h`     | Host-Adresse        |
| `--config` | `-c`     | Konfigurationsdatei |
| `--ssl`    | `-s`     | SSL aktivieren      |

### Beispiele

```bash
# Standard-Webserver
dotnet run --project HypnoScript.CLI -- serve

# Mit spezifischem Port
dotnet run --project HypnoScript.CLI -- serve --port 8080

# Mit SSL
dotnet run --project HypnoScript.CLI -- serve --ssl

# Mit Konfiguration
dotnet run --project HypnoScript.CLI -- serve --config server.json
```

## validate - Syntax prüfen

Prüft die Syntax von HypnoScript-Dateien.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- validate <datei> [optionen]
```

### Optionen

| Option       | Kurzform | Beschreibung        |
| ------------ | -------- | ------------------- |
| `--strict`   | `-s`     | Strikte Validierung |
| `--warnings` | `-w`     | Warnungen anzeigen  |
| `--output`   | `-o`     | Validierungs-Report |

### Beispiele

```bash
# Syntax prüfen
dotnet run --project HypnoScript.CLI -- validate script.hyp

# Strikte Validierung
dotnet run --project HypnoScript.CLI -- validate script.hyp --strict

# Mit Warnungen
dotnet run --project HypnoScript.CLI -- validate script.hyp --warnings

# Report generieren
dotnet run --project HypnoScript.CLI -- validate script.hyp --output validation.json
```

## format - Code formatieren

Formatiert HypnoScript-Code.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- format <datei> [optionen]
```

### Optionen

| Option       | Kurzform | Beschreibung             |
| ------------ | -------- | ------------------------ |
| `--check`    | `-c`     | Nur prüfen, nicht ändern |
| `--in-place` | `-i`     | Datei direkt ändern      |
| `--output`   | `-o`     | Ausgabedatei             |

### Beispiele

```bash
# Code formatieren
dotnet run --project HypnoScript.CLI -- format script.hyp

# Nur prüfen
dotnet run --project HypnoScript.CLI -- format script.hyp --check

# Direkt ändern
dotnet run --project HypnoScript.CLI -- format script.hyp --in-place

# In neue Datei
dotnet run --project HypnoScript.CLI -- format script.hyp --output formatted.hyp
```

## lint - Code-Analyse

Führt statische Code-Analyse durch.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- lint <datei> [optionen]
```

### Optionen

| Option       | Kurzform | Beschreibung        |
| ------------ | -------- | ------------------- |
| `--rules`    | `-r`     | Lint-Regeln         |
| `--severity` | `-s`     | Mindest-Schweregrad |
| `--output`   | `-o`     | Lint-Report         |

### Beispiele

```bash
# Code-Analyse
dotnet run --project HypnoScript.CLI -- lint script.hyp

# Mit spezifischen Regeln
dotnet run --project HypnoScript.CLI -- lint script.hyp --rules "style,performance"

# Nur Fehler
dotnet run --project HypnoScript.CLI -- lint script.hyp --severity error

# Report generieren
dotnet run --project HypnoScript.CLI -- lint script.hyp --output lint-report.json
```

## package - Paket erstellen

Erstellt ein ausführbares Paket.

### Syntax

```bash
dotnet run --project HypnoScript.CLI -- package <datei> [optionen]
```

### Optionen

| Option           | Kurzform | Beschreibung                |
| ---------------- | -------- | --------------------------- |
| `--output`       | `-o`     | Ausgabedatei                |
| `--runtime`      | `-r`     | Ziel-Runtime                |
| `--dependencies` | `-d`     | Abhängigkeiten einschließen |

### Beispiele

```bash
# Paket erstellen
dotnet run --project HypnoScript.CLI -- package script.hyp

# Mit Runtime
dotnet run --project HypnoScript.CLI -- package script.hyp --runtime win-x64

# Mit Abhängigkeiten
dotnet run --project HypnoScript.CLI -- package script.hyp --dependencies

# Spezifische Ausgabe
dotnet run --project HypnoScript.CLI -- package script.hyp --output myapp.exe
```

## Globale Optionen

Alle Befehle unterstützen diese globalen Optionen:

| Option        | Kurzform | Beschreibung                         |
| ------------- | -------- | ------------------------------------ |
| `--help`      | `-h`     | Hilfe anzeigen                       |
| `--version`   | `-V`     | Version anzeigen                     |
| `--verbose`   | `-v`     | Detaillierte Ausgabe                 |
| `--quiet`     | `-q`     | Minimale Ausgabe                     |
| `--config`    | `-c`     | Konfigurationsdatei                  |
| `--log-level` | `-l`     | Log-Level (debug, info, warn, error) |

## Konfigurationsdatei

Die CLI kann über eine `hypnoscript.config.json` konfiguriert werden:

```json
{
  "defaultOutput": "console",
  "enableDebug": false,
  "logLevel": "info",
  "timeout": 30000,
  "maxMemory": 512,
  "testFramework": {
    "autoRun": true,
    "reportFormat": "detailed"
  },
  "server": {
    "port": 8080,
    "host": "localhost"
  },
  "formatting": {
    "indentSize": 2,
    "maxLineLength": 80
  },
  "linting": {
    "rules": ["style", "performance", "security"],
    "severity": "warning"
  }
}
```

## Umgebungsvariablen

| Variable                | Beschreibung             |
| ----------------------- | ------------------------ |
| `HYPNOSCRIPT_HOME`      | Installationsverzeichnis |
| `HYPNOSCRIPT_LOG_LEVEL` | Log-Level                |
| `HYPNOSCRIPT_CONFIG`    | Konfigurationsdatei      |
| `HYPNOSCRIPT_TIMEOUT`   | Standard-Timeout         |

## Beispiele für komplexe Workflows

### Entwicklungsworkflow

```bash
# 1. Syntax prüfen
dotnet run --project HypnoScript.CLI -- validate script.hyp

# 2. Code formatieren
dotnet run --project HypnoScript.CLI -- format script.hyp --in-place

# 3. Lint-Analyse
dotnet run --project HypnoScript.CLI -- lint script.hyp

# 4. Tests ausführen
dotnet run --project HypnoScript.CLI -- test *.hyp

# 5. Programm ausführen
dotnet run --project HypnoScript.CLI -- run script.hyp
```

### CI/CD-Pipeline

```bash
# Build und Test
dotnet run --project HypnoScript.CLI -- build script.hyp --optimize
dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json
dotnet run --project HypnoScript.CLI -- lint script.hyp --severity error

# Deployment
dotnet run --project HypnoScript.CLI -- package script.hyp --runtime linux-x64
dotnet run --project HypnoScript.CLI -- serve --port 8080 --ssl
```

### Debugging-Workflow

```bash
# 1. Syntax prüfen
dotnet run --project HypnoScript.CLI -- validate script.hyp

# 2. Debug-Modus mit Trace
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace --variables

# 3. Schritt-für-Schritt
dotnet run --project HypnoScript.CLI -- debug script.hyp --step
```

## Nächste Schritte

- [Konfiguration](./configuration) - Erweiterte Konfiguration
- [Testing](./testing) - Test-Framework
- [Debugging](./debugging) - Debugging-Tools
- [Enterprise-Features](./enterprise-features) - Enterprise-Features

---

**Beherrschst du die CLI-Befehle? Dann lerne die [Konfiguration](./configuration) kennen!** ⚙️
