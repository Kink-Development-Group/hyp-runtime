---
title: CLI Testing
---

The Rust CLI does not include a separate test framework. Instead, you treat each `.hyp` file as an independent script and run it with `hypnoscript run`. The files in the `hypnoscript-tests/` folder provide examples of assertions and error messages.

## Running Tests

```bash
# Run a single test file
hypnoscript run hypnoscript-tests/test_basic.hyp

# Run all files in the folder
for file in hypnoscript-tests/*.hyp; do
    echo "== $file =="
    hypnoscript run "$file"
done
```

## Type Checking First

```bash
hypnoscript check hypnoscript-tests/test_basic.hyp
```

This way you can detect type errors before assertions are triggered. The CLI does not automatically abort on errors, so it's worth running a separate `check` before `run`.

## Integration into Scripts

- **PowerShell:**

  ```powershell
  Get-ChildItem hypnoscript-tests -Filter *.hyp | ForEach-Object {
      Write-Host "== $($_.Name) =="
      hypnoscript run $_.FullName
  }
  ```

- **Makefile:**

  ```makefile
  test:
      @# Replace leading spaces with tabs, as Make requires this
      @for file in hypnoscript-tests/*.hyp; do \
          echo "== $$file =="; \
          hypnoscript run $$file || exit 1; \
      done
  ```

## Assertions

The test files use `assert` statements as well as `observe` to check expected values. If an assertion block fails, the CLI displays an error message but continues execution. Therefore, make sure to search for error messages in the test script or terminate the script with `snap;` if needed.

Learn more about available commands in [CLI Commands](./commands).
