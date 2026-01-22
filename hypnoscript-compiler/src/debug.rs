//! Debug infrastructure for HypnoScript interpreter.
//!
//! This module provides comprehensive debugging capabilities including:
//! - Breakpoints (set, remove, query)
//! - Step-through execution (step into, step over, step out)
//! - Call stack inspection
//! - Watch expressions
//! - Variable inspection
//!
//! # Architecture
//!
//! The debug system integrates with the interpreter by checking for
//! breakpoints and step conditions before each statement execution.
//!
//! # Examples
//!
//! ```rust,no_run
//! use hypnoscript_compiler::debug::{DebugState, StepMode};
//!
//! let mut debug_state = DebugState::new();
//! debug_state.set_breakpoint(10);
//! debug_state.set_step_mode(StepMode::StepInto);
//! ```

use crate::interpreter::Value;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Errors that can occur during debugging operations.
#[derive(Error, Debug)]
pub enum DebugError {
    #[error("Breakpoint at line {0} already exists")]
    BreakpointExists(usize),

    #[error("Breakpoint at line {0} does not exist")]
    BreakpointNotFound(usize),

    #[error("Invalid debug command: {0}")]
    InvalidCommand(String),

    #[error("Expression evaluation failed: {0}")]
    ExpressionError(String),

    #[error("Debug session not active")]
    SessionNotActive,
}

/// Result type for debug operations.
pub type DebugResult<T> = Result<T, DebugError>;

/// Represents the current stepping mode during debugging.
///
/// This controls how the debugger advances through code:
/// - `None`: Normal execution (no stepping)
/// - `StepInto`: Execute one statement, entering function calls
/// - `StepOver`: Execute one statement, not entering function calls
/// - `StepOut`: Continue until current function returns
/// - `Continue`: Run until next breakpoint or end
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepMode {
    /// No stepping - normal execution
    #[default]
    None,
    /// Step into function calls
    StepInto,
    /// Step over function calls
    StepOver,
    /// Step out of current function
    StepOut,
    /// Continue to next breakpoint
    Continue,
}

impl std::fmt::Display for StepMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepMode::None => write!(f, "none"),
            StepMode::StepInto => write!(f, "step-into"),
            StepMode::StepOver => write!(f, "step-over"),
            StepMode::StepOut => write!(f, "step-out"),
            StepMode::Continue => write!(f, "continue"),
        }
    }
}

/// Represents a single call frame in the execution stack.
///
/// Call frames track the current execution context including
/// function name, source location, and local variable bindings.
///
/// # Examples
///
/// ```rust,no_run
/// use hypnoscript_compiler::debug::CallFrame;
/// use hypnoscript_compiler::Value;
/// use std::collections::HashMap;
///
/// let frame = CallFrame {
///     function_name: "myFunction".to_string(),
///     line: 42,
///     column: 1,
///     local_variables: HashMap::new(),
///     is_native: false,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct CallFrame {
    /// Name of the function being executed
    pub function_name: String,
    /// Current line number (1-indexed)
    pub line: usize,
    /// Current column number (1-indexed)
    pub column: usize,
    /// Snapshot of local variables in this frame
    pub local_variables: HashMap<String, Value>,
    /// Whether this is a native/builtin function
    pub is_native: bool,
}

impl CallFrame {
    /// Creates a new call frame.
    pub fn new(function_name: String, line: usize) -> Self {
        Self {
            function_name,
            line,
            column: 1,
            local_variables: HashMap::new(),
            is_native: false,
        }
    }

    /// Creates a call frame for a native function.
    pub fn native(function_name: String) -> Self {
        Self {
            function_name,
            line: 0,
            column: 0,
            local_variables: HashMap::new(),
            is_native: true,
        }
    }

    /// Updates the source location.
    pub fn set_location(&mut self, line: usize, column: usize) {
        self.line = line;
        self.column = column;
    }

    /// Adds or updates a local variable.
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.local_variables.insert(name, value);
    }
}

impl std::fmt::Display for CallFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_native {
            write!(f, "{} (native)", self.function_name)
        } else {
            write!(f, "{} at line {}", self.function_name, self.line)
        }
    }
}

