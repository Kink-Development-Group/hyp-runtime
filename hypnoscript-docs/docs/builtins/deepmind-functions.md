---
description: Higher-order control-flow and composition builtins for HypnoScript.
---

# DeepMind Functions

DeepMind builtins extend HypnoScript with powerful control-flow and functional programming patterns. They
work hand-in-hand with `suggestion` blocks and allow loops, delays, error handling, and function composition
to be expressed declaratively.

## Overview

| Function             | Return value | Brief Description                        |
| -------------------- | ------------ | ---------------------------------------- |
| `RepeatAction`       | `void`       | Repeat an action a fixed number of times |
| `DelayedSuggestion`  | `void`       | Run an action after a millisecond delay  |
| `IfTranced`          | `void`       | Conditionally execute two suggestions    |
| `RepeatUntil`        | `void`       | Repeat action until condition is `true`  |
| `RepeatWhile`        | `void`       | Repeat action while condition is `true`  |
| `SequentialTrance`   | `void`       | Execute a list of actions sequentially   |
| `Compose` / `Pipe`   | `suggestion` | Combine functions                        |
| `TryOrAwaken`        | `void`       | Handle error path                        |
| `EnsureAwakening`    | `void`       | Guarantee cleanup execution              |
| `MeasureTranceDepth` | `number`     | Measure runtime in milliseconds          |
| `Memoize`            | `suggestion` | Cache function results                   |

:::tip Naming conventions
All DeepMind builtins use PascalCase (`RepeatAction`) and accept `suggestion()` blocks as parameters.
Signatures are case-insensitive, so `repeataction` works too.
:::

## Repetition & Timing

### RepeatAction(times, action)

- **Signature:** `(times: number, action: () -> void) -> void`
- **Description:** Executes `action` `times` times. Negative values are ignored.

```hyp
RepeatAction(3, suggestion() {
    observe "Affirmation";
});
```

### DelayedSuggestion(action, delayMs)

- **Signature:** `(action: () -> void, delay: number) -> void`
- **Description:** Executes `action` after `delay` milliseconds. Execution blocks until the time elapses.

```hyp
DelayedSuggestion(suggestion() {
    observe "Welcome after 2 seconds";
}, 2000);
```

## Conditional Execution

### IfTranced(condition, thenAction, elseAction)

- **Signature:** `(condition: boolean, then: () -> void, otherwise: () -> void) -> void`
- **Description:** Evaluates condition; if `true` runs `then`, otherwise runs `otherwise`.

```hyp
IfTranced(audienceSize > 10,
    suggestion() { observe "Large group"; },
    suggestion() { observe "Intimate session"; }
);
```

## Composition & Pipelines

### Compose(f, g)

- **Signature:** `(f: (B) -> C, g: (A) -> B) -> (A -> C)`
- **Description:** First `g`, then `f`. Useful for reusable data pipelines.

```hyp
suggestion double(x: number): number { awaken x * 2; }
suggestion addTen(x: number): number { awaken x + 10; }

induce transformer = Compose(double, addTen);
induce result: number = transformer(5); // 30
```

### Pipe(f, g)

- **Signature:** `(f: (A) -> B, g: (B) -> C) -> (A -> C)`
- **Description:** Reverse order: first `f`, then `g`.

```hyp
induce pipeline = Pipe(double, addTen);
observe pipeline(5); // 20
```

## Loop Control

### RepeatUntil(action, condition)

- **Signature:** `(action: () -> void, condition: () -> boolean) -> void`
- **Description:** Executes `action` while `condition()` returns `false`. Condition is checked after each iteration.

```hyp
induce counter: number = 0;
RepeatUntil(
    suggestion() { counter = counter + 1; },
    suggestion(): boolean { awaken counter >= 5; }
);
```

### RepeatWhile(condition, action)

- **Signature:** `(condition: () -> boolean, action: () -> void) -> void`
- **Description:** Checks `condition()` before each iteration; if `true` runs `action`, otherwise ends the loop.

```hyp
induce energy: number = 3;
RepeatWhile(
    suggestion(): boolean { awaken energy > 0; },
    suggestion() {
        observe "Energy left: " + energy;
        energy = energy - 1;
    }
);
```

## Sequences & Error Handling

### SequentialTrance(actions)

- **Signature:** `(actions: (() -> void)[]) -> void`
- **Description:** Executes a list of `suggestion` blocks sequentially.

```hyp
SequentialTrance([
    suggestion() { observe "Phase 1"; },
    suggestion() { observe "Phase 2"; },
    suggestion() { observe "Phase 3"; }
]);
```

### TryOrAwaken(tryAction, catchAction)

- **Signature:** `(try: () -> Result<void, string>, catch: (error: string) -> void) -> void`
- **Description:** Executes `try` and calls `catch` with the error message on failure.

```hyp
TryOrAwaken(
    suggestion(): Result<void, string> {
        if (audienceSize < 0) {
            awaken Err("Negative Audience");
        }
        observe "Session starts";
        awaken Ok(());
    },
    suggestion(error: string) {
        observe "Error: " + error;
    }
);
```

### EnsureAwakening(mainAction, cleanup)

- **Signature:** `(main: () -> void, cleanup: () -> void) -> void`
- **Description:** Executes `main` and guarantees that `cleanup` is called afterwards.

```hyp
EnsureAwakening(
    suggestion() {
        observe "Open file";
    },
    suggestion() {
        observe "Close file";
    }
);
```

## Measurement & Memoization

### MeasureTranceDepth(action)

- **Signature:** `(action: () -> void) -> number`
- **Description:** Executes `action` and returns the duration in milliseconds.

```hyp
induce duration: number = MeasureTranceDepth(suggestion() {
    RepeatAction(1000, suggestion() { observe "Tick"; });
});
observe "Runtime: " + duration + " ms";
```

### Memoize(f)

- **Signature:** `(f: (A) -> R) -> (A -> R)`
- **Description:** Returns a wrapper function. In the current runtime version the result is not cached permanently,
  but the interface stays stable for future optimizations.

```hyp
suggestion square(x: number): number { awaken x * x; }
induce memoSquare = Memoize(square);

observe memoSquare(4); // 16
observe memoSquare(4); // 16 (future calls from cache)
```

## Usage Tips

- `RepeatAction`, `RepeatUntil`, and `RepeatWhile` block synchronously; use `DelayedSuggestion` for simple
  time-based control.
- Combine `Compose` and `Pipe` with array or string builtins to keep filter-map-reduce chains readable.
- `TryOrAwaken` expects a `Result`-like return value. Use `Ok(())` for success and `Err("Message")` for errors.
- `MeasureTranceDepth` is suitable for quick performance measurements without extra tooling.

## See also

- [Builtin Overview](./overview)
- [Complete Reference – DeepMind](./_complete-reference#deepmind-builtins-higher-order-functions)
- [Show CLI builtins](../cli/commands#builtins)
