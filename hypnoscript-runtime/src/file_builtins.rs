use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// File I/O builtin functions
pub struct FileBuiltins;

impl FileBuiltins {
    /// Read entire file as string
    pub fn read_file(path: &str) -> io::Result<String> {
        fs::read_to_string(path)
    }

    /// Write string to file
    pub fn write_file(path: &str, content: &str) -> io::Result<()> {
        fs::write(path, content)
    }

    /// Append string to file
    pub fn append_file(path: &str, content: &str) -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
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

    #[test]
    fn test_file_operations() {
        let test_file = "/tmp/test_hypnoscript.txt";
        
        // Write file
        assert!(FileBuiltins::write_file(test_file, "Hello, World!").is_ok());
        
        // Check exists
        assert!(FileBuiltins::file_exists(test_file));
        assert!(FileBuiltins::is_file(test_file));
        
        // Read file
        let content = FileBuiltins::read_file(test_file).unwrap();
        assert_eq!(content, "Hello, World!");
        
        // Append
        assert!(FileBuiltins::append_file(test_file, " More text.").is_ok());
        let content = FileBuiltins::read_file(test_file).unwrap();
        assert_eq!(content, "Hello, World! More text.");
        
        // Get size
        let size = FileBuiltins::get_file_size(test_file).unwrap();
        assert!(size > 0);
        
        // Delete
        assert!(FileBuiltins::delete_file(test_file).is_ok());
        assert!(!FileBuiltins::file_exists(test_file));
    }

    #[test]
    fn test_path_operations() {
        assert_eq!(FileBuiltins::get_file_extension("test.txt"), Some("txt".to_string()));
        assert_eq!(FileBuiltins::get_file_name("test.txt"), Some("test".to_string()));
        assert_eq!(FileBuiltins::get_file_extension("test"), None);
    }
}
