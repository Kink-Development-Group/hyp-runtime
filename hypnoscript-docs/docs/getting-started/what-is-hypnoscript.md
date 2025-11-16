# What is HypnoScript?

HypnoScript is a statically typed scripting language with hypnotic syntax. Instead of `class`, `function` or `print`, you'll find terms like `session`, `suggestion` and `observe`. The Rust-based implementation provides a lexer, parser, type checker, interpreter and WASM code generator in a compact toolchain bundle.

## Design Principles

- **Readability First** – Hypnotic keywords should be fun without losing comprehensibility.
- **Static Safety** – The type checker validates variables, function signatures, return values and session members.
- **Deterministic Execution** – The interpreter executes programs reproducibly and reports type errors, but doesn't necessarily abort.
- **One Binary, All Steps** – The CLI covers lexing, parsing, type checking, execution and optional WASM target.

## Language at a Glance

| Element                           | Description                                                                                     |
| --------------------------------- | ----------------------------------------------------------------------------------------------- |
| `Focus { ... } Relax`             | Wraps every program. `Relax` marks the end and is mandatory.                                    |
| `entrance { ... }`                | Optional startup block for initialization, greeting or setup.                                   |
| `finale { ... }`                  | Optional cleanup block that executes before `Relax`.                                            |
| `induce` / `implant`              | Declares mutable variables with optional type.                                                  |
| `freeze`                          | Declares constants.                                                                             |
| `observe` / `whisper` / `command` | Output with newline, without newline, or bold/imperative respectively.                          |
| `suggestion`                      | Defines functions; `awaken` (or `return`) returns values.                                       |
| `session`                         | Object-oriented structures with `expose` (public), `conceal` (private) and `dominant` (static). |
| `anchor`                          | Stores the current value of an expression for later.                                            |
| `oscillate`                       | Toggle for boolean variables.                                                                   |
| `deepFocus`                       | Optional addition after `if (...)` for more dramatic conditional blocks.                        |

## Example Program

```hyp
Focus {
    entrance {
        observe "Welcome to HypnoScript";
    }

    freeze MAX_DEPTH: number = 3;
    induce depth: number = 0;

    while (depth goingDeeper MAX_DEPTH) {
        observe "Depth: " + depth;
        depth = depth + 1;
    }

    suggestion introduce(name: string): string {
        awaken "Hello, " + name + "!";
    }

    observe introduce("Hypnotized Person");

    session Subject {
        expose name: string;
        conceal level: number;

        suggestion constructor(name: string) {
            this.name = name;
            this.level = 0;
        }

        expose suggestion deepen() {
            this.level = this.level + 1;
            observe this.name + " goes deeper: " + this.level;
        }
    }

    induce alice: Subject = Subject("Alice");
    alice.deepen();
} Relax
```

## Platform Components

- **Lexer & Parser** – Deliver token streams and ASTs, including hypnotic operator synonyms (`youAreFeelingVerySleepy`, `underMyControl`, …).
- **Type Checker** – Registers all builtins, checks function and session signatures, visibilities and conversions.
- **Interpreter** – Executes AST nodes, manages sessions, static fields, triggers and builtins.
- **WASM Code Generator** – Creates WebAssembly Text (.wat) for selected constructs.
- **CLI** – `hypnoscript` combines all steps: `run`, `lex`, `parse`, `check`, `compile-wasm`, `builtins`, `version`.

## Typical Use Cases

- **Script Experiments** – Combination of unusual syntax and familiar control structures.
- **Teaching & Workshops** – Shows how parser, type checker and interpreter work together.
- **Tooling Demos** – Example of how a language can be completely implemented in Rust.
- **Web-WASM Experiments** – Export program parts to `.wat` and use them in WebAssembly projects.

## Further Resources

- [Core Concepts](./core-concepts) – Overview of language elements, type system and runtime.
- [Installation](./installation) – Local setup of the toolchain.
- [Quick Start](./quick-start) – Your first script in minutes.
- [Language Reference](../language-reference/syntax) – Grammar, operators, functions, sessions.
- [Builtin Overview](../builtins/overview) – All standard functions by category.

HypnoScript makes hypnotic metaphors programmable – with an honest Rust foundation under the hood.
