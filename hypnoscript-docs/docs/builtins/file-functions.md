---
title: File Functions
---

# File Functions

File builtins let a HypnoScript program read, write, and inspect the filesystem — anchoring trance state to disk.

## Overview

| Function             | Signature                          | Brief Description                        |
| -------------------- | ---------------------------------- | ---------------------------------------- |
| `ReadFile`           | `(path: string) -> string`         | Read entire file as string               |
| `WriteFile`          | `(path: string, content: string) -> void` | Write string to file (creates parents) |
| `AppendFile`         | `(path: string, content: string) -> void` | Append string to file             |
| `DeleteFile`         | `(path: string) -> void`           | Delete a file                            |
| `CreateDirectory`    | `(path: string) -> void`           | Create a directory (recursively)         |
| `FileExists`         | `(path: string) -> boolean`        | Check whether a path exists              |
| `IsFile`             | `(path: string) -> boolean`        | Check whether a path is a file           |
| `IsDirectory`        | `(path: string) -> boolean`        | Check whether a path is a directory      |
| `ListDirectory`      | `(path: string) -> string[]`       | List directory entries                   |
| `GetFileSize`        | `(path: string) -> number`         | File size in bytes                       |
| `CopyFile`           | `(from: string, to: string) -> number` | Copy a file, returns bytes copied   |
| `RenameFile`         | `(from: string, to: string) -> void` | Rename / move a file                   |
| `GetFileExtension`   | `(path: string) -> string`         | Extract file extension (pure)            |
| `GetFileName`        | `(path: string) -> string`         | Extract file name (pure)                 |
| `GetParentDirectory` | `(path: string) -> string`         | Extract parent directory (pure)          |

## Example

```hyp
Focus {
    entrance {
        WriteFile("notes.txt", "You are getting sleepy...");
        AppendFile("notes.txt", "\nDeeper and deeper.");

        if (FileExists("notes.txt")) {
            induce content: string = ReadFile("notes.txt");
            observe content;
            observe "Size: " + GetFileSize("notes.txt") + " bytes";
        }

        DeleteFile("notes.txt");
    }
} Relax
```

## Filesystem Sandbox

All file builtins can be confined to a single directory — a protective circle around the session. Activate the sandbox with the `HYPNO_SANDBOX` environment variable or the `--sandbox <dir>` flag on `exec`:

```bash
hypnoscript exec script.hyp --sandbox ./workspace
# or
HYPNO_SANDBOX=./workspace hypnoscript exec script.hyp
```

With a sandbox root configured:

- **Relative paths** (`"notes.txt"`, `"data/log.txt"`) resolve against the sandbox root — not the process working directory.
- **Escape attempts** are rejected with a `PermissionDenied` error: `..` traversal, absolute paths outside the root, and symlinks that point outside the root.
- **Pure path-string helpers** (`GetFileExtension`, `GetFileName`, `GetParentDirectory`) never touch the filesystem and are unaffected.

Without a sandbox root, behavior is unchanged — file builtins access the filesystem freely.

```hyp
Focus {
    entrance {
        // With --sandbox ./workspace:
        WriteFile("safe.txt", "ok");          // -> ./workspace/safe.txt
        ReadFile("../outside.txt");           // -> PermissionDenied error
        ReadFile("/etc/passwd");              // -> PermissionDenied error
    }
} Relax
```

## See also

- [Builtin Overview](./overview)
- [CLI Commands – exec](../cli/commands#exec---execute-a-program)
- [CLI Configuration – Environment Variables](../cli/configuration#environment-variables)
