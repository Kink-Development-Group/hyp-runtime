use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;
use crate::sandbox;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// File I/O builtin functions
///
/// Provides comprehensive file system operations including reading, writing,
/// directory management, and file metadata queries.
///
/// # Sandboxing
///
/// All functions that touch the filesystem validate their paths through the
/// [`sandbox`] module. When a sandbox root is configured (via
/// `HYPNO_SANDBOX` or [`sandbox::set_sandbox_root`]), access outside that
/// root is rejected with a `PermissionDenied` error. Pure path-string
/// helpers (`GetFileExtension`, `JoinPath`, ...) are unaffected.
pub struct FileBuiltins;

impl BuiltinModule for FileBuiltins {
    fn module_name() -> &'static str {
        "File"
    }

    fn description() -> &'static str {
        "File I/O and file system operations"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let locale = crate::localization::detect_locale(locale);
        let msg = LocalizedMessage::new("File I/O and file system operations")
            .with_translation("de", "File I/O and file system operations")
            .with_translation(
                "fr",
                "Opérations d'E/S de fichiers et de système de fichiers",
            )
            .with_translation("es", "Operaciones de E/S de archivos y sistema de archivos");
        msg.resolve(&locale).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "ReadFile",
            "WriteFile",
            "AppendFile",
            "ReadLines",
            "WriteLines",
            "FileExists",
            "IsFile",
            "IsDirectory",
            "DeleteFile",
            "CreateDirectory",
            "ListDirectory",
            "GetFileSize",
            "GetFileExtension",
            "GetFileName",
            "GetParentDirectory",
            "JoinPath",
            "CopyFile",
            "MoveFile",
        ]
    }
}

impl FileBuiltins {
    /// Ensure the parent directory of a path exists
    fn ensure_parent_dir(path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    /// Read entire file as string
    pub fn read_file(path: &str) -> io::Result<String> {
        fs::read_to_string(sandbox::checked_path(path)?)
    }

    /// Write string to file
    pub fn write_file(path: &str, content: &str) -> io::Result<()> {
        let path = sandbox::checked_path(path)?;
        Self::ensure_parent_dir(&path)?;
        fs::write(&path, content)
    }

    /// Append string to file
    pub fn append_file(path: &str, content: &str) -> io::Result<()> {
        let path = sandbox::checked_path(path)?;
        Self::ensure_parent_dir(&path)?;

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        file.write_all(content.as_bytes())
    }

    /// Read file contents as lines
    pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
        let file = fs::File::open(sandbox::checked_path(path)?)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    /// Write lines to a file using `\n` separators
    pub fn write_lines(path: &str, lines: &[String]) -> io::Result<()> {
        let path = sandbox::checked_path(path)?;
        Self::ensure_parent_dir(&path)?;
        let mut file = fs::File::create(&path)?;
        for (index, line) in lines.iter().enumerate() {
            if index > 0 {
                file.write_all(b"\n")?;
            }
            file.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    /// Check if file exists (paths outside the sandbox report `false`)
    pub fn file_exists(path: &str) -> bool {
        sandbox::checked_path(path).is_ok_and(|p| p.exists())
    }

    /// Check if path is file (paths outside the sandbox report `false`)
    pub fn is_file(path: &str) -> bool {
        sandbox::checked_path(path).is_ok_and(|p| p.is_file())
    }

    /// Check if path is directory (paths outside the sandbox report `false`)
    pub fn is_directory(path: &str) -> bool {
        sandbox::checked_path(path).is_ok_and(|p| p.is_dir())
    }

    /// Delete file
    pub fn delete_file(path: &str) -> io::Result<()> {
        fs::remove_file(sandbox::checked_path(path)?)
    }

    /// Create directory
    pub fn create_directory(path: &str) -> io::Result<()> {
        fs::create_dir_all(sandbox::checked_path(path)?)
    }

    /// List files in directory
    pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(sandbox::checked_path(path)?)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }
        Ok(files)
    }

    /// Get file size in bytes
    pub fn get_file_size(path: &str) -> io::Result<u64> {
        fs::metadata(sandbox::checked_path(path)?).map(|m| m.len())
    }

    /// Copy file
    pub fn copy_file(from: &str, to: &str) -> io::Result<u64> {
        fs::copy(sandbox::checked_path(from)?, sandbox::checked_path(to)?)
    }

    /// Rename/move file
    pub fn rename_file(from: &str, to: &str) -> io::Result<()> {
        fs::rename(sandbox::checked_path(from)?, sandbox::checked_path(to)?)
    }

    /// Get file extension
    pub fn get_file_extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    /// Get file name without extension
    pub fn get_file_name(path: &str) -> Option<String> {
        Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }

    /// Get parent directory
    pub fn get_parent_directory(path: &str) -> Option<String> {
        Path::new(path)
            .parent()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
    }

    /// Copy a directory recursively
    pub fn copy_directory_recursive(from: &str, to: &str) -> io::Result<()> {
        let source = &sandbox::checked_path(from)?;
        let target = &sandbox::checked_path(to)?;
        if !source.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Source path is not a directory",
            ));
        }
        if let Some(parent) = target.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        fs::create_dir_all(target)?;
        copy_dir_contents(source, target)
    }
}

