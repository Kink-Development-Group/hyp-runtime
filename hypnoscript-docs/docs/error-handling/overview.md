---
title: Error Handling Overview
---

# Error Handling Overview

Error handling is a central component of HypnoScript. The system distinguishes between syntax, type, and runtime errors.

## Error Types

- **Syntax errors:** Detected during parsing and output with a clear error message.
- **Type errors:** The `TypeChecker` checks type consistency and reports errors with specific codes (e.g., `TYPE002`).
- **Runtime errors:** Errors during execution are detected and output by the interpreter.

## Error Output

Errors are output in the CLI and console, e.g.:

```
[ERROR] Execution failed: Variable 'x' not defined
```

## ErrorReporter

The central mechanism for error output in the compiler is the `ErrorReporter`:

```csharp
ErrorReporter.Report("Type mismatch: ...", line, column, "TYPE002");
```

## Error Codes

Each error is provided with a code that identifies the error type (e.g., `TYPE002` for type errors).

## Tips

- Use the debug and verbose options to get stacktraces and additional error details.
- Check the error output for specific codes to quickly identify error sources.
