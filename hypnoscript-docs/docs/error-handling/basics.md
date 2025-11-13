# Error Handling Basics

HypnoScript surfaces recoverable issues as `WARN` events and fatal problems as `ERROR` events. Understanding the distinction keeps trance sessions safe and predictable.

## Categorizing Issues

- **Warnings** signal soft failures—missing optional cues, transient network hiccups, or deprecated suggestions. The session continues unless you explicitly abort.
- **Errors** terminate execution. They commonly arise from type mismatches, invalid hypnotic targets, or uncaught runtime panics in custom extensions.

## Handling Warnings

Use the `ON WARN` block to intercept warnings and supply fallback logic:

```hypnoscript
ON WARN (event) {
  LOG "Switching to calm fallback";
  SUGGEST calm_state();
}
```

## Handling Errors

Wrap dangerous instructions with `TRY`/`CATCH` to restore the participant to a safe default state before propagating the failure:

```hypnoscript
TRY {
  SUGGEST deep_trance();
} CATCH (err) {
  LOG err.message;
  SUGGEST safe_exit();
}
```
