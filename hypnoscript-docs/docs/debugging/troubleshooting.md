# Troubleshooting

When a HypnoScript session misbehaves, work through the following checklist before diving into the runtime internals.

## Confirm the Execution Environment

- Verify the CLI version with `hypnoscript --version` and ensure it matches the runtime bundled with your project.
- Inspect `hypnoscript.toml` for stale paths—especially the `session_dir` and custom induction libraries.

## Inspect Runtime Logs

Enable verbose logging with `--trace` to capture stack transitions and variable bindings. Store the resulting log alongside the failing script so regressions can be compared.

## Reduce the Scenario

Comment out non-essential trance steps until the failure disappears. This narrows down the instruction or builtin that triggers the issue and keeps the reproduction file short.

## Validate External Integrations

Check network credentials, file permissions, and long-running hypnotic hooks whenever a script depends on external systems. Most “hangs” originate from constrained resources rather than the interpreter itself.
