---
title: CLI Debugging
---

Die HypnoScript CLI setzt beim Debugging auf wenige, aber wirkungsvolle Mechanismen. Dieser Leitfaden zeigt, wie du Fehler schnell eingrenzt und welche Commande dir helfen, den Programmzustand sichtbar zu machen.

## Debug- und Verbose-Modus

- `--debug` zeigt den Quelltext, die erzeugten Tokens, den AST sowie die Ergebnisse des Type Checkers, bevor der Interpreter startet.
- `--verbose` ergänzt Statusmeldungen (z.B. "Running file" oder "Program executed successfully").
- Beide Flags lassen sich kombinieren: `hypnoscript run script.hyp --debug --verbose`.

## Token- und AST-Analyse

```bash
hypnoscript lex script.hyp
hypnoscript parse script.hyp
```

- Use `lex`, um zu kontrollieren, welche Schlüsselwörter und Literale der Lexer erkennt.
- `parse` liefert den vollständigen AST – ideal, wenn Kontrollstrukturen oder Sessions nicht wie erwartet aufgebaut werden.

## Typprüfung without execution

```bash
hypnoscript check script.hyp
```

- Der Type Checker meldet fehlende Functionen, falsche Return valuee oder ungeeignete Zuweisungen.
- Die CLI führt das Programm auch bei Typfehlern aus; verwende `check`, um Fehler schon vorher einzufangen.

## Typischer Debug-Workflow

```bash
# 1. Type Checking
hypnoscript check scripts/deep_trance.hyp

# 2. Tokens & AST inspizieren
hypnoscript lex scripts/deep_trance.hyp
hypnoscript parse scripts/deep_trance.hyp

# 3. Mit Debug-Ausgabe ausführen
hypnoscript run scripts/deep_trance.hyp --debug
```

## Tipps

- Kommentiere komplexe Bereiche temporär aus (`//`) und führe den Rest mit `--debug` aus, um das Problem lokal einzugrenzen.
- Bei Array-Operationen hilft `hypnoscript builtins`, um passende Hilfsfunktionen zu finden (z.B. `ArrayJoin`, `ArrayContains`).
- Speichere Debug-Outputn mit `> debug.log`, falls du sie später vergleichen möchtest (`hypnoscript run script.hyp --debug > debug.log`).
