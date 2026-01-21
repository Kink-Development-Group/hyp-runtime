---
title: CLI Testing
---

The Rust CLI does not include a separate test framework. Instead, you treat each `.hyp` file as an independent script and run it with `hypnoscript exec`. The files in the `hypnoscript-tests/` folder provide examples of assertions and error messages.

## Running Tests

```bash
# Run a single test file
hypnoscript exec hypnoscript-tests/test_basic.hyp

# Run all files in the folder
for file in hypnoscript-tests/*.hyp; do
    echo "== $file =="
    hypnoscript exec "$file"
done
```

## Type Checking First

```bash
hypnoscript check hypnoscript-tests/test_basic.hyp
```

This way you can detect type errors before assertions are triggered. The CLI does not automatically abort on errors, so it's worth running a separate `check` before `exec`.

## Integration into Scripts

- **PowerShell:**

  ```powershell
  Get-ChildItem hypnoscript-tests -Filter *.hyp | ForEach-Object {
      Write-Host "== $($_.Name) =="
      hypnoscript exec $_.FullName
  }
  ```

- **Makefile:**

  ```makefile
  test:
      @# Replace leading spaces with tabs, as Make requires this
      @for file in hypnoscript-tests/*.hyp; do \
          echo "== $$file =="; \
          hypnoscript exec $$file || exit 1; \
      done
  ```

## Assertions

The test files use `assert` statements as well as `observe` to check expected values. If an assertion block fails, the CLI displays an error message but continues execution. Therefore, make sure to search for error messages in the test script or terminate the script with `snap;` if needed.

## Debug-Assertions

Use the built-in assertion functions for automated testing:

```hyp
Focus
    induce result = calculate(5, 3);
    assertEqual(result, 8, "calculate should add correctly");

    induce items = getItems();
    assertTruthy(ArrayLength(items) > 0, "should return items");
Relax
```

Learn more about available commands in [CLI Commands](./commands).
