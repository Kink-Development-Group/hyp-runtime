---
title: Performance Debugging
---

# Performance Debugging

Performance analysis and optimization are essential for efficient HypnoScript projects. The most important tools and methods:

## Retrieving Performance Metrics

Use the built-in function `GetPerformanceMetrics` to obtain runtime data:

```hyp
induce metrics = GetPerformanceMetrics();
observe metrics;
```

## CLI Commands for Performance

- **Profiling:**

  ```bash
  dotnet run --project HypnoScript.CLI -- profile script.hyp --debug
  ```

  (Profiling is prepared but not yet fully implemented.)

- **Benchmarking:**

  ```bash
  dotnet run --project HypnoScript.CLI -- benchmark script.hyp --debug
  ```

  (Benchmarking is prepared but not yet fully implemented.)

- **Optimization:**
  ```bash
  dotnet run --project HypnoScript.CLI -- optimize script.hyp --debug
  ```
  (Optimizes the generated code, e.g., by removing unnecessary operations.)

## Code Optimization

- The `ILCodeOptimizer` removes unnecessary operations in the generated code.
- The `TypeChecker` uses caching for repeated type checks.

## Tips

- Analyze execution time with `Execution time: ...ms` from the CLI output.
- Monitor memory and CPU usage with external tools or the planned monitoring features.
