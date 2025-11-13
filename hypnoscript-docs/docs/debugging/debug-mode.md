# Debug Mode

Debug mode provides fine-grained insight into HypnoScript execution, exposing the virtual machine state, trance stack, and hypnotic suggestions as they execute.

## Enabling Debug Mode

Run any script with the `--debug` flag: `hypnoscript --debug session.hyp`. The CLI generates a structured log under `target/debug-logs/` with a timestamped filename.

## Output Format

The debug log is newline-delimited JSON. Each entry includes:

- `phase`: parser, compiler, or runtime
- `instruction`: mnemonic of the instruction currently executing
- `context`: key variables and induction parameters at that step

## Integrating With Editors

The HypnoScript VS Code extension reads the debug log and overlays inline diagnostics. Open the “Hypnotic Timeline” panel to replay the execution while watching stack depth and suggestion intensity changes.

## Performance Considerations

Debug mode slows execution because every instruction emits detailed telemetry. Avoid using it during latency-sensitive live inductions; capture traces in staging first, review them, and then rerun in release mode.
