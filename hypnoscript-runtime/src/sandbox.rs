//! Optional filesystem sandbox for HypnoScript programs.
//!
//! When a sandbox root is configured (either programmatically via
//! [`set_sandbox_root`] or through the `HYPNO_SANDBOX` environment variable),
//! every path used by the file builtins is resolved and validated against
//! that root. Paths that escape the root — via `..` components, absolute
//! paths, or symlinks — are rejected with [`io::ErrorKind::PermissionDenied`].
//!
//! Without a configured root the sandbox is inactive and paths pass through
//! unchanged, preserving the historical behaviour of trusted scripts.

use std::env;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::RwLock;

/// Global sandbox root. `None` inside the option means "explicitly disabled";
/// the outer `Option` distinguishes "not yet initialised from environment".
static SANDBOX_ROOT: RwLock<Option<Option<PathBuf>>> = RwLock::new(None);

/// Configures the sandbox root directory for file builtins.
///
/// The directory must exist; it is canonicalised so symlink tricks cannot
/// widen the sandbox later. Passing a root replaces any previously
/// configured one.
pub fn set_sandbox_root(root: impl AsRef<Path>) -> io::Result<()> {
    let canonical = root.as_ref().canonicalize()?;
    *SANDBOX_ROOT.write().expect("sandbox lock poisoned") = Some(Some(canonical));
    Ok(())
}

/// Disables the sandbox (also suppresses `HYPNO_SANDBOX` for this process).
pub fn disable_sandbox() {
    *SANDBOX_ROOT.write().expect("sandbox lock poisoned") = Some(None);
}

/// Returns the active sandbox root, initialising it from the
/// `HYPNO_SANDBOX` environment variable on first use.
pub fn sandbox_root() -> Option<PathBuf> {
    if let Some(state) = SANDBOX_ROOT.read().expect("sandbox lock poisoned").as_ref() {
        return state.clone();
    }

    let from_env = env::var_os("HYPNO_SANDBOX")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .and_then(|p| p.canonicalize().ok());
    let mut guard = SANDBOX_ROOT.write().expect("sandbox lock poisoned");
    // Another thread may have initialised the state in the meantime.
    if guard.is_none() {
        *guard = Some(from_env);
    }
    guard.as_ref().and_then(|state| state.clone())
}

/// Returns whether the sandbox is currently active.
pub fn is_sandboxed() -> bool {
    sandbox_root().is_some()
}

/// Validates `path` against the sandbox and returns the path to use for the
/// actual filesystem operation.
///
/// With an inactive sandbox the input is returned unchanged. With an active
/// sandbox the path is resolved (relative paths against the sandbox root,
/// symlinks via the deepest existing ancestor) and must stay within the root.
pub fn checked_path(path: &str) -> io::Result<PathBuf> {
    let Some(root) = sandbox_root() else {
        return Ok(PathBuf::from(path));
    };

    let candidate = Path::new(path);
    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        // Relative paths are interpreted relative to the sandbox root so a
        // sandboxed script has a stable, predictable working directory.
        root.join(candidate)
    };

    let resolved = resolve_symlinks(&normalize_lexically(&absolute))?;
    if resolved.starts_with(&root) {
        Ok(resolved)
    } else {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!(
                "Path '{}' escapes the HypnoScript sandbox '{}'",
                path,
                root.display()
            ),
        ))
    }
}

/// Resolves `.` and `..` components without touching the filesystem.
/// `..` at the root is dropped (it cannot climb above the filesystem root).
fn normalize_lexically(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !matches!(
                    result.components().next_back(),
                    None | Some(Component::RootDir) | Some(Component::Prefix(_))
                ) {
                    result.pop();
                }
            }
            other => result.push(other.as_os_str()),
        }
    }
    result
}

/// Canonicalises the deepest existing ancestor of `path` (resolving
/// symlinks), then re-appends the non-existing tail. This lets sandboxed
/// scripts create new files while still preventing symlink escapes through
/// existing directories.
fn resolve_symlinks(path: &Path) -> io::Result<PathBuf> {
    let mut existing = path.to_path_buf();
    let mut tail: Vec<std::ffi::OsString> = Vec::new();

    while !existing.exists() {
        match existing.file_name() {
            Some(name) => {
                tail.push(name.to_os_string());
                existing.pop();
            }
            // Lexical normalisation already removed `.`/`..`, so this only
            // triggers at the filesystem root, which always exists.
            None => break,
        }
    }

    let mut resolved = existing.canonicalize()?;
    for part in tail.iter().rev() {
        resolved.push(part);
    }
    Ok(resolved)
}

/// The sandbox root is process-global state; tests that configure it (or
/// that perform real file I/O which an active sandbox would block) must be
/// serialised through this lock.
#[cfg(test)]
pub(crate) static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::MutexGuard;

    fn with_sandbox(root: &Path) -> MutexGuard<'static, ()> {
        let guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_sandbox_root(root).expect("failed to set sandbox root");
        guard
    }

    fn temp_sandbox_dir(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("hypno_sandbox_{}_{}", name, std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn inactive_sandbox_passes_paths_through() {
        let guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        disable_sandbox();
        assert!(!is_sandboxed());
        assert_eq!(
            checked_path("/etc/hostname").unwrap(),
            PathBuf::from("/etc/hostname")
        );
        drop(guard);
    }

    #[test]
    fn sandbox_allows_paths_inside_root() {
        let dir = temp_sandbox_dir("inside");
        let guard = with_sandbox(&dir);

        let resolved = checked_path("notes/output.txt").unwrap();
        assert!(resolved.starts_with(dir.canonicalize().unwrap()));

        disable_sandbox();
        drop(guard);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn sandbox_rejects_escapes() {
        let dir = temp_sandbox_dir("escape");
        let guard = with_sandbox(&dir);

        for attempt in ["../outside.txt", "/etc/passwd", "a/../../outside.txt"] {
            let err = checked_path(attempt).expect_err(attempt);
            assert_eq!(err.kind(), io::ErrorKind::PermissionDenied, "{attempt}");
        }

        disable_sandbox();
        drop(guard);
        let _ = fs::remove_dir_all(dir);
    }

    #[cfg(unix)]
    #[test]
    fn sandbox_rejects_symlink_escapes() {
        let dir = temp_sandbox_dir("symlink");
        let guard = with_sandbox(&dir);

        let link = dir.join("sneaky");
        let _ = fs::remove_file(&link);
        std::os::unix::fs::symlink("/", &link).unwrap();

        let err = checked_path("sneaky/etc/passwd").expect_err("symlink escape");
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);

        disable_sandbox();
        drop(guard);
        let _ = fs::remove_dir_all(dir);
    }
}
