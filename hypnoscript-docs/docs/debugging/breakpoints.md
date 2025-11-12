# Breakpoints

Breakpoints let you pause a HypnoScript session at precise trance steps to inspect memory and hypnotic state transitions.

## Setting Breakpoints

- In the CLI, use `hypnoscript debug script.hyp --break label_name` to stop before the instruction tagged with `label_name`.
- Inside editors that support the HypnoScript language server, click the gutter to toggle a breakpoint; the location is saved in `.hypdbg` files.

## Inspecting State

While paused, you can:

- Run `state show` to dump the trance stack and current suggestion payload.
- Evaluate expressions with `eval <expression>` to probe variable values without resuming the script.

## Stepping Controls

Use the following commands to progress through the script:

- `step` advances a single instruction, entering nested suggestions.
- `next` executes the current instruction and pauses at the following one, skipping over nested sequences.
- `continue` resumes execution until the next breakpoint or the end of the session.
