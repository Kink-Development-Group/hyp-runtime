---
title: Error Handling Overview
---

# Error Handling Overview

Fehlerbehandlung ist ein zentraler Bestandteil von HypnoScript. Das System unterscheidet zwischen Syntax-, Typ- und Laufzeitfehlern.

## Fehlerarten

- **Syntaxfehler:** Werden beim Parsen erkannt und mit einer klaren Fehlermeldung ausgegeben.
- **Typfehler:** Der `TypeChecker` prüft Typkonsistenz und meldet Fehler mit spezifischen Codes (z.B. `TYPE002`).
- **Laufzeitfehler:** Während der Ausführung werden Fehler im Interpreter erkannt und ausgegeben.

## Fehlerausgabe

Fehler werden im CLI und in der Konsole ausgegeben, z.B.:

```
[ERROR] Execution failed: Variable 'x' not defined
```

## ErrorReporter

Der zentrale Mechanismus zur Fehlerausgabe im Compiler ist der `ErrorReporter`:

```csharp
ErrorReporter.Report("Type mismatch: ...", line, column, "TYPE002");
```

## Fehlercodes

Jeder Fehler ist mit einem Code versehen, der die Fehlerart kennzeichnet (z.B. `TYPE002` für Typfehler).

## Tipps

- Nutzen Sie die Debug- und Verbose-Optionen, um Stacktraces und zusätzliche Fehlerdetails zu erhalten.
- Prüfen Sie die Fehlerausgabe auf spezifische Codes, um Fehlerquellen schnell zu identifizieren.