/// A watch expression that is evaluated when the debugger pauses.
#[derive(Debug, Clone)]
pub struct WatchExpression {
    /// The expression string to evaluate
    pub expression: String,
    /// Unique ID for this watch
    pub id: usize,
    /// Last evaluated value (if any)
    pub last_value: Option<Value>,
    /// Whether the watch is enabled
    pub enabled: bool,
}

impl WatchExpression {
    /// Creates a new watch expression.
    pub fn new(id: usize, expression: String) -> Self {
        Self {
            expression,
            id,
            last_value: None,
            enabled: true,
        }
    }

    /// Updates the last evaluated value.
    pub fn set_value(&mut self, value: Value) {
        self.last_value = Some(value);
    }

    /// Toggles the enabled state.
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

/// Represents the reason why the debugger paused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauseReason {
    /// Hit a user-set breakpoint
    Breakpoint(usize),
    /// Completed a step operation
    Step,
    /// Hit a programmatic breakpoint (from code)
    ProgrammaticBreakpoint,
    /// User requested pause
    UserRequest,
    /// Exception/error occurred
    Exception(String),
}

impl std::fmt::Display for PauseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PauseReason::Breakpoint(line) => write!(f, "Breakpoint at line {}", line),
            PauseReason::Step => write!(f, "Step completed"),
            PauseReason::ProgrammaticBreakpoint => write!(f, "Programmatic breakpoint"),
            PauseReason::UserRequest => write!(f, "Paused by user"),
            PauseReason::Exception(msg) => write!(f, "Exception: {}", msg),
        }
    }
}

/// Complete debug state for the interpreter.
///
/// This structure maintains all debugging information including:
/// - Set of breakpoints
/// - Current stepping mode
/// - Call stack
/// - Watch expressions
/// - Pause state
///
/// # Examples
///
/// ```rust,no_run
/// use hypnoscript_compiler::debug::{DebugState, StepMode};
///
/// let mut state = DebugState::new();
///
/// // Set breakpoints
/// state.set_breakpoint(10);
/// state.set_breakpoint(25);
///
/// // Configure stepping
/// state.set_step_mode(StepMode::StepInto);
///
/// // Add watch expression
/// state.add_watch("myVariable".to_string());
/// ```
#[derive(Debug, Default)]
pub struct DebugState {
    /// Set of line numbers with breakpoints
    breakpoints: HashSet<usize>,
    /// Current stepping mode
    step_mode: StepMode,
    /// Call stack (bottom to top)
    call_stack: Vec<CallFrame>,
    /// Watch expressions
    watches: Vec<WatchExpression>,
    /// Next watch ID
    next_watch_id: usize,
    /// Whether the debugger is currently paused
    is_paused: bool,
    /// Reason for the current pause (if paused)
    pause_reason: Option<PauseReason>,
    /// Current line being executed
    current_line: usize,
    /// Current column being executed
    current_column: usize,
    /// Depth at which step over was initiated (for step-over logic)
    step_over_depth: Option<usize>,
    /// Depth at which step out was initiated (for step-out logic)
    step_out_depth: Option<usize>,
    /// Whether to break on next statement (for step operations)
    break_on_next: bool,
    /// Source code lines (for display)
    source_lines: Vec<String>,
}

impl DebugState {
    /// Creates a new debug state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a debug state with source code for display.
    pub fn with_source(source: &str) -> Self {
        Self {
            source_lines: source.lines().map(String::from).collect(),
            ..Default::default()
        }
    }

    /// Sets the source code for display.
    pub fn set_source(&mut self, source: &str) {
        self.source_lines = source.lines().map(String::from).collect();
    }

    // === Breakpoint Management ===

    /// Sets a breakpoint at the given line number.
    ///
    /// # Arguments
    /// * `line` - Line number (1-indexed)
    ///
    /// # Returns
    /// `true` if breakpoint was newly added, `false` if already existed
    pub fn set_breakpoint(&mut self, line: usize) -> bool {
        self.breakpoints.insert(line)
    }

