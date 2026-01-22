# Debug Mode

Debug mode provides an interactive REPL environment for debugging HypnoScript programs with breakpoints, step-by-step execution, and variable inspection.

## Enable Debug Mode

Start a script with the `--debug` flag:

```bash
hypnoscript exec --debug session.hyp
```

## CLI Debug Options

| Option                  | Description                                                      |
| ----------------------- | ---------------------------------------------------------------- |
| `--debug`               | Enables interactive debug mode                                   |
| `--breakpoints <LINES>` | Sets initial breakpoints at the specified lines (e.g. `5,10,15`) |
| `--watch <VARS>`        | Watches variables during execution (e.g. `counter,result`)       |
| `--trace-file <FILE>`   | Writes a debug trace to a file                                   |
| `--verbose`             | Enables verbose output                                           |

### Example with Options

```bash
# Debug with initial breakpoints and watch expressions
hypnoscript exec --debug --breakpoints 5,12,25 --watch x,y,result myscript.hyp
```

## Debug Commands

In interactive debug mode, the following commands are available:

### Execution Control

| Command    | Alias | Description                                            |
| ---------- | ----- | ------------------------------------------------------ |
| `continue` | `c`   | Continues execution until the next breakpoint          |
| `step`     | `s`   | Executes one line and stops (step into)                |
| `next`     | `n`   | Executes one line, skipping over functions (step over) |
| `finish`   | `f`   | Runs until the end of the current function (step out)  |
| `run`      | `r`   | Starts execution from the beginning                    |

### Breakpoint Management

| Command         | Alias      | Description                                  |
| --------------- | ---------- | -------------------------------------------- |
| `break <line>`  | `b <line>` | Sets a breakpoint at the specified line      |
| `delete <line>` | `d <line>` | Removes the breakpoint at the specified line |
| `breakpoints`   | `bl`       | Shows all active breakpoints                 |
| `clear`         |            | Removes all breakpoints                      |

### Variable Inspection

| Command        | Alias      | Description                   |
| -------------- | ---------- | ----------------------------- |
| `locals`       | `l`        | Shows all local variables     |
| `globals`      | `g`        | Shows all global variables    |
| `print <var>`  | `p <var>`  | Shows the value of a variable |
| `watch <expr>` | `w <expr>` | Adds a watch expression       |
| `unwatch <id>` |            | Removes a watch expression    |
| `watches`      |            | Shows all watch expressions   |

### Source View

| Command              | Alias | Description                              |
| -------------------- | ----- | ---------------------------------------- |
| `list`               |       | Shows source around the current position |
| `list <start> <end>` |       | Shows source from line start to end      |
| `where`              | `bt`  | Shows the current call stack             |

### Other Commands

| Command | Description            |
| ------- | ---------------------- |
| `help`  | Shows the help         |
| `quit`  | Ends the debug session |

## Example Debug Session

```bash
$ hypnoscript exec --debug examples/calculator.hyp

HypnoScript Debugger v1.2.0
Type 'help' for available commands.

(hypno-debug) b 10
Breakpoint set at line 10

(hypno-debug) b 25
Breakpoint set at line 25

(hypno-debug) run
Starting execution...

-> Breakpoint hit at line 10
   10 |   induce result = calculate(a, b);

(hypno-debug) locals
Local variables:
  a: Int = 42
  b: Int = 17

(hypno-debug) step
   11 |   observe("Result: " + result);

(hypno-debug) print result
result = 59

(hypno-debug) continue
Result: 59

-> Breakpoint hit at line 25

(hypno-debug) where
Call stack:
  #0: main() at calculator.hyp:25
  #1: <entry> at calculator.hyp:1

(hypno-debug) continue
Program finished.

(hypno-debug) quit
```

## Source Context

When execution is paused, the debugger shows source context with the current position:

```hyp
    8 |   induce a = 42;
    9 |   induce b = 17;
 -> 10 |   induce result = calculate(a, b);
   11 |   observe("Result: " + result);
   12 | }
```

The arrow `->` marks the current execution position.

## Watch Expressions

Watch expressions are evaluated automatically on each stop:

```bash
(hypno-debug) watch counter
Watch #1 added: counter

(hypno-debug) watch result * 2
Watch #2 added: result * 2

(hypno-debug) step
   15 |   counter = counter + 1;

Watch Values:
  #1 counter = 5
  #2 result * 2 = 118
```

## Trace File

With `--trace-file`, you can log debug execution to a file:

```bash
hypnoscript exec --debug --trace-file debug.log myscript.hyp
```

The trace file contains:

- Executed lines with timestamps
- Variable changes
- Breakpoint hits
- Call stack changes

## Performance Notes

Debug mode slows execution significantly because every instruction is monitored. Use it for:

- Development and troubleshooting
- Understanding program flow
- Testing new features

Disable debug mode for production execution.
