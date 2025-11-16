---
sidebar_position: 3
---

# Examplee: CLI-Workflows

Diese Seite zeigt typische CLI-Workflows für die HypnoScript-Entwicklung, von einfachen Skript-Ausführungen bis hin zu komplexen Automatisierungsabläufen.

## Basic Entwicklungsworkflows

### Einfaches Skript ausführen

```bash
# Skript direkt ausführen
dotnet run --project HypnoScript.CLI -- run hello.hyp

# Mit detaillierter Ausgabe
dotnet run --project HypnoScript.CLI -- run script.hyp --verbose

# Mit Timeout für lange Skripte
dotnet run --project HypnoScript.CLI -- run long_script.hyp --timeout 60
```

### Syntax prüfen und validieren

```bash
# Syntax prüfen
dotnet run --project HypnoScript.CLI -- validate script.hyp

# Strikte Validierung mit Warnungen
dotnet run --project HypnoScript.CLI -- validate script.hyp --strict --warnings

# Validierungs-Report generieren
dotnet run --project HypnoScript.CLI -- validate *.hyp --output validation-report.json
```

### Code formatieren

```bash
# Code formatieren und in neue Datei schreiben
dotnet run --project HypnoScript.CLI -- format script.hyp --output formatted.hyp

# Direkt in der Datei formatieren
dotnet run --project HypnoScript.CLI -- format script.hyp --in-place

# Nur prüfen, ob Formatierung nötig ist
dotnet run --project HypnoScript.CLI -- format script.hyp --check
```

## Testen und Debugging

### Tests ausführen

```bash
# Alle Tests im aktuellen Verzeichnis
dotnet run --project HypnoScript.CLI -- test *.hyp

# Spezifische Test-Datei
dotnet run --project HypnoScript.CLI -- test test_math.hyp

# Tests mit Filter
dotnet run --project HypnoScript.CLI -- test *.hyp --filter "math"

# JSON-Report für CI/CD
dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json
```

### Debug-Modus

```bash
# Debug-Modus mit Trace
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace

# Schritt-für-Schritt-Ausführung
dotnet run --project HypnoScript.CLI -- debug script.hyp --step

# Mit Breakpoints
dotnet run --project HypnoScript.CLI -- debug script.hyp --breakpoints breakpoints.txt

# Variablen anzeigen
dotnet run --project HypnoScript.CLI -- debug script.hyp --variables
```

### Code-Analyse

```bash
# Lint-Analyse
dotnet run --project HypnoScript.CLI -- lint script.hyp

# Mit spezifischen Regeln
dotnet run --project HypnoScript.CLI -- lint script.hyp --rules "style,performance"

# Nur Fehler anzeigen
dotnet run --project HypnoScript.CLI -- lint script.hyp --severity error

# Lint-Report generieren
dotnet run --project HypnoScript.CLI -- lint *.hyp --output lint-report.json
```

## Build und Deployment

### Kompilieren

```bash
# Standard-Kompilierung
dotnet run --project HypnoScript.CLI -- build script.hyp

# Mit Optimierungen
dotnet run --project HypnoScript.CLI -- build script.hyp --optimize

# Debug-Version
dotnet run --project HypnoScript.CLI -- build script.hyp --debug

# WebAssembly-Target
dotnet run --project HypnoScript.CLI -- build script.hyp --target wasm
```

### Pakete erstellen

```bash
# Ausführbares Paket erstellen
dotnet run --project HypnoScript.CLI -- package script.hyp

# Mit Runtime-spezifischen Abhängigkeiten
dotnet run --project HypnoScript.CLI -- package script.hyp --runtime win-x64 --dependencies

# Spezifische Ausgabedatei
dotnet run --project HypnoScript.CLI -- package script.hyp --output myapp.exe
```

### Webserver starten

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

## Automatisierung und CI/CD

### Entwicklungsworkflow-Skript

```bash
#!/bin/bash
# dev-workflow.sh

echo "=== HypnoScript Development Workflow ==="

# 1. Syntax prüfen
echo "1. Validating syntax..."
dotnet run --project HypnoScript.CLI -- validate *.hyp

# 2. Code formatieren
echo "2. Formatting code..."
dotnet run --project HypnoScript.CLI -- format *.hyp --in-place

# 3. Lint-Analyse
echo "3. Running lint analysis..."
dotnet run --project HypnoScript.CLI -- lint *.hyp --severity error

# 4. Tests ausführen
echo "4. Running tests..."
dotnet run --project HypnoScript.CLI -- test *.hyp

# 5. Build erstellen
echo "5. Building..."
dotnet run --project HypnoScript.CLI -- build main.hyp --optimize

echo "Workflow completed!"
```

### CI/CD Pipeline (GitHub Actions)

