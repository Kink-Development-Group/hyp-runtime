//! Debug builtin functions for HypnoScript.
//!
//! This module provides debugging utilities that can be called from HypnoScript code,
//! including value inspection, programmatic breakpoints, timing, and assertions.
//!
//! # Available Functions
//!
//! - `inspect(value)` - Returns value with type information
//! - `breakpoint()` - Triggers a programmatic breakpoint
//! - `stackTrace()` - Returns the current call stack as a string
//! - `measureTime(fn)` - Measures execution time of a function
//! - `assertEqual(left, right, message)` - Assertion with message
//! - `assertTruthy(value, message)` - Assert value is truthy
//! - `typeOf(value)` - Returns the type name of a value
//! - `dump(value)` - Prints detailed value representation
//!
//! # Examples
//!
//! ```hyp
//! Focus {
//!     induce x: number = 42;
//!     induce info = Debug.inspect(x);  // "42 (number)"
//!
//!     Debug.assertEqual(x, 42, "x should be 42");
//!     Debug.breakpoint();  // Pauses if debugger attached
//! } Relax
//! ```

use crate::builtin_trait::BuiltinModule;
use crate::localization::LocalizedMessage;
use std::time::{Duration, Instant};

/// Debug builtin functions for HypnoScript.
///
/// Provides debugging utilities including value inspection, assertions,
/// timing measurements, and programmatic breakpoints.
pub struct DebugBuiltins;

impl BuiltinModule for DebugBuiltins {
    fn module_name() -> &'static str {
        "Debug"
    }

    fn description() -> &'static str {
        "Debugging utilities for HypnoScript programs"
    }

    fn description_localized(locale: Option<&str>) -> String {
        let msg = LocalizedMessage::new("Debugging utilities for HypnoScript programs")
            .with_translation("de", "Debugging utilities for HypnoScript programs")
            .with_translation(
                "fr",
                "Utilitaires de débogage pour les programmes HypnoScript",
            )
            .with_translation("es", "Utilidades de depuración para programas HypnoScript");

        let loc = crate::localization::detect_locale(locale);
        msg.resolve(&loc).to_string()
    }

    fn function_names() -> &'static [&'static str] {
        &[
            "inspect",
            "breakpoint",
            "stackTrace",
            "measureTime",
            "assertEqual",
            "assertNotEqual",
            "assertTruthy",
            "assertFalsy",
            "assertNull",
            "assertNotNull",
            "typeOf",
            "dump",
            "log",
            "warn",
            "error",
            "trace",
            "time",
            "timeEnd",
        ]
    }
}

/// Represents the result of a debug assertion.
#[derive(Debug, Clone)]
pub struct AssertionResult {
    /// Whether the assertion passed
    pub passed: bool,
    /// The assertion message
    pub message: String,
    /// Additional context
    pub context: Option<String>,
}

impl AssertionResult {
    /// Creates a passing assertion result.
    pub fn pass(message: impl Into<String>) -> Self {
        Self {
            passed: true,
            message: message.into(),
            context: None,
        }
    }

    /// Creates a failing assertion result.
    pub fn fail(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            message: message.into(),
            context: None,
        }
    }

    /// Adds context to the result.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Result of a timing measurement.
#[derive(Debug, Clone)]
pub struct TimingResult {
    /// The measured duration
    pub duration: Duration,
    /// Label for the timing
    pub label: String,
}

impl TimingResult {
    /// Formats the timing result for display.
    pub fn format(&self) -> String {
        let micros = self.duration.as_micros();
        if micros < 1000 {
            format!("{}: {}µs", self.label, micros)
        } else if micros < 1_000_000 {
            format!("{}: {:.2}ms", self.label, micros as f64 / 1000.0)
        } else {
            format!("{}: {:.2}s", self.label, micros as f64 / 1_000_000.0)
        }
    }
}

