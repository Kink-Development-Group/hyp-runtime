---
title: CLI Basics
---

The HypnoScript Command Line Interface (CLI) is the fastest tool to build, check, and run HypnoScript scripts. This page guides you through the most important subcommands and typical workflows.

## Help & Orientation

```bash
# Global help
hypnoscript --help

# Show version and features
hypnoscript version

# Help for a subcommand
hypnoscript run --help
```

The output always lists all available subcommands and their options. If a command seems unfamiliar, it's worth looking at `--help` – the text is generated directly from the actual CLI.

## Running Scripts

```bash
# Standard execution
hypnoscript run demo.hyp

# With additional output
hypnoscript run demo.hyp --verbose

# With debug information
hypnoscript run demo.hyp --debug
```

- `--verbose` outputs status messages like "Running file" or success messages.
- `--debug` additionally shows source code, token list, type checking results and the interpretation flow.
- Errors in the type checker don't stop execution – they are reported, then the interpreter continues.

## Analysis Tools

| Command                           | Purpose                                      |
| --------------------------------- | -------------------------------------------- |
| `hypnoscript lex <file>`          | Shows all tokens with index, type and lexeme |
| `hypnoscript parse <file>`        | Outputs the formatted Abstract Syntax Tree   |
| `hypnoscript check <file>`        | Checks types and reports inconsistencies     |
| `hypnoscript compile-wasm <file>` | Generates WebAssembly Text Format (`.wat`)   |

These tools can be ideally combined to narrow down parser or type errors. Example:

```bash
hypnoscript check scripts/report.hyp
hypnoscript parse scripts/report.hyp
hypnoscript compile-wasm scripts/report.hyp -o report.wat
```

## Exploring the Standard Library

```bash
hypnoscript builtins
```

The command groups all built-in functions by category (Math, String, Array, System, ...). Use it to quickly find suitable helpers.

## Typical Workflow

1. **Preparation** – Run `hypnoscript check` on all scripts.
2. **Error Analysis** – Use `lex` or `parse` for problems to inspect the specific section.
3. **Execution** – Test with `run`, activate `--debug` if needed.
4. **Deployment** – Optionally use `compile-wasm` if the script should run in the browser or a WASM environment.

```bash
# Example: complete round
hypnoscript check examples/inventory.hyp
hypnoscript run examples/inventory.hyp --debug
hypnoscript compile-wasm examples/inventory.hyp -o inventory.wat
```

## Tips & Tricks

- **Quick Iteration:** Use `--debug` as soon as something seems odd – tokens and AST immediately reveal whether the parser understood your intention.
- **Bundle Outputs:** Pipe the output to a file (`hypnoscript run script.hyp > output.txt`) to document longer runs.
- **Platform-agnostic:** On Windows, macOS and Linux, the commands are identical. The only requirement is that the `hypnoscript` binary is in the `PATH`.
- **Tests as Scripts:** The files in the `hypnoscript-tests/` folder can be started directly with `hypnoscript run`. This shows you real examples of control flow and sessions.

## Further Links

- [CLI Overview](../cli/overview) – Installation, binary variants and workflow
- [CLI Commands](../cli/commands) – Complete reference with all options
- [Language Reference](../language-reference/syntax) – Detailed grammar description
