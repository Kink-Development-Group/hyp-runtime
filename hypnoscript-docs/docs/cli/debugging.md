---
title: CLI Debugging
---

# CLI Debugging

Die HypnoScript CLI bietet zahlreiche Optionen für Debugging und Fehleranalyse.

## Debug- und Verbose-Optionen

- `--debug`: Aktiviert Debug-Ausgaben (z.B. Stacktraces, interne Statusmeldungen)
- `--verbose`: Zeigt zusätzliche Details zu Token, AST und Ausführung

## Wichtige CLI-Befehle

- `run <file.hyp> [--debug] [--verbose]`: Skript ausführen
- `test <file.hyp> [--debug] [--verbose]`: Tests ausführen und Assertion-Fehler anzeigen
- `profile <file.hyp> [--debug] [--verbose]`: Profiling (geplant)
- `benchmark <file.hyp> [--debug] [--verbose]`: Benchmarking (geplant)
- `optimize <file.hyp> [--debug] [--verbose]`: Code-Optimierung (geplant)

## Debug-Ausgaben interpretieren

- Assertion-Fehler werden klar hervorgehoben
- Fehlerausgaben enthalten ggf. Stacktraces (bei `--debug`)
- Zusammenfassungen am Ende zeigen, wie viele Tests bestanden/fehlgeschlagen sind

## Beispiel

```bash
dotnet run --project HypnoScript.CLI -- test test_basic.hyp --debug --verbose
```

## Tipps

- Nutzen Sie die CLI-Optionen gezielt, um Fehlerquellen schnell zu identifizieren
- Kombinieren Sie Debug- und Verbose-Flags für maximale Transparenz
