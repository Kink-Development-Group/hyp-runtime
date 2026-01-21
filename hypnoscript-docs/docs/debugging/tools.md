---
sidebar_position: 1
---

# Debugging Tools

HypnoScript provides comprehensive debugging capabilities for developing and troubleshooting scripts.

## CLI Debug Commands

### Start Debug Mode

```bash
# Start debug mode
hypnoscript exec --debug script.hyp

# With detailed output
hypnoscript exec --debug --verbose script.hyp

# With initial breakpoints
hypnoscript exec --debug --breakpoints 10,25,42 script.hyp

# With watch expressions
hypnoscript exec --debug --watch counter,result script.hyp

# With trace file
hypnoscript exec --debug --trace-file debug.log script.hyp
```

### Combined Options

```bash
# Complete debug session with all options
hypnoscript exec --debug \
    --breakpoints 10,25 \
    --watch x,y,result \
    --trace-file session.log \
    --verbose \
    script.hyp
```

## Debug Builtin Functions

HypnoScript provides several built-in functions for debugging:

### inspect(value)

Returns a detailed representation of a value, including type information:

```hypnoscript
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

Returns the type of a value as a string:

```hypnoscript
Focus
    observe(typeOf(42));       // "Int"
    observe(typeOf("Hello"));  // "String"
    observe(typeOf([1,2,3]));  // "Array"
    observe(typeOf(true));     // "Bool"
Relax
```

### stackTrace()

Returns the current call stack as a string:

```hypnoscript
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

Prints the value in a formatted way and returns it (useful for debugging in expressions):

```hypnoscript
Focus
    induce result = dump(calculateValue()) * 2;
    // Prints calculateValue() and continues using it
Relax
```

## Assertion Functions

### assertEqual(actual, expected, message?)

Checks whether two values are equal:

```hypnoscript
Focus
    induce result = calculate(5, 3);
    assertEqual(result, 8, "Addition should yield 8");
Relax
```

### assertTruthy(value, message?)

Checks whether a value is evaluated as truthy:

```hypnoscript
Focus
    induce items = getItems();
    assertTruthy(ArrayLength(items) > 0, "Items should be present");
Relax
```

## Timing Functions

### time(label) / timeEnd(label)

Measures execution time between two points:

```hypnoscript
Focus
    time("operation");

    // Time-consuming operation
    induce result = complexCalculation();

    timeEnd("operation");
    // Output: operation: 123.45ms
Relax
```

### measureTime(label, callback)

Measures the execution time of a function:

```hypnoscript
Focus
    induce result = measureTime("sort", suggestion() {
        awaken sortArray(largeArray);
    });
    // Output: sort: 45.67ms
Relax
```

## Logging Functions

### log(message) / warn(message) / error(message)

Different log levels for structured output:

```hypnoscript
Focus
    log("Info: processing started");
    warn("Warning: file not found, using default");
    error("Error: invalid input value");
Relax
```

### trace(message)

Prints a message with stack trace:

```hypnoscript
Focus
    suggestion processItem(item) {
        trace("Processing item");
        // Output includes current position and call stack
    }
Relax
```

### breakpoint()

Pauses execution in debug mode:

```hypnoscript
Focus
    induce x = 10;

    breakpoint();  // Pauses here when --debug is active

    induce y = x * 2;
Relax
```

## Interactive Debug Commands

In interactive debug mode, these commands are available:

### Execution Control

| Command    | Alias | Description                          |
| ---------- | ----- | ------------------------------------ |
| `continue` | `c`   | Continue to the next breakpoint      |
| `step`     | `s`   | Execute one line (step into)         |
| `next`     | `n`   | Execute one line, skipping functions |
| `finish`   | `f`   | Run to the end of the function       |
| `run`      | `r`   | Restart execution                    |

### Variable Inspection

| Command        | Alias      | Description            |
| -------------- | ---------- | ---------------------- |
| `locals`       | `l`        | Show local variables   |
| `globals`      | `g`        | Show global variables  |
| `print <var>`  | `p <var>`  | Print a variable       |
| `watch <expr>` | `w <expr>` | Add a watch expression |
| `watches`      |            | Show all watches       |

### Breakpoint Management

| Command         | Alias      | Description           |
| --------------- | ---------- | --------------------- |
| `break <line>`  | `b <line>` | Set a breakpoint      |
| `delete <line>` | `d <line>` | Delete a breakpoint   |
| `breakpoints`   | `bl`       | Show all breakpoints  |
| `clear`         |            | Clear all breakpoints |

### Navigation

| Command              | Alias | Description                    |
| -------------------- | ----- | ------------------------------ |
| `list`               |       | Source around current position |
| `list <start> <end>` |       | Show source range              |
| `where`              | `bt`  | Show call stack                |
| `help`               |       | Show help                      |
| `quit`               |       | Exit debug session             |

## Debug Trace File

With `--trace-file`, a detailed trace log is created:

```bash
hypnoscript exec --debug --trace-file debug.log script.hyp
```

The trace file contains:

- **Executed lines** with timestamps
- **Variable changes** on each step
- **Breakpoint hits** with context
- **Call stack changes** on function calls
- **Timing information** for performance analysis

### Example Trace Output

```bash
[00:00.001] EXEC   script.hyp:5   induce x = 10;
[00:00.001] VAR    x = Int(10)
[00:00.002] EXEC   script.hyp:6   induce y = 20;
[00:00.002] VAR    y = Int(20)
[00:00.003] BREAK  script.hyp:7   Breakpoint hit
[00:00.015] EXEC   script.hyp:7   induce result = x + y;
[00:00.015] VAR    result = Int(30)
[00:00.016] CALL   script.hyp:8   -> processResult()
[00:00.020] RET    script.hyp:8   <- processResult() = Null
```

## Best Practices

### 1. Remove Debugging Code

Remove debugging functions before production use:

```hypnoscript
// Development
breakpoint();
dump(value);

// Production - remove these lines
```

### 2. Meaningful Timer Labels

Use descriptive labels for timing:

```hypnoscript
time("database-query");
time("json-parsing");
time("api-call-users");
```

### 3. Assertions for Tests

Use assertions for automated tests:

```hypnoscript
Focus
    induce result = add(2, 3);
    assertEqual(result, 5, "add() should add correctly");

    induce isEmpty = isListEmpty([]);
    assertTruthy(isEmpty, "Empty list should be empty");
Relax
```

### 4. Trace Files for Analysis

Use trace files for post-mortem analysis:

```bash
# Create trace
hypnoscript exec --debug --trace-file crash.log problematic.hyp

# Analyze later
cat crash.log | grep "ERROR\|BREAK"
```
