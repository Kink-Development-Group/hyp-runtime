//! Debug REPL for interactive HypnoScript debugging.
//!
//! This module provides an interactive debugging session that allows users to:
//! - Set and remove breakpoints
//! - Step through code execution
//! - Inspect variables and the call stack
//! - Evaluate expressions
//! - Watch expressions
//!
//! # Usage
//!
//! The debug REPL is invoked when running a HypnoScript file with the `--debug` flag:
//!
//! ```bash
//! hypnoscript exec myfile.hyp --debug
//! ```
//!
//! This enters an interactive session where users can use debug commands
//! before and during program execution.

use anyhow::{Result, anyhow};
use hypnoscript_compiler::{DebugCommand, Interpreter, StepMode, debug_help};
use hypnoscript_lexer_parser::{Lexer, Parser as HypnoParser};
use std::collections::HashSet;
use std::io::{self, Write};

/// Configuration for a debug session.
#[derive(Debug, Clone, Default)]
pub struct DebugConfig {
    /// Initial breakpoints to set (line numbers)
    pub breakpoints: Vec<usize>,
    /// Initial watch expressions
    pub watch_expressions: Vec<String>,
    /// Whether to enable verbose output
    pub verbose: bool,
    /// Path to trace output file (if any)
    pub trace_file: Option<String>,
}

impl DebugConfig {
    /// Creates a new debug configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds initial breakpoints.
    pub fn with_breakpoints(mut self, breakpoints: Vec<usize>) -> Self {
        self.breakpoints = breakpoints;
        self
    }

    /// Adds initial watch expressions.
    pub fn with_watches(mut self, watches: Vec<String>) -> Self {
        self.watch_expressions = watches;
        self
    }

    /// Enables verbose output.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Sets the trace output file.
    pub fn with_trace_file(mut self, trace_file: Option<String>) -> Self {
        self.trace_file = trace_file;
        self
    }
}

/// Result of executing a debug command.
#[derive(Debug)]
#[allow(dead_code)]
pub enum CommandResult {
    /// Continue running until next pause
    Continue,
    /// Step to next statement
    Step(StepMode),
    /// Quit the debug session
    Quit,
    /// Output to display to user
    Output(String),
    /// No action needed
    None,
}

/// Interactive debug session.
pub struct DebugSession {
    /// The interpreter instance
    interpreter: Interpreter,
    /// Source code being debugged
    source: String,
    /// Parsed AST
    ast: Option<hypnoscript_lexer_parser::ast::AstNode>,
    /// Configuration
    config: DebugConfig,
    /// Whether the session has started
    started: bool,
    /// Whether execution has finished
    finished: bool,
    /// Current execution state
    execution_state: ExecutionState,
}

/// State of the debug execution.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
enum ExecutionState {
    /// Before execution started
    NotStarted,
    /// Running (not paused)
    Running,
    /// Paused at a breakpoint or step
    Paused,
    /// Execution completed
    Finished,
}

impl DebugSession {
    /// Creates a new debug session for the given source file.
    pub fn new(source: String, config: DebugConfig) -> Result<Self> {
        let mut interpreter = Interpreter::new();
        interpreter.enable_debug_mode(&source);

        // Set initial breakpoints
        for &line in &config.breakpoints {
            interpreter.set_breakpoint(line);
        }

        // Add initial watch expressions
        for expr in &config.watch_expressions {
            interpreter.add_watch(expr.clone());
        }

        Ok(Self {
            interpreter,
            source,
            ast: None,
            config,
            started: false,
            finished: false,
            execution_state: ExecutionState::NotStarted,
        })
    }

    /// Parses the source code.
    pub fn parse(&mut self) -> Result<()> {
        let mut lexer = Lexer::new(&self.source);
        let tokens = lexer.lex().map_err(|e| anyhow!("Lexer error: {}", e))?;

        let mut parser = HypnoParser::new(tokens);
        let ast = parser
            .parse_program()
            .map_err(|e| anyhow!("Parser error: {}", e))?;

        self.ast = Some(ast);
        Ok(())
    }

    /// Runs the interactive debug session.
    pub fn run(&mut self) -> Result<()> {
        // Parse if not already done
        if self.ast.is_none() {
            self.parse()?;
        }

        self.print_welcome();
        self.print_source_info();

        // Enter the REPL loop
        self.repl_loop()
    }

    /// The main REPL loop.
    fn repl_loop(&mut self) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            // Print prompt
            print!("(debug) ");
            stdout.flush()?;