    /// Removes a breakpoint at the given line number.
    ///
    /// # Arguments
    /// * `line` - Line number (1-indexed)
    ///
    /// # Returns
    /// `true` if breakpoint was removed, `false` if it didn't exist
    pub fn remove_breakpoint(&mut self, line: usize) -> bool {
        self.breakpoints.remove(&line)
    }

    /// Checks if a breakpoint exists at the given line.
    pub fn has_breakpoint(&self, line: usize) -> bool {
        self.breakpoints.contains(&line)
    }

    /// Returns all set breakpoints.
    pub fn breakpoints(&self) -> &HashSet<usize> {
        &self.breakpoints
    }

    /// Clears all breakpoints.
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    /// Returns the number of breakpoints.
    pub fn breakpoint_count(&self) -> usize {
        self.breakpoints.len()
    }

    // === Step Mode Management ===

    /// Gets the current stepping mode.
    pub fn step_mode(&self) -> StepMode {
        self.step_mode
    }

    /// Sets the stepping mode.
    pub fn set_step_mode(&mut self, mode: StepMode) {
        self.step_mode = mode;

        match mode {
            StepMode::StepInto => {
                self.break_on_next = true;
                self.step_over_depth = None;
                self.step_out_depth = None;
            }
            StepMode::StepOver => {
                self.break_on_next = true;
                self.step_over_depth = Some(self.call_stack.len());
                self.step_out_depth = None;
            }
            StepMode::StepOut => {
                self.break_on_next = false;
                self.step_over_depth = None;
                self.step_out_depth = Some(self.call_stack.len());
            }
            StepMode::Continue => {
                self.break_on_next = false;
                self.step_over_depth = None;
                self.step_out_depth = None;
            }
            StepMode::None => {
                self.break_on_next = false;
                self.step_over_depth = None;
                self.step_out_depth = None;
            }
        }
    }

    // === Call Stack Management ===

    /// Pushes a new call frame onto the stack.
    pub fn push_frame(&mut self, frame: CallFrame) {
        self.call_stack.push(frame);
    }

    /// Pops the top call frame from the stack.
    pub fn pop_frame(&mut self) -> Option<CallFrame> {
        let frame = self.call_stack.pop();

        // Check if we should break for step-out
        if let Some(depth) = self.step_out_depth
            && self.call_stack.len() < depth
        {
            self.break_on_next = true;
            self.step_out_depth = None;
        }

        frame
    }

    /// Returns the current call stack depth.
    pub fn stack_depth(&self) -> usize {
        self.call_stack.len()
    }

    /// Returns the current (top) call frame.
    pub fn current_frame(&self) -> Option<&CallFrame> {
        self.call_stack.last()
    }

    /// Returns a mutable reference to the current call frame.
    pub fn current_frame_mut(&mut self) -> Option<&mut CallFrame> {
        self.call_stack.last_mut()
    }

    /// Returns the entire call stack.
    pub fn call_stack(&self) -> &[CallFrame] {
        &self.call_stack
    }

