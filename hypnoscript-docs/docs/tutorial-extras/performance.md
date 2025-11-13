# Performance Tuning

These tips help you speed up long HypnoScript sessions and reduce latency in interactive inductions.

## Profile First

Use `hypnoscript --profile session.hyp` to generate a flame graph that highlights expensive suggestions and loops. Focus optimization efforts on hotspots instead of guessing.

## Avoid Excessive Context Switching

Batch related suggestions into a single induction block. Rapidly alternating between incompatible trance states forces the runtime to rebuild safety guards and slows execution.

## Cache External Resources

When scripts fetch media or data from remote services, cache the results in a session-scoped dictionary. This prevents repeated requests and keeps the participant immersed.

## Tune Garbage Collection

Set `runtime.gc_threshold` in `hypnoscript.toml` to balance memory usage and pause times. Lower values trigger more frequent cleanup while higher values prioritize throughput.
