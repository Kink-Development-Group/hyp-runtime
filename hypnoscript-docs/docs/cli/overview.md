---
sidebar_position: 1
---

# CLI Übersicht

Die HypnoScript Command Line Interface (CLI) ist ein in Rust gebautes Einzelbinary (`hypnoscript`). Es bündelt Lexer, Parser, Type Checker, Interpreter und den WASM-Codegenerator in einem Tool.

## Installation

### Vorgefertigte Pakete

1. Lade das passende Archiv aus den [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases).
2. Entpacke das Archiv und füge den Binärpfad deiner `PATH`-Umgebungsvariable hinzu.
3. Prüfe die Installation mit `hypnoscript version`.

### Aus dem Quellcode bauen

```bash
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime
cargo build --release -p hypnoscript-cli
# Optional installieren
cargo install --path hypnoscript-cli
```

Die kompilierten Binaries findest du unter `target/release/`.

## Schnellstart

```bash
# Hilfe anzeigen
hypnoscript --help

# Versionshinweis
hypnoscript version

# Programm ausführen
hypnoscript run hello.hyp
```

Alle Subcommands sind bewusst schlank gehalten. Für einen tieferen Blick sieh dir die folgenden Abschnitte an.

## Befehlsüberblick

| Befehl         | Kurzbeschreibung                                 |
| -------------- | ------------------------------------------------ |
| `run`          | Führt ein HypnoScript-Programm aus               |
| `run --debug`  | Zeigt zusätzlich Tokens, AST und Typprüfung      |
| `lex`          | Tokenisiert eine Datei                           |
| `parse`        | Zeigt den AST                                    |
| `check`        | Führt Type Checking durch                        |
| `compile-wasm` | Generiert WebAssembly Text Format (.wat)         |
| `self-update`  | Prüft Releases und führt den neuen Installer aus |
| `builtins`     | Listet alle verfügbaren Builtin-Funktionen       |
| `version`      | Zeigt Versions- und Featureinformationen         |

Weitere Details liefert die Seite [CLI-Befehle](./commands).

## Typischer Workflow

```bash
# 1. Type Checking ohne Ausführung
hypnoscript check my_script.hyp

# 2. Bei Fehlern AST prüfen
hypnoscript parse my_script.hyp

# 3. Debug-Ausgabe aktivieren
hypnoscript run my_script.hyp --debug

# 4. Optional WASM generieren
hypnoscript compile-wasm my_script.hyp -o my_script.wat
```

## Plattformhinweise

- **Windows**: Nutze das ZIP aus dem Release, entpacke in `%LOCALAPPDATA%\Programs\hypnoscript` und ergänze den Pfad.
- **macOS / Linux**: Archiv nach `/usr/local/bin` oder `~/.local/bin` kopieren.
- Für portable Nutzung kannst du den Binary-Pfad direkt angeben (`./hypnoscript run demo.hyp`).

## Updates & Wartung

- **Self-Update:** `hypnoscript self-update` prüft Releases und startet automatisch das neue `install.sh`. Mit `--check` wird nur geprüft, `--force` erzwingt eine Neuinstallation, `--include-prerelease` aktiviert RC-/Beta-Builds.
- **Installer im Release:** Jedes Release enthält zusätzlich zu den Binaries ein `share/hypnoscript/install.sh`, sodass du Updates auch offline starten kannst (z.B. `bash share/hypnoscript/install.sh --check`).
- **Windows-Einschränkung:** Auf Windows steht derzeit nur `--check` zur Verfügung; Installation erfolgt weiterhin über das manuell heruntergeladene Archiv.

## Nächste Schritte

- [CLI-Befehle](./commands) – Details zu allen Subcommands
- [CLI Basics](../getting-started/cli-basics) – Schritt-für-Schritt-Anleitung
- [Sprachreferenz](../language-reference/syntax) – Grammatik & Beispiele

---

**Tipp:** `hypnoscript builtins` verschafft dir einen schnellen Überblick über die Standardbibliothek.