// Thread-local storage for timing measurements.
thread_local! {
    static TIMERS: std::cell::RefCell<std::collections::HashMap<String, Instant>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

impl DebugBuiltins {
    /// Inspects a value and returns a string with type information.
    ///
    /// # Arguments
    /// * `value` - The value as a display string
    /// * `type_name` - The type name
    ///
    /// # Returns
    /// A string in the format "value (type)"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hypnoscript_runtime::DebugBuiltins;
    ///
    /// let result = DebugBuiltins::inspect("42", "number");
    /// assert_eq!(result, "42 (number)");
    /// ```
    pub fn inspect(value: &str, type_name: &str) -> String {
        format!("{} ({})", value, type_name)
    }

    /// Inspects a value with detailed representation.
    ///
    /// # Arguments
    /// * `value` - The value as a display string
    /// * `type_name` - The type name
    /// * `details` - Optional additional details
    ///
    /// # Returns
    /// A detailed inspection string
    pub fn inspect_detailed(value: &str, type_name: &str, details: Option<&str>) -> String {
        match details {
            Some(d) => format!("{} ({}) [{}]", value, type_name, d),
            None => format!("{} ({})", value, type_name),
        }
    }

    /// Returns the type name for a value category.
    ///
    /// # Arguments
    /// * `type_name` - The internal type name
    ///
    /// # Returns
    /// A user-friendly type name
    pub fn type_of(type_name: &str) -> String {
        match type_name {
            "Number" | "number" => "number".to_string(),
            "String" | "string" => "string".to_string(),
            "Boolean" | "boolean" | "bool" => "boolean".to_string(),
            "Array" | "array" => "array".to_string(),
            "Function" | "function" => "function".to_string(),
            "Session" | "session" => "session".to_string(),
            "Instance" | "instance" => "instance".to_string(),
            "Promise" | "promise" => "promise".to_string(),
            "Record" | "record" => "record".to_string(),
            "Null" | "null" => "null".to_string(),
            other => other.to_string(),
        }
    }

    /// Triggers a programmatic breakpoint.
    ///
    /// This function signals to the debugger (if attached) that execution
    /// should pause at this point. If no debugger is attached, this is a no-op.
    ///
    /// # Returns
    /// `true` if a debugger was attached, `false` otherwise
    pub fn breakpoint() -> bool {
        // In a real implementation, this would signal the debugger
        // For now, we just print a message
        eprintln!("[DEBUG] Programmatic breakpoint triggered");
        true
    }

    /// Returns a formatted stack trace.
    ///
    /// # Arguments
    /// * `frames` - List of (function_name, line_number) tuples
    ///
    /// # Returns
    /// A formatted stack trace string
    pub fn stack_trace(frames: &[(String, usize)]) -> String {
        if frames.is_empty() {
            return "  (empty stack)".to_string();
        }

        frames
            .iter()
            .enumerate()
            .map(|(i, (name, line))| {
                if *line > 0 {
                    format!("  #{} {} at line {}", i, name, line)
                } else {
                    format!("  #{} {} (native)", i, name)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Starts a timer with the given label.
    ///
    /// # Arguments
    /// * `label` - The timer label
    pub fn time(label: &str) {
        TIMERS.with(|timers| {
            timers
                .borrow_mut()
                .insert(label.to_string(), Instant::now());
        });
    }

    /// Stops a timer and returns the elapsed time.
    ///
    /// # Arguments
    /// * `label` - The timer label
    ///
    /// # Returns
    /// The timing result, or None if timer wasn't started
    pub fn time_end(label: &str) -> Option<TimingResult> {
        TIMERS.with(|timers| {
            timers.borrow_mut().remove(label).map(|start| TimingResult {
                duration: start.elapsed(),
                label: label.to_string(),
            })
        })
    }

    /// Measures the execution time of a closure.
    ///
    /// # Arguments
    /// * `f` - The closure to measure
    ///
    /// # Returns
    /// A tuple of (result, duration)
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Asserts that two values are equal.
    ///
    /// # Arguments
    /// * `left` - Left value as string
    /// * `right` - Right value as string
    /// * `message` - Optional message for failure
    ///
    /// # Returns
    /// `Ok(())` if equal, `Err(message)` if not
    pub fn assert_equal(left: &str, right: &str, message: Option<&str>) -> Result<(), String> {
        if left == right {
            Ok(())
        } else {
            let msg = message.unwrap_or("Assertion failed: values are not equal");
            Err(format!("{}\n  left:  {}\n  right: {}", msg, left, right))
        }
    }

    /// Asserts that two values are not equal.
    ///
    /// # Arguments
    /// * `left` - Left value as string
    /// * `right` - Right value as string
    /// * `message` - Optional message for failure
    ///
    /// # Returns
    /// `Ok(())` if not equal, `Err(message)` if equal
    pub fn assert_not_equal(left: &str, right: &str, message: Option<&str>) -> Result<(), String> {
        if left != right {
            Ok(())
        } else {
            let msg = message.unwrap_or("Assertion failed: values should not be equal");
            Err(format!("{}\n  value: {}", msg, left))
        }
    }

    /// Asserts that a value is truthy.
    ///
    /// # Arguments
    /// * `is_truthy` - Whether the value is truthy
    /// * `value_repr` - String representation of the value
    /// * `message` - Optional message for failure
    ///
    /// # Returns
    /// `Ok(())` if truthy, `Err(message)` if not
    pub fn assert_truthy(
        is_truthy: bool,
        value_repr: &str,
        message: Option<&str>,
    ) -> Result<(), String> {
        if is_truthy {
            Ok(())
        } else {
            let msg = message.unwrap_or("Assertion failed: expected truthy value");
            Err(format!("{}\n  got: {}", msg, value_repr))
        }
    }

    /// Asserts that a value is falsy.
    ///
    /// # Arguments
    /// * `is_truthy` - Whether the value is truthy
    /// * `value_repr` - String representation of the value
    /// * `message` - Optional message for failure
    ///
    /// # Returns
    /// `Ok(())` if falsy, `Err(message)` if truthy
    pub fn assert_falsy(
        is_truthy: bool,
        value_repr: &str,
        message: Option<&str>,
    ) -> Result<(), String> {
        if !is_truthy {
            Ok(())
        } else {
            let msg = message.unwrap_or("Assertion failed: expected falsy value");
            Err(format!("{}\n  got: {}", msg, value_repr))
        }
    }

    /// Asserts that a value is null.
    ///
    /// # Arguments
    /// * `is_null` - Whether the value is null
    /// * `value_repr` - String representation of the value
    /// * `message` - Optional message for failure
    ///
    /// # Returns
    /// `Ok(())` if null, `Err(message)` if not
    pub fn assert_null(
        is_null: bool,
        value_repr: &str,
        message: Option<&str>,
    ) -> Result<(), String> {
        if is_null {
            Ok(())
        } else {
            let msg = message.unwrap_or("Assertion failed: expected null");
            Err(format!("{}\n  got: {}", msg, value_repr))
        }
    }

    /// Asserts that a value is not null.
    ///
    /// # Arguments
    /// * `is_null` - Whether the value is null
    /// * `value_repr` - String representation of the value
    /// * `message` - Optional message for failure
    ///
    /// # Returns
    /// `Ok(())` if not null, `Err(message)` if null
    pub fn assert_not_null(
        is_null: bool,
        value_repr: &str,
        message: Option<&str>,
    ) -> Result<(), String> {
        if !is_null {
            Ok(())
        } else {
            let msg = message.unwrap_or("Assertion failed: expected non-null value");
            Err(format!("{}\n  got: {}", msg, value_repr))
        }
    }

    /// Logs a debug message.
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn log(message: &str) {
        eprintln!("[DEBUG] {}", message);
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn warn(message: &str) {
        eprintln!("[WARN] {}", message);
    }

    /// Logs an error message.
    ///
    /// # Arguments
    /// * `message` - The message to log
    pub fn error(message: &str) {
        eprintln!("[ERROR] {}", message);
    }

    /// Logs a trace message with caller information.
    ///
    /// # Arguments
    /// * `message` - The message to log
    /// * `location` - Optional location information (file:line)
    pub fn trace(message: &str, location: Option<&str>) {
        match location {
            Some(loc) => eprintln!("[TRACE] {} at {}", message, loc),
            None => eprintln!("[TRACE] {}", message),
        }
    }

    /// Dumps a value with full details.
    ///
    /// # Arguments
    /// * `value` - The value representation
    /// * `type_name` - The type name
    /// * `details` - Additional details about the value
    pub fn dump(value: &str, type_name: &str, details: &str) {
        eprintln!("┌─ Debug Dump ─────────────────────");
        eprintln!("│ Type:  {}", type_name);
        eprintln!("│ Value: {}", value);
        if !details.is_empty() {
            eprintln!("│ Details: {}", details);
        }
        eprintln!("└──────────────────────────────────");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect() {
        assert_eq!(DebugBuiltins::inspect("42", "number"), "42 (number)");
        assert_eq!(DebugBuiltins::inspect("hello", "string"), "hello (string)");
        assert_eq!(DebugBuiltins::inspect("true", "boolean"), "true (boolean)");
    }

    #[test]
    fn test_inspect_detailed() {
        assert_eq!(
            DebugBuiltins::inspect_detailed("42", "number", None),
            "42 (number)"
        );
        assert_eq!(
            DebugBuiltins::inspect_detailed("arr", "array", Some("length: 5")),
            "arr (array) [length: 5]"
        );
    }

    #[test]
    fn test_type_of() {
        assert_eq!(DebugBuiltins::type_of("Number"), "number");
        assert_eq!(DebugBuiltins::type_of("String"), "string");
        assert_eq!(DebugBuiltins::type_of("Boolean"), "boolean");
        assert_eq!(DebugBuiltins::type_of("Null"), "null");
        assert_eq!(DebugBuiltins::type_of("CustomType"), "CustomType");
    }

    #[test]
    fn test_stack_trace() {
        let frames = vec![
            ("main".to_string(), 10),
            ("helper".to_string(), 25),
            ("nested".to_string(), 30),
        ];
        let trace = DebugBuiltins::stack_trace(&frames);
        assert!(trace.contains("#0 main at line 10"));
        assert!(trace.contains("#1 helper at line 25"));
        assert!(trace.contains("#2 nested at line 30"));
    }

    #[test]
    fn test_stack_trace_empty() {
        let frames: Vec<(String, usize)> = vec![];
        let trace = DebugBuiltins::stack_trace(&frames);
        assert_eq!(trace, "  (empty stack)");
    }

    #[test]
    fn test_stack_trace_native() {
        let frames = vec![("println".to_string(), 0)];
        let trace = DebugBuiltins::stack_trace(&frames);
        assert!(trace.contains("(native)"));
    }

    #[test]
    fn test_timing() {
        DebugBuiltins::time("test_timer");
        std::thread::sleep(std::time::Duration::from_millis(10));
        let result = DebugBuiltins::time_end("test_timer");
        assert!(result.is_some());
        let timing = result.unwrap();
        assert!(timing.duration.as_millis() >= 10);
        assert_eq!(timing.label, "test_timer");
    }

    #[test]
    fn test_timing_nonexistent() {
        let result = DebugBuiltins::time_end("nonexistent_timer");
        assert!(result.is_none());
    }

    #[test]
    fn test_measure_time() {
        let (result, duration) = DebugBuiltins::measure_time(|| {
            std::thread::sleep(std::time::Duration::from_millis(5));
            42
        });
        assert_eq!(result, 42);
        assert!(duration.as_millis() >= 5);
    }

    #[test]
    fn test_assert_equal_pass() {
        assert!(DebugBuiltins::assert_equal("42", "42", None).is_ok());
        assert!(DebugBuiltins::assert_equal("hello", "hello", Some("custom message")).is_ok());
    }

    #[test]
    fn test_assert_equal_fail() {
        let result = DebugBuiltins::assert_equal("42", "43", None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("not equal"));
        assert!(err.contains("42"));
        assert!(err.contains("43"));
    }

    #[test]
    fn test_assert_not_equal_pass() {
        assert!(DebugBuiltins::assert_not_equal("42", "43", None).is_ok());
    }

    #[test]
    fn test_assert_not_equal_fail() {
        let result = DebugBuiltins::assert_not_equal("42", "42", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_assert_truthy() {
        assert!(DebugBuiltins::assert_truthy(true, "true", None).is_ok());
        assert!(DebugBuiltins::assert_truthy(false, "false", None).is_err());
    }

    #[test]
    fn test_assert_falsy() {
        assert!(DebugBuiltins::assert_falsy(false, "false", None).is_ok());
        assert!(DebugBuiltins::assert_falsy(true, "true", None).is_err());
    }

    #[test]
    fn test_assert_null() {
        assert!(DebugBuiltins::assert_null(true, "null", None).is_ok());
        assert!(DebugBuiltins::assert_null(false, "42", None).is_err());
    }

    #[test]
    fn test_assert_not_null() {
        assert!(DebugBuiltins::assert_not_null(false, "42", None).is_ok());
        assert!(DebugBuiltins::assert_not_null(true, "null", None).is_err());
    }

    #[test]
    fn test_timing_result_format() {
        let result = TimingResult {
            duration: Duration::from_micros(500),
            label: "test".to_string(),
        };
        assert!(result.format().contains("µs"));

        let result_ms = TimingResult {
            duration: Duration::from_millis(50),
            label: "test".to_string(),
        };
        assert!(result_ms.format().contains("ms"));

        let result_s = TimingResult {
            duration: Duration::from_secs(2),
            label: "test".to_string(),
        };
        assert!(result_s.format().contains("s"));
    }

    #[test]
    fn test_module_info() {
        assert_eq!(DebugBuiltins::module_name(), "Debug");
        assert!(!DebugBuiltins::description().is_empty());
        assert!(!DebugBuiltins::function_names().is_empty());
    }
}