    /// Formats the call stack for display.
    pub fn format_call_stack(&self) -> String {
        if self.call_stack.is_empty() {
            return "  (empty stack)".to_string();
        }

        self.call_stack
            .iter()
            .rev()
            .enumerate()
            .map(|(i, frame)| format!("  #{} {}", i, frame))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // === Watch Expression Management ===

    /// Adds a watch expression.
    ///
    /// # Returns
    /// The ID of the new watch expression
    pub fn add_watch(&mut self, expression: String) -> usize {
        let id = self.next_watch_id;
        self.next_watch_id += 1;
        self.watches.push(WatchExpression::new(id, expression));
        id
    }

    /// Removes a watch expression by ID.
    pub fn remove_watch(&mut self, id: usize) -> bool {
        if let Some(pos) = self.watches.iter().position(|w| w.id == id) {
            self.watches.remove(pos);
            true
        } else {
            false
        }
    }

    /// Returns all watch expressions.
    pub fn watches(&self) -> &[WatchExpression] {
        &self.watches
    }

    /// Returns a mutable reference to all watches.
    pub fn watches_mut(&mut self) -> &mut Vec<WatchExpression> {
        &mut self.watches
    }

    /// Clears all watch expressions.
    pub fn clear_watches(&mut self) {
        self.watches.clear();
    }

    // === Pause State Management ===

    /// Checks if the debugger is paused.
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Pauses the debugger with a reason.
    pub fn pause(&mut self, reason: PauseReason) {
        self.is_paused = true;
        self.pause_reason = Some(reason);
    }

    /// Resumes execution.
    pub fn resume(&mut self) {
        self.is_paused = false;
        self.pause_reason = None;
    }

    /// Gets the current pause reason.
    pub fn pause_reason(&self) -> Option<&PauseReason> {
        self.pause_reason.as_ref()
    }

    // === Location Tracking ===

    /// Updates the current execution location.
    pub fn set_location(&mut self, line: usize, column: usize) {
        self.current_line = line;
        self.current_column = column;

        if let Some(frame) = self.call_stack.last_mut() {
            frame.set_location(line, column);
        }
    }

    /// Gets the current line number.
    pub fn current_line(&self) -> usize {
        self.current_line
    }

    /// Gets the current column number.
    pub fn current_column(&self) -> usize {
        self.current_column
    }

    // === Breakpoint Checking ===

    /// Checks if execution should pause at the current location.
    ///
    /// This method considers:
    /// - Breakpoints at the current line
    /// - Step mode and break conditions
    ///
    /// # Returns
    /// `Some(PauseReason)` if should pause, `None` otherwise
    pub fn should_pause(&mut self, line: usize) -> Option<PauseReason> {
        // Update current location
        self.current_line = line;

        // Check for breakpoint
        if self.breakpoints.contains(&line) {
            return Some(PauseReason::Breakpoint(line));
        }

        // Check for step conditions
        if self.break_on_next {
            // For step-over, only break if we're at or above the original depth
            if let Some(depth) = self.step_over_depth {
                if self.call_stack.len() <= depth {
                    self.break_on_next = false;
                    return Some(PauseReason::Step);
                }
            } else {
                self.break_on_next = false;
                return Some(PauseReason::Step);
            }
        }

        None
    }

    /// Triggers a programmatic breakpoint (from code).
    pub fn trigger_breakpoint(&mut self) -> PauseReason {
        PauseReason::ProgrammaticBreakpoint
    }

    // === Source Display ===

    /// Gets the source line at the given line number.
    pub fn get_source_line(&self, line: usize) -> Option<&str> {
        if line > 0 && line <= self.source_lines.len() {
            Some(&self.source_lines[line - 1])
        } else {
            None
        }
    }

    /// Formats source lines around the current position for display.
    ///
    /// # Arguments
    /// * `context` - Number of lines before and after to include
    pub fn format_source_context(&self, context: usize) -> String {
        if self.source_lines.is_empty() || self.current_line == 0 {
            return "(no source available)".to_string();
        }

        let start = self.current_line.saturating_sub(context).max(1);
        let end = (self.current_line + context).min(self.source_lines.len());

        let mut result = String::new();
        for line_num in start..=end {
            let marker = if line_num == self.current_line {
                "→"
            } else if self.breakpoints.contains(&line_num) {
                "●"
            } else {
                " "
            };

            if let Some(line) = self.get_source_line(line_num) {
                result.push_str(&format!("{} {:4} | {}\n", marker, line_num, line));
            }
        }

        result
    }

    // === Debug Information ===

    /// Returns a summary of the current debug state.
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        if !self.breakpoints.is_empty() {
            parts.push(format!("breakpoints: {:?}", self.breakpoints));
        }

        parts.push(format!("step_mode: {}", self.step_mode));
        parts.push(format!("stack_depth: {}", self.call_stack.len()));
        parts.push(format!("paused: {}", self.is_paused));

        if self.current_line > 0 {
            parts.push(format!("line: {}", self.current_line));
        }

        parts.join(", ")
    }
}

