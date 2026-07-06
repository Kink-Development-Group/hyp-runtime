//! The HypnoScript tree-walking interpreter.
//!
//! Split into focused submodules:
//! - [`error`]: runtime error types
//! - [`value`]: runtime values (numbers, strings, functions, promises, ...)
//! - [`session`]: session (class) definitions and instances
//! - [`builtins`]: dispatch into the `hypnoscript-runtime` builtin modules

mod builtins;
mod error;
mod session;
mod value;

pub use error::InterpreterError;
pub use value::{FunctionValue, Promise, RecordValue, Value};

pub use session::{SessionDefinition, SessionInstance};

use error::localized;
use session::{SessionFieldDefinition, SessionMethodDefinition};

use hypnoscript_lexer_parser::ast::{
    AstNode, Pattern, SessionField, SessionMember, SessionMethod, SessionVisibility,
    VariableStorage,
};
use hypnoscript_runtime::CoreBuiltins;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Type alias for debug pause callback to reduce type complexity.
type DebugPauseCallback =
    Box<dyn Fn(&mut Interpreter, crate::debug::PauseReason) -> crate::debug::StepMode>;

#[derive(Clone, Copy, Debug)]
enum ScopeLayer {
    Local,
    Global,
    Shared,
}

#[derive(Debug, Clone)]
struct ExecutionContextFrame {
    session_name: Option<String>,
}

/// The HypnoScript interpreter.
///
/// Executes HypnoScript AST nodes using a tree-walking interpretation strategy.
/// Supports:
/// - Variable scopes (global, shared, local)
/// - Functions and triggers
/// - Sessions (OOP)
/// - Pattern matching (`entrain`/`when`)
/// - Async execution (`mesmerize`/`await`)
/// - Channels for inter-task communication
/// - 180+ builtin functions
///
/// # Architecture
///
/// The interpreter maintains:
/// - `globals`: Top-level variables in `Focus { ... } Relax` scope
/// - `shared`: Variables declared with `sharedTrance` (module-level)
/// - `locals`: Stack of local scopes (function calls, loops, blocks)
/// - `const_globals`/`const_locals`: Tracks immutable variables (`freeze`)
/// - `execution_context`: Call stack for session method dispatch
/// - `tranceify_types`: Record type definitions
/// - `async_runtime`: Optional async task executor
/// - `channel_registry`: Optional channel system for message passing
///
/// # Examples
///
/// ```rust
/// use hypnoscript_compiler::Interpreter;
/// use hypnoscript_lexer_parser::Parser;
/// use hypnoscript_lexer_parser::Lexer;
///
/// let source = r#"
///     Focus {
///         entrance {
///             observe "Hello, World!";
///         }
///     } Relax;
/// "#;
///
/// let mut lexer = Lexer::new(source);
/// let tokens = lexer.lex().unwrap();
/// let mut parser = Parser::new(tokens);
/// let ast = parser.parse_program().unwrap();
/// let mut interpreter = Interpreter::new();
/// interpreter.execute_program(ast).unwrap();
/// ```
pub struct Interpreter {
    globals: HashMap<String, Value>,
    shared: HashMap<String, Value>,
    const_globals: HashSet<String>,
    locals: Vec<HashMap<String, Value>>,
    const_locals: Vec<HashSet<String>>,
    execution_context: Vec<ExecutionContextFrame>,
    /// Tranceify type definitions (field names for each type)
    tranceify_types: HashMap<String, Vec<String>>,
    /// Label declared immediately before a loop statement; the loop claims it
    /// on entry so `snap label;` / `sink label;` can target that loop.
    pending_loop_label: Option<String>,

    /// Optional async runtime for true async execution
    pub async_runtime: Option<std::sync::Arc<crate::async_runtime::AsyncRuntime>>,

    /// Optional channel registry for inter-task communication
    pub channel_registry: Option<std::sync::Arc<crate::channel_system::ChannelRegistry>>,

    /// Optional debug state for step-through debugging
    pub debug_state: Option<crate::debug::DebugState>,

