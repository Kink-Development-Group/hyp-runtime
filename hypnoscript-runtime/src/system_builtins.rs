use std::env;

/// System information builtin functions
pub struct SystemBuiltins;

impl SystemBuiltins {
    /// Get current directory
    pub fn get_current_directory() -> String {
        env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| ".".to_string())
    }

    /// Get environment variable
    pub fn get_env_var(name: &str) -> Option<String> {
        env::var(name).ok()
    }

    /// Set environment variable
    ///
    /// # Errors
    /// Returns an error if:
    /// - `name` is empty
    /// - `name` or `value` contains null bytes
    /// - On Windows, `name` contains '='
    pub fn set_env_var(name: &str, value: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Environment variable name cannot be empty".to_string());
        }
        if name.contains('\0') {
            return Err("Environment variable name cannot contain null bytes".to_string());
        }
        if value.contains('\0') {
            return Err("Environment variable value cannot contain null bytes".to_string());
        }
        if cfg!(windows) && name.contains('=') {
            return Err("Environment variable name cannot contain '=' on Windows".to_string());
        }

        // SAFETY: Environment variable names/values are validated above to satisfy
        // the platform-specific requirements of `std::env::set_var` on the 2024
        // edition, which now enforces these preconditions in an unsafe API.
        unsafe {
            env::set_var(name, value);
        }
        Ok(())
    }

    /// Get operating system
    pub fn get_operating_system() -> String {
        env::consts::OS.to_string()
    }

    /// Get architecture
    pub fn get_architecture() -> String {
        env::consts::ARCH.to_string()
    }

    /// Get number of CPU cores
    pub fn get_cpu_count() -> usize {
        num_cpus::get()
    }

    /// Get hostname
    pub fn get_hostname() -> String {
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Get username
    pub fn get_username() -> String {
        env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string())
    }

    /// Get home directory
    pub fn get_home_directory() -> String {
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string())
    }

    /// Get temporary directory
    pub fn get_temp_directory() -> String {
        env::temp_dir()
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "/tmp".to_string())
    }

    /// Get program arguments
    pub fn get_args() -> Vec<String> {
        env::args().collect()
    }

    /// Exit program with code
    pub fn exit(code: i32) -> ! {
        std::process::exit(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_operating_system() {
        let os = SystemBuiltins::get_operating_system();
        assert!(!os.is_empty());
    }

    #[test]
    fn test_get_architecture() {
        let arch = SystemBuiltins::get_architecture();
        assert!(!arch.is_empty());
    }

    #[test]
    fn test_get_cpu_count() {
        let count = SystemBuiltins::get_cpu_count();
        assert!(count > 0);
    }

    #[test]
    fn test_current_directory() {
        let dir = SystemBuiltins::get_current_directory();
        assert!(!dir.is_empty());
    }
}
