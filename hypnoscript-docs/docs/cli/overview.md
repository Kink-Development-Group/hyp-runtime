---
sidebar_position: 1
---

# CLI Overview

The HypnoScript Command Line Interface (CLI) is a single binary (`hypnoscript`) built in Rust. It bundles lexer, parser, type checker, interpreter and the WASM code generator in one tool.

## Installation

### Pre-built Packages

1. Download the appropriate archive from [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases).
2. Extract the archive and add the binary path to your `PATH` environment variable.
3. Verify the installation with `hypnoscript version`.

### Building from Source

```bash
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime
cargo build --release -p hypnoscript-cli
# Optionally install
cargo install --path hypnoscript-cli
```

You'll find the compiled binaries under `target/release/`.

## Quick Start

```bash
# Show help
hypnoscript --help

# Version information
hypnoscript version

# Run a program
hypnoscript run hello.hyp
```

All subcommands are intentionally kept lean. For a deeper look, check out the following sections.

## Command Overview

| Command        | Brief Description                                |
| -------------- | ------------------------------------------------ |
| `run`          | Executes a HypnoScript program                   |
| `run --debug`  | Additionally shows tokens, AST and type checking |
| `lex`          | Tokenizes a file                                 |
| `parse`        | Shows the AST                                    |
| `check`        | Performs type checking                           |
| `compile-wasm` | Generates WebAssembly Text Format (.wat)         |
| `self-update`  | Checks releases and runs the new installer       |
| `builtins`     | Lists all available builtin functions            |
| `version`      | Shows version and feature information            |

Further details are provided on the [CLI Commands](./commands) page.

## Typical Workflow

```bash
# 1. Type checking without execution
hypnoscript check my_script.hyp

# 2. Check AST if there are errors
hypnoscript parse my_script.hyp

# 3. Enable debug output
hypnoscript run my_script.hyp --debug

# 4. Optionally generate WASM
hypnoscript compile-wasm my_script.hyp -o my_script.wat
```

## Platform Notes

- **Windows**: Use the ZIP from the release, extract to `%LOCALAPPDATA%\Programs\hypnoscript` and add the path.
- **macOS / Linux**: Copy the archive to `/usr/local/bin` or `~/.local/bin`.
- For portable use, you can specify the binary path directly (`./hypnoscript run demo.hyp`).

## Updates & Maintenance

- **Self-Update:** `hypnoscript self-update` checks releases and automatically starts the new `install.sh`. With `--check` only checks, `--force` forces a reinstallation, `--include-prerelease` enables RC/beta builds.
- **Installer in Release:** Each release additionally contains a `share/hypnoscript/install.sh` in addition to the binaries, so you can also start updates offline (e.g. `bash share/hypnoscript/install.sh --check`).
- **Windows Limitation:** On Windows, currently only `--check` is available; installation is still done via the manually downloaded archive.

## Next Steps

- [CLI Commands](./commands) – Details on all subcommands
- [CLI Basics](../getting-started/cli-basics) – Step-by-step guide
- [Language Reference](../language-reference/syntax) – Grammar & examples

---

**Tip:** `hypnoscript builtins` gives you a quick overview of the standard library.
