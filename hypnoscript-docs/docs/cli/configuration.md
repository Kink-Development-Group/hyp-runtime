---
sidebar_position: 3
---

# CLI Configuration

The Rust-based HypnoScript CLI deliberately avoids global configuration files. Instead, you control behavior exclusively through subcommands and their flags. This guide shows which switches are available and how to automate them with shell scripts or tooling.

## CLI Runtime Flags

| Subcommand                          | Options                | Effect                                                                    |
| ----------------------------------- | ---------------------- | ------------------------------------------------------------------------- |
| `run <file>`                        | `--debug`, `--verbose` | Debug shows tokens, AST, and type checks; verbose outputs status messages |
| `compile-wasm`                      | `--output <file>`      | Selects the name of the `.wat` file (default: `<input>.wat`)              |
| `version`                           | _(none)_               | Outputs toolchain information                                             |
| `lex`, `parse`, `check`, `builtins` | _(none)_               | Use no additional options                                                 |

More flags currently don't exist. This makes the CLI simple but also very predictable – especially for scripts and CI.

## Creating Custom Wrappers

If you frequently want to use the same options, a small wrapper script is worthwhile.

### PowerShell (Windows)

```powershell
function Invoke-HypnoScriptRun {
    param(
        [Parameter(Mandatory=$true)]
        [string]$File,
        [switch]$Debug,
        [switch]$Verbose
    )

    $args = @('run', $File)
    if ($Debug) { $args += '--debug' }
    if ($Verbose) { $args += '--verbose' }
    hypnoscript @args
}

# Usage
Invoke-HypnoScriptRun -File 'scripts/demo.hyp' -Verbose
```

### Bash / Zsh (macOS, Linux)

```bash
hyp() {
  local mode="$1"; shift
  case "$mode" in
    run)
      hypnoscript run "$@" --verbose ;;
    check)
      hypnoscript check "$@" ;;
    *)
      hypnoscript "$mode" "$@" ;;
  esac
}

# Example
hyp run scripts/demo.hyp
```

You can store such wrappers under version control in the project (`scripts/`).

## Project-Related Workflows

Even without a configuration file, you can bundle processes:

- **`package.json` / npm scripts:** `"check": "hypnoscript check src/**/*.hyp"`
- **Makefile:** `check: ; hypnoscript check $(FILE)`
- **CI Pipeline:** Use the `run`, `check`, and `compile-wasm` commands directly in your jobs.

This documents how the project should be built or checked – without custom CLI config.

## Environment Variables

The CLI currently does not read any special `HYPNOSCRIPT_*` variables. You can still use environment variables to control file paths or flags:

```bash
export HYPNO_DEFAULT=examples/intro.hyp
hypnoscript run "$HYPNO_DEFAULT"
```

Or in PowerShell:

```powershell
$env:DEFAULT_HYP = 'examples/intro.hyp'
hypnoscript run $env:DEFAULT_HYP --debug
```

Such variables are purely conventional – the CLI does not automatically access them.

## Recommendations

- **Document Wrappers:** Create a README in the `scripts/` folder so others can follow the workflow.
- **Use `--debug` sparingly:** In CI pipelines, `--verbose` is often sufficient. Debug output can become huge.
- **Pin Version:** Reference a specific version in scripts (`hypnoscript version`) or store the binary as an artifact to get reproducible builds.

## Troubleshooting

1. **`hypnoscript` not found**

```bash
# Check if the binary is in PATH
which hypnoscript    # macOS/Linux
Get-Command hypnoscript | Select-Object Source  # PowerShell

# If not present: add path
export PATH="$PATH:$HOME/.cargo/bin"          # Example Linux
```

2. **No execution permissions**

```bash
chmod +x hypnoscript            # macOS/Linux
Set-ExecutionPolicy RemoteSigned # Windows PowerShell (if needed)
```

3. **Unexpected output / syntax errors**

```bash
# Run again with debug info
hypnoscript run script.hyp --debug

# Check tokens
hypnoscript lex script.hyp
```

## Next Steps

- [CLI Overview](./overview) – Installation paths & workflow
- [CLI Commands](./commands) – Complete reference of subcommands
- [CLI Basics](../getting-started/cli-basics) – Everyday examples

---

**Tip:** Build custom wrappers in `scripts/` to simplify recurring calls.
