---
title: Development Debugging
---

# Development Debugging

This page provides comprehensive guidance for debugging HypnoScript applications during development.

## Overview

HypnoScript provides several debugging tools and techniques to help you identify and resolve issues in your scripts. This guide covers both built-in debugging features and development practices.

## Built-in Debugging Functions

### Logging and Tracing

HypnoScript includes several built-in functions for debugging:

```hypno
// Basic logging
Log("info", "This is an informational message");
Log("warning", "This is a warning message");
Log("error", "This is an error message");

// Tracing execution flow
Trace("Entering function calculateTotal");
// ... your code ...
Trace("Exiting function calculateTotal");
```

### Exception Handling

```hypno
try {
    // Potentially problematic code
    result = Divide(a, b);
} catch (error) {
    // Get detailed exception information
    exceptionInfo = GetExceptionInfo(error);
    Log("error", "Exception occurred: " + exceptionInfo);
}
```

### Call Stack Inspection

```hypno
// Get current call stack for debugging
callStack = GetCallStack();
Log("debug", "Current call stack: " + callStack);
```

## CLI Debugging Commands

### Linting for Static Analysis

Use the lint command to identify potential issues before execution:

```bash
hyp lint script.hyp
```

This will check for:

- Syntax errors
- Type mismatches
- Undefined variables
- Unused variables
- Potential runtime issues

### Profiling for Performance Issues

```bash
hyp profile script.hyp
```

This provides:

- Execution time analysis
- Memory usage statistics
- Function call frequency
- Performance bottlenecks

### Benchmarking

```bash
hyp benchmark script.hyp --iterations 100
```

This measures:

- Average execution time
- Performance variance
- Memory allocation patterns

## Development Best Practices

### 1. Use Descriptive Variable Names

```hypno
// Good
userAge = 25;
totalPrice = CalculateTotal(items);

// Avoid
a = 25;
t = Calc(items);
```

### 2. Add Comments for Complex Logic

```hypno
// Calculate weighted average based on user preferences
weightedScore = 0;
totalWeight = 0;

for (i = 0; i < Length(scores); i++) {
    // Apply user preference weight to each score
    weightedScore = weightedScore + (scores[i] * weights[i]);
    totalWeight = totalWeight + weights[i];
}

averageScore = weightedScore / totalWeight;
```

### 3. Validate Input Data

```hypno
function ProcessUserData(userData) {
    // Validate required fields
    if (IsNull(userData.name) || IsEmpty(userData.name)) {
        throw "User name is required";
    }

    if (userData.age < 0 || userData.age > 150) {
        throw "Invalid age value";
    }

    // Process valid data
    return ProcessValidUser(userData);
}
```

### 4. Use Type Checking

```hypno
function SafeDivide(a, b) {
    // Ensure both parameters are numbers
    if (!IsNumber(a) || !IsNumber(b)) {
        throw "Both parameters must be numbers";
    }

    // Check for division by zero
    if (b == 0) {
        throw "Division by zero is not allowed";
    }

    return a / b;
}
```

## Common Debugging Scenarios

### 1. Variable Scope Issues

```hypno
// Problem: Variable not accessible
function OuterFunction() {
    localVar = "local";

    function InnerFunction() {
        // This will fail - localVar is not in scope
        Log("info", localVar);
    }

    InnerFunction();
}

// Solution: Pass variables as parameters
function OuterFunction() {
    localVar = "local";

    function InnerFunction(param) {
        Log("info", param);
    }

    InnerFunction(localVar);
}
```

### 2. Type Conversion Issues

```hypno
// Problem: Unexpected type conversion
userInput = "123";
result = userInput + 5; // Results in "1235" (string concatenation)

// Solution: Explicit type conversion
userInput = "123";
result = ToNumber(userInput) + 5; // Results in 128 (numeric addition)
```

### 3. Array Index Issues

```hypno
// Problem: Array index out of bounds
items = [1, 2, 3];
value = items[5]; // Will cause an error

// Solution: Check array bounds
items = [1, 2, 3];
if (5 < Length(items)) {
    value = items[5];
} else {
    Log("warning", "Array index 5 is out of bounds");
}
```

## Debugging Tools Integration

### IDE Integration

Most modern IDEs support HypnoScript debugging through:

- Syntax highlighting
- Error detection
- Code completion
- Integrated terminal for CLI commands

### External Debugging

For complex debugging scenarios, you can:

1. **Export debug information:**

   ```bash
   hyp run script.hyp --debug --output debug.log
   ```

2. **Use verbose logging:**

   ```bash
   hyp run script.hyp --verbose
   ```

3. **Generate execution traces:**
   ```bash
   hyp profile script.hyp --trace --output trace.json
   ```

## Performance Debugging

### Memory Leaks

Monitor memory usage patterns:

```hypno
// Track memory usage
initialMemory = GetMemoryUsage();
// ... your code ...
finalMemory = GetMemoryUsage();
Log("info", "Memory used: " + (finalMemory - initialMemory));
```

### Slow Operations

Identify performance bottlenecks:

```hypno
// Benchmark specific operations
startTime = GetCurrentTime();
// ... operation to benchmark ...
endTime = GetCurrentTime();
Log("info", "Operation took: " + (endTime - startTime) + "ms");
```

## Error Reporting

When reporting bugs, include:

1. **Script content** (minimal reproduction case)
2. **Expected vs actual behavior**
3. **Error messages** (if any)
4. **Environment details** (OS, HypnoScript version)
5. **Steps to reproduce**

Example bug report:

```
Title: Division by zero not properly handled in SafeDivide function

Description:
The SafeDivide function should handle division by zero gracefully, but it's throwing an unhandled exception.

Steps to reproduce:
1. Create a script with: result = SafeDivide(10, 0);
2. Run the script
3. Observe unhandled exception

Expected behavior:
Function should return null or throw a specific error message.

Actual behavior:
Unhandled runtime exception occurs.

Environment:
- OS: Windows 10
- HypnoScript version: 1.0.0
```

## Conclusion

Effective debugging in HypnoScript requires a combination of:

- Using built-in debugging functions
- Following development best practices
- Leveraging CLI debugging commands
- Understanding common pitfalls
- Proper error reporting

By following these guidelines, you can quickly identify and resolve issues in your HypnoScript applications.
