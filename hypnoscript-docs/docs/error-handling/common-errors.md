# Common Errors

The table below lists frequently reported error codes and the usual steps to resolve them.

| Code   | Meaning                                | Typical Fix                                                                                  |
| ------ | -------------------------------------- | -------------------------------------------------------------------------------------------- |
| HS1001 | Unknown suggestion or induction        | Check the spelling or ensure the module exposing the suggestion is imported.                 |
| HS2004 | Type mismatch while binding a variable | Coerce the value with `CAST` or adjust the variable declaration to match the inferred type.  |
| HS3010 | Session timeout reached                | Increase the timeout in `hypnoscript.toml` or optimize long-running routines.                |
| HS4002 | Unsafe file access blocked             | Add the directory to the allowed paths list or run with elevated permissions if appropriate. |

## Next Steps

If an error is missing from this table, enable debug mode, capture the full trace, and file an issue with the log attached so the runtime team can expand the catalog.
