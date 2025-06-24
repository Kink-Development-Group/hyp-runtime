---
sidebar_position: 1
---

# Installation

Lerne, wie du HypnoScript auf deinem System installierst und einrichtest.

## Voraussetzungen

### Systemanforderungen

- **Betriebssystem**: Windows 10+, macOS 10.15+, oder Linux (Ubuntu 18.04+, CentOS 7+)
- **.NET**: .NET 8.0 SDK oder höher
- **RAM**: Mindestens 512 MB verfügbarer RAM
- **Festplatte**: 100 MB freier Speicherplatz

### .NET Installation

HypnoScript benötigt .NET 8.0 oder höher. Falls noch nicht installiert:

#### Windows

```powershell
# Download von Microsoft
winget install Microsoft.DotNet.SDK.8
# oder
choco install dotnet-sdk
```

#### macOS

```bash
# Mit Homebrew
brew install dotnet

# Oder Download von Microsoft
curl -sSL https://dot.net/v1/dotnet-install.sh | bash
```

#### Linux (Ubuntu/Debian)

```bash
# Repository hinzufügen
wget https://packages.microsoft.com/config/ubuntu/$(lsb_release -rs)/packages-microsoft-prod.deb -O packages-microsoft-prod.deb
sudo dpkg -i packages-microsoft-prod.deb
rm packages-microsoft-prod.deb

# .NET installieren
sudo apt-get update
sudo apt-get install -y dotnet-sdk-8.0
```

## Installation von HypnoScript

### Option 1: Aus dem Repository (Empfohlen)

```bash
# Repository klonen
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# Projekt bauen
dotnet build

# Testen der Installation
dotnet run --project HypnoScript.CLI -- --help
```

### Option 2: Release-Download

1. Gehe zu [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases)
2. Lade die neueste Version für dein Betriebssystem herunter
3. Entpacke das Archiv
4. Führe die ausführbare Datei aus

### Option 3: Globale Installation (Entwicklung)

```bash
# Repository klonen
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# Globale Installation
dotnet tool install --global --add-source ./HypnoScript.CLI/bin/Debug/net8.0 HypnoScript.CLI

# Oder mit dotnet run
dotnet run --project HypnoScript.CLI -- run example.hyp
```

## Verifikation der Installation

### Test der Installation

```bash
# Version anzeigen
dotnet run --project HypnoScript.CLI -- --version

# Hilfe anzeigen
dotnet run --project HypnoScript.CLI -- --help

# Einfaches Test-Programm
echo 'Focus { entrance { observe "Installation erfolgreich!"; } } Relax;' > test.hyp
dotnet run --project HypnoScript.CLI -- run test.hyp
```

### Erwartete Ausgabe

```
HypnoScript CLI v1.0.0
Installation erfolgreich!
```

## Konfiguration

### Umgebungsvariablen

```bash
# Windows (PowerShell)
$env:HYPNOSCRIPT_HOME = "C:\path\to\hyp-runtime"

# macOS/Linux
export HYPNOSCRIPT_HOME="/path/to/hyp-runtime"
```

### Konfigurationsdatei

Erstelle eine `hypnoscript.config.json` im Projektverzeichnis:

```json
{
  "defaultOutput": "console",
  "enableDebug": false,
  "logLevel": "info",
  "timeout": 30000,
  "maxMemory": 512
}
```

## IDE-Integration

### Visual Studio Code

1. Installiere die C# Extension
2. Öffne das HypnoScript-Projekt
3. Erstelle eine `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Run HypnoScript",
      "type": "coreclr",
      "request": "launch",
      "preLaunchTask": "build",
      "program": "${workspaceFolder}/HypnoScript.CLI/bin/Debug/net8.0/HypnoScript.CLI.dll",
      "args": ["run", "${file}"],
      "cwd": "${workspaceFolder}",
      "console": "internalConsole",
      "stopAtEntry": false
    }
  ]
}
```

### JetBrains Rider

1. Öffne das Projekt in Rider
2. Konfiguriere Run Configurations
3. Setze die CLI als Startup Project

## Troubleshooting

### Häufige Probleme

#### .NET nicht gefunden

```bash
# Prüfe .NET Installation
dotnet --version

# Falls nicht installiert, siehe .NET Installation oben
```

#### Build-Fehler

```bash
# Dependencies wiederherstellen
dotnet restore

# Clean und Rebuild
dotnet clean
dotnet build
```

#### Berechtigungsfehler (Linux/macOS)

```bash
# Ausführungsrechte setzen
chmod +x HypnoScript.CLI/bin/Debug/net8.0/HypnoScript.CLI

# Oder mit sudo (nicht empfohlen)
sudo dotnet run --project HypnoScript.CLI -- run test.hyp
```

#### Pfad-Probleme

```bash
# Prüfe aktuelles Verzeichnis
pwd

# Navigiere zum Projektverzeichnis
cd /path/to/hyp-runtime

# Prüfe Projektstruktur
ls -la
```

### Support

Bei Problemen:

1. **GitHub Issues**: [Issues erstellen](https://github.com/Kink-Development-Group/hyp-runtime/issues)
2. **Discussions**: [Community-Diskussionen](https://github.com/Kink-Development-Group/hyp-runtime/discussions)
3. **Dokumentation**: Siehe [Troubleshooting Guide](../development/debugging)

## Nächste Schritte

- [Schnellstart-Guide](./quick-start) - Erstelle dein erstes HypnoScript-Programm
- [Hello World](./hello-world) - Lerne die Grundlagen
- [CLI-Grundlagen](./cli-basics) - Verstehe die Kommandozeilen-Tools
- [Sprachreferenz](../language-reference/syntax) - Lerne die Syntax

---

**Installation erfolgreich? Dann lass uns mit dem [Schnellstart-Guide](./quick-start) beginnen!** 🚀

## Installation via Paketmanager

### Windows (winget)

Mit dem Windows Package Manager (ab Windows 10):

```powershell
winget install HypnoScript.HypnoScript
```

### Linux (APT)

Für Debian/Ubuntu-basierte Systeme:

```bash
sudo apt update
sudo apt install hypnoscript
```

Die Pakete installieren die CLI und Runtime global und machen den Befehl `hypnoscript` überall verfügbar.
