---
title: Debugging Best Practices
---

# Debugging Best Practices

HypnoScript bietet verschiedene Mechanismen, um Fehler frühzeitig zu erkennen und die Codequalität zu sichern. Hier sind bewährte Methoden für effektives Debugging:

## Assertions nutzen

Verwenden Sie die `assert`-Anweisung, um Annahmen im Code zu überprüfen. Assertion-Fehler werden im CLI und in der Testausgabe hervorgehoben.

```hyp
assert(x > 0, "x muss positiv sein");
```

Assertion-Fehler werden gesammelt und am Ende der Ausführung ausgegeben:

```
❌ 1 assertion(s) failed:
   - x muss positiv sein
```

## Tests strukturieren

- Gruppieren Sie Tests in separaten `.hyp`-Fileen.
- Usen Sie den CLI-Command `test`, um alle oder einzelne Tests auszuführen:

```bash
dotnet run --project HypnoScript.CLI -- test test_basic.hyp --debug
```

## Debug- und Verbose-Flags

- `--debug`: Shows zusätzliche Debug-Outputn (z.B. Stacktraces bei Fehlern).
- `--verbose`: Shows detaillierte Analysen zu Tokens, AST und Ausführung.

## Fehlerausgaben interpretieren

- Assertion-Fehler werden speziell markiert.
- Prüfen Sie die Zusammenfassung am Ende der Testausgabe auf fehlgeschlagene Assertions.

## Weitere Tipps

- Setzen Sie Breakpoints strategisch mit `assert` oder durch gezielte Outputn (`observe`).
- Usen Sie die CLI-Optionen, um gezielt einzelne Tests oder Module zu debuggen.
