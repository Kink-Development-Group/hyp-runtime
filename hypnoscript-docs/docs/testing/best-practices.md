# Testing Best Practices

HypnoScript projects benefit from a layered testing strategy that mixes lightweight smoke tests with deeper integration suites. The guidance below captures the conventions used across the core repositories.

## Combine CLI and Runtime Coverage

- Exercise the CLI with representative `.hyp` scripts to confirm argument parsing and exit codes.
- Pair those checks with runtime-focused tests that interact with the VM APIs directly so error handling is covered even when the CLI is not involved.

## Keep Fixtures Focused

Store only the data each scenario needs in `docs/testing/fixtures`. Reuse shared setup utilities from the `hypnoscript-tests` package to avoid duplicating long trance sequences across files.

## Automate Cross-Platform Runs

Use the GitHub Actions matrix (Linux, macOS, Windows) as the source of truth. Local scripts should mirror the workflow commands to reduce surprises before merges.

## Watch for Non-Determinism

Disable time-sensitive or network-dependent routines unless they are required for the scenario. When randomness is essential, seed the generator through `SYSTEM::set_seed` so replays behave consistently.
