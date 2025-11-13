---
title: CLI Basics
---

Die HypnoScript Command Line Interface (CLI) ist das schnellste Werkzeug, um HypnoScript-Skripte zu bauen, zu prüfen und auszuführen. Diese Seite führt dich durch die wichtigsten Subcommands und typischen Arbeitsabläufe.

## Hilfe & Orientierung

```bash
# Globale Hilfe
hypnoscript --help

# Version und Features anzeigen
hypnoscript version

# Hilfe für einen Subcommand
hypnoscript run --help
```

Die Ausgabe listet immer alle verfügbaren Subcommands sowie deren Optionen auf. Falls ein Befehl unbekannt wirkt, lohnt sich ein Blick in `--help` – der Text wird direkt aus der tatsächlichen CLI generiert.

## Skripte ausführen

```bash
# Standardausführung
hypnoscript run demo.hyp

# Mit zusätzlicher Ausgabe
hypnoscript run demo.hyp --verbose

# Mit Debug-Informationen
hypnoscript run demo.hyp --debug
```

- `--verbose` gibt Statusmeldungen wie "Running file" oder Erfolgsmeldungen aus.
- `--debug` zeigt zusätzlich Quelltext, Tokenliste, Type-Checking-Ergebnisse und den Ablauf der Interpretation.
- Fehler im Type Checker halten die Ausführung nicht auf – sie werden gemeldet, anschließend läuft der Interpreter weiter.

## Analysewerkzeuge

| Befehl                            | Zweck                                          |
| --------------------------------- | ---------------------------------------------- |
| `hypnoscript lex <file>`          | Zeigt alle Token mit Index, Typ und Lexem      |
| `hypnoscript parse <file>`        | Gibt den formatierten Abstract Syntax Tree aus |
| `hypnoscript check <file>`        | Prüft Typen und meldet Inkonsistenzen          |
| `hypnoscript compile-wasm <file>` | Generiert WebAssembly Text Format (`.wat`)     |

Diese Tools lassen sich ideal kombinieren, um Parser- oder Typfehler einzugrenzen. Beispiel:

```bash
hypnoscript check scripts/report.hyp
hypnoscript parse scripts/report.hyp
hypnoscript compile-wasm scripts/report.hyp -o report.wat
```

## Standardbibliothek erkunden

```bash
hypnoscript builtins
```

Der Befehl gruppiert alle eingebauten Funktionen nach Kategorie (Math, String, Array, System, ...). Nutze ihn, um schnell passende Helfer zu finden.

## Typischer Workflow

1. **Vorbereitung** – `hypnoscript check` auf allen Skripten laufen lassen.
2. **Fehleranalyse** – bei Problemen `lex` oder `parse` verwenden, um den konkreten Abschnitt zu inspizieren.
3. **Ausführung** – mit `run` testen, bei Bedarf `--debug` aktivieren.
4. **Deployment** – optional `compile-wasm`, wenn das Skript im Browser oder in einer WASM-Umgebung laufen soll.

```bash
# Beispiel: komplette Runde
hypnoscript check examples/inventory.hyp
hypnoscript run examples/inventory.hyp --debug
hypnoscript compile-wasm examples/inventory.hyp -o inventory.wat
```

## Tipps & Tricks

- **Schnelle Iteration:** Nutze `--debug`, sobald etwas merkwürdig wirkt – Token und AST verraten sofort, ob der Parser deine Absicht verstanden hat.
- **Ausgaben bündeln:** Pipe die Ausgabe in eine Datei (`hypnoscript run script.hyp > output.txt`), um längere Läufe zu dokumentieren.
- **Platform-agnostisch:** Unter Windows, macOS und Linux sind die Befehle identisch. Einzige Voraussetzung ist, dass der `hypnoscript`-Binary im `PATH` liegt.
- **Tests als Skripte:** Die Dateien im Ordner `hypnoscript-tests/` lassen sich direkt mit `hypnoscript run` starten. So siehst du reale Beispiele für Kontrollfluss und Sessions.

## Weiterführende Links

- [CLI Übersicht](../cli/overview) – Installation, Binary-Varianten und Workflow
- [CLI-Befehle](../cli/commands) – Vollständige Referenz mit allen Optionen
- [Sprachreferenz](../language-reference/syntax) – Detaillierte Beschreibung der Grammatik