            // Read command
            let mut input = String::new();
            if stdin.read_line(&mut input)? == 0 {
                // EOF
                break;
            }

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            // Parse and execute command
            match self.execute_command(input) {
                Ok(CommandResult::Quit) => {
                    println!("Debugging session ended.");
                    break;
                }
                Ok(CommandResult::Continue) => {
                    self.run_until_pause()?;
                }
                Ok(CommandResult::Step(mode)) => {
                    self.step_execution(mode)?;
                }
                Ok(CommandResult::Output(msg)) => {
                    println!("{}", msg);
                }
                Ok(CommandResult::None) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }

            // Check if execution finished
            if self.finished {
                println!("\n✅ Program execution completed.");
                break;
            }
        }

        Ok(())
    }

    /// Executes a debug command.
    fn execute_command(&mut self, input: &str) -> Result<CommandResult> {
        let cmd = DebugCommand::parse(input).map_err(|e| anyhow!("{}", e))?;

        match cmd {
            DebugCommand::Break(line) => {
                if self.interpreter.set_breakpoint(line) {
                    Ok(CommandResult::Output(format!(
                        "Breakpoint set at line {}",
                        line
                    )))
                } else {
                    Ok(CommandResult::Output(format!(
                        "Breakpoint already exists at line {}",
                        line
                    )))
                }
            }

            DebugCommand::Delete(line) => {
                if self.interpreter.remove_breakpoint(line) {
                    Ok(CommandResult::Output(format!(
                        "Breakpoint removed at line {}",
                        line
                    )))
                } else {
                    Ok(CommandResult::Output(format!(
                        "No breakpoint at line {}",
                        line
                    )))
                }
            }

            DebugCommand::DeleteAll => {
                self.interpreter.clear_breakpoints();
                Ok(CommandResult::Output("All breakpoints removed".to_string()))
            }

            DebugCommand::Continue => {
                if !self.started {
                    self.start_execution()?;
                }
                self.interpreter.set_step_mode(StepMode::Continue);
                Ok(CommandResult::Continue)
            }

            DebugCommand::StepInto => {
                if !self.started {
                    self.start_execution()?;
                }
                Ok(CommandResult::Step(StepMode::StepInto))
            }

            DebugCommand::StepOver => {
                if !self.started {
                    self.start_execution()?;
                }
                Ok(CommandResult::Step(StepMode::StepOver))
            }

            DebugCommand::StepOut => {
                if !self.started {
                    self.start_execution()?;
                }
                Ok(CommandResult::Step(StepMode::StepOut))
            }

            DebugCommand::Print(expr) => {
                let result = self.evaluate_expression(&expr)?;
                Ok(CommandResult::Output(format!("{} = {}", expr, result)))
            }

            DebugCommand::Locals => {
                let locals = self.interpreter.debug_locals();
                if locals.is_empty() {
                    Ok(CommandResult::Output("  (no local variables)".to_string()))
                } else {
                    let output = locals
                        .iter()
                        .map(|(name, value)| format!("  {} = {}", name, value))
                        .collect::<Vec<_>>()
                        .join("\n");
                    Ok(CommandResult::Output(format!(
                        "Local variables:\n{}",
                        output
                    )))
                }
            }

            DebugCommand::Stack => {
                let stack = self.interpreter.debug_call_stack();
                if stack.is_empty() {
                    Ok(CommandResult::Output("  (empty stack)".to_string()))
                } else {
                    let output = stack
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, frame)| format!("  #{} {}", i, frame))
                        .collect::<Vec<_>>()
                        .join("\n");
                    Ok(CommandResult::Output(format!("Call stack:\n{}", output)))
                }
            }

            DebugCommand::Watch(expr) => {
                if let Some(id) = self.interpreter.add_watch(expr.clone()) {
                    Ok(CommandResult::Output(format!(
                        "Watch #{} added: {}",
                        id, expr
                    )))
                } else {
                    Ok(CommandResult::Output(
                        "Failed to add watch expression".to_string(),
                    ))
                }
            }

            DebugCommand::Unwatch(id) => {
                if self.interpreter.remove_watch(id) {
                    Ok(CommandResult::Output(format!("Watch #{} removed", id)))
                } else {
                    Ok(CommandResult::Output(format!("Watch #{} not found", id)))
                }
            }

            DebugCommand::Watches => {
                let watches = self.format_watches();
                Ok(CommandResult::Output(watches))
            }

            DebugCommand::ListBreakpoints => {
                let breakpoints = self.interpreter.breakpoints();
                if breakpoints.is_empty() {
                    Ok(CommandResult::Output("  (no breakpoints set)".to_string()))
                } else {
                    let mut sorted: Vec<_> = breakpoints.into_iter().collect();
                    sorted.sort();
                    let output = sorted
                        .iter()
                        .map(|line| format!("  Line {}", line))
                        .collect::<Vec<_>>()
                        .join("\n");
                    Ok(CommandResult::Output(format!("Breakpoints:\n{}", output)))
                }
            }

            DebugCommand::Help => Ok(CommandResult::Output(debug_help())),

            DebugCommand::Quit => Ok(CommandResult::Quit),

            DebugCommand::Where => {
                let context = self.interpreter.debug_source_context(3);
                Ok(CommandResult::Output(context))
            }

            DebugCommand::List(line) => {
                let output = self.list_source(line);
                Ok(CommandResult::Output(output))
            }
        }
    }

    /// Starts program execution.
    fn start_execution(&mut self) -> Result<()> {
        if self.started {
            return Ok(());
        }

        self.started = true;
        self.execution_state = ExecutionState::Running;

        if self.config.verbose {
            println!("Starting program execution...");
        }

        Ok(())
    }

    /// Runs until next pause point (breakpoint or step).
    fn run_until_pause(&mut self) -> Result<()> {
        if self.finished {
            println!("Program has already finished.");
            return Ok(());
        }

        // Execute the program
        if let Some(ast) = self.ast.take() {
            match self.interpreter.execute_program(ast.clone()) {
                Ok(()) => {
                    self.finished = true;
                    self.execution_state = ExecutionState::Finished;
                }
                Err(e) => {
                    eprintln!("Runtime error: {}", e);
                    self.finished = true;
                    self.execution_state = ExecutionState::Finished;
                }
            }
            self.ast = Some(ast);
        }

        Ok(())
    }

    /// Performs a step execution.
    fn step_execution(&mut self, mode: StepMode) -> Result<()> {
        self.interpreter.set_step_mode(mode);
        self.run_until_pause()
    }

    /// Evaluates an expression in the current context.
    fn evaluate_expression(&self, expr: &str) -> Result<String> {
        // Check if it's a simple variable reference
        let all_vars = self.interpreter.debug_all_variables();

        // Try direct variable lookup first
        if let Some(value) = all_vars.get(expr.trim()) {
            return Ok(value.to_string());
        }

        // For more complex expressions, we would need to parse and evaluate them
        // For now, we handle some simple cases
        let trimmed = expr.trim();

        // Check for member access (e.g., "obj.field")
        if let Some(dot_pos) = trimmed.find('.') {
            let obj_name = &trimmed[..dot_pos];
            let field_name = &trimmed[dot_pos + 1..];

            if let Some(obj_value) = all_vars.get(obj_name) {
                return Ok(format!(
                    "{}.{} on {} (member access not fully supported in REPL)",
                    obj_name, field_name, obj_value
                ));
            }
        }

        // Variable not found
        Err(anyhow!("Cannot evaluate '{}': variable not found", expr))
    }

    /// Formats watch expressions for display.
    fn format_watches(&self) -> String {
        // Get watches from debug state
        if let Some(ref state) = self.interpreter.debug_state {
            let watches = state.watches();
            if watches.is_empty() {
                return "  (no watch expressions)".to_string();
            }

            let output: Vec<String> = watches
                .iter()
                .map(|w| {
                    let value = match self.evaluate_expression(&w.expression) {
                        Ok(v) => v,
                        Err(_) => "<undefined>".to_string(),
                    };
                    format!("  #{} {} = {}", w.id, w.expression, value)
                })
                .collect();

            format!("Watches:\n{}", output.join("\n"))
        } else {
            "  (debug mode not enabled)".to_string()
        }
    }

    /// Lists source code around the specified line.
    fn list_source(&self, center_line: Option<usize>) -> String {
        let lines: Vec<&str> = self.source.lines().collect();
        if lines.is_empty() {
            return "(no source available)".to_string();
        }

        let center = center_line.unwrap_or(1).saturating_sub(1);
        let start = center.saturating_sub(5);
        let end = (center + 5).min(lines.len() - 1);

        let breakpoints: HashSet<usize> = self.interpreter.breakpoints().into_iter().collect();

        let mut output = String::new();
        for (i, line) in lines.iter().enumerate().skip(start).take(end - start + 1) {
            let line_num = i + 1;
            let marker = if breakpoints.contains(&line_num) {
                "●"
            } else {
                " "
            };
            output.push_str(&format!("{} {:4} | {}\n", marker, line_num, line));
        }

        output
    }

    /// Prints welcome message.
    fn print_welcome(&self) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!(
            "║           HypnoScript Debugger v{}                  ║",
            env!("CARGO_PKG_VERSION")
        );
        println!("║  Type 'help' for available commands, 'quit' to exit         ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
    }

    /// Prints source information.
    fn print_source_info(&self) {
        let line_count = self.source.lines().count();
        println!("Source: {} lines", line_count);

        let breakpoints = self.interpreter.breakpoints();
        if !breakpoints.is_empty() {
            let mut sorted: Vec<_> = breakpoints.into_iter().collect();
            sorted.sort();
            println!("Breakpoints: {:?}", sorted);
        }

        if !self.config.watch_expressions.is_empty() {
            println!("Watches: {:?}", self.config.watch_expressions);
        }

        println!();
        println!("Use 'continue' or 'c' to start execution.");
        println!("Use 'step' or 's' to step through code.");
        println!();
    }
}

