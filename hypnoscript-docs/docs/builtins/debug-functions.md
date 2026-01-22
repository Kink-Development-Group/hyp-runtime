---
sidebar_position: 18
---

# Debug Functions

These functions help with development and troubleshooting of HypnoScript programs.

## Inspection

### inspect(value)

Returns a detailed representation of a value, including type information.

```hyp
Focus
    induce arr = [1, 2, 3];
    observe(inspect(arr));
    // Output: Array[Int](3) = [1, 2, 3]

    induce obj = { name: "Test", value: 42 };
    observe(inspect(obj));
    // Output: Object { name: String = "Test", value: Int = 42 }
Relax
```

### typeOf(value)

Returns the type of a value as a string.

```hyp
Focus
    observe(typeOf(42));       // "Int"
    observe(typeOf("Hello"));  // "String"
    observe(typeOf([1,2,3]));  // "Array"
    observe(typeOf(true));     // "Bool"
    observe(typeOf(null));     // "Null"
Relax
```

### stackTrace()

Returns the current call stack as a string.

```hyp
Focus
    suggestion innerFunction() {
        observe(stackTrace());
    }

    suggestion outerFunction() {
        innerFunction();
    }

    outerFunction();
    // Output:
    // Call Stack:
    //   #0: innerFunction() at script.hyp:3
    //   #1: outerFunction() at script.hyp:7
    //   #2: <main> at script.hyp:10
Relax
```

### dump(value)

Prints a formatted value and returns it. Useful for debugging in expressions.

```hyp
Focus
    // Inline debugging
    induce result = dump(calculate(5, 3)) * 2;
    // Prints calculate(5, 3) and continues using the result
Relax
```

## Assertions

### assertEqual(actual, expected, message?)

Checks whether two values are equal. Throws an error otherwise.

```hyp
Focus
    induce result = add(2, 3);
    assertEqual(result, 5);                    // Without message
    assertEqual(result, 5, "add should return 5");  // With message
Relax
```

### assertTruthy(value, message?)

Checks whether a value evaluates to true.

```hyp
Focus
    induce items = getItems();
    assertTruthy(ArrayLength(items) > 0, "Should contain items");

    induce user = getCurrentUser();
    assertTruthy(user, "User should exist");
Relax
```

## Timing

### time(label)

Starts a timer with the given label.

```hyp
Focus
    time("database-query");

    // Time-consuming operation...
    induce result = queryDatabase();
Relax
```

### timeEnd(label)

Ends the timer and prints the elapsed time.

```hyp
Focus
    time("operation");

    induce result = complexCalculation();

    timeEnd("operation");
    // Output: operation: 123.45ms
Relax
```

### measureTime(label, callback)

Measures the execution time of a function and returns the result.

```hyp
Focus
    induce sorted = measureTime("sort", suggestion() {
        awaken sortArray(largeArray);
    });
    // Output: sort: 45.67ms
    // sorted contains the sorted array
Relax
```

## Logging

### log(message)

Prints an info message.

```hyp
Focus
    log("Processing started");
    log("Step 1 completed");
Relax
```

### warn(message)

Prints a warning.

```hyp
Focus
    warn("Configuration not found, using default");
    warn("Deprecated API call");
Relax
```

### error(message)

Prints an error message.

```hyp
Focus
    error("Critical error: file not found");
Relax
```

### trace(message)

Prints a message with stack trace.

```hyp
Focus
    suggestion processItem(item) {
        trace("Processing item");
        // Output contains message + current call stack
    }
Relax
```

## Breakpoints

### breakpoint()

Pauses execution when in debug mode (`--debug`).

```hyp
Focus
    induce x = 10;

    breakpoint();  // Pauses here in debug mode

    induce y = x * 2;
    observe(y);
Relax
```

In normal execution (without `--debug`) this function has no effect.

## Usage

### Enable Debugging

```bash
# Start debug mode
hypnoscript exec script.hyp --debug

# With initial breakpoints
hypnoscript exec script.hyp --debug --breakpoints 10,25

# With watch expressions
hypnoscript exec script.hyp --debug --watch counter,result
```

### Remove Debugging Code

Before production use, debug calls should be removed:

```hyp
// Development
breakpoint();
dump(value);
time("operation");
timeEnd("operation");

// Remove these lines before production
```

## See also

- [Debug Mode](../debugging/debug-mode) - Interactive debugger
- [Breakpoints](../debugging/breakpoints) - Breakpoint management
- [Debugging Tools](../debugging/tools) - Complete reference
