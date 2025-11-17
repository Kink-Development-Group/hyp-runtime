---
title: Advanced CLI Commands
---

The HypnoScript CLI deliberately keeps the number of subcommands small. There are currently no hidden or "advanced" commands – instead, you combine the existing tools flexibly.

## Useful Combinations

- **Syntax + Execution:** `hypnoscript check file.hyp && hypnoscript run file.hyp --debug`
- **WASM Pipeline:** `hypnoscript compile-wasm file.hyp && wat2wasm file.wat`
- **AST Comparison:** `hypnoscript parse file.hyp > ast.log`

## Alias Ideas

- `alias hrun='hypnoscript run --debug'`
- `function hcheck() { hypnoscript check "$1" && hypnoscript run "$1"; }`

## Update Automation

- **CI Check for Updates:** `hypnoscript self-update --check || echo "Update available"`
- **Air-Gapped Updates:** Extract packages from the release and run `share/hypnoscript/install.sh --prefix ~/.local` manually
- **Script Chains:** `hypnoscript self-update --quiet --no-sudo && hypnoscript version` for low-maintenance deployments

Find more commands on the [CLI Commands](./commands) page.
