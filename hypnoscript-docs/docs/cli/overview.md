---
sidebar_position: 1
---

# CLI Übersicht

Die HypnoScript Command Line Interface (CLI) bietet eine vollständige Entwicklungsumgebung für HypnoScript-Programme mit umfangreichen Features für Entwicklung, Testing und Deployment.

## Installation

```bash
# Repository klonen
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# Projekt bauen
dotnet build

# CLI verwenden
dotnet run --project HypnoScript.CLI -- --help
```

## Installation via Paketmanager

### Windows (winget)

```powershell
winget install HypnoScript.HypnoScript
```

### Linux (APT)

```bash
sudo apt update
sudo apt install hypnoscript
```

## Automatisierte Releases & Paketmanager

Die aktuellen Installationspakete (ZIP für Windows/winget, .deb für Linux/APT) werden bei jedem Release automatisch gebaut und als Artefakte auf GitHub bereitgestellt:

- [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases)

### Installation mit winget (Windows)

```powershell
winget install HypnoScript.HypnoScript
```

### Installation mit APT (Linux)

```bash
sudo apt update
sudo apt install hypnoscript
```

## Grundlegende Verwendung

```bash
# Programm ausführen
dotnet run --project HypnoScript.CLI -- run programm.hyp

# Version anzeigen
dotnet run --project HypnoScript.CLI -- --version

# Hilfe anzeigen
dotnet run --project HypnoScript.CLI -- --help
```

## Verfügbare Befehle

| Befehl     | Beschreibung         | Beispiel              |
| ---------- | -------------------- | --------------------- |
| `run`      | Programm ausführen   | `run script.hyp`      |
| `test`     | Tests ausführen      | `test *.hyp`          |
| `build`    | Programm kompilieren | `build script.hyp`    |
| `debug`    | Debug-Modus          | `debug script.hyp`    |
| `serve`    | Webserver starten    | `serve --port 8080`   |
| `validate` | Syntax prüfen        | `validate script.hyp` |

## Globale Optionen

| Option      | Kurzform | Beschreibung         |
| ----------- | -------- | -------------------- |
| `--verbose` | `-v`     | Detaillierte Ausgabe |
| `--quiet`   | `-q`     | Minimale Ausgabe     |
| `--config`  | `-c`     | Konfigurationsdatei  |
| `--output`  | `-o`     | Ausgabedatei         |
| `--timeout` | `-t`     | Timeout in Sekunden  |

## Konfiguration

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
  }
}
```

### Umgebungsvariablen

```bash
# Windows
set HYPNOSCRIPT_HOME=C:\path\to\hyp-runtime
set HYPNOSCRIPT_LOG_LEVEL=debug

# Linux/macOS
export HYPNOSCRIPT_HOME=/path/to/hyp-runtime
export HYPNOSCRIPT_LOG_LEVEL=debug
```

## Beispiele

### Einfaches Programm ausführen

```bash
# Programm erstellen
echo 'Focus { entrance { observe "Hallo Welt!"; } } Relax;' > hello.hyp

# Programm ausführen
dotnet run --project HypnoScript.CLI -- run hello.hyp
```

### Mit Parametern

```bash
# Programm mit Argumenten
dotnet run --project HypnoScript.CLI -- run script.hyp --arg1 value1 --arg2 value2
```

### Debug-Modus

```bash
# Mit Debug-Informationen
dotnet run --project HypnoScript.CLI -- debug script.hyp --verbose
```

### Tests ausführen

```bash
# Alle Tests im Verzeichnis
dotnet run --project HypnoScript.CLI -- test *.hyp

# Spezifische Test-Datei
dotnet run --project HypnoScript.CLI -- test test_math.hyp
```

## Nächste Schritte

- [CLI-Befehle](./commands) - Detaillierte Befehlsreferenz
- [Konfiguration](./configuration) - Erweiterte Konfiguration
- [Testing](./testing) - Test-Framework
- [Debugging](./debugging) - Debugging-Tools

---

**Bereit für die detaillierte Befehlsreferenz?** 🚀