fn copy_dir_contents(source: &Path, target: &Path) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = target.join(entry.file_name());
        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_contents(&path, &dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    fn temp_file_path(name: &str) -> PathBuf {
        let mut path = env::temp_dir();
        path.push(name);
        path
    }

    fn unique_test_file() -> PathBuf {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        temp_file_path(&format!("hypnoscript_test_{}.txt", timestamp))
    }

    fn unique_test_directory() -> PathBuf {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        temp_file_path(&format!("hypnoscript_dir_{}", timestamp))
    }

    /// Disables the process-global sandbox and returns a guard that keeps
    /// the sandbox tests from re-enabling it while this test runs.
    fn without_sandbox() -> std::sync::MutexGuard<'static, ()> {
        let guard = sandbox::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        sandbox::disable_sandbox();
        guard
    }

    #[test]
    fn test_file_operations() {
        let _guard = without_sandbox();
        let test_file = unique_test_file();
        let test_file_str = test_file.to_string_lossy().into_owned();

        // Write file
        assert!(FileBuiltins::write_file(&test_file_str, "Hello, World!").is_ok());

        // Check exists
        assert!(FileBuiltins::file_exists(&test_file_str));
        assert!(FileBuiltins::is_file(&test_file_str));

        // Read file
        let content = FileBuiltins::read_file(&test_file_str).unwrap();
        assert_eq!(content, "Hello, World!");

        // Append
        assert!(FileBuiltins::append_file(&test_file_str, " More text.").is_ok());
        let content = FileBuiltins::read_file(&test_file_str).unwrap();
        assert_eq!(content, "Hello, World! More text.");

        // Get size
        let size = FileBuiltins::get_file_size(&test_file_str).unwrap();
        assert!(size > 0);

        // Delete
        assert!(FileBuiltins::delete_file(&test_file_str).is_ok());
        assert!(!FileBuiltins::file_exists(&test_file_str));

        // Clean up in case delete failed silently on certain platforms
        let _ = fs::remove_file(&test_file);
    }

    #[test]
    fn test_path_operations() {
        assert_eq!(
            FileBuiltins::get_file_extension("test.txt"),
            Some("txt".to_string())
        );
        assert_eq!(
            FileBuiltins::get_file_name("test.txt"),
            Some("test".to_string())
        );
        assert_eq!(FileBuiltins::get_file_extension("test"), None);
    }

    #[test]
    fn test_read_write_lines() {
        let _guard = without_sandbox();
        let test_file = unique_test_file();
        let path = test_file.to_string_lossy().into_owned();
        let lines = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        FileBuiltins::write_lines(&path, &lines).unwrap();
        let read_back = FileBuiltins::read_lines(&path).unwrap();
        assert_eq!(lines, read_back);
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_copy_directory_recursive() {
        let _guard = without_sandbox();
        let source = unique_test_directory();
        let dest = unique_test_directory();
        fs::create_dir_all(&source).unwrap();
        let nested = source.join("nested");
        fs::create_dir_all(&nested).unwrap();
        let file_path = nested.join("file.txt");
        fs::write(&file_path, "hello").unwrap();

        FileBuiltins::copy_directory_recursive(source.to_str().unwrap(), dest.to_str().unwrap())
            .unwrap();

        let copied_file = dest.join("nested").join("file.txt");
        assert!(copied_file.exists());
        assert_eq!(fs::read_to_string(copied_file).unwrap(), "hello");

        let _ = fs::remove_dir_all(source);
        let _ = fs::remove_dir_all(dest);
    }
}