    /// Callback invoked when debugger pauses (for REPL integration)
    #[allow(dead_code)]
    debug_pause_callback: Option<DebugPauseCallback>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            shared: HashMap::new(),
            const_globals: HashSet::new(),
            locals: Vec::new(),
            const_locals: Vec::new(),
            execution_context: Vec::new(),
            tranceify_types: HashMap::new(),
            pending_loop_label: None,
            async_runtime: None,
            channel_registry: None,
            debug_state: None,
            debug_pause_callback: None,
        }
    }

    /// Create interpreter with async runtime support
    pub fn with_async_runtime() -> Result<Self, InterpreterError> {
        let runtime = crate::async_runtime::AsyncRuntime::new().map_err(|e| {
            InterpreterError::Runtime(format!("Failed to create async runtime: {}", e))
        })?;
        let registry = crate::channel_system::ChannelRegistry::new();

        Ok(Self {
            globals: HashMap::new(),
            shared: HashMap::new(),
            const_globals: HashSet::new(),
            locals: Vec::new(),
            const_locals: Vec::new(),
            execution_context: Vec::new(),
            tranceify_types: HashMap::new(),
            pending_loop_label: None,
            async_runtime: Some(std::sync::Arc::new(runtime)),
            channel_registry: Some(std::sync::Arc::new(registry)),
            debug_state: None,
            debug_pause_callback: None,
        })
    }

    /// Enable async runtime for existing interpreter
    pub fn enable_async_runtime(&mut self) -> Result<(), InterpreterError> {
        if self.async_runtime.is_none() {
            let runtime = crate::async_runtime::AsyncRuntime::new().map_err(|e| {
                InterpreterError::Runtime(format!("Failed to create async runtime: {}", e))
            })?;
            let registry = crate::channel_system::ChannelRegistry::new();

            self.async_runtime = Some(std::sync::Arc::new(runtime));
            self.channel_registry = Some(std::sync::Arc::new(registry));
        }
        Ok(())
    }

    // === Debug Mode Methods ===

    /// Enables debug mode with the given source code.
    ///
    /// When debug mode is enabled, the interpreter will check for breakpoints
    /// and step conditions before each statement execution.
    ///
    /// # Arguments
    /// * `source` - The source code (for display in debugger)
    pub fn enable_debug_mode(&mut self, source: &str) {
        self.debug_state = Some(crate::debug::DebugState::with_source(source));
    }

    /// Enables debug mode without source code.
    pub fn enable_debug_mode_no_source(&mut self) {
        self.debug_state = Some(crate::debug::DebugState::new());
    }

    /// Disables debug mode.
    pub fn disable_debug_mode(&mut self) {
        self.debug_state = None;
    }

    /// Returns whether debug mode is enabled.
    pub fn is_debug_mode(&self) -> bool {
        self.debug_state.is_some()
    }

    /// Sets a breakpoint at the given line number.
    ///
    /// # Arguments
    /// * `line` - Line number (1-indexed)
    ///
    /// # Returns
    /// `true` if breakpoint was newly added, `false` if already existed or debug mode not enabled
    pub fn set_breakpoint(&mut self, line: usize) -> bool {
        if let Some(ref mut state) = self.debug_state {
            state.set_breakpoint(line)
        } else {
            false
        }
    }

    /// Removes a breakpoint at the given line number.
    pub fn remove_breakpoint(&mut self, line: usize) -> bool {
        if let Some(ref mut state) = self.debug_state {
            state.remove_breakpoint(line)
        } else {
            false
        }
    }

    /// Checks if a breakpoint exists at the given line.
    pub fn has_breakpoint(&self, line: usize) -> bool {
        self.debug_state
            .as_ref()
            .map(|s| s.has_breakpoint(line))
            .unwrap_or(false)
    }

    /// Returns all breakpoints.
    pub fn breakpoints(&self) -> Vec<usize> {
        self.debug_state
            .as_ref()
            .map(|s| s.breakpoints().iter().copied().collect())
            .unwrap_or_default()
    }

    /// Clears all breakpoints.
    pub fn clear_breakpoints(&mut self) {
        if let Some(ref mut state) = self.debug_state {
            state.clear_breakpoints();
        }
    }

    /// Sets the step mode for debugging.
    pub fn set_step_mode(&mut self, mode: crate::debug::StepMode) {
        if let Some(ref mut state) = self.debug_state {
            state.set_step_mode(mode);
        }
    }

    /// Gets the current step mode.
    pub fn step_mode(&self) -> crate::debug::StepMode {
        self.debug_state
            .as_ref()
            .map(|s| s.step_mode())
            .unwrap_or(crate::debug::StepMode::None)
    }

    /// Adds a watch expression.
    ///
    /// # Returns
    /// The ID of the new watch expression, or None if debug mode not enabled
    pub fn add_watch(&mut self, expression: String) -> Option<usize> {
        self.debug_state.as_mut().map(|s| s.add_watch(expression))
    }

    /// Removes a watch expression by ID.
    pub fn remove_watch(&mut self, id: usize) -> bool {
        self.debug_state
            .as_mut()
            .map(|s| s.remove_watch(id))
            .unwrap_or(false)
    }

    /// Returns the current call stack for debugging.
    pub fn debug_call_stack(&self) -> Vec<crate::debug::CallFrame> {
        self.debug_state
            .as_ref()
            .map(|s| s.call_stack().to_vec())
            .unwrap_or_default()
    }

    /// Returns local variables in the current scope.
    pub fn debug_locals(&self) -> HashMap<String, Value> {
        if let Some(scope) = self.locals.last() {
            scope.clone()
        } else {
            HashMap::new()
        }
    }

    /// Returns global variables.
    pub fn debug_globals(&self) -> HashMap<String, Value> {
        self.globals.clone()
    }

    /// Returns all visible variables (locals + globals).
    pub fn debug_all_variables(&self) -> HashMap<String, Value> {
        let mut vars = self.globals.clone();
        vars.extend(self.shared.clone());
        for scope in &self.locals {
            vars.extend(scope.clone());
        }
        vars
    }

    /// Returns the current debug state summary.
    pub fn debug_summary(&self) -> String {
        self.debug_state
            .as_ref()
            .map(|s| s.summary())
            .unwrap_or_else(|| "Debug mode not enabled".to_string())
    }

    /// Formats the source context around the current line.
    pub fn debug_source_context(&self, context_lines: usize) -> String {
        self.debug_state
            .as_ref()
            .map(|s| s.format_source_context(context_lines))
            .unwrap_or_else(|| "(no source available)".to_string())
    }

    /// Checks if the debugger should pause before executing a statement.
    /// Returns the pause reason if should pause, None otherwise.
    #[allow(dead_code)]
    fn check_debug_pause(&mut self, line: usize) -> Option<crate::debug::PauseReason> {
        if let Some(ref mut state) = self.debug_state {
            state.should_pause(line)
        } else {
            None
        }
    }

    /// Handles a debug pause event.
    /// This is called when a breakpoint is hit or step completes.
    #[allow(dead_code)]
    fn handle_debug_pause(&mut self, reason: crate::debug::PauseReason) {
        if let Some(ref mut state) = self.debug_state {
            state.pause(reason);
        }
    }

    /// Resumes execution after a pause.
    pub fn debug_resume(&mut self) {
        if let Some(ref mut state) = self.debug_state {
            state.resume();
        }
    }

    /// Pushes a debug call frame when entering a function.
    #[allow(dead_code)]
    fn debug_push_frame(&mut self, function_name: &str, line: usize) {
        if let Some(ref mut state) = self.debug_state {
            state.push_frame(crate::debug::CallFrame::new(
                function_name.to_string(),
                line,
            ));
        }
    }

    /// Pops a debug call frame when exiting a function.
    #[allow(dead_code)]
    fn debug_pop_frame(&mut self) {
        if let Some(ref mut state) = self.debug_state {
            state.pop_frame();
        }
    }

    /// Triggers a programmatic breakpoint from code.
    pub fn trigger_breakpoint(&mut self) -> Option<crate::debug::PauseReason> {
        self.debug_state.as_mut().map(|s| s.trigger_breakpoint())
    }

    pub fn execute_program(&mut self, program: AstNode) -> Result<(), InterpreterError> {
        if let AstNode::Program(statements) = program {
            for stmt in statements {
                self.execute_statement(&stmt)?;
            }
            Ok(())
        } else {
            Err(InterpreterError::Runtime(
                "Expected program node".to_string(),
            ))
        }
    }

    fn execute_statement(&mut self, stmt: &AstNode) -> Result<(), InterpreterError> {
        match stmt {
            AstNode::VariableDeclaration {
                name,
                type_annotation: _,
                initializer,
                is_constant,
                storage,
            } => {
                let value = if let Some(init) = initializer {
                    self.evaluate_expression(init)?
                } else {
                    Value::Null
                };
                self.define_variable(*storage, name.clone(), value, *is_constant);
                Ok(())
            }

            AstNode::AnchorDeclaration { name, source } => {
                // Anchor saves the current value of a variable
                let value = self.evaluate_expression(source)?;
                let scope = self.resolve_assignment_scope(name);
                self.set_variable(name.clone(), value, scope)?;
                Ok(())
            }

            AstNode::FunctionDeclaration {
                name,
                parameters,
                return_type: _,
                body,
            } => {
                let param_names: Vec<String> = parameters.iter().map(|p| p.name.clone()).collect();
                let func = FunctionValue::new_global(name.clone(), param_names, body.clone());
                self.define_variable(
                    VariableStorage::Local,
                    name.clone(),
                    Value::Function(func),
                    false,
                );
                Ok(())
            }

            AstNode::TriggerDeclaration {
                name,
                parameters,
                return_type: _,
                body,
            } => {
                // Triggers are handled like functions
                let param_names: Vec<String> = parameters.iter().map(|p| p.name.clone()).collect();
                let func = FunctionValue::new_global(name.clone(), param_names, body.clone());
                self.define_variable(
                    VariableStorage::Local,
                    name.clone(),
                    Value::Function(func),
                    false,
                );
                Ok(())
            }

            AstNode::SessionDeclaration { name, members } => {
                let session = self.build_session_definition(name, members)?;
                self.define_variable(
                    VariableStorage::Local,
                    name.clone(),
                    Value::Session(session.clone()),
                    false,
                );
                self.initialize_static_fields(session)?;
                Ok(())
            }

            AstNode::TranceifyDeclaration { name, fields } => {
                // Register the tranceify type definition
                let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
                self.tranceify_types.insert(name.clone(), field_names);
                Ok(())
            }

            AstNode::EntranceBlock(statements) | AstNode::FinaleBlock(statements) => {
                for stmt in statements {
                    self.execute_statement(stmt)?;
                }
                Ok(())
            }

            AstNode::ObserveStatement(expr) => {
                let value = self.evaluate_expression(expr)?;
                CoreBuiltins::observe(&value.to_string());
                Ok(())
            }

            AstNode::WhisperStatement(expr) => {
                let value = self.evaluate_expression(expr)?;
                CoreBuiltins::whisper(&value.to_string());
                Ok(())
            }

            AstNode::CommandStatement(expr) => {
                let value = self.evaluate_expression(expr)?;
                CoreBuiltins::command(&value.to_string());
                Ok(())
            }

            AstNode::MurmurStatement(expr) => {
                let value = self.evaluate_expression(expr)?;
                // Murmur is like whisper but even quieter (debug level)
                CoreBuiltins::whisper(&format!("[DEBUG] {}", value));
                Ok(())
            }

            AstNode::OscillateStatement { target } => {
                // Toggle a boolean variable
                if let AstNode::Identifier(name) = target.as_ref() {
                    match self.get_variable(name) {
                        Ok(value) => match value {
                            Value::Boolean(b) => {
                                let scope = self.resolve_assignment_scope(name);
                                self.set_variable(name.clone(), Value::Boolean(!b), scope)?;
                                Ok(())
                            }
                            _ => Err(InterpreterError::Runtime(format!(
                                "Oscillate target '{}' must be boolean, got {:?}",
                                name, value
                            ))),
                        },
                        Err(e) => Err(e),
                    }
                } else {
                    Err(InterpreterError::Runtime(
                        "Oscillate requires a variable identifier".to_string(),
                    ))
                }
            }

            AstNode::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_value = self.evaluate_expression(condition)?;
                if cond_value.is_truthy() {
                    for stmt in then_branch {
                        self.execute_statement(stmt)?;
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            }

            AstNode::DeepFocusStatement { condition, body } => {
                // DeepFocus is like if but with deeper scope/emphasis
                let cond_value = self.evaluate_expression(condition)?;
                if cond_value.is_truthy() {
                    for stmt in body {
                        self.execute_statement(stmt)?;
                    }
                }
                Ok(())
            }

            AstNode::WhileStatement { condition, body } => {
                let my_label = self.pending_loop_label.take();
                loop {
                    let cond_value = self.evaluate_expression(condition)?;
                    if !cond_value.is_truthy() {
                        break;
                    }

                    match self.execute_block(body) {
                        Err(InterpreterError::BreakOutsideLoop) => break,
                        Err(InterpreterError::ContinueOutsideLoop) => continue,
                        Err(InterpreterError::LabeledBreak(l)) if Some(&l) == my_label.as_ref() => {
                            break;
                        }
                        Err(InterpreterError::LabeledContinue(l))
                            if Some(&l) == my_label.as_ref() =>
                        {
                            continue;
                        }
                        Err(e) => return Err(e),
                        Ok(()) => {}
                    }
                }
                Ok(())
            }

            AstNode::LoopStatement {
                init,
                condition,
                update,
                body,
            } => {
                let my_label = self.pending_loop_label.take();
                if let Some(init_stmt) = init.as_ref() {
                    self.execute_statement(init_stmt)?;
                }

                loop {
                    if let Some(cond_expr) = condition.as_ref() {
                        let cond_value = self.evaluate_expression(cond_expr)?;
                        if !cond_value.is_truthy() {
                            break;
                        }
                    }

                    // 'continue' skips the rest of the body but not the update.
                    match self.execute_loop_body(body) {
                        Err(InterpreterError::BreakOutsideLoop) => break,
                        Err(InterpreterError::LabeledBreak(l)) if Some(&l) == my_label.as_ref() => {
                            break;
                        }
                        Err(InterpreterError::ContinueOutsideLoop) => {}
                        Err(InterpreterError::LabeledContinue(l))
                            if Some(&l) == my_label.as_ref() => {}
                        Err(e) => return Err(e),
                        Ok(()) => {}
                    }

                    if let Some(update_stmt) = update.as_ref() {
                        self.execute_statement(update_stmt)?;
                    }
                }
                Ok(())
            }

            AstNode::LabeledStatement { label, body } => {
                self.pending_loop_label = Some(label.clone());
                let result = self.execute_statement(body);
                self.pending_loop_label = None;
                match result {
                    // Fallback for labels on non-loop statements: a matching
                    // labeled break simply exits the labeled statement.
                    Err(InterpreterError::LabeledBreak(l)) if l == *label => Ok(()),
                    other => other,
                }
            }

            AstNode::SuspendStatement => {
                // Suspend is an infinite pause - in practice, this should wait for external input
                // For now, we'll just log a warning
                CoreBuiltins::whisper("[SUSPEND] Program suspended - press Ctrl+C to exit");
                std::thread::sleep(std::time::Duration::from_secs(3600)); // Sleep for 1 hour
                Ok(())
            }

            AstNode::DriftStatement { duration } => {
                let value = self.evaluate_expression(duration)?;
                let milliseconds = value.to_number().map_err(|_| {
                    InterpreterError::TypeError(localized(
                        "drift duration must be a number of milliseconds",
                        "drift-Dauer muss eine Zahl in Millisekunden sein",
                    ))
                })?;
                CoreBuiltins::drift(milliseconds.max(0.0) as u64);
                Ok(())
            }

            AstNode::ReturnStatement(value) => {
                let ret_value = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Null
                };
                Err(InterpreterError::Return(ret_value))
            }

            AstNode::BreakStatement { label } => match label {
                Some(label) => Err(InterpreterError::LabeledBreak(label.clone())),
                None => Err(InterpreterError::BreakOutsideLoop),
            },

            AstNode::ContinueStatement { label } => match label {
                Some(label) => Err(InterpreterError::LabeledContinue(label.clone())),
                None => Err(InterpreterError::ContinueOutsideLoop),
            },

            AstNode::ExpressionStatement(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }

            _ => Err(InterpreterError::Runtime(format!(
                "Unsupported statement: {:?}",
                stmt
            ))),
        }
    }

    fn execute_block(&mut self, statements: &[AstNode]) -> Result<(), InterpreterError> {
        self.push_scope();
        let result = (|| {
            for stmt in statements {
                self.execute_statement(stmt)?;
            }
            Ok(())
        })();
        self.pop_scope();
        result
    }

    /// Execute loop bodies without creating a new scope so variables
    /// persist across iterations (matching HypnoScript semantics)
    fn execute_loop_body(&mut self, statements: &[AstNode]) -> Result<(), InterpreterError> {
        for stmt in statements {
            self.execute_statement(stmt)?;
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, expr: &AstNode) -> Result<Value, InterpreterError> {
        match expr {
            AstNode::NumberLiteral(n) => Ok(Value::Number(*n)),

            AstNode::StringLiteral(s) => Ok(Value::String(s.clone())),

            AstNode::BooleanLiteral(b) => Ok(Value::Boolean(*b)),
            AstNode::NullLiteral => Ok(Value::Null),

            AstNode::Identifier(name) => self.get_variable(name),

            AstNode::ArrayLiteral(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.evaluate_expression(elem)?);
                }
                Ok(Value::Array(values))
            }

            AstNode::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(&left_val, operator, &right_val)
            }

            AstNode::UnaryExpression { operator, operand } => {
                let operand_val = self.evaluate_expression(operand)?;
                match operator.as_str() {
                    "-" => Ok(Value::Number(-operand_val.to_number()?)),
                    "!" => Ok(Value::Boolean(!operand_val.is_truthy())),
                    _ => Err(InterpreterError::Runtime(format!(
                        "Unknown unary operator: {}",
                        operator
                    ))),
                }
            }

            AstNode::CallExpression { callee, arguments } => self.evaluate_call(callee, arguments),

            AstNode::MemberExpression { object, property } => {
                let owner = self.evaluate_expression(object)?;
                self.resolve_member_value(owner, property)
            }

            AstNode::AssignmentExpression { target, value } => match target.as_ref() {
                AstNode::Identifier(name) => {
                    let val = self.evaluate_expression(value)?;
                    let scope = self.resolve_assignment_scope(name);
                    self.set_variable(name.clone(), val.clone(), scope)?;
                    Ok(val)
                }
                AstNode::MemberExpression { object, property } => {
                    let owner = self.evaluate_expression(object)?;
                    let val = self.evaluate_expression(value)?;
                    self.assign_member_value(owner, property, val.clone())?;
                    Ok(val)
                }
                _ => Err(InterpreterError::Runtime(localized(
                    "Invalid assignment target",
                    "Invalid assignment target",
                ))),
            },

            AstNode::IndexExpression { object, index } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;

                if let Value::Array(arr) = obj {
                    let i = idx.to_number()? as usize;
                    arr.get(i).cloned().ok_or_else(|| {
                        InterpreterError::Runtime(format!("Index {} out of bounds", i))
                    })
                } else {
                    Err(InterpreterError::TypeError(
                        "Cannot index non-array".to_string(),
                    ))
                }
            }

            AstNode::AwaitExpression { expression } => {
                // Evaluate the expression - it might return a Promise
                let value = self.evaluate_expression(expression)?;

                // If it's a Promise, await it (resolve it)
                if let Value::Promise(promise_ref) = value {
                    let promise = promise_ref.borrow();
                    if promise.is_resolved() {
                        // Promise is already resolved, return its value
                        Ok(promise.get_value().unwrap_or(Value::Null))
                    } else {
                        // Promise not yet resolved - in a real async system, we'd wait
                        // For now, return null (could simulate delay here)
                        drop(promise); // Release borrow before potentially waiting

                        // Simulate async operation with small delay
                        std::thread::sleep(std::time::Duration::from_millis(10));

                        // Re-check if resolved after wait
                        let promise = promise_ref.borrow();
                        Ok(promise.get_value().unwrap_or(Value::Null))
                    }
                } else {
                    // Not a promise, just return the value
                    Ok(value)
                }
            }

            AstNode::NullishCoalescing { left, right } => {
                let left_val = self.evaluate_expression(left)?;
                if matches!(left_val, Value::Null) {
                    self.evaluate_expression(right)
                } else {
                    Ok(left_val)
                }
            }

            AstNode::OptionalChaining { object, property } => {
                let obj = self.evaluate_expression(object)?;
                if matches!(obj, Value::Null) {
                    Ok(Value::Null)
                } else {
                    self.resolve_member_value(obj, property)
                }
            }

            AstNode::OptionalIndexing { object, index } => {
                let obj = self.evaluate_expression(object)?;
                if matches!(obj, Value::Null) {
                    return Ok(Value::Null);
                }

                let idx = self.evaluate_expression(index)?;
                if let Value::Array(arr) = obj {
                    let i = idx.to_number()? as usize;
                    Ok(arr.get(i).cloned().unwrap_or(Value::Null))
                } else {
                    Err(InterpreterError::TypeError(
                        "Cannot index non-array".to_string(),
                    ))
                }
            }

            AstNode::EntrainExpression {
                subject,
                cases,
                default,
            } => {
                let subject_value = self.evaluate_expression(subject)?;

                // Try to match each case
                for case in cases {
                    if let Some(matched_env) = self.match_pattern(&case.pattern, &subject_value)? {
                        self.push_scope();
                        for (name, value) in &matched_env {
                            self.define_variable(
                                VariableStorage::Local,
                                name.clone(),
                                value.clone(),
                                false,
                            );
                        }

                        let case_result = (|| -> Result<Option<Value>, InterpreterError> {
                            if let Some(guard) = &case.guard {
                                let guard_result = self.evaluate_expression(guard)?;
                                if !guard_result.is_truthy() {
                                    return Ok(None);
                                }
                            }

                            let value = self.execute_entrain_body(&case.body)?;
                            Ok(Some(value))
                        })();

                        self.pop_scope();

                        match case_result? {
                            Some(value) => return Ok(value),
                            None => continue,
                        }
                    }
                }

                // No case matched - try default
                if let Some(default_body) = default {
                    self.push_scope();
                    let result = self.execute_entrain_body(default_body);
                    self.pop_scope();
                    result
                } else {
                    Err(InterpreterError::Runtime(
                        "No pattern matched and no default case provided".to_string(),
                    ))
                }
            }

            AstNode::RecordLiteral { type_name, fields } => {
                // Check if the tranceify type is defined
                if !self.tranceify_types.contains_key(type_name) {
                    return Err(InterpreterError::Runtime(format!(
                        "Undefined tranceify type '{}'",
                        type_name
                    )));
                }

                // Evaluate all field values
                let mut field_values = HashMap::new();
                for field_init in fields {
                    let value = self.evaluate_expression(&field_init.value)?;
                    field_values.insert(field_init.name.clone(), value);
                }

                Ok(Value::Record(RecordValue {
                    type_name: type_name.clone(),
                    fields: field_values,
                }))
            }

            _ => Err(InterpreterError::Runtime(format!(
                "Unsupported expression: {:?}",
                expr
            ))),
        }
    }

    /// Match a pattern against a value, returning bindings if successful
    fn match_pattern(
        &mut self,
        pattern: &Pattern,
        value: &Value,
    ) -> Result<Option<std::collections::HashMap<String, Value>>, InterpreterError> {
        use std::collections::HashMap;

        match pattern {
            Pattern::Literal(lit_node) => {
                let lit_value = self.evaluate_expression(lit_node)?;
                if self.values_equal(&lit_value, value) {
                    Ok(Some(HashMap::new()))
                } else {
                    Ok(None)
                }
            }

            Pattern::Identifier(name) => {
                let mut bindings = HashMap::new();
                bindings.insert(name.clone(), value.clone());
                Ok(Some(bindings))
            }

            Pattern::Typed {
                name,
                type_annotation,
            } => {
                // Check type match
                let type_matches = match type_annotation.to_lowercase().as_str() {
                    "number" => matches!(value, Value::Number(_)),
                    "string" => matches!(value, Value::String(_)),
                    "boolean" => matches!(value, Value::Boolean(_)),
                    "array" => matches!(value, Value::Array(_)),
                    _ => true, // Unknown types always match for now
                };

                if !type_matches {
                    return Ok(None);
                }

                let mut bindings = HashMap::new();
                if let Some(name) = name {
                    bindings.insert(name.clone(), value.clone());
                }
                Ok(Some(bindings))
            }

            Pattern::Array { elements, rest } => {
                if let Value::Array(arr) = value {
                    let mut bindings = HashMap::new();

                    // Match array elements
                    for (i, elem_pattern) in elements.iter().enumerate() {
                        if i >= arr.len() {
                            return Ok(None); // Not enough elements
                        }

                        if let Some(elem_bindings) = self.match_pattern(elem_pattern, &arr[i])? {
                            bindings.extend(elem_bindings);
                        } else {
                            return Ok(None);
                        }
                    }

                    // Handle rest pattern
                    if let Some(rest_name) = rest {
                        let rest_elements: Vec<Value> =
                            arr.iter().skip(elements.len()).cloned().collect();
                        bindings.insert(rest_name.clone(), Value::Array(rest_elements));
                    } else if arr.len() > elements.len() {
                        return Ok(None); // Too many elements and no rest pattern
                    }

                    Ok(Some(bindings))
                } else {
                    Ok(None)
                }
            }

            Pattern::Record { type_name, fields } => {
                if let Value::Record(record) = value {
                    if &record.type_name != type_name {
                        return Ok(None);
                    }

                    let mut bindings = HashMap::new();
                    for field_pattern in fields {
                        let field_value = match record.fields.get(&field_pattern.name) {
                            Some(value) => value.clone(),
                            None => return Ok(None),
                        };

                        if let Some(sub_pattern) = &field_pattern.pattern {
                            if let Some(sub_bindings) =
                                self.match_pattern(sub_pattern, &field_value)?
                            {
                                bindings.extend(sub_bindings);
                            } else {
                                return Ok(None);
                            }
                        } else {
                            bindings.insert(field_pattern.name.clone(), field_value);
                        }
                    }

                    Ok(Some(bindings))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn execute_entrain_body(&mut self, body: &[AstNode]) -> Result<Value, InterpreterError> {
        let mut last_value = Value::Null;

        for node in body {
            match node {
                AstNode::ExpressionStatement(expr) => {
                    last_value = self.evaluate_expression(expr)?;
                }
                _ if node.is_expression() => {
                    last_value = self.evaluate_expression(node)?;
                }
                _ => match self.execute_statement(node) {
                    Ok(()) => {
                        last_value = Value::Null;
                    }
                    Err(InterpreterError::Return(value)) => {
                        return Err(InterpreterError::Return(value));
                    }
                    Err(InterpreterError::BreakOutsideLoop) => {
                        return Err(InterpreterError::BreakOutsideLoop);
                    }
                    Err(InterpreterError::ContinueOutsideLoop) => {
                        return Err(InterpreterError::ContinueOutsideLoop);
                    }
                    Err(err) => return Err(err),
                },
            }
        }

        Ok(last_value)
    }

    fn evaluate_binary_op(
        &self,
        left: &Value,
        op: &str,
        right: &Value,
    ) -> Result<Value, InterpreterError> {
        let normalized = op.to_ascii_lowercase();

        match normalized.as_str() {
            "+" => {
                // If either operand is a string, perform string concatenation
                match (left, right) {
                    (Value::String(s1), Value::String(s2)) => {
                        Ok(Value::String(format!("{}{}", s1, s2)))
                    }
                    (Value::String(s), _) => Ok(Value::String(format!("{}{}", s, right))),
                    (_, Value::String(s)) => Ok(Value::String(format!("{}{}", left, s))),
                    _ => {
                        // Both are numeric, perform addition
                        Ok(Value::Number(left.to_number()? + right.to_number()?))
                    }
                }
            }
            "-" => Ok(Value::Number(left.to_number()? - right.to_number()?)),
            "*" => Ok(Value::Number(left.to_number()? * right.to_number()?)),
            "/" => Ok(Value::Number(left.to_number()? / right.to_number()?)),
            "%" => Ok(Value::Number(left.to_number()? % right.to_number()?)),
            "==" | "youarefeelingverysleepy" => Ok(Value::Boolean(self.values_equal(left, right))),
            "!=" | "youcannotresist" | "notsodeep" => {
                Ok(Value::Boolean(!self.values_equal(left, right)))
            }
            ">" | "lookatthewatch" => Ok(Value::Boolean(left.to_number()? > right.to_number()?)),
            "<" | "fallundermyspell" => Ok(Value::Boolean(left.to_number()? < right.to_number()?)),
            ">=" | "deeplygreater" | "youreyesaregettingheavy" => {
                Ok(Value::Boolean(left.to_number()? >= right.to_number()?))
            }
            "<=" | "deeplyless" | "goingdeeper" => {
                Ok(Value::Boolean(left.to_number()? <= right.to_number()?))
            }
            "&&" | "undermycontrol" => Ok(Value::Boolean(left.is_truthy() && right.is_truthy())),
            "||" | "resistanceisfutile" => {
                Ok(Value::Boolean(left.is_truthy() || right.is_truthy()))
            }
            _ => Err(InterpreterError::Runtime(format!(
                "Unknown binary operator: {}",
                op
            ))),
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    fn evaluate_call(
        &mut self,
        callee: &AstNode,
        arguments: &[AstNode],
    ) -> Result<Value, InterpreterError> {
        let args: Vec<Value> = arguments
            .iter()
            .map(|arg| self.evaluate_expression(arg))
            .collect::<Result<_, _>>()?;

        if let AstNode::Identifier(name) = callee {
            if let Some(result) = self.call_builtin(name, &args)? {
                return Ok(result);
            }

            let callee_value = self.get_variable(name)?;
            return self.invoke_callable(&callee_value, &args);
        }

        let callee_value = self.evaluate_expression(callee)?;
        self.invoke_callable(&callee_value, &args)
    }

    fn invoke_callable(
        &mut self,
        callee: &Value,
        args: &[Value],
    ) -> Result<Value, InterpreterError> {
        match callee {
            Value::Function(func) => self.call_function(func, args),
            Value::Session(session) => self.instantiate_session(session.clone(), args),
            Value::Null => Err(InterpreterError::Runtime(localized(
                "Cannot call null value",
                "Cannot call null value",
            ))),
            _ => Err(InterpreterError::Runtime(localized(
                "Value is not callable",
                "Value is not callable",
            ))),
        }
    }

    fn call_function(
        &mut self,
        function: &FunctionValue,
        args: &[Value],
    ) -> Result<Value, InterpreterError> {
        if function.parameters.len() != args.len() {
            return Err(InterpreterError::Runtime(localized(
                &format!(
                    "Expected {} arguments, received {}",
                    function.parameters.len(),
                    args.len()
                ),
                &format!(
                    "Expected {} arguments, received {}",
                    function.parameters.len(),
                    args.len()
                ),
            )));
        }

        let session_name = function.session_name().map(|name| name.to_string());
        if session_name.is_some() {
            self.execution_context.push(ExecutionContextFrame {
                session_name: session_name.clone(),
            });
        }

        self.push_scope();

        if let Some(instance) = function.this_binding() {
            self.define_variable(
                VariableStorage::Local,
                "this".to_string(),
                Value::Instance(instance),
                true,
            );
        }

        for (param, arg) in function.parameters.iter().zip(args.iter()) {
            self.define_variable(VariableStorage::Local, param.clone(), arg.clone(), false);
        }

        let result = (|| {
            for stmt in &function.body {
                self.execute_statement(stmt)?;
            }
            Ok(Value::Null)
        })();

        self.pop_scope();

        if session_name.is_some() {
            self.execution_context.pop();
        }

        match result {
            Err(InterpreterError::Return(val)) => Ok(val),
            Err(e) => Err(e),
            Ok(value) => Ok(value),
        }
    }

    fn build_session_definition(
        &mut self,
        name: &str,
        members: &[SessionMember],
    ) -> Result<Rc<SessionDefinition>, InterpreterError> {
        let mut definition = SessionDefinition::new(name.to_string());

        for member in members {
            match member {
                SessionMember::Field(field) => {
                    self.register_session_field(&mut definition, field)?
                }
                SessionMember::Method(method) => {
                    self.register_session_method(&mut definition, method)?
                }
            }
        }

        Ok(Rc::new(definition))
    }

    fn register_session_field(
        &self,
        definition: &mut SessionDefinition,
        field: &SessionField,
    ) -> Result<(), InterpreterError> {
        let initializer = field.initializer.as_ref().map(|expr| (**expr).clone());
        let field_def = SessionFieldDefinition {
            name: field.name.clone(),
            type_annotation: field.type_annotation.clone(),
            visibility: field.visibility,
            initializer: initializer.clone(),
        };

        if field.is_static {
            definition.push_static_field(field_def, initializer)
        } else {
            definition.push_field(field_def)
        }
    }

    fn register_session_method(
        &self,
        definition: &mut SessionDefinition,
        method: &SessionMethod,
    ) -> Result<(), InterpreterError> {
        if method.is_constructor && method.is_static {
            return Err(InterpreterError::Runtime(localized(
                &format!(
                    "Constructor in session '{}' cannot be static",
                    definition.name()
                ),
                &format!(
                    "Constructor in session '{}' cannot be static",
                    definition.name()
                ),
            )));
        }

        let parameters = method.parameters.iter().map(|p| p.name.clone()).collect();

        let method_def = SessionMethodDefinition {
            name: method.name.clone(),
            parameters,
            body: method.body.clone(),
            visibility: method.visibility,
            is_static: method.is_static,
            is_constructor: method.is_constructor,
        };

        definition.push_method(method_def)
    }

    fn initialize_static_fields(
        &mut self,
        session: Rc<SessionDefinition>,
    ) -> Result<(), InterpreterError> {
        if session.static_field_order().is_empty() {
            return Ok(());
        }

        self.execution_context.push(ExecutionContextFrame {
            session_name: Some(session.name().to_string()),
        });

        let result = (|| {
            for field_name in session.static_field_order().to_vec() {
                if let Some(initializer) = session.take_static_field_initializer(&field_name) {
                    let value = self.evaluate_expression(&initializer)?;
                    session.set_static_field_value(&field_name, value)?;
                }
            }
            Ok(())
        })();

        self.execution_context.pop();
        result
    }

    fn instantiate_session(
        &mut self,
        session: Rc<SessionDefinition>,
        args: &[Value],
    ) -> Result<Value, InterpreterError> {
        let instance = Rc::new(RefCell::new(SessionInstance::new(session.clone())));
        self.initialize_instance_fields(instance.clone())?;

        if let Some(constructor) = session.constructor() {
            if constructor.parameters.len() != args.len() {
                return Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Constructor for session '{}' expects {} arguments, received {}",
                        session.name(),
                        constructor.parameters.len(),
                        args.len()
                    ),
                    &format!(
                        "Constructor for session '{}' expects {} arguments, received {}",
                        session.name(),
                        constructor.parameters.len(),
                        args.len()
                    ),
                )));
            }

            let function = FunctionValue::new_session_member(
                session.name().to_string(),
                constructor,
                Some(instance.clone()),
            );
            self.call_function(&function, args)?;
        } else if !args.is_empty() {
            return Err(InterpreterError::Runtime(localized(
                &format!(
                    "Session '{}' does not define a constructor but arguments were provided",
                    session.name()
                ),
                &format!(
                    "Session '{}' does not define a constructor but arguments were provided",
                    session.name()
                ),
            )));
        }

        Ok(Value::Instance(instance))
    }

    fn initialize_instance_fields(
        &mut self,
        instance: Rc<RefCell<SessionInstance>>,
    ) -> Result<(), InterpreterError> {
        let definition = {
            let borrow = instance.borrow();
            borrow.definition()
        };

        if definition.field_order().is_empty() {
            return Ok(());
        }

        self.execution_context.push(ExecutionContextFrame {
            session_name: Some(definition.name().to_string()),
        });
        self.push_scope();
        self.define_variable(
            VariableStorage::Local,
            "this".to_string(),
            Value::Instance(instance.clone()),
            true,
        );

        let result = (|| {
            for field_name in definition.field_order().to_vec() {
                if let Some(field_def) = definition.get_field_definition(&field_name)
                    && let Some(initializer) = &field_def.initializer
                {
                    let value = self.evaluate_expression(initializer)?;
                    instance.borrow_mut().set_field(&field_name, value);
                }
            }
            Ok(())
        })();

        self.pop_scope();
        self.execution_context.pop();
        result
    }

    fn resolve_member_value(
        &mut self,
        target: Value,
        property: &str,
    ) -> Result<Value, InterpreterError> {
        match target {
            Value::Instance(instance_rc) => {
                let definition = {
                    let borrow = instance_rc.borrow();
                    borrow.definition()
                };

                if let Some(method_def) = definition.get_method_definition(property) {
                    self.ensure_visibility(
                        method_def.visibility,
                        definition.name(),
                        "method",
                        property,
                    )?;
                    let function = FunctionValue::new_session_member(
                        definition.name().to_string(),
                        method_def,
                        Some(instance_rc.clone()),
                    );
                    return Ok(Value::Function(function));
                }

                if let Some(field_def) = definition.get_field_definition(property) {
                    self.ensure_visibility(
                        field_def.visibility,
                        definition.name(),
                        "field",
                        property,
                    )?;
                    return Ok(instance_rc
                        .borrow()
                        .get_field(property)
                        .unwrap_or(Value::Null));
                }

                if let Some(static_field) = definition.get_static_field_snapshot(property) {
                    self.ensure_visibility(
                        static_field.definition.visibility,
                        definition.name(),
                        "field",
                        property,
                    )?;
                    return Ok(static_field.value);
                }

                if let Some(static_method) = definition.get_static_method_definition(property) {
                    self.ensure_visibility(
                        static_method.visibility,
                        definition.name(),
                        "method",
                        property,
                    )?;
                    let function = FunctionValue::new_session_member(
                        definition.name().to_string(),
                        static_method,
                        None,
                    );
                    return Ok(Value::Function(function));
                }

                Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Session instance of '{}' has no member '{}'",
                        definition.name(),
                        property
                    ),
                    &format!(
                        "Session instance of '{}' has no member '{}'",
                        definition.name(),
                        property
                    ),
                )))
            }
            Value::Session(session_rc) => {
                if let Some(static_field) = session_rc.get_static_field_snapshot(property) {
                    self.ensure_visibility(
                        static_field.definition.visibility,
                        session_rc.name(),
                        "field",
                        property,
                    )?;
                    return Ok(static_field.value);
                }

                if let Some(method_def) = session_rc.get_static_method_definition(property) {
                    self.ensure_visibility(
                        method_def.visibility,
                        session_rc.name(),
                        "method",
                        property,
                    )?;
                    let function = FunctionValue::new_session_member(
                        session_rc.name().to_string(),
                        method_def,
                        None,
                    );
                    return Ok(Value::Function(function));
                }

                Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Session '{}' has no static member '{}'",
                        session_rc.name(),
                        property
                    ),
                    &format!(
                        "Session '{}' has no static member '{}'",
                        session_rc.name(),
                        property
                    ),
                )))
            }
            Value::Record(record) => {
                // Access field from record
                if let Some(field_value) = record.fields.get(property) {
                    Ok(field_value.clone())
                } else {
                    Err(InterpreterError::Runtime(format!(
                        "Record of type '{}' has no field '{}'",
                        record.type_name, property
                    )))
                }
            }
            other => Err(InterpreterError::Runtime(localized(
                &format!("Cannot access member '{}' on value '{}'", property, other),
                &format!("Cannot access member '{}' on value '{}'", property, other),
            ))),
        }
    }

    fn assign_member_value(
        &mut self,
        target: Value,
        property: &str,
        value: Value,
    ) -> Result<(), InterpreterError> {
        match target {
            Value::Instance(instance_rc) => {
                let definition = {
                    let borrow = instance_rc.borrow();
                    borrow.definition()
                };

                if let Some(field_def) = definition.get_field_definition(property) {
                    self.ensure_visibility(
                        field_def.visibility,
                        definition.name(),
                        "field",
                        property,
                    )?;
                    instance_rc.borrow_mut().set_field(property, value);
                    return Ok(());
                }

                if definition.get_method_definition(property).is_some()
                    || definition.get_static_method_definition(property).is_some()
                {
                    return Err(InterpreterError::Runtime(localized(
                        &format!("Cannot assign to method '{}'", property),
                        &format!("Cannot assign to method '{}'", property),
                    )));
                }

                if definition.get_static_field_snapshot(property).is_some() {
                    return Err(InterpreterError::Runtime(localized(
                        &format!(
                            "Assign static field '{}' through session '{}', not an instance",
                            property,
                            definition.name()
                        ),
                        &format!(
                            "Assign static field '{}' through session '{}', not an instance",
                            property,
                            definition.name()
                        ),
                    )));
                }

                Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Session instance of '{}' has no field '{}'",
                        definition.name(),
                        property
                    ),
                    &format!(
                        "Session instance of '{}' has no field '{}'",
                        definition.name(),
                        property
                    ),
                )))
            }
            Value::Session(session_rc) => {
                if let Some(static_field) = session_rc.get_static_field_snapshot(property) {
                    self.ensure_visibility(
                        static_field.definition.visibility,
                        session_rc.name(),
                        "field",
                        property,
                    )?;
                    session_rc.set_static_field_value(property, value)?;
                    return Ok(());
                }

                if session_rc.get_static_method_definition(property).is_some() {
                    return Err(InterpreterError::Runtime(localized(
                        &format!("Cannot assign to static method '{}'", property),
                        &format!("Cannot assign to static method '{}'", property),
                    )));
                }

                Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Session '{}' has no static field '{}'",
                        session_rc.name(),
                        property
                    ),
                    &format!(
                        "Session '{}' has no static field '{}'",
                        session_rc.name(),
                        property
                    ),
                )))
            }
            _ => Err(InterpreterError::Runtime(localized(
                "Assignment target is not a session member",
                "Assignment target is not a session member",
            ))),
        }
    }

    fn ensure_visibility(
        &self,
        visibility: SessionVisibility,
        session_name: &str,
        member_kind: &str,
        member_name: &str,
    ) -> Result<(), InterpreterError> {
        if visibility == SessionVisibility::Private && !self.is_access_allowed(session_name) {
            return Err(InterpreterError::Runtime(localized(
                &format!(
                    "Access denied to private {} '{}' of session '{}'",
                    member_kind, member_name, session_name
                ),
                &format!(
                    "Access denied to private {} '{}' of session '{}'",
                    member_kind, member_name, session_name
                ),
            )));
        }
        Ok(())
    }

    fn is_access_allowed(&self, session_name: &str) -> bool {
        self.execution_context
            .iter()
            .rev()
            .find_map(|frame| frame.session_name.as_deref())
            == Some(session_name)
    }
    fn push_scope(&mut self) {
        self.locals.push(HashMap::new());
        self.const_locals.push(HashSet::new());
    }

    fn pop_scope(&mut self) {
        self.locals.pop();
        self.const_locals.pop();
    }

    fn define_variable(
        &mut self,
        storage: VariableStorage,
        name: String,
        value: Value,
        is_constant: bool,
    ) {
        match storage {
            VariableStorage::SharedTrance => {
                self.shared.insert(name.clone(), value);
                if is_constant {
                    self.const_globals.insert(name);
                } else {
                    self.const_globals.remove(&name);
                }
            }
            VariableStorage::Local => {
                if let Some(scope) = self.locals.last_mut() {
                    scope.insert(name.clone(), value);
                    if let Some(const_scope) = self.const_locals.last_mut() {
                        if is_constant {
                            const_scope.insert(name);
                        } else {
                            const_scope.remove(&name);
                        }
                    }
                } else {
                    self.globals.insert(name.clone(), value);
                    if is_constant {
                        self.const_globals.insert(name);
                    } else {
                        self.const_globals.remove(&name);
                    }
                }
            }
        }
    }

    fn set_variable(
        &mut self,
        name: String,
        value: Value,
        scope_hint: ScopeLayer,
    ) -> Result<(), InterpreterError> {
        let check_const = |is_const: bool| -> Result<(), InterpreterError> {
            if is_const {
                return Err(InterpreterError::Runtime(localized(
                    &format!("Cannot reassign constant variable '{}'", name),
                    &format!("Cannot reassign constant variable '{}'", name),
                )));
            }
            Ok(())
        };

        match scope_hint {
            ScopeLayer::Local => {
                if let Some((scope, consts)) = self.locals.last_mut().zip(self.const_locals.last())
                    && let Some(slot) = scope.get_mut(&name)
                {
                    check_const(consts.contains(&name))?;
                    *slot = value;
                    return Ok(());
                }
            }
            ScopeLayer::Global => {
                if self.globals.contains_key(&name) {
                    check_const(self.const_globals.contains(&name))?;
                    self.globals.insert(name, value);
                    return Ok(());
                }
            }
            ScopeLayer::Shared => {
                if self.shared.contains_key(&name) {
                    check_const(self.const_globals.contains(&name))?;
                    self.shared.insert(name, value);
                    return Ok(());
                }
            }
        }

        for idx in (0..self.locals.len()).rev() {
            if self.locals[idx].contains_key(&name) {
                let is_const = self
                    .const_locals
                    .get(idx)
                    .map(|set| set.contains(&name))
                    .unwrap_or(false);
                check_const(is_const)?;
                self.locals[idx].insert(name.clone(), value);
                return Ok(());
            }
        }

        if self.globals.contains_key(&name) {
            check_const(self.const_globals.contains(&name))?;
            self.globals.insert(name, value);
            return Ok(());
        }

        if self.shared.contains_key(&name) {
            check_const(self.const_globals.contains(&name))?;
            self.shared.insert(name, value);
            return Ok(());
        }

        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name.clone(), value);
            return Ok(());
        }

        self.globals.insert(name.clone(), value);
        Ok(())
    }

    fn resolve_assignment_scope(&self, name: &str) -> ScopeLayer {
        if self
            .locals
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
        {
            ScopeLayer::Local
        } else if self.shared.contains_key(name) {
            ScopeLayer::Shared
        } else if self.globals.contains_key(name) {
            ScopeLayer::Global
        } else {
            ScopeLayer::Local
        }
    }

    fn get_variable(&self, name: &str) -> Result<Value, InterpreterError> {
        // Search in local scopes (from innermost to outermost)
        for scope in self.locals.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }

        if let Some(value) = self.globals.get(name) {
            return Ok(value.clone());
        }

        if let Some(value) = self.shared.get(name) {
            return Ok(value.clone());
        }

        Err(InterpreterError::UndefinedVariable(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hypnoscript_lexer_parser::{Lexer, Parser};

    #[test]
    fn test_simple_program() {
        let source = r#"
Focus {
    induce x: number = 42;
    induce y: number = 10;
    induce sum: number = x + y;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens.clone());
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("parse error: {err}");
                for token in tokens {
                    eprintln!("token: {:?}", token);
                }
                panic!("failed to parse test program");
            }
        };

        let mut interpreter = Interpreter::new();
        let result = interpreter.execute_program(ast);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_statement() {
        let source = r#"
Focus {
    induce x: number = 10;
    if (x > 5) deepFocus {
        induce result: number = 1;
    }
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens.clone());
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("parse error: {err}");
                for token in tokens {
                    eprintln!("token: {:?}", token);
                }
                panic!("failed to parse test program");
            }
        };

        let mut interpreter = Interpreter::new();
        let result = interpreter.execute_program(ast);
        assert!(result.is_ok());
    }

    #[test]
    fn test_session_constructor_and_methods() {
        let source = r#"
Focus {
    session Counter {
        expose value: number;

        suggestion constructor(initial: number) {
            this.value = initial;
        }

        suggestion inc() {
            this.value = this.value + 1;
        }

        suggestion current(): number {
            awaken this.value;
        }
    }

    induce counter = Counter(5);
    counter.inc();
    counter.inc();
    induce current: number = counter.current();
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut interpreter = Interpreter::new();
        if let Err(err) = interpreter.execute_program(ast) {
            panic!("interpreter error: {err:?}");
        }

        let current = interpreter.get_variable("current").unwrap();
        assert_eq!(current, Value::Number(7.0));
    }

    #[test]
    fn test_hypnotic_operator_synonyms_execution() {
        let source = r#"
Focus {
    induce a: number = 10;
    induce b: number = 5;

    induce eq: boolean = a youAreFeelingVerySleepy b;
    induce neq: boolean = a youCannotResist b;
    induce ge: boolean = a yourEyesAreGettingHeavy 9;
    induce le: boolean = b goingDeeper 4;
    induce both: boolean = ge underMyControl neq;
    induce either: boolean = le resistanceIsFutile eq;
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut interpreter = Interpreter::new();
        if let Err(err) = interpreter.execute_program(ast) {
            panic!("interpreter error: {err:?}");
        }

        assert_eq!(
            interpreter.get_variable("eq").unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            interpreter.get_variable("neq").unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            interpreter.get_variable("ge").unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            interpreter.get_variable("le").unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            interpreter.get_variable("both").unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            interpreter.get_variable("either").unwrap(),
            Value::Boolean(false)
        );
    }

    #[test]
    fn test_private_field_access_rejected() {
        let source = r#"
Focus {
    session Account {
        conceal balance: number;

        suggestion constructor(amount: number) {
            this.balance = amount;
        }

        suggestion read(): number {
            awaken this.balance;
        }
    }

    induce account = Account(100);
    // The following line should fail because balance is private
    induce leaked = account.balance;
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut interpreter = Interpreter::new();
        let result = interpreter.execute_program(ast);
        assert!(matches!(
            result,
            Err(InterpreterError::Runtime(message)) if message.contains("Access denied")
        ));
    }

    #[test]
    fn test_static_field_and_method() {
        let source = r#"
Focus {
    session Config {
        dominant expose version: string = "1.0";

        dominant suggestion setVersion(newVersion: string) {
            Config.version = newVersion;
        }
    }

    Config.setVersion("2.5");
    induce activeVersion: string = Config.version;
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut interpreter = Interpreter::new();
        interpreter.execute_program(ast).unwrap();

        let result = interpreter.get_variable("activeVersion").unwrap();
        assert_eq!(result, Value::String("2.5".to_string()));
    }

    #[test]
    fn test_entrain_record_pattern_matching_with_guard() {
        let source = r#"
Focus {
    tranceify HypnoGuest {
        name: string;
        isInTrance: boolean;
        depth: number;
    }

    entrance {
        induce guest = HypnoGuest {
            name: "Luna",
            isInTrance: true,
            depth: 7
        };

        induce status: string = entrain guest {
            when HypnoGuest { name: alias } => alias;
            otherwise => "Unknown";
        };
    }
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens.clone());
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("parse error: {err}");
                for token in tokens {
                    eprintln!("token: {:?}", token);
                }
                panic!("failed to parse test program");
            }
        };

        let mut interpreter = Interpreter::new();
        if let Err(err) = interpreter.execute_program(ast) {
            panic!("interpreter error: {err:?}");
        }

        let status = interpreter.get_variable("status").unwrap();
        assert_eq!(status, Value::String("Luna".to_string()));
    }

    /// Run a program and return the interpreter for state inspection.
    fn run_program(source: &str) -> Interpreter {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.execute_program(ast);
        assert!(result.is_ok(), "execution failed: {:?}", result.err());
        interpreter
    }

    #[test]
    fn test_labeled_break_exits_outer_loop() {
        let interpreter = run_program(
            r#"
Focus {
    induce hits: number = 0;
    entrance {
        outer: loop (induce i: number = 0; i < 5; i = i + 1) {
            loop (induce j: number = 0; j < 5; j = j + 1) {
                hits = hits + 1;
                if (hits == 3) {
                    snap outer;
                }
            }
        }
    }
} Relax
"#,
        );
        assert_eq!(
            interpreter.debug_globals().get("hits"),
            Some(&Value::Number(3.0)),
            "labeled break must exit both loops"
        );
    }

    #[test]
    fn test_labeled_continue_advances_outer_loop() {
        let interpreter = run_program(
            r#"
Focus {
    induce outerRuns: number = 0;
    induce innerRuns: number = 0;
    entrance {
        outer: loop (induce i: number = 0; i < 3; i = i + 1) {
            outerRuns = outerRuns + 1;
            loop (induce j: number = 0; j < 3; j = j + 1) {
                innerRuns = innerRuns + 1;
                sink outer;
            }
            outerRuns = outerRuns + 100;
        }
    }
} Relax
"#,
        );
        let globals = interpreter.debug_globals();
        assert_eq!(
            globals.get("outerRuns"),
            Some(&Value::Number(3.0)),
            "outer body after inner loop must be skipped"
        );
        assert_eq!(
            globals.get("innerRuns"),
            Some(&Value::Number(3.0)),
            "inner loop must run once per outer iteration"
        );
    }

    #[test]
    fn test_labeled_break_with_unknown_label_errors() {
        let source = r#"
Focus {
    entrance {
        loop (induce i: number = 0; i < 3; i = i + 1) {
            snap nowhere;
        }
    }
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.execute_program(ast);
        assert!(result.is_err(), "unknown label must be a runtime error");
        assert!(result.unwrap_err().to_string().contains("nowhere"));
    }

    #[test]
    fn test_null_literal_and_nullish_operators() {
        let interpreter = run_program(
            r#"
Focus {
    induce value: number = 0;
    induce state: string = "";
    entrance {
        induce maybe: number? = null;
        value = maybe lucidFallback 42;
        state = entrain maybe {
            when null => "empty";
            otherwise => "filled";
        };
    }
} Relax
"#,
        );
        let globals = interpreter.debug_globals();
        assert_eq!(
            globals.get("value"),
            Some(&Value::Number(42.0)),
            "lucidFallback must supply the default"
        );
        assert_eq!(
            globals.get("state"),
            Some(&Value::String("empty".to_string())),
            "null pattern must match"
        );
    }

    #[test]
    fn test_entrain_record_pattern_default_scope_cleanup() {
        let source = r#"
Focus {
    tranceify HypnoGuest {
        depth: number;
    }

    entrance {
        induce guest = HypnoGuest { depth: 2 };

        induce outcome = entrain guest {
            when HypnoGuest { depth: stage } => stage;
            otherwise => "fallback";
        };
    }
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens.clone());
        let ast = match parser.parse_program() {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("parse error: {err}");
                for token in tokens {
                    eprintln!("token: {:?}", token);
                }
                panic!("failed to parse test program");
            }
        };

        let mut interpreter = Interpreter::new();
        if let Err(err) = interpreter.execute_program(ast) {
            panic!("interpreter error: {err:?}");
        }

        let outcome = interpreter.get_variable("outcome").unwrap();
        assert_eq!(outcome, Value::Number(2.0));
        assert!(matches!(
            interpreter.get_variable("stage"),
            Err(InterpreterError::UndefinedVariable(_))
        ));
    }

    // === Debug Mode Tests ===

    #[test]
    fn test_enable_debug_mode() {
        let source = "Focus { induce x = 42; } Relax";
        let mut interpreter = Interpreter::new();

        assert!(!interpreter.is_debug_mode());
        interpreter.enable_debug_mode(source);
        assert!(interpreter.is_debug_mode());

        interpreter.disable_debug_mode();
        assert!(!interpreter.is_debug_mode());
    }

    #[test]
    fn test_breakpoint_management() {
        let source = "Focus { induce x = 42; } Relax";
        let mut interpreter = Interpreter::new();
        interpreter.enable_debug_mode(source);

        // Set breakpoints
        assert!(interpreter.set_breakpoint(10));
        assert!(interpreter.set_breakpoint(20));

        // Check breakpoints
        assert!(interpreter.has_breakpoint(10));
        assert!(interpreter.has_breakpoint(20));
        assert!(!interpreter.has_breakpoint(15));

        // Get all breakpoints
        let breakpoints = interpreter.breakpoints();
        assert_eq!(breakpoints.len(), 2);
        assert!(breakpoints.contains(&10));
        assert!(breakpoints.contains(&20));

        // Remove breakpoint
        assert!(interpreter.remove_breakpoint(10));
        assert!(!interpreter.has_breakpoint(10));
        assert!(interpreter.has_breakpoint(20));

        // Clear all breakpoints
        interpreter.clear_breakpoints();
        assert!(interpreter.breakpoints().is_empty());
    }

    #[test]
    fn test_step_mode() {
        use crate::debug::StepMode;

        let source = "Focus { induce x = 42; } Relax";
        let mut interpreter = Interpreter::new();
        interpreter.enable_debug_mode(source);

        assert_eq!(interpreter.step_mode(), StepMode::None);

        interpreter.set_step_mode(StepMode::StepInto);
        assert_eq!(interpreter.step_mode(), StepMode::StepInto);

        interpreter.set_step_mode(StepMode::StepOver);
        assert_eq!(interpreter.step_mode(), StepMode::StepOver);

        interpreter.set_step_mode(StepMode::Continue);
        assert_eq!(interpreter.step_mode(), StepMode::Continue);
    }

    #[test]
    fn test_watch_expressions() {
        let source = "Focus { induce x = 42; } Relax";
        let mut interpreter = Interpreter::new();
        interpreter.enable_debug_mode(source);

        // Add watches
        let id1 = interpreter.add_watch("x".to_string());
        let id2 = interpreter.add_watch("y + z".to_string());

        assert!(id1.is_some());
        assert!(id2.is_some());

        // Remove watch
        assert!(interpreter.remove_watch(id1.unwrap()));
        assert!(!interpreter.remove_watch(id1.unwrap())); // Already removed
    }

    #[test]
    fn test_debug_variable_inspection() {
        let source = r#"
Focus {
    induce x: number = 42;
    induce y: string = "hello";
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut interpreter = Interpreter::new();
        interpreter.enable_debug_mode(source);
        interpreter.execute_program(ast).unwrap();

        // Check globals
        let globals = interpreter.debug_globals();
        assert!(globals.contains_key("x"));
        assert!(globals.contains_key("y"));
        assert_eq!(globals.get("x").unwrap(), &Value::Number(42.0));
        assert_eq!(
            globals.get("y").unwrap(),
            &Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_debug_summary() {
        let source = "Focus { induce x = 42; } Relax";
        let mut interpreter = Interpreter::new();

        // Without debug mode
        let summary = interpreter.debug_summary();
        assert!(summary.contains("not enabled"));

        // With debug mode
        interpreter.enable_debug_mode(source);
        interpreter.set_breakpoint(10);
        let summary = interpreter.debug_summary();
        assert!(summary.contains("breakpoints"));
        assert!(summary.contains("10"));
    }

    #[test]
    fn test_debug_mode_no_source() {
        let mut interpreter = Interpreter::new();
        interpreter.enable_debug_mode_no_source();

        assert!(interpreter.is_debug_mode());
        assert!(interpreter.set_breakpoint(5));
        assert!(interpreter.has_breakpoint(5));
    }

    #[test]
    fn test_debug_with_execution() {
        let source = r#"
Focus {
    induce x: number = 10;
    induce y: number = 20;
    induce sum: number = x + y;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut interpreter = Interpreter::new();
        interpreter.enable_debug_mode(source);
        interpreter.set_breakpoint(3); // Set breakpoint at line 3

        // Execute should still work
        let result = interpreter.execute_program(ast);
        assert!(result.is_ok());

        // Verify execution completed correctly
        assert_eq!(
            interpreter.get_variable("sum").unwrap(),
            Value::Number(30.0)
        );
    }
}
