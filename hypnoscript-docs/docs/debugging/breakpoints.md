# Breakpoints

Breakpoints pause a HypnoScript execution at precise locations to inspect variables and program state.

## Set Breakpoints via CLI

### Using the --breakpoints Flag

Set initial breakpoints when starting debug mode:

```bash
# Single breakpoint at line 10
hypnoscript exec --debug --breakpoints 10 script.hyp

# Multiple breakpoints
hypnoscript exec --debug --breakpoints 10,25,42 script.hyp
```

### Interactive in Debug Mode

In interactive debug mode, breakpoints can be set dynamically:

```bash
(hypno-debug) break 15
Breakpoint set at line 15

(hypno-debug) b 30
Breakpoint set at line 30
```

## Breakpoint Commands

| Command         | Alias      | Description                                  |
| --------------- | ---------- | -------------------------------------------- |
| `break <line>`  | `b <line>` | Sets a breakpoint at the specified line      |
| `delete <line>` | `d <line>` | Removes the breakpoint at the specified line |
| `breakpoints`   | `bl`       | Shows all active breakpoints                 |
| `clear`         |            | Removes all breakpoints                      |

## List Breakpoints

```bash
(hypno-debug) breakpoints
Active breakpoints:
  Line 10
  Line 25
  Line 42

(hypno-debug) bl
Active breakpoints:
  Line 10
  Line 25
  Line 42
```

## Remove Breakpoints

```bash
# Remove a single breakpoint
(hypno-debug) delete 25
Breakpoint removed at line 25

# Use alias
(hypno-debug) d 42
Breakpoint removed at line 42

# Remove all breakpoints
(hypno-debug) clear
All breakpoints removed
```

## Inspect State at a Breakpoint

When execution pauses at a breakpoint:

```bash
-> Breakpoint hit at line 10
   10 |   induce result = calculate(a, b);

(hypno-debug) locals
Local variables:
  a: Int = 42
  b: Int = 17

(hypno-debug) print result
result: Null (not initialized)

(hypno-debug) where
Call Stack:
  #0: processData() at script.hyp:10
  #1: main() at script.hyp:5
  #2: <entry> at script.hyp:1
```

## Stepping Control

After a breakpoint, you can control execution:

| Command    | Alias | Description                                           |
| ---------- | ----- | ----------------------------------------------------- |
| `step`     | `s`   | Executes one line and stops (step into)               |
| `next`     | `n`   | Executes one line, skipping functions (step over)     |
| `finish`   | `f`   | Runs until the end of the current function (step out) |
| `continue` | `c`   | Continues execution to the next breakpoint            |

### Example

```bash
-> Breakpoint hit at line 10
   10 |   induce result = calculate(a, b);

(hypno-debug) step
  -> Line 20 (in calculate())
   20 |   awaken a + b;

(hypno-debug) finish
  -> Back to line 11
   11 |   observe(result);

(hypno-debug) continue
Result: 59
Program finished.
```

## Set Breakpoints in Code

You can also set breakpoints programmatically with the `breakpoint()` builtin function:

```hypnoscript
Focus
    induce x = 10;

    breakpoint();  // Pauses here in debug mode

    induce y = x * 2;
    observe(y);
Relax
```

The `breakpoint()` function only activates in debug mode and has no effect during normal execution.

## Best Practices

1. **Strategic placement**: Set breakpoints before complex calculations or after function calls
2. **Use sparingly**: Too many breakpoints can make debugging harder
3. **Use watch expressions**: Combine breakpoints with `--watch` for automatic variable monitoring
4. **Remember cleanup**: Remove `breakpoint()` calls before production use
