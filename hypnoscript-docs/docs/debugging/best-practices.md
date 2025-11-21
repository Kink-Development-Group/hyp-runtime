---
title: Debugging Best Practices
---

# Debugging Best Practices

HypnoScript offers various mechanisms to detect errors early and ensure code quality. Here are proven methods for effective debugging:

## Using Assertions

Use the `assert` statement to verify assumptions in the code. Assertion errors are highlighted in the CLI and test output.

```hyp
assert(x > 0, "x must be positive");
```

Assertion errors are collected and output at the end of execution:

```
❌ 1 assertion(s) failed:
   - x must be positive
```

## Structuring Tests

- Group tests in separate `.hyp` files.
- Use the CLI command `test` to run all or individual tests:

```bash
dotnet run --project HypnoScript.CLI -- test test_basic.hyp --debug
```

## Debug and Verbose Flags

- `--debug`: Shows additional debug output (e.g., stacktraces on errors).
- `--verbose`: Shows detailed analysis of tokens, AST, and execution.

## Interpreting Error Output

- Assertion errors are specially marked.
- Check the summary at the end of the test output for failed assertions.

## Additional Tips

- Set breakpoints strategically with `assert` or through targeted output (`observe`).
- Use the CLI options to debug specific tests or modules.