```yaml
name: HypnoScript CI/CD

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Setup .NET
        uses: actions/setup-dotnet@v3
        with:
          dotnet-version: '8.0.x'

      - name: Validate syntax
        run: dotnet run --project HypnoScript.CLI -- validate *.hyp

      - name: Run tests
        run: dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json

      - name: Upload test results
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: test-results.json

      - name: Build optimized version
        run: dotnet run --project HypnoScript.CLI -- build main.hyp --optimize

      - name: Create package
        run: dotnet run --project HypnoScript.CLI -- package main.hyp --runtime linux-x64
```

### Deployment-Skript

```bash
#!/bin/bash
# deploy.sh

echo "=== HypnoScript Deployment ==="

# Umgebungsvariablen prüfen
if [ -z "$DEPLOY_PATH" ]; then
    echo "Error: DEPLOY_PATH not set"
    exit 1
fi

# Build erstellen
echo "Building application..."
dotnet run --project HypnoScript.CLI -- build main.hyp --optimize

# Tests ausführen
echo "Running tests..."
dotnet run --project HypnoScript.CLI -- test *.hyp

# Paket erstellen
echo "Creating deployment package..."
dotnet run --project HypnoScript.CLI -- package main.hyp --runtime linux-x64 --output app

# Deployment
echo "Deploying to $DEPLOY_PATH..."
cp app $DEPLOY_PATH/
chmod +x $DEPLOY_PATH/app

echo "Deployment completed!"
```

## Konfiguration und Umgebung

### Konfigurationsdatei (hypnoscript.config.json)

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

### environment variablen

```bash
# HypnoScript-spezifische Umgebungsvariablen
export HYPNOSCRIPT_HOME="/opt/hypnoscript"
export HYPNOSCRIPT_LOG_LEVEL="debug"
export HYPNOSCRIPT_CONFIG="./config.json"
export HYPNOSCRIPT_TIMEOUT="60000"

# Skript mit Umgebungsvariablen ausführen
dotnet run --project HypnoScript.CLI -- run script.hyp
```

## Monitoring und Logging

### Logging-Konfiguration

```bash
# Detailliertes Logging
dotnet run --project HypnoScript.CLI -- run script.hyp --log-level debug

# Nur Fehler loggen
dotnet run --project HypnoScript.CLI -- run script.hyp --log-level error

# Logs in Datei umleiten
dotnet run --project HypnoScript.CLI -- run script.hyp --verbose > script.log 2>&1
```

### Performance-Monitoring

```bash
# Mit Performance-Metriken
dotnet run --project HypnoScript.CLI -- run script.hyp --verbose --metrics

# Memory-Usage überwachen
dotnet run --project HypnoScript.CLI -- run script.hyp --max-memory 1024
```

## Best Practices

### Skript-Organisation

```bash
# Projektstruktur
my-project/
├── src/
│   ├── main.hyp
│   ├── utils.hyp
│   └── config.hyp
├── tests/
│   ├── test_main.hyp
│   └── test_utils.hyp
├── scripts/
│   ├── build.sh
│   └── deploy.sh
├── config/
│   └── hypnoscript.config.json
└── output/
    └── dist/
```

### Automatisierte Workflows

```bash
# Pre-commit Hook (.git/hooks/pre-commit)
#!/bin/bash

echo "Running HypnoScript pre-commit checks..."

# Syntax prüfen
dotnet run --project HypnoScript.CLI -- validate *.hyp
if [ $? -ne 0 ]; then
    echo "Syntax validation failed!"
    exit 1
fi

# Code formatieren
dotnet run --project HypnoScript.CLI -- format *.hyp --in-place

# Tests ausführen
dotnet run --project HypnoScript.CLI -- test *.hyp
if [ $? -ne 0 ]; then
    echo "Tests failed!"
    exit 1
fi

echo "Pre-commit checks passed!"
```

### Error Handling

```bash
# Robuster Workflow mit Fehlerbehandlung
#!/bin/bash

set -e  # Exit on error

echo "Starting robust workflow..."

# Funktion für Fehlerbehandlung
handle_error() {
    echo "Error occurred in line $1"
    echo "Cleaning up..."
    # Cleanup-Code hier
    exit 1
}

trap 'handle_error $LINENO' ERR

# Workflow-Schritte
dotnet run --project HypnoScript.CLI -- validate *.hyp
dotnet run --project HypnoScript.CLI -- test *.hyp
dotnet run --project HypnoScript.CLI -- build main.hyp --optimize

echo "Workflow completed successfully!"
```

## Next Steps

- [CLI-Commande Referenz](../cli/commands) - Vollständige CLI-Referenz
- [Konfiguration](../cli/configuration) - Advanced Konfiguration
- [Runtime-Features](../enterprise/features) - Runtime-Functionen

---

**CLI-Workflows gemeistert? Dann lerne [erweiterte Konfiguration](../cli/configuration) kennen!** ⚙️
