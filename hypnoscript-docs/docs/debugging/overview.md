# Debugging Overview

HypnoScript provides comprehensive debugging capabilities to help you identify and fix issues in your scripts.

## Debugging Features

### 1. Built-in Debugging Functions

HypnoScript includes several built-in functions for debugging:

```hyp
// Print debug information
DebugPrint("Variable value: " + myVariable);
DebugPrintType(myVariable);

// Memory and performance debugging
DebugPrintMemory();
DebugPrintStackTrace();
DebugPrintEnvironment();

// Performance metrics
var metrics = GetPerformanceMetrics();
DebugPrint("CPU Time: " + metrics["cpu_time"]);
DebugPrint("Memory Usage: " + metrics["memory_usage"]);
```

### 2. CLI Debugging Options

Use the `--debug` flag with CLI commands for enhanced debugging:

```bash
# Run with debug output
dotnet run -- run script.hyp --debug

# Compile with debug information
dotnet run -- compile script.hyp --debug

# Analyze with detailed output
dotnet run -- analyze script.hyp --debug
```

### 3. Configuration-Based Debugging

Configure debugging behavior in your application settings:

```json
{
  "Development": {
    "DebugMode": true,
    "DetailedErrorReporting": true,
    "EnableProfiling": true,
    "EnableStackTrace": true
  }
}
```

### 4. Error Reporting

HypnoScript provides detailed error reporting with:

- **Line numbers and file locations**
- **Stack traces** for function calls
- **Type information** for variables
- **Context information** for better error understanding

### 5. Performance Profiling

Use the profiling command to analyze script performance:

```bash
dotnet run -- profile script.hyp --verbose
```

This provides:

- Execution time analysis
- Memory usage tracking
- Function call frequency
- Performance bottlenecks identification

### 6. Logging System

Configure logging levels and outputs:

```json
{
  "Logging": {
    "LogLevel": "DEBUG",
    "EnableFileLogging": true,
    "LogFilePath": "logs/hypnoscript.log",
    "IncludeTimestamps": true,
    "IncludeThreadInfo": true
  }
}
```

### 7. Interactive Debugging

For interactive debugging sessions:

```bash
# Start with interactive mode
dotnet run -- run script.hyp --debug --verbose

# Use breakpoints and step-through execution
# (Available in development builds)
```

## Debugging Best Practices

### 1. Use Descriptive Variable Names

```hyp
// Good
induce userName: string = "John";
induce userAge: number = 25;

// Avoid
induce a: string = "John";
induce b: number = 25;
```

### 2. Add Debug Statements Strategically

```hyp
Focus {
  induce counter: number = 0;
  DebugPrint("Starting loop with counter: " + counter);

  while (counter < 10) {
    DebugPrint("Counter value: " + counter);
    counter = counter + 1;
  }

  DebugPrint("Loop completed. Final counter: " + counter);
} Relax
```

### 3. Validate Input Data

```hyp
Focus {
  induce userInput: string = Input("Enter a number: ");

  if (IsNumber(userInput)) {
    induce number: number = ToInt(userInput);
    DebugPrint("Valid number entered: " + number);
  } else {
    DebugPrint("Invalid input: " + userInput);
    Observe("Please enter a valid number");
  }
} Relax
```

### 4. Use Type checking

```hyp
Focus {
  induce data: any = GetData();

  if (IsString(data)) {
    DebugPrint("Data is string: " + data);
  } else if (IsNumber(data)) {
    DebugPrint("Data is number: " + data);
  } else if (IsArray(data)) {
    DebugPrint("Data is array with " + ArrayLength(data) + " elements");
  } else {
    DebugPrint("Unknown data type: " + TypeOf(data));
  }
} Relax
```

### 5. Monitor Performance

```hyp
Focus {
  var startTime = GetCurrentTime();

  // Your code here
  induce result: number = CalculateComplexOperation();

  var endTime = GetCurrentTime();
  var duration = endTime - startTime;

  DebugPrint("Operation took " + duration + " seconds");

  if (duration > 5) {
    DebugPrint("WARNING: Operation took longer than expected");
  }
} Relax
```

## Common Debugging Scenarios

### 1. Variable Scope Issues

```hyp
Focus {
  induce globalVar: string = "Global";

  Tranceify LocalScope {
    induce localVar: string = "Local";
    DebugPrint("Inside scope - Global: " + globalVar + ", Local: " + localVar);
  }

  DebugPrint("Outside scope - Global: " + globalVar);
  // localVar is not accessible here
} Relax
```

### 2. Function Parameters Issues

```hyp
Focus {
  function ValidateUser(name: string, age: number): boolean {
    DebugPrint("Validating user: " + name + ", age: " + age);

    if (IsNullOrEmpty(name)) {
      DebugPrint("ERROR: Name is null or empty");
      return false;
    }

    if (age < 0 || age > 150) {
      DebugPrint("ERROR: Invalid age: " + age);
      return false;
    }

    DebugPrint("User validation successful");
    return true;
  }

  induce isValid: boolean = ValidateUser("John", 25);
  DebugPrint("Validation result: " + isValid);
} Relax
```

### 3. Array and Collection Issues

```hyp
Focus {
  induce numbers: number[] = [1, 2, 3, 4, 5];
  DebugPrint("Array length: " + ArrayLength(numbers));

  for (induce i: number = 0; i < ArrayLength(numbers); i = i + 1) {
    DebugPrint("Element " + i + ": " + numbers[i]);
  }

  // Check for out-of-bounds access
  if (ArrayLength(numbers) > 10) {
    DebugPrint("WARNING: Large array detected");
  }
} Relax
```

## Debugging Tools Integration

### 1. IDE Integration

- **Visual Studio Code**: Use the HypnoScript extension for syntax highlighting and debugging
- **Visual Studio**: Full debugging support with breakpoints and variable inspection
- **JetBrains Rider**: Advanced debugging features with step-through execution

### 2. External Tools

- **Log analyzers**: Parse and analyze log files for patterns
- **Performance profilers**: Detailed performance analysis
- **Memory analyzers**: Track memory usage and identify leaks

### 3. Continuous Integration

- **Automated testing**: Catch issues early in development
- **Code quality checks**: Ensure code meets standards
- **Performance regression testing**: Monitor performance over time

## Getting Help

If you encounter issues that you can't resolve with the debugging tools:

1. **Check the logs**: Look for error messages and warnings
2. **Review the documentation**: Consult the language reference
3. **Search the community**: Check forums and GitHub issues
4. **Create a minimal example**: Reproduce the issue in a simple script
5. **Report the issue**: Include debug output and error messages

Remember: Good debugging practices lead to more maintainable and reliable code!
