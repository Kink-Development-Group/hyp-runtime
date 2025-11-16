---
title: Performance Debugging
---

# Performance Debugging

Leistungsanalyse und Optimierung sind essenziell für effiziente HypnoScript-Projekte. Die wichtigsten Tools und Methoden:

## Performance-Metriken abrufen

Usen Sie die eingebaute Function `GetPerformanceMetrics`, um Laufzeitdaten zu erhalten:

```hyp
induce metrics = GetPerformanceMetrics();
observe metrics;
```

## CLI-Commande für Performance

- **Profiling:**

  ```bash
  dotnet run --project HypnoScript.CLI -- profile script.hyp --debug
  ```

  (Profiling ist vorbereitet, aber noch nicht voll implementiert.)

- **Benchmarking:**

  ```bash
  dotnet run --project HypnoScript.CLI -- benchmark script.hyp --debug
  ```

  (Benchmarking ist vorbereitet, aber noch nicht voll implementiert.)

- **Optimierung:**
  ```bash
  dotnet run --project HypnoScript.CLI -- optimize script.hyp --debug
  ```
  (Optimiert den generierten Code, z.B. durch Entfernen überflüssiger Operationen.)

## Code-Optimierung

- Der `ILCodeOptimizer` entfernt unnötige Operationen im generierten Code.
- Der `TypeChecker` verwendet Caching für wiederholte Typüberprüfungen.

## Tipps

- Analysieren Sie die Ausführungszeit mit `Execution time: ...ms` aus der CLI-Output.
- Überwachen Sie Speicher- und CPU-Auslastung mit externen Tools oder den geplanten Monitoring-Features.
