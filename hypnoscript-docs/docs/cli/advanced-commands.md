---
title: Advanced CLI Commands
---

Die HypnoScript CLI hält die Zahl der Subcommands bewusst klein. Es gibt aktuell keine versteckten oder „fortgeschrittenen“ Befehle – stattdessen kombinierst du die vorhandenen Tools flexibel.

## Nützliche Kombinationen

- **Syntax + Ausführung:** `hypnoscript check file.hyp && hypnoscript run file.hyp --debug`
- **WASM-Pipeline:** `hypnoscript compile-wasm file.hyp && wat2wasm file.wat`
- **AST-Vergleich:** `hypnoscript parse file.hyp > ast.log`

## Alias-Ideen

- `alias hrun='hypnoscript run --debug'`
- `function hcheck() { hypnoscript check "$1" && hypnoscript run "$1"; }`

Weitere Befehle findest du auf der Seite [CLI-Befehle](./commands).
