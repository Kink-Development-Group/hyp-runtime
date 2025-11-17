---
sidebar_position: 3
---

# Interpreter

The HypnoScript interpreter is the heart of the runtime and processes HypnoScript code at runtime.

## Architecture

### Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Lexer         │    │   Parser        │    │   Interpreter   │
│                 │    │                 │    │                 │
│ - Tokenization  │───▶│ - AST Creation  │───▶│ - Code Execution│
│ - Syntax Check  │    │ - Semantic Check│    │ - Session Mgmt  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Processing Pipeline

1. **Lexer**: Breaks source code into tokens
2. **Parser**: Creates Abstract Syntax Tree (AST)
3. **Interpreter**: Executes AST

## Interpreter Features

### Dynamic Typing

```hyp
// Variables can change their type at runtime
induce x = 42;        // Integer
induce x = "Hello";   // String
induce x = [1,2,3];   // Array
```

### Session Management

```hyp
// Sessions are automatically managed
induce session = Session("MySession");
SessionSet(session, "key", "value");
induce value = SessionGet(session, "key");
```

### Error Handling

```hyp
// Robust error handling
if (ArrayLength(arr) > 0) {
    induce element = ArrayGet(arr, 0);
} else {
    observe "Array is empty";
}
```

## Interpreter Configuration

### Memory Management

```json
{
  "maxMemory": 512,
  "gcThreshold": 0.8,
  "stackSize": 1024
}
```

### Performance Optimizations

- **JIT-Compilation**: Frequently executed code blocks are compiled
- **Caching**: Function results are cached
- **Lazy Evaluation**: Expressions are evaluated only when needed

## Debugging Features

### Trace Mode

```bash
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace
```

### Breakpoints

```hyp
// Set breakpoint
breakpoint;

// Conditional breakpoints
if (counter == 42) {
    breakpoint;
}
```

### Variable Inspection

```hyp
// Inspect variables at runtime
observe "Variable x: " + x;
observe "Array length: " + ArrayLength(arr);
```

## Session Management

### Session Lifecycle

1. **Creation**: `Session("name")`
2. **Usage**: `SessionSet()`, `SessionGet()`
3. **Cleanup**: Automatic after program end

### Session Types

```hyp
// Standard session
induce session = Session("Standard");

// Persistent session
induce persistentSession = Session("Persistent", true);

// Shared session
induce sharedSession = Session("Shared", false, true);
```

## Builtin Functions Integration

### Function Call Mechanism

```hyp
// Direct call
induce result = ArraySum([1,2,3]);

// With error handling
if (IsValidEmail(email)) {
    observe "Email is valid";
} else {
    observe "Email is invalid";
}
```

### Function Categories

- **Array Functions**: `ArrayGet`, `ArraySet`, `ArraySort`
- **String Functions**: `Length`, `Substring`, `ToUpper`
- **Math Functions**: `Sin`, `Cos`, `Sqrt`, `Pow`
- **System Functions**: `GetCurrentTime`, `GetMachineName`
- **Utility Functions**: `Clamp`, `IsEven`, `GenerateUUID`

## Performance Monitoring

### Memory Usage

```hyp
induce memoryUsage = GetMemoryUsage();
observe "Memory usage: " + memoryUsage + " bytes";
```

### CPU Usage

```hyp
induce cpuUsage = GetCPUUsage();
observe "CPU usage: " + cpuUsage + "%";
```

### Execution Time

```hyp
induce startTime = GetCurrentTime();
// Execute code
induce endTime = GetCurrentTime();
induce executionTime = endTime - startTime;
observe "Execution time: " + executionTime + " ms";
```

## Extensibility

### Custom Functions

```hyp
// Define custom functions
suggestion customFunction(param) {
    awaken param * 2;
}

// Use them
induce result = customFunction(21);
```

### Plugin System

```hyp
// Load plugins (conceptual)
LoadPlugin("math-extensions");
LoadPlugin("network-utils");
```

## Best Practices

### Memory Management

```hyp
// Avoid large arrays
induce largeArray = [];
for (induce i = 0; i < 1000000; induce i = i + 1) {
    // Process in chunks
    if (i % 1000 == 0) {
        // Process chunk
    }
}
```

### Error Handling

```hyp
// Robust error handling
suggestion safeArrayAccess(arr, index) {
    if (index < 0 || index >= Length(arr)) {
        awaken null;
    }
    return ArrayGet(arr, index);
}
```

### Performance Optimization

```hyp
// Efficient loops
induce length = Length(arr);
for (induce i = 0; i < length; induce i = i + 1) {
    // Code
}
```

## Troubleshooting

### Common Problems

#### Memory Leaks

```hyp
// Explicitly delete sessions
SessionDelete(session);
```

#### Infinite Loops

```hyp
// Set timeout
induce startTime = GetCurrentTime();
while (condition) {
    if (GetCurrentTime() - startTime > 5000) {
        break; // 5 second timeout
    }
    // Code
}
```

#### Stack Overflow

```hyp
// Limit recursion
suggestion factorial(n, depth = 0) {
    if (depth > 1000) {
        awaken null; // Avoid stack overflow
    }
    if (n <= 1) return 1;
    return n * factorial(n - 1, depth + 1);
}
```

## Next Steps

- [Runtime](./runtime) - Runtime Architecture
- [Compiler](./compiler) - Code Generation
- [API](./api) - Programming Interface
- [Debugging](../cli/debugging) - Debugging Tools

---

**Do you understand the interpreter? Then learn about the [Runtime Architecture](./runtime)!** ⚙️
