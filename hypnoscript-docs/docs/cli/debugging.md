---
title: CLI Debugging
---

The HypnoScript CLI relies on a few but effective debugging mechanisms. This guide shows how to quickly isolate errors and which commands help make program state visible.

## Debug and Verbose Mode

- `--debug` displays the source code, generated tokens, the AST, and type checker results before the interpreter starts.
- `--verbose` adds status messages (e.g., "Running file" or "Program executed successfully").
- Both flags can be combined: `hypnoscript run script.hyp --debug --verbose`.

## Token and AST Analysis

```bash
hypnoscript lex script.hyp
hypnoscript parse script.hyp
```

- Use `lex` to check which keywords and literals the lexer recognizes.
- `parse` provides the complete AST – ideal when control structures or sessions are not built as expected.

## Type Checking Without Execution

```bash
hypnoscript check script.hyp
```

- The type checker reports missing functions, incorrect return values, or unsuitable assignments.
- The CLI executes the program even with type errors; use `check` to catch errors beforehand.

## Typical Debug Workflow

```bash
# 1. Type Checking
hypnoscript check scripts/deep_trance.hyp

# 2. Inspect tokens & AST
hypnoscript lex scripts/deep_trance.hyp
hypnoscript parse scripts/deep_trance.hyp

# 3. Run with debug output
hypnoscript run scripts/deep_trance.hyp --debug
```

## Tips

- Temporarily comment out complex sections (`//`) and run the rest with `--debug` to isolate the problem locally.
- For array operations, `hypnoscript builtins` helps find suitable helper functions (e.g., `ArrayJoin`, `ArrayContains`).
- Save debug output with `> debug.log` if you want to compare it later (`hypnoscript run script.hyp --debug > debug.log`).
