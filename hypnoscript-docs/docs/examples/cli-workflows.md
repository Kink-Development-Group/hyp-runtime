---
sidebar_position: 3
---

# Examples: CLI Workflows

This page shows typical CLI workflows for HypnoScript development, from simple script executions to complex automation processes.

## Basic Development Workflows

### Run Simple Script

```bash
# Run script directly
dotnet run --project HypnoScript.CLI -- run hello.hyp

# With detailed output
dotnet run --project HypnoScript.CLI -- run script.hyp --verbose

# With timeout for long scripts
dotnet run --project HypnoScript.CLI -- run long_script.hyp --timeout 60
```

### Check and Validate Syntax

```bash
# Check syntax
dotnet run --project HypnoScript.CLI -- validate script.hyp

# Strict validation with warnings
dotnet run --project HypnoScript.CLI -- validate script.hyp --strict --warnings

# Generate validation report
dotnet run --project HypnoScript.CLI -- validate *.hyp --output validation-report.json
```

### Format Code

```bash
# Format Code und in neue Datei schreiben
dotnet run --project HypnoScript.CLI -- format script.hyp --output formatted.hyp

# Format directly in file
dotnet run --project HypnoScript.CLI -- format script.hyp --in-place

# Only check if formatting is needed
dotnet run --project HypnoScript.CLI -- format script.hyp --check
```

## Testing and Debugging

### Run Tests

```bash
# All tests in current directory
dotnet run --project HypnoScript.CLI -- test *.hyp

# Specific test file
dotnet run --project HypnoScript.CLI -- test test_math.hyp

# Tests with filter
dotnet run --project HypnoScript.CLI -- test *.hyp --filter "math"

# JSON report for CI/CD
dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json
```

### Debug Mode

```bash
# Debug Mode mit Trace
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace

# Step-by-step execution
dotnet run --project HypnoScript.CLI -- debug script.hyp --step

# With breakpoints
dotnet run --project HypnoScript.CLI -- debug script.hyp --breakpoints breakpoints.txt

# Show variables
dotnet run --project HypnoScript.CLI -- debug script.hyp --variables
```

### Code Analysis

```bash
# Lint analysis
dotnet run --project HypnoScript.CLI -- lint script.hyp

# With specific rules
dotnet run --project HypnoScript.CLI -- lint script.hyp --rules "style,performance"

# Show only errors
dotnet run --project HypnoScript.CLI -- lint script.hyp --severity error

# Generate lint report
dotnet run --project HypnoScript.CLI -- lint *.hyp --output lint-report.json
```

## Build and Deployment

### Compile

```bash
# Standard compilation
dotnet run --project HypnoScript.CLI -- build script.hyp

# With optimizations
dotnet run --project HypnoScript.CLI -- build script.hyp --optimize

# Debug version
dotnet run --project HypnoScript.CLI -- build script.hyp --debug

# WebAssembly target
dotnet run --project HypnoScript.CLI -- build script.hyp --target wasm
```

### Create Packages

```bash
# Create executable package
dotnet run --project HypnoScript.CLI -- package script.hyp

# With runtime-specific dependencies
dotnet run --project HypnoScript.CLI -- package script.hyp --runtime win-x64 --dependencies

# Specific output file
dotnet run --project HypnoScript.CLI -- package script.hyp --output myapp.exe
```

### Start Web Server

```bash
# Standard web server
dotnet run --project HypnoScript.CLI -- serve

# With specific port
dotnet run --project HypnoScript.CLI -- serve --port 8080

# With SSL
dotnet run --project HypnoScript.CLI -- serve --ssl

# With configuration
dotnet run --project HypnoScript.CLI -- serve --config server.json
```

## Automation and CI/CD

### Development Workflow Script

```bash
#!/bin/bash
# dev-workflow.sh

echo "=== HypnoScript Development Workflow ==="

# 1. Check syntax
echo "1. Validating syntax..."
dotnet run --project HypnoScript.CLI -- validate *.hyp

# 2. Format Code
echo "2. Formatting code..."
dotnet run --project HypnoScript.CLI -- format *.hyp --in-place

# 3. Lint analysis
echo "3. Running lint analysis..."
dotnet run --project HypnoScript.CLI -- lint *.hyp --severity error

# 4. Run Tests
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

# Run Tests
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

# Check syntax
dotnet run --project HypnoScript.CLI -- validate *.hyp
if [ $? -ne 0 ]; then
    echo "Syntax validation failed!"
    exit 1
fi

# Format Code
dotnet run --project HypnoScript.CLI -- format *.hyp --in-place

# Run Tests
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