/// Runs an interactive debug session for a HypnoScript file.
///
/// # Arguments
/// * `source` - The source code to debug
/// * `config` - Debug configuration
///
/// # Returns
/// `Ok(())` on successful completion, `Err` on failure
pub fn run_debug_session(source: String, config: DebugConfig) -> Result<()> {
    let mut session = DebugSession::new(source, config)?;
    session.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_config_default() {
        let config = DebugConfig::default();
        assert!(config.breakpoints.is_empty());
        assert!(config.watch_expressions.is_empty());
        assert!(!config.verbose);
        assert!(config.trace_file.is_none());
    }

    #[test]
    fn test_debug_config_builder() {
        let config = DebugConfig::new()
            .with_breakpoints(vec![5, 10, 15])
            .with_watches(vec!["x".to_string(), "y".to_string()])
            .with_verbose(true)
            .with_trace_file(Some("trace.log".to_string()));

        assert_eq!(config.breakpoints, vec![5, 10, 15]);
        assert_eq!(config.watch_expressions, vec!["x", "y"]);
        assert!(config.verbose);
        assert_eq!(config.trace_file, Some("trace.log".to_string()));
    }

    #[test]
    fn test_debug_session_creation() {
        let source = "Focus { induce x = 42; } Relax".to_string();
        let config = DebugConfig::default();
        let session = DebugSession::new(source, config);
        assert!(session.is_ok());
    }

    #[test]
    fn test_debug_session_with_breakpoints() {
        let source = "Focus { induce x = 42; observe x; } Relax".to_string();
        let config = DebugConfig::new().with_breakpoints(vec![1, 2]);
        let session = DebugSession::new(source, config).unwrap();

        assert!(session.interpreter.has_breakpoint(1));
        assert!(session.interpreter.has_breakpoint(2));
    }

    #[test]
    fn test_debug_session_parse() {
        let source = "Focus { induce x: number = 42; } Relax".to_string();
        let config = DebugConfig::default();
        let mut session = DebugSession::new(source, config).unwrap();

        assert!(session.parse().is_ok());
        assert!(session.ast.is_some());
    }

    #[test]
    fn test_command_result_variants() {
        // Test that all variants can be created
        let _ = CommandResult::Continue;
        let _ = CommandResult::Step(StepMode::StepInto);
        let _ = CommandResult::Quit;
        let _ = CommandResult::Output("test".to_string());
        let _ = CommandResult::None;
    }

    #[test]
    fn test_list_source() {
        let source =
            "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9\nline 10"
                .to_string();
        let config = DebugConfig::default();
        let session = DebugSession::new(source, config).unwrap();

        let output = session.list_source(Some(5));
        assert!(output.contains("line 1"));
        assert!(output.contains("line 5"));
        assert!(output.contains("line 10"));
    }

    #[test]
    fn test_list_source_with_breakpoints() {
        let source = "line 1\nline 2\nline 3".to_string();
        let config = DebugConfig::new().with_breakpoints(vec![2]);
        let session = DebugSession::new(source, config).unwrap();

        let output = session.list_source(None);
        assert!(output.contains("●")); // Breakpoint marker
    }
}
