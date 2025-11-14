use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// File I/O builtin functions
pub struct FileBuiltins;

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
        fs::read_to_string(path)
    }

    /// Write string to file
    pub fn write_file(path: &str, content: &str) -> io::Result<()> {
        let path_ref = Path::new(path);
        Self::ensure_parent_dir(path_ref)?;
        fs::write(path_ref, content)
    }

    /// Append string to file
    pub fn append_file(path: &str, content: &str) -> io::Result<()> {
        let path_ref = Path::new(path);
        Self::ensure_parent_dir(path_ref)?;

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path_ref)?;
        file.write_all(content.as_bytes())
    }

    /// Check if file exists
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Check if path is file
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// Check if path is directory
    pub fn is_directory(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    /// Delete file
    pub fn delete_file(path: &str) -> io::Result<()> {
        fs::remove_file(path)
    }

    /// Create directory
    pub fn create_directory(path: &str) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    /// List files in directory
    pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }
        Ok(files)
    }

    /// Get file size in bytes
    pub fn get_file_size(path: &str) -> io::Result<u64> {
        fs::metadata(path).map(|m| m.len())
    }

    /// Copy file
    pub fn copy_file(from: &str, to: &str) -> io::Result<u64> {
        fs::copy(from, to)
    }

    /// Rename/move file
    pub fn rename_file(from: &str, to: &str) -> io::Result<()> {
        fs::rename(from, to)
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

    #[test]
    fn test_file_operations() {
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
}
