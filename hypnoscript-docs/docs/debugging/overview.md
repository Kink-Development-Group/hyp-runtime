# Debugging Overview

HypnoScript provides comprehensive debugging features to identify and fix errors in your scripts.

## Debugging Features

### 1. Interactive Debug Mode

Debug mode starts an interactive REPL session with full control over execution:

```bash
# Start debug mode
hypnoscript exec script.hyp --debug

# With initial breakpoints
hypnoscript exec script.hyp --debug --breakpoints 10,25,42

# With watch expressions
hypnoscript exec script.hyp --debug --watch counter,result
```

In interactive mode you can:

- Set and remove breakpoints
- Step through code line by line
- Inspect variables
- View the call stack

### 2. Builtin Debug Functions

HypnoScript includes several built-in functions for debugging:

```hyp
Focus
   // Detailed value inspection
    induce data = { name: "Test", value: 42 };
    observe(inspect(data));

   // Type checking
    observe(typeOf(data));  // "Object"

   // Print stack trace
    observe(stackTrace());

   // Debug output with different levels
   log("Info message");
   warn("Warning");
   error("Error");
   trace("With stack trace");

   // Assertions for tests
   assertEqual(1 + 1, 2, "Math should work");
   assertTruthy(data.value > 0, "Value should be positive");

   // Performance measurement
    time("operation");
    // ... Code ...
   timeEnd("operation");  // Outputs elapsed time

   // Programmatic breakpoint
   breakpoint();  // Pauses in debug mode
Relax
```

### 3. CLI Debug Options

| Option                  | Description                                   |
| ----------------------- | --------------------------------------------- |
| `--debug`               | Enables interactive debug mode                |
| `--verbose`             | Shows additional information during execution |
| `--breakpoints <LINES>` | Sets initial breakpoints (comma-separated)    |
| `--watch <VARS>`        | Watches variables (comma-separated)           |
| `--trace-file <FILE>`   | Writes debug trace to a file                  |

### 4. Debug Commands

In interactive debug mode, these commands are available:

#### Execution Control

- `continue` / `c` - Continue to the next breakpoint
- `step` / `s` - Execute one line (step into)
- `next` / `n` - Execute one line, skipping functions
- `finish` / `f` - Run until the end of the function
- `run` / `r` - Restart execution

#### Breakpoint Management

- `break <line>` / `b <line>` - Set a breakpoint
- `delete <line>` / `d <line>` - Delete a breakpoint
- `breakpoints` / `bl` - Show all breakpoints
- `clear` - Clear all breakpoints

#### Variable Inspection

- `locals` / `l` - Show local variables
- `globals` / `g` - Show global variables
- `print <var>` / `p <var>` - Print a variable
- `watch <expr>` / `w <expr>` - Add a watch expression

#### Navigation

- `list` - Source around current position
- `where` / `bt` - Show call stack
- `help` - Show help
- `quit` - Exit debug session

### 5. Trace Files

With `--trace-file`, you can create a detailed trace log:

```bash
hypnoscript exec script.hyp --debug --trace-file debug.log
```

The trace file contains:

- Executed lines with timestamps
- Variable changes
- Breakpoint hits
- Call stack changes

### 6. Error Reporting

HypnoScript provides detailed error reports with:

- **Line numbers and file locations**
- **Stack traces** for function calls
- **Type information** for variables
- **Context information** for better understanding

## Example Debug Session

```hyp
$ hypnoscript exec calculator.hyp --debug --breakpoints 10

HypnoScript Debugger v1.3.0
Type 'help' for available commands.

(hypno-debug) run
Starting execution...

-> Breakpoint hit at line 10
   10 |   induce result = add(a, b);

(hypno-debug) locals
Local variables:
  a: Int = 5
  b: Int = 3

(hypno-debug) step
   -> Line 15 (in add())
   15 |   awaken x + y;

(hypno-debug) print x
x = 5

(hypno-debug) finish
   -> Back in line 11
   11 |   observe(result);

(hypno-debug) print result
result = 8

(hypno-debug) continue
8
Program finished.
```

## Further Documentation

- [Debug Mode](./debug-mode) - Complete command reference
- [Breakpoints](./breakpoints) - Detailed breakpoint documentation
- [Debugging Tools](./tools) - All builtin functions
- [Best Practices](./best-practices) - Tips for effective debugging
- [Troubleshooting](./troubleshooting) - Solve common issues