/// Debug command parsed from user input.
#[derive(Debug, Clone, PartialEq)]
pub enum DebugCommand {
    /// Set a breakpoint at line
    Break(usize),
    /// Delete a breakpoint at line
    Delete(usize),
    /// Delete all breakpoints
    DeleteAll,
    /// Continue execution
    Continue,
    /// Step into (next statement, entering functions)
    StepInto,
    /// Step over (next statement, not entering functions)
    StepOver,
    /// Step out of current function
    StepOut,
    /// Print/evaluate an expression
    Print(String),
    /// Show local variables
    Locals,
    /// Show call stack
    Stack,
    /// Add a watch expression
    Watch(String),
    /// Remove a watch expression by ID
    Unwatch(usize),
    /// Show all watches
    Watches,
    /// List breakpoints
    ListBreakpoints,
    /// Show help
    Help,
    /// Quit debugging session
    Quit,
    /// Show current source location
    Where,
    /// List source code around current line
    List(Option<usize>),
}

impl DebugCommand {
    /// Parses a debug command from a string.
    pub fn parse(input: &str) -> DebugResult<Self> {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();

        let cmd = parts[0].to_lowercase();
        let arg = parts.get(1).map(|s| s.trim());

        match cmd.as_str() {
            "b" | "break" => {
                let line = arg
                    .ok_or_else(|| {
                        DebugError::InvalidCommand("break requires a line number".into())
                    })?
                    .parse::<usize>()
                    .map_err(|_| DebugError::InvalidCommand("invalid line number".into()))?;
                Ok(DebugCommand::Break(line))
            }
            "d" | "delete" => {
                if let Some(arg_str) = arg {
                    if arg_str == "all" || arg_str == "*" {
                        Ok(DebugCommand::DeleteAll)
                    } else {
                        let line = arg_str.parse::<usize>().map_err(|_| {
                            DebugError::InvalidCommand("invalid line number".into())
                        })?;
                        Ok(DebugCommand::Delete(line))
                    }
                } else {
                    Err(DebugError::InvalidCommand(
                        "delete requires a line number or 'all'".into(),
                    ))
                }
            }
            "c" | "continue" => Ok(DebugCommand::Continue),
            "s" | "step" => Ok(DebugCommand::StepInto),
            "n" | "next" => Ok(DebugCommand::StepOver),
            "o" | "out" => Ok(DebugCommand::StepOut),
            "p" | "print" => {
                let expr = arg
                    .ok_or_else(|| {
                        DebugError::InvalidCommand("print requires an expression".into())
                    })?
                    .to_string();
                Ok(DebugCommand::Print(expr))
            }
            "l" | "locals" => Ok(DebugCommand::Locals),
            "st" | "stack" => Ok(DebugCommand::Stack),
            "w" | "watch" => {
                let expr = arg
                    .ok_or_else(|| {
                        DebugError::InvalidCommand("watch requires an expression".into())
                    })?
                    .to_string();
                Ok(DebugCommand::Watch(expr))
            }
            "unwatch" => {
                let id = arg
                    .ok_or_else(|| {
                        DebugError::InvalidCommand("unwatch requires a watch ID".into())
                    })?
                    .parse::<usize>()
                    .map_err(|_| DebugError::InvalidCommand("invalid watch ID".into()))?;
                Ok(DebugCommand::Unwatch(id))
            }
            "watches" => Ok(DebugCommand::Watches),
            "bl" | "breakpoints" => Ok(DebugCommand::ListBreakpoints),
            "h" | "help" | "?" => Ok(DebugCommand::Help),
            "q" | "quit" | "exit" => Ok(DebugCommand::Quit),
            "where" => Ok(DebugCommand::Where),
            "list" | "ls" => {
                let line = arg.and_then(|s| s.parse::<usize>().ok());
                Ok(DebugCommand::List(line))
            }
            "" => Err(DebugError::InvalidCommand("empty command".into())),
            _ => Err(DebugError::InvalidCommand(format!(
                "unknown command: {}",
                cmd
            ))),
        }
    }
}

