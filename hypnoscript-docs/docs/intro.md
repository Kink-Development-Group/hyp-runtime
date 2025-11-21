---
sidebar_position: 1
---

# Welcome to HypnoScript

HypnoScript is a modern, esoteric programming language that combines hypnotic metaphors with a pragmatic Rust toolchain. The syntax resembles TypeScript but uses keywords like `Focus`, `induce`, `observe` or `Relax` to directly express hypnotic concepts.

## What is HypnoScript?

The current runtime consists entirely of Rust crates and provides:

- 🦀 **Native Toolchain** – Lexer, parser, static type checker, interpreter and WASM code generator are fully implemented in Rust.
- 🎯 **Hypnotic Syntax** – Language constructs like `deepFocus`, `snap`, `anchor` or `oscillate` translate hypnotic imagery into code.
- 🔒 **Static Type System** – The type checker knows numbers, strings, booleans, arrays, functions and sessions including visibility modifiers.
- 📦 **Standard Library** – Math, string, array, file, statistics, system, time and validation builtins are available out of the box.
- 🛠️ **CLI for the Entire Workflow** – A single binary (`hypnoscript`) offers `run`, `lex`, `parse`, `check`, `compile-wasm`, `builtins` and `version`.

The language is cross-platform (Windows/macOS/Linux) and generates native binaries as well as optional WebAssembly output.

## Syntax Fundamentals

| Concept                | Description                                                                                          |
| ---------------------- | ---------------------------------------------------------------------------------------------------- |
| `Focus { ... } Relax`  | Encloses every program (entry and exit point).                                                       |
| `entrance { ... }`     | Optional startup block for initialization or greeting.                                               |
| `finale { ... }`       | Optional cleanup block that is guaranteed to run at the end.                                         |
| `induce` / `freeze`    | Declares variables (`induce`/`implant`) or constants (`freeze`).                                     |
| `observe` / `whisper`  | Output with or without newline. `command` highlights text emphatically.                              |
| `if`, `while`, `loop`  | Control structures with hypnotic operator synonyms (`youAreFeelingVerySleepy`, `underMyControl`, …). |
| `suggestion`           | Function definition (global or within sessions).                                                     |
| `session`              | Object-oriented structures with fields (`expose`/`conceal`), constructors and static members.        |
| `anchor` / `oscillate` | Stores values between calls or toggles booleans.                                                     |

```hyp
Focus {
    entrance {
        observe "Welcome to the trance";
    }

    induce counter: number = 0;
    while (counter goingDeeper 3) {
        observe "Depth: " + counter;
        counter = counter + 1;
    }

    suggestion hypnoticSum(values: number[]): number {
        awaken ArraySum(values);
    }

    observe "Sum: " + ToString(hypnoticSum([2, 4, 6]));
} Relax
```

## Standard Library Overview

The builtins are organized into categories. A detailed reference can be found under [Standard Library](./builtins/overview).

- **Mathematics** – `Sin`, `Cos`, `Tan`, `Sqrt`, `Pow`, `Factorial`, `Clamp`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci`, …
- **Strings** – `Length`, `ToUpper`, `ToLower`, `Trim`, `Reverse`, `Replace`, `Split`, `Substring`, `PadLeft`, `IsWhitespace`, …
- **Arrays** – `ArrayLength`, `ArrayIsEmpty`, `ArraySum`, `ArrayAverage`, `ArraySlice`, `ArrayDistinct`, …
- **Files** – `ReadFile`, `WriteFile`, `AppendFile`, `ListDirectory`, `GetFileExtension`, …
- **System** – `GetOperatingSystem`, `GetUsername`, `GetArgs`, `Exit`, `GetCurrentDirectory`, …
- **Time & Date** – `CurrentTimestamp`, `CurrentDateTime`, `IsLeapYear`, `DayOfWeek`, …
- **Statistics** – `Mean`, `Median`, `Mode`, `StandardDeviation`, `Correlation`, `LinearRegression`, …
- **Validation** – `IsValidEmail`, `MatchesPattern`, `IsInRange`, `IsNumeric`, `IsLowercase`, …
- **Hypnotic Core Functions** – `Observe`, `Whisper`, `Command`, `Drift`, `DeepTrance`, `HypnoticCountdown`, `TranceInduction`, `HypnoticVisualization`.

## Development Workflow

```bash
# Read source, lex, parse, check and run
hypnoscript lex   examples/test.hyp
hypnoscript parse examples/test.hyp
hypnoscript check examples/test.hyp
hypnoscript run   examples/test.hyp

# Generate to WebAssembly (wat)
hypnoscript compile-wasm examples/test.hyp --output output.wat

# List all builtins
hypnoscript builtins
```

The interpreter executes programs deterministically. Type checking errors are reported but do not block execution – ideal for exploratory work.

## Next Steps

- [Installation](./getting-started/installation)
- [Quick Start](./getting-started/quick-start)
- [Core Concepts](./getting-started/core-concepts)
- [Language Reference](./language-reference/syntax)
- [Standard Library](./builtins/overview)

## Community & License

- GitHub: [Kink-Development-Group/hyp-runtime](https://github.com/Kink-Development-Group/hyp-runtime)
- Issues & Roadmap: [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
- License: [MIT](https://opensource.org/license/mit/)
