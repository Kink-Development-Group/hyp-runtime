# Core Concepts

This overview summarizes the most important building blocks of the current HypnoScript implementation. When you read the code or tests in the repository, you'll find exactly these concepts.

## Program Structure

- **Focus/Relax**: Every script starts with `Focus {` and ends with `} Relax`.
- **`entrance`**: Optional block directly after `Focus`, ideal for setup and greeting.
- **`finale`**: Optional block before `Relax`, always executed (cleanup).

```hyp
Focus {
    entrance { observe "Hello"; }
    // ... regular code ...
    finale { observe "Goodbye"; }
} Relax
```

## Declarations & Types

- `induce name: string = "Text";` – mutable variable.
- `implant` – alias for `induce`.
- `freeze PI: number = 3.14159;` – constant.
- Arrays are denoted with `[]`: `induce values: number[] = [1, 2, 3];`.
- Supported types: `number`, `string`, `boolean`, arrays, functions, sessions. A `trance` type exists in the type system but is not currently actively used.

## Control & Operators

- `if`, `else if`, `else`
- `while` for conditional loops
- `loop` supports both the infinite loop `loop { ... }` and a classic header `loop (init; condition; update) { ... }`; `pendulum (...)` is an alias that always requires a condition.
- `snap` (alias `break`), `sink` (alias `continue`)
- Hypnotic operators like `youAreFeelingVerySleepy` (`==`) or `underMyControl` (`&&`)
- Booleans can be toggled with `oscillate flag;`

## Functions

- Defined with `suggestion name(params): returnType { ... }`
- `awaken` (or `return`) exits a function.
- Triggers use `trigger name = suggestion(...) { ... }` and behave like callbacks.

```hyp
suggestion greet(name: string) {
    observe "Hello, " + name + "!";
}

trigger onWelcome = suggestion(person: string) {
    greet(person);
}
```

## Sessions (Object Orientation)

- `session Name { ... }` creates a class.
- Fields: `expose` (public) or `conceal` (private). `dominant` makes fields or methods static.
- Methods use `suggestion`, `imperativeSuggestion` or `dominantSuggestion` (the latter enforces static).
- Constructors: `suggestion constructor(...) { ... }`.
- The interpreter injects `this` for instance methods and prevents static members from being accessed via instances (and vice versa).

```hyp
session Counter {
    expose name: string;
    conceal value: number = 0;

    suggestion constructor(name: string) {
        this.name = name;
    }

    expose suggestion increment() {
        this.value = this.value + 1;
        observe this.name + ": " + this.value;
    }
}

induce c: Counter = Counter("HypnoBot");
c.increment();
```

## Builtins

The type checker registers all standard functions. Important categories:

- **Math**: `Sin`, `Cos`, `Sqrt`, `Pow`, `Clamp`, `Factorial`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci`, …
- **Strings**: `Length`, `ToUpper`, `Trim`, `Replace`, `Split`, `Substring`, `PadLeft`, `IsWhitespace`, …
- **Arrays**: `ArrayLength`, `ArrayIsEmpty`, `ArraySum`, `ArrayAverage`, `ArraySlice`, `ArrayDistinct`, …
- **System & Files**: `GetOperatingSystem`, `GetUsername`, `GetArgs`, `ReadFile`, `WriteFile`, `ListDirectory`, …
- **Time & Statistics**: `CurrentTimestamp`, `CurrentDate`, `Mean`, `Median`, `StandardDeviation`, `Correlation`, …
- **Validation & Utility**: `IsValidEmail`, `MatchesPattern`, `HashString`, `SimpleRandom`, …

All builtins are compactly available via `hypnoscript builtins`.

## CLI Workflow

```bash
hypnoscript lex file.hyp       # Show tokens
hypnoscript parse file.hyp     # Inspect AST
hypnoscript check file.hyp     # Type checking
hypnoscript run file.hyp       # Execute
hypnoscript compile-wasm file.hyp -o file.wat
hypnoscript version            # Toolchain info
```

- `--debug` with the `run` command shows intermediate steps (source, tokens, type check).
- `--verbose` adds additional status messages.

## Where to Continue Reading

- [Quick Start](./quick-start) – Your first script step by step
- [CLI Basics](./cli-basics) – All subcommands in detail
- [Syntax Reference](../language-reference/syntax) – Complete grammar
- [Builtin Overview](../builtins/overview) – All functions by category

With these concepts, you can read the repository code easily and write your own scripts.