/// Formats help text for debug commands.
pub fn format_help() -> String {
    r#"HypnoScript Debugger Commands:

  Breakpoints:
    b, break <line>      Set breakpoint at line
    d, delete <line>     Remove breakpoint at line
    d, delete all        Remove all breakpoints
    bl, breakpoints      List all breakpoints

  Execution:
    c, continue          Continue to next breakpoint
    s, step              Step into (enter functions)
    n, next              Step over (skip function internals)
    o, out               Step out of current function

  Inspection:
    p, print <expr>      Evaluate and print expression
    l, locals            Show local variables
    st, stack            Show call stack
    where                Show current location
    list [line]          Show source code around line

  Watches:
    w, watch <expr>      Add watch expression
    unwatch <id>         Remove watch by ID
    watches              List all watches

  Other:
    h, help, ?           Show this help
    q, quit, exit        End debug session
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_set_and_remove() {
        let mut state = DebugState::new();

        assert!(state.set_breakpoint(10));
        assert!(state.has_breakpoint(10));
        assert!(!state.has_breakpoint(11));

        // Setting same breakpoint again returns false
        assert!(!state.set_breakpoint(10));

        assert!(state.remove_breakpoint(10));
        assert!(!state.has_breakpoint(10));

        // Removing non-existent breakpoint returns false
        assert!(!state.remove_breakpoint(10));
    }

    #[test]
    fn test_breakpoint_clear() {
        let mut state = DebugState::new();
        state.set_breakpoint(10);
        state.set_breakpoint(20);
        state.set_breakpoint(30);

        assert_eq!(state.breakpoint_count(), 3);
        state.clear_breakpoints();
        assert_eq!(state.breakpoint_count(), 0);
    }

    #[test]
    fn test_step_mode() {
        let mut state = DebugState::new();

        assert_eq!(state.step_mode(), StepMode::None);

        state.set_step_mode(StepMode::StepInto);
        assert_eq!(state.step_mode(), StepMode::StepInto);

        state.set_step_mode(StepMode::Continue);
        assert_eq!(state.step_mode(), StepMode::Continue);
    }

    #[test]
    fn test_call_stack() {
        let mut state = DebugState::new();

        assert_eq!(state.stack_depth(), 0);
        assert!(state.current_frame().is_none());

        state.push_frame(CallFrame::new("main".to_string(), 1));
        assert_eq!(state.stack_depth(), 1);

        state.push_frame(CallFrame::new("helper".to_string(), 10));
        assert_eq!(state.stack_depth(), 2);
        assert_eq!(state.current_frame().unwrap().function_name, "helper");

        state.pop_frame();
        assert_eq!(state.stack_depth(), 1);
        assert_eq!(state.current_frame().unwrap().function_name, "main");
    }

    #[test]
    fn test_watch_expressions() {
        let mut state = DebugState::new();

        let id1 = state.add_watch("x".to_string());
        let id2 = state.add_watch("y + z".to_string());

        assert_eq!(state.watches().len(), 2);
        assert_eq!(state.watches()[0].id, id1);
        assert_eq!(state.watches()[1].id, id2);

        assert!(state.remove_watch(id1));
        assert_eq!(state.watches().len(), 1);
        assert!(!state.remove_watch(id1)); // Already removed
    }

    #[test]
    fn test_pause_and_resume() {
        let mut state = DebugState::new();

        assert!(!state.is_paused());

        state.pause(PauseReason::Breakpoint(10));
        assert!(state.is_paused());
        assert!(matches!(
            state.pause_reason(),
            Some(PauseReason::Breakpoint(10))
        ));

        state.resume();
        assert!(!state.is_paused());
        assert!(state.pause_reason().is_none());
    }

    #[test]
    fn test_should_pause_on_breakpoint() {
        let mut state = DebugState::new();
        state.set_breakpoint(10);

        assert!(state.should_pause(5).is_none());
        assert!(matches!(
            state.should_pause(10),
            Some(PauseReason::Breakpoint(10))
        ));
    }

    #[test]
    fn test_should_pause_on_step() {
        let mut state = DebugState::new();
        state.set_step_mode(StepMode::StepInto);

        // First check should trigger pause
        assert!(matches!(state.should_pause(5), Some(PauseReason::Step)));

        // Subsequent check should not (break_on_next was consumed)
        assert!(state.should_pause(6).is_none());
    }

    #[test]
    fn test_source_context() {
        let source = "line 1\nline 2\nline 3\nline 4\nline 5";
        let mut state = DebugState::with_source(source);

        state.set_location(3, 1);
        let context = state.format_source_context(1);

        assert!(context.contains("line 2"));
        assert!(context.contains("line 3"));
        assert!(context.contains("line 4"));
        assert!(context.contains("→")); // Current line marker
    }

    #[test]
    fn test_command_parsing() {
        assert_eq!(
            DebugCommand::parse("break 10").unwrap(),
            DebugCommand::Break(10)
        );
        assert_eq!(
            DebugCommand::parse("b 20").unwrap(),
            DebugCommand::Break(20)
        );
        assert_eq!(
            DebugCommand::parse("delete 10").unwrap(),
            DebugCommand::Delete(10)
        );
        assert_eq!(
            DebugCommand::parse("d all").unwrap(),
            DebugCommand::DeleteAll
        );
        assert_eq!(
            DebugCommand::parse("continue").unwrap(),
            DebugCommand::Continue
        );
        assert_eq!(DebugCommand::parse("c").unwrap(), DebugCommand::Continue);
        assert_eq!(DebugCommand::parse("step").unwrap(), DebugCommand::StepInto);
        assert_eq!(DebugCommand::parse("s").unwrap(), DebugCommand::StepInto);
        assert_eq!(DebugCommand::parse("next").unwrap(), DebugCommand::StepOver);
        assert_eq!(DebugCommand::parse("n").unwrap(), DebugCommand::StepOver);
        assert_eq!(DebugCommand::parse("out").unwrap(), DebugCommand::StepOut);
        assert_eq!(DebugCommand::parse("o").unwrap(), DebugCommand::StepOut);
        assert_eq!(
            DebugCommand::parse("print x").unwrap(),
            DebugCommand::Print("x".to_string())
        );
        assert_eq!(
            DebugCommand::parse("p myVar").unwrap(),
            DebugCommand::Print("myVar".to_string())
        );
        assert_eq!(DebugCommand::parse("locals").unwrap(), DebugCommand::Locals);
        assert_eq!(DebugCommand::parse("l").unwrap(), DebugCommand::Locals);
        assert_eq!(DebugCommand::parse("stack").unwrap(), DebugCommand::Stack);
        assert_eq!(DebugCommand::parse("st").unwrap(), DebugCommand::Stack);
        assert_eq!(
            DebugCommand::parse("watch counter").unwrap(),
            DebugCommand::Watch("counter".to_string())
        );
        assert_eq!(DebugCommand::parse("help").unwrap(), DebugCommand::Help);
        assert_eq!(DebugCommand::parse("quit").unwrap(), DebugCommand::Quit);
    }

    #[test]
    fn test_command_parsing_errors() {
        assert!(DebugCommand::parse("break").is_err());
        assert!(DebugCommand::parse("break abc").is_err());
        assert!(DebugCommand::parse("delete").is_err());
        assert!(DebugCommand::parse("print").is_err());
        assert!(DebugCommand::parse("unknown").is_err());
        assert!(DebugCommand::parse("").is_err());
    }

    #[test]
    fn test_call_frame_display() {
        let frame = CallFrame::new("myFunction".to_string(), 42);
        assert_eq!(format!("{}", frame), "myFunction at line 42");

        let native = CallFrame::native("println".to_string());
        assert_eq!(format!("{}", native), "println (native)");
    }

    #[test]
    fn test_pause_reason_display() {
        assert_eq!(
            format!("{}", PauseReason::Breakpoint(10)),
            "Breakpoint at line 10"
        );
        assert_eq!(format!("{}", PauseReason::Step), "Step completed");
        assert_eq!(format!("{}", PauseReason::UserRequest), "Paused by user");
    }
}
