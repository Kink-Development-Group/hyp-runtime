use hypnoscript_lexer_parser::ast::{
    AstNode, Pattern, SessionField, SessionMember, SessionMethod, SessionVisibility,
};
use hypnoscript_runtime::{
    ArrayBuiltins, CoreBuiltins, FileBuiltins, HashingBuiltins, MathBuiltins, StatisticsBuiltins,
    StringBuiltins, SystemBuiltins, TimeBuiltins, ValidationBuiltins,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Runtime error: {0}")]
    Runtime(String),
    #[error("Break statement outside of loop")]
    BreakOutsideLoop,
    #[error("Continue statement outside of loop")]
    ContinueOutsideLoop,
    #[error("Return from function: {0:?}")]
    Return(Value),
    #[error("Variable '{0}' not found")]
    UndefinedVariable(String),
    #[error("Type error: {0}")]
    TypeError(String),
}

/// Provide a simple locale-aware message while we prepare full i18n plumbing.
fn localized(en: &str, de: &str) -> String {
    format!("{} (DE: {})", en, de)
}

/// Represents a callable suggestion within the interpreter.
#[derive(Debug, Clone)]
pub struct FunctionValue {
    name: String,
    parameters: Vec<String>,
    body: Vec<AstNode>,
    this_binding: Option<Rc<RefCell<SessionInstance>>>,
    session_name: Option<String>,
    is_static: bool,
    is_constructor: bool,
}

impl FunctionValue {
    fn new_global(name: String, parameters: Vec<String>, body: Vec<AstNode>) -> Self {
        Self {
            name,
            parameters,
            body,
            this_binding: None,
            session_name: None,
            is_static: false,
            is_constructor: false,
        }
    }

    fn new_session_member(
        session_name: String,
        method: &SessionMethodDefinition,
        this_binding: Option<Rc<RefCell<SessionInstance>>>,
    ) -> Self {
        Self {
            name: format!("{}::{}", session_name, method.name),
            parameters: method.parameters.clone(),
            body: method.body.clone(),
            this_binding,
            session_name: Some(session_name),
            is_static: method.is_static,
            is_constructor: method.is_constructor,
        }
    }

    fn this_binding(&self) -> Option<Rc<RefCell<SessionInstance>>> {
        self.this_binding.as_ref().map(Rc::clone)
    }

    fn session_name(&self) -> Option<&str> {
        self.session_name.as_deref()
    }
}

impl PartialEq for FunctionValue {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.parameters == other.parameters
            && self.body == other.body
            && self.session_name == other.session_name
            && self.is_static == other.is_static
            && self.is_constructor == other.is_constructor
    }
}

impl Eq for FunctionValue {}

/// Definition of a session field (instance scope).
#[derive(Debug, Clone)]
struct SessionFieldDefinition {
    name: String,
    #[allow(dead_code)]
    type_annotation: Option<String>,
    visibility: SessionVisibility,
    initializer: Option<AstNode>,
}

/// Definition of a session method.
#[derive(Debug, Clone)]
struct SessionMethodDefinition {
    name: String,
    parameters: Vec<String>,
    body: Vec<AstNode>,
    visibility: SessionVisibility,
    is_static: bool,
    is_constructor: bool,
}

/// Runtime data for a static field, including its initializer AST.
#[derive(Debug, Clone)]
struct SessionStaticField {
    definition: SessionFieldDefinition,
    initializer: Option<AstNode>,
    value: Value,
}

/// Stores metadata and static members for a session (class-like construct).
#[derive(Debug)]
pub struct SessionDefinition {
    name: String,
    fields: HashMap<String, SessionFieldDefinition>,
    field_order: Vec<String>,
    methods: HashMap<String, SessionMethodDefinition>,
    static_methods: HashMap<String, SessionMethodDefinition>,
    static_fields: RefCell<HashMap<String, SessionStaticField>>,
    static_field_order: Vec<String>,
    constructor: Option<SessionMethodDefinition>,
}

impl SessionDefinition {
    fn new(name: String) -> Self {
        Self {
            name,
            fields: HashMap::new(),
            field_order: Vec::new(),
            methods: HashMap::new(),
            static_methods: HashMap::new(),
            static_fields: RefCell::new(HashMap::new()),
            static_field_order: Vec::new(),
            constructor: None,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn push_field(&mut self, field: SessionFieldDefinition) -> Result<(), InterpreterError> {
        if self.fields.contains_key(&field.name)
            || self.static_fields.borrow().contains_key(&field.name)
        {
            return Err(InterpreterError::Runtime(localized(
                &format!(
                    "Duplicate session field '{}' in session '{}'",
                    field.name, self.name
                ),
                &format!("Doppeltes Feld '{}' in Session '{}'", field.name, self.name),
            )));
        }
        self.field_order.push(field.name.clone());
        self.fields.insert(field.name.clone(), field);
        Ok(())
    }

    fn push_static_field(
        &mut self,
        field: SessionFieldDefinition,
        initializer: Option<AstNode>,
    ) -> Result<(), InterpreterError> {
        if self.fields.contains_key(&field.name)
            || self.static_fields.borrow().contains_key(&field.name)
        {
            return Err(InterpreterError::Runtime(localized(
                &format!(
                    "Duplicate session field '{}' in session '{}'",
                    field.name, self.name
                ),
                &format!("Doppeltes Feld '{}' in Session '{}'", field.name, self.name),
            )));
        }
        self.static_field_order.push(field.name.clone());
        self.static_fields.borrow_mut().insert(
            field.name.clone(),
            SessionStaticField {
                definition: field,
                initializer,
                value: Value::Null,
            },
        );
        Ok(())
    }

    fn push_method(&mut self, method: SessionMethodDefinition) -> Result<(), InterpreterError> {
        if method.is_constructor {
            if self.constructor.is_some() {
                return Err(InterpreterError::Runtime(localized(
                    &format!("Multiple constructors declared in session '{}'", self.name),
                    &format!(
                        "Mehrere Konstruktoren in Session '{}' deklariert",
                        self.name
                    ),
                )));
            }
            self.constructor = Some(method);
            return Ok(());
        }

        if method.is_static {
            if self.static_methods.contains_key(&method.name) {
                return Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Duplicate static method '{}' in session '{}'",
                        method.name, self.name
                    ),
                    &format!(
                        "Doppelte statische Methode '{}' in Session '{}'",
                        method.name, self.name
                    ),
                )));
            }
            self.static_methods.insert(method.name.clone(), method);
        } else {
            if self.methods.contains_key(&method.name) {
                return Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Duplicate method '{}' in session '{}'",
                        method.name, self.name
                    ),
                    &format!(
                        "Doppelte Methode '{}' in Session '{}'",
                        method.name, self.name
                    ),
                )));
            }
            self.methods.insert(method.name.clone(), method);
        }
        Ok(())
    }

    fn get_field_definition(&self, name: &str) -> Option<&SessionFieldDefinition> {
        self.fields.get(name)
    }

    fn get_method_definition(&self, name: &str) -> Option<&SessionMethodDefinition> {
        self.methods.get(name)
    }

    fn get_static_method_definition(&self, name: &str) -> Option<&SessionMethodDefinition> {
        self.static_methods.get(name)
    }

    fn get_static_field_snapshot(&self, name: &str) -> Option<SessionStaticField> {
        self.static_fields.borrow().get(name).cloned()
    }

    fn set_static_field_value(&self, name: &str, value: Value) -> Result<(), InterpreterError> {
        let mut fields = self.static_fields.borrow_mut();
        match fields.get_mut(name) {
            Some(field) => {
                field.value = value;
                Ok(())
            }
            None => Err(InterpreterError::Runtime(localized(
                &format!(
                    "Static field '{}' not found on session '{}'",
                    name, self.name
                ),
                &format!(
                    "Statisches Feld '{}' nicht in Session '{}' gefunden",
                    name, self.name
                ),
            ))),
        }
    }

    fn take_static_field_initializer(&self, name: &str) -> Option<AstNode> {
        self.static_fields
            .borrow()
            .get(name)
            .and_then(|field| field.initializer.clone())
    }

    fn field_order(&self) -> &[String] {
        &self.field_order
    }

    fn static_field_order(&self) -> &[String] {
        &self.static_field_order
    }

    fn constructor(&self) -> Option<&SessionMethodDefinition> {
        self.constructor.as_ref()
    }
}

/// Runtime representation of a session instance.
#[derive(Debug)]
pub struct SessionInstance {
    definition: Rc<SessionDefinition>,
    field_values: HashMap<String, Value>,
}

impl SessionInstance {
    fn new(definition: Rc<SessionDefinition>) -> Self {
        let mut field_values = HashMap::new();
        for name in definition.field_order() {
            field_values.insert(name.clone(), Value::Null);
        }
        Self {
            definition,
            field_values,
        }
    }

    fn definition(&self) -> Rc<SessionDefinition> {
        Rc::clone(&self.definition)
    }

    fn definition_name(&self) -> &str {
        self.definition.name()
    }

    fn get_field(&self, name: &str) -> Option<Value> {
        self.field_values.get(name).cloned()
    }

    fn set_field(&mut self, name: &str, value: Value) {
        self.field_values.insert(name.to_string(), value);
    }
}

#[derive(Debug, Clone)]
struct ExecutionContextFrame {
    session_name: Option<String>,
}

/// Simple Promise/Future wrapper for async operations
#[derive(Debug, Clone)]
pub struct Promise {
    /// The resolved value (if completed)
    value: Option<Value>,
    /// Whether the promise is resolved
    resolved: bool,
}

impl Promise {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            value: None,
            resolved: false,
        }
    }

    #[allow(dead_code)]
    fn resolve(value: Value) -> Self {
        Self {
            value: Some(value),
            resolved: true,
        }
    }

    fn is_resolved(&self) -> bool {
        self.resolved
    }

    fn get_value(&self) -> Option<Value> {
        self.value.clone()
    }
}

/// Runtime value in HypnoScript
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Function(FunctionValue),
    Session(Rc<SessionDefinition>),
    Instance(Rc<RefCell<SessionInstance>>),
    Promise(Rc<RefCell<Promise>>),
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Function(fa), Value::Function(fb)) => fa == fb,
            (Value::Session(sa), Value::Session(sb)) => Rc::ptr_eq(sa, sb),
            (Value::Instance(ia), Value::Instance(ib)) => Rc::ptr_eq(ia, ib),
            (Value::Promise(pa), Value::Promise(pb)) => Rc::ptr_eq(pa, pb),
            _ => false,
        }
    }
}

impl Eq for Value {}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Function(_) | Value::Session(_) | Value::Instance(_) | Value::Promise(_) => true,
        }
    }

    pub fn to_number(&self) -> Result<f64, InterpreterError> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::String(s) => s.parse::<f64>().map_err(|_| {
                InterpreterError::TypeError(format!("Cannot convert '{}' to number", s))
            }),
            Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            _ => Err(InterpreterError::TypeError(
                "Cannot convert to number".to_string(),
            )),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
            Value::Function(func) => write!(f, "<function {}>", func.name),
            Value::Session(session) => write!(f, "<session {}>", session.name()),
            Value::Instance(instance) => {
                let name = instance.borrow().definition_name().to_string();
                write!(f, "<session-instance {}>", name)
            }
            Value::Promise(promise) => {
                if promise.borrow().is_resolved() {
                    write!(f, "<promise resolved>")
                } else {
                    write!(f, "<promise pending>")
                }
            }
        }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
    execution_context: Vec<ExecutionContextFrame>,

    /// Optional async runtime for true async execution
    pub async_runtime: Option<std::sync::Arc<crate::async_runtime::AsyncRuntime>>,

    /// Optional channel registry for inter-task communication
    pub channel_registry: Option<std::sync::Arc<crate::channel_system::ChannelRegistry>>,
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
            locals: Vec::new(),
            execution_context: Vec::new(),
            async_runtime: None,
            channel_registry: None,
        }
    }

    /// Create interpreter with async runtime support
    pub fn with_async_runtime() -> Result<Self, InterpreterError> {
        let runtime = crate::async_runtime::AsyncRuntime::new()
            .map_err(|e| InterpreterError::Runtime(format!("Failed to create async runtime: {}", e)))?;
        let registry = crate::channel_system::ChannelRegistry::new();

        Ok(Self {
            globals: HashMap::new(),
            locals: Vec::new(),
            execution_context: Vec::new(),
            async_runtime: Some(std::sync::Arc::new(runtime)),
            channel_registry: Some(std::sync::Arc::new(registry)),
        })
    }

    /// Enable async runtime for existing interpreter
    pub fn enable_async_runtime(&mut self) -> Result<(), InterpreterError> {
        if self.async_runtime.is_none() {
            let runtime = crate::async_runtime::AsyncRuntime::new()
                .map_err(|e| InterpreterError::Runtime(format!("Failed to create async runtime: {}", e)))?;
            let registry = crate::channel_system::ChannelRegistry::new();

            self.async_runtime = Some(std::sync::Arc::new(runtime));
            self.channel_registry = Some(std::sync::Arc::new(registry));
        }
        Ok(())
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
                is_constant: _,
            } => {
                let value = if let Some(init) = initializer {
                    self.evaluate_expression(init)?
                } else {
                    Value::Null
                };
                self.set_variable(name.clone(), value);
                Ok(())
            }

            AstNode::AnchorDeclaration { name, source } => {
                // Anchor saves the current value of a variable
                let value = self.evaluate_expression(source)?;
                self.set_variable(name.clone(), value);
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
                self.set_variable(name.clone(), Value::Function(func));
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
                self.set_variable(name.clone(), Value::Function(func));
                Ok(())
            }

            AstNode::SessionDeclaration { name, members } => {
                let session = self.build_session_definition(name, members)?;
                self.set_variable(name.clone(), Value::Session(session.clone()));
                self.initialize_static_fields(session)?;
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
                CoreBuiltins::whisper(&format!("[DEBUG] {}", value.to_string()));
                Ok(())
            }

            AstNode::OscillateStatement { target } => {
                // Toggle a boolean variable
                if let AstNode::Identifier(name) = target.as_ref() {
                    match self.get_variable(name) {
                        Ok(value) => match value {
                            Value::Boolean(b) => {
                                self.set_variable(name.clone(), Value::Boolean(!b));
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
                loop {
                    let cond_value = self.evaluate_expression(condition)?;
                    if !cond_value.is_truthy() {
                        break;
                    }

                    match self.execute_block(body) {
                        Err(InterpreterError::BreakOutsideLoop) => break,
                        Err(InterpreterError::ContinueOutsideLoop) => continue,
                        Err(e) => return Err(e),
                        Ok(()) => {}
                    }
                }
                Ok(())
            }

            AstNode::LoopStatement { body } => {
                loop {
                    match self.execute_block(body) {
                        Err(InterpreterError::BreakOutsideLoop) => break,
                        Err(InterpreterError::ContinueOutsideLoop) => continue,
                        Err(e) => return Err(e),
                        Ok(()) => {}
                    }
                }
                Ok(())
            }

            AstNode::PendulumStatement {
                init,
                condition,
                update,
                body,
            } => {
                // Execute init (if present)
                if let Some(init_node) = init {
                    self.execute_statement(init_node)?;
                }

                // Loop while condition is true
                loop {
                    let cond_value = self.evaluate_expression(condition)?;
                    if !cond_value.is_truthy() {
                        break;
                    }

                    // Execute body without creating new scope (to preserve variable modifications)
                    match self.execute_pendulum_body(body) {
                        Err(InterpreterError::BreakOutsideLoop) => break,
                        Err(InterpreterError::ContinueOutsideLoop) => {
                            // Execute update before continuing
                            if let Some(update_node) = update {
                                self.evaluate_expression(update_node)?;
                            }
                            continue;
                        }
                        Err(e) => return Err(e),
                        Ok(()) => {}
                    }

                    // Execute update
                    if let Some(update_node) = update {
                        self.evaluate_expression(update_node)?;
                    }
                }
                Ok(())
            }

            AstNode::SuspendStatement => {
                // Suspend is an infinite pause - in practice, this should wait for external input
                // For now, we'll just log a warning
                CoreBuiltins::whisper("[SUSPEND] Program suspended - press Ctrl+C to exit");
                std::thread::sleep(std::time::Duration::from_secs(3600)); // Sleep for 1 hour
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

            AstNode::BreakStatement => Err(InterpreterError::BreakOutsideLoop),

            AstNode::ContinueStatement => Err(InterpreterError::ContinueOutsideLoop),

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

    /// Execute pendulum loop body without creating a new scope
    /// This allows variables to persist across loop iterations
    fn execute_pendulum_body(&mut self, statements: &[AstNode]) -> Result<(), InterpreterError> {
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
                    self.set_variable(name.clone(), val.clone());
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
                    "Ungültiges Zuweisungsziel",
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
                        // Check guard condition if present
                        if let Some(guard) = &case.guard {
                            // Temporarily add pattern bindings to globals
                            for (name, value) in &matched_env {
                                self.globals.insert(name.clone(), value.clone());
                            }

                            let guard_result = self.evaluate_expression(guard)?;

                            // Remove pattern bindings
                            for (name, _) in &matched_env {
                                self.globals.remove(name);
                            }

                            if !guard_result.is_truthy() {
                                continue;
                            }
                        }

                        // Pattern matched and guard passed - execute body
                        for (name, value) in matched_env {
                            self.globals.insert(name, value);
                        }

                        let mut result = Value::Null;
                        for stmt in &case.body {
                            // Case bodies can contain both statements and expressions
                            match stmt {
                                // Try to evaluate as expression first
                                _ => result = self.evaluate_expression(stmt)?,
                            }
                        }

                        return Ok(result);
                    }
                }

                // No case matched - try default
                if let Some(default_body) = default {
                    let mut result = Value::Null;
                    for stmt in default_body {
                        result = self.evaluate_expression(stmt)?;
                    }
                    Ok(result)
                } else {
                    Err(InterpreterError::Runtime(
                        "No pattern matched and no default case provided".to_string(),
                    ))
                }
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
                        let rest_elements: Vec<Value> = arr.iter().skip(elements.len()).cloned().collect();
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
                // For now, we'll match against objects (which we don't have yet)
                // This is a placeholder for when we implement records/objects
                let _ = (type_name, fields);
                Err(InterpreterError::Runtime(
                    "Record pattern matching not yet fully implemented".to_string(),
                ))
            }
        }
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
                if let (Value::String(s1), Value::String(s2)) = (left, right) {
                    Ok(Value::String(format!("{}{}", s1, s2)))
                } else {
                    Ok(Value::Number(left.to_number()? + right.to_number()?))
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
                "Null-Wert kann nicht aufgerufen werden",
            ))),
            _ => Err(InterpreterError::Runtime(localized(
                "Value is not callable",
                "Wert ist nicht aufrufbar",
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
                    "Erwartet {} Argumente, erhalten {}",
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
            self.set_variable("this".to_string(), Value::Instance(instance));
        }

        for (param, arg) in function.parameters.iter().zip(args.iter()) {
            self.set_variable(param.clone(), arg.clone());
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
                    "Konstruktor in Session '{}' darf nicht statisch sein",
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
                        "Konstruktor der Session '{}' erwartet {} Argumente, erhalten {}",
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
                    "Session '{}' definiert keinen Konstruktor, dennoch wurden Argumente übergeben",
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
        self.set_variable("this".to_string(), Value::Instance(instance.clone()));

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
                        "Session-Instanz von '{}' besitzt kein Mitglied '{}'",
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
                        "Session '{}' besitzt kein statisches Mitglied '{}'",
                        session_rc.name(),
                        property
                    ),
                )))
            }
            other => Err(InterpreterError::Runtime(localized(
                &format!("Cannot access member '{}' on value '{}'", property, other),
                &format!(
                    "Mitglied '{}' kann auf Wert '{}' nicht zugegriffen werden",
                    property, other
                ),
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
                        &format!("Zuweisung zur Methode '{}' nicht möglich", property),
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
                            "Statisches Feld '{}' muss über die Session '{}' gesetzt werden",
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
                        "Session-Instanz von '{}' besitzt kein Feld '{}'",
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
                        &format!(
                            "Zuweisung zu statischer Methode '{}' nicht möglich",
                            property
                        ),
                    )));
                }

                Err(InterpreterError::Runtime(localized(
                    &format!(
                        "Session '{}' has no static field '{}'",
                        session_rc.name(),
                        property
                    ),
                    &format!(
                        "Session '{}' besitzt kein statisches Feld '{}'",
                        session_rc.name(),
                        property
                    ),
                )))
            }
            _ => Err(InterpreterError::Runtime(localized(
                "Assignment target is not a session member",
                "Zuweisungsziel ist kein Session-Mitglied",
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
                    "Zugriff auf privates {} '{}' der Session '{}' verweigert",
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

    fn call_builtin(
        &mut self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        if let Some(result) = self.call_math_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_string_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_array_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_core_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_file_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_hashing_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_statistics_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_system_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_time_builtin(name, args)? {
            return Ok(Some(result));
        }

        if let Some(result) = self.call_validation_builtin(name, args)? {
            return Ok(Some(result));
        }

        Ok(None)
    }

    fn call_math_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "Sin" => Some(Value::Number(MathBuiltins::sin(
                self.number_arg(args, 0, name)?,
            ))),
            "Cos" => Some(Value::Number(MathBuiltins::cos(
                self.number_arg(args, 0, name)?,
            ))),
            "Tan" => Some(Value::Number(MathBuiltins::tan(
                self.number_arg(args, 0, name)?,
            ))),
            "Sqrt" => Some(Value::Number(MathBuiltins::sqrt(
                self.number_arg(args, 0, name)?,
            ))),
            "Log" => Some(Value::Number(MathBuiltins::log(
                self.number_arg(args, 0, name)?,
            ))),
            "Log10" => Some(Value::Number(MathBuiltins::log10(
                self.number_arg(args, 0, name)?,
            ))),
            "Abs" => Some(Value::Number(MathBuiltins::abs(
                self.number_arg(args, 0, name)?,
            ))),
            "Floor" => Some(Value::Number(MathBuiltins::floor(
                self.number_arg(args, 0, name)?,
            ))),
            "Ceil" => Some(Value::Number(MathBuiltins::ceil(
                self.number_arg(args, 0, name)?,
            ))),
            "Round" => Some(Value::Number(MathBuiltins::round(
                self.number_arg(args, 0, name)?,
            ))),
            "Min" => Some(Value::Number(MathBuiltins::min(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Max" => Some(Value::Number(MathBuiltins::max(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Pow" => Some(Value::Number(MathBuiltins::pow(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Factorial" => Some(Value::Number(MathBuiltins::factorial(
                self.integer_arg(args, 0, name)?,
            ) as f64)),
            "Gcd" => Some(Value::Number(MathBuiltins::gcd(
                self.integer_arg(args, 0, name)?,
                self.integer_arg(args, 1, name)?,
            ) as f64)),
            "Lcm" => Some(Value::Number(MathBuiltins::lcm(
                self.integer_arg(args, 0, name)?,
                self.integer_arg(args, 1, name)?,
            ) as f64)),
            "IsPrime" => Some(Value::Boolean(MathBuiltins::is_prime(
                self.integer_arg(args, 0, name)?,
            ))),
            "Fibonacci" => Some(Value::Number(MathBuiltins::fibonacci(
                self.integer_arg(args, 0, name)?,
            ) as f64)),
            "Clamp" => Some(Value::Number(MathBuiltins::clamp(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
                self.number_arg(args, 2, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    fn call_string_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "Length" => Some(Value::Number(
                StringBuiltins::length(&self.string_arg(args, 0, name)?) as f64,
            )),
            "ToUpper" => Some(Value::String(StringBuiltins::to_upper(
                &self.string_arg(args, 0, name)?,
            ))),
            "ToLower" => Some(Value::String(StringBuiltins::to_lower(
                &self.string_arg(args, 0, name)?,
            ))),
            "Trim" => Some(Value::String(StringBuiltins::trim(
                &self.string_arg(args, 0, name)?,
            ))),
            "IndexOf" => Some(Value::Number(StringBuiltins::index_of(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ) as f64)),
            "Replace" => Some(Value::String(StringBuiltins::replace(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
                &self.string_arg(args, 2, name)?,
            ))),
            "Reverse" => Some(Value::String(StringBuiltins::reverse(
                &self.string_arg(args, 0, name)?,
            ))),
            "Capitalize" => Some(Value::String(StringBuiltins::capitalize(
                &self.string_arg(args, 0, name)?,
            ))),
            "StartsWith" => Some(Value::Boolean(StringBuiltins::starts_with(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "EndsWith" => Some(Value::Boolean(StringBuiltins::ends_with(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "Contains" => Some(Value::Boolean(StringBuiltins::contains(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "Split" => {
                let items = StringBuiltins::split(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .into_iter()
                .map(Value::String)
                .collect();
                Some(Value::Array(items))
            }
            "Substring" => Some(Value::String(StringBuiltins::substring(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
                self.usize_arg(args, 2, name)?,
            ))),
            "Repeat" => Some(Value::String(StringBuiltins::repeat(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
            ))),
            "PadLeft" => Some(Value::String(StringBuiltins::pad_left(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
                self.char_arg(args, 2, name)?,
            ))),
            "PadRight" => Some(Value::String(StringBuiltins::pad_right(
                &self.string_arg(args, 0, name)?,
                self.usize_arg(args, 1, name)?,
                self.char_arg(args, 2, name)?,
            ))),
            "IsEmpty" => Some(Value::Boolean(StringBuiltins::is_empty(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsWhitespace" => Some(Value::Boolean(StringBuiltins::is_whitespace(
                &self.string_arg(args, 0, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    fn call_array_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "ArrayLength" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::Number(ArrayBuiltins::length(&array) as f64))
            }
            "ArrayIsEmpty" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::Boolean(ArrayBuiltins::is_empty(&array)))
            }
            "ArrayGet" => {
                let array = self.array_arg(args, 0, name)?;
                let index = self.usize_arg(args, 1, name)?;
                let value = ArrayBuiltins::get(&array, index).unwrap_or(Value::Null);
                Some(value)
            }
            "ArrayIndexOf" => {
                let array = self.array_arg(args, 0, name)?;
                let target = self.arg(args, 1, name)?.clone();
                Some(Value::Number(
                    ArrayBuiltins::index_of(&array, &target) as f64
                ))
            }
            "ArrayContains" => {
                let array = self.array_arg(args, 0, name)?;
                let target = self.arg(args, 1, name)?.clone();
                Some(Value::Boolean(ArrayBuiltins::contains(&array, &target)))
            }
            "ArrayReverse" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::Array(ArrayBuiltins::reverse(&array)))
            }
            "ArraySum" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::sum(&numbers)))
            }
            "ArrayAverage" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::average(&numbers)))
            }
            "ArrayMin" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::min(&numbers)))
            }
            "ArrayMax" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                Some(Value::Number(ArrayBuiltins::max(&numbers)))
            }
            "ArraySort" => {
                let array = self.array_arg(args, 0, name)?;
                let numbers = self.values_to_numbers(&array, name)?;
                let sorted = ArrayBuiltins::sort(&numbers)
                    .into_iter()
                    .map(Value::Number)
                    .collect();
                Some(Value::Array(sorted))
            }
            "ArrayFirst" => {
                let array = self.array_arg(args, 0, name)?;
                Some(ArrayBuiltins::first(&array).unwrap_or(Value::Null))
            }
            "ArrayLast" => {
                let array = self.array_arg(args, 0, name)?;
                Some(ArrayBuiltins::last(&array).unwrap_or(Value::Null))
            }
            "ArrayTake" => {
                let array = self.array_arg(args, 0, name)?;
                let count = self.usize_arg(args, 1, name)?;
                Some(Value::Array(ArrayBuiltins::take(&array, count)))
            }
            "ArraySkip" => {
                let array = self.array_arg(args, 0, name)?;
                let count = self.usize_arg(args, 1, name)?;
                Some(Value::Array(ArrayBuiltins::skip(&array, count)))
            }
            "ArraySlice" => {
                let array = self.array_arg(args, 0, name)?;
                let start = self.usize_arg(args, 1, name)?;
                let end = self.usize_arg(args, 2, name)?;
                Some(Value::Array(ArrayBuiltins::slice(&array, start, end)))
            }
            "ArrayJoin" => {
                let array = self.array_arg(args, 0, name)?;
                let separator = self.string_arg(args, 1, name)?;
                Some(Value::String(ArrayBuiltins::join(&array, &separator)))
            }
            "ArrayCount" => {
                let array = self.array_arg(args, 0, name)?;
                let target = self.arg(args, 1, name)?.clone();
                Some(Value::Number(ArrayBuiltins::count(&array, &target) as f64))
            }
            "ArrayDistinct" => {
                let array = self.array_arg(args, 0, name)?;
                Some(Value::Array(ArrayBuiltins::distinct(&array)))
            }
            _ => None,
        };

        Ok(result)
    }

    fn call_core_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "Observe" => {
                let message = self.arg(args, 0, name)?.to_string();
                CoreBuiltins::observe(&message);
                Some(Value::Null)
            }
            "Drift" => {
                let duration = self.number_arg(args, 0, name)?;
                CoreBuiltins::drift(duration.max(0.0) as u64);
                Some(Value::Null)
            }
            "DeepTrance" => {
                let duration = self.number_arg(args, 0, name)?;
                CoreBuiltins::deep_trance(duration.max(0.0) as u64);
                Some(Value::Null)
            }
            "HypnoticCountdown" => {
                CoreBuiltins::hypnotic_countdown(self.integer_arg(args, 0, name)?);
                Some(Value::Null)
            }
            "TranceInduction" => {
                CoreBuiltins::trance_induction(&self.string_arg(args, 0, name)?);
                Some(Value::Null)
            }
            "HypnoticVisualization" => {
                CoreBuiltins::hypnotic_visualization(&self.string_arg(args, 0, name)?);
                Some(Value::Null)
            }
            "ToInt" => Some(Value::Number(
                CoreBuiltins::to_int(self.number_arg(args, 0, name)?) as f64,
            )),
            "ToDouble" => Some(Value::Number(
                CoreBuiltins::to_double(&self.string_arg(args, 0, name)?)
                    .map_err(InterpreterError::Runtime)?,
            )),
            "ToString" => Some(Value::String(
                args.first()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "null".to_string()),
            )),
            "ToBoolean" => Some(Value::Boolean(CoreBuiltins::to_boolean(
                &self.string_arg(args, 0, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    fn call_file_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "ReadFile" => Some(Value::String(
                FileBuiltins::read_file(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?,
            )),
            "WriteFile" => {
                FileBuiltins::write_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "AppendFile" => {
                FileBuiltins::append_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "FileExists" => Some(Value::Boolean(FileBuiltins::file_exists(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsFile" => Some(Value::Boolean(FileBuiltins::is_file(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsDirectory" => Some(Value::Boolean(FileBuiltins::is_directory(
                &self.string_arg(args, 0, name)?,
            ))),
            "DeleteFile" => {
                FileBuiltins::delete_file(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "CreateDirectory" => {
                FileBuiltins::create_directory(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "ListDirectory" => {
                let files = FileBuiltins::list_directory(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))?
                    .into_iter()
                    .map(Value::String)
                    .collect();
                Some(Value::Array(files))
            }
            "GetFileSize" => Some(Value::Number(
                FileBuiltins::get_file_size(&self.string_arg(args, 0, name)?)
                    .map_err(|e| InterpreterError::Runtime(e.to_string()))? as f64,
            )),
            "CopyFile" => Some(Value::Number(
                FileBuiltins::copy_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))? as f64,
            )),
            "RenameFile" => {
                FileBuiltins::rename_file(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(|e| InterpreterError::Runtime(e.to_string()))?;
                Some(Value::Null)
            }
            "GetFileExtension" => Some(self.option_string_to_value(
                FileBuiltins::get_file_extension(&self.string_arg(args, 0, name)?),
            )),
            "GetFileName" => Some(self.option_string_to_value(FileBuiltins::get_file_name(
                &self.string_arg(args, 0, name)?,
            ))),
            "GetParentDirectory" => Some(self.option_string_to_value(
                FileBuiltins::get_parent_directory(&self.string_arg(args, 0, name)?),
            )),
            _ => None,
        };

        Ok(result)
    }

    fn call_hashing_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "HashString" => Some(Value::Number(HashingBuiltins::hash_string(
                &self.string_arg(args, 0, name)?,
            ) as f64)),
            "HashNumber" => Some(Value::Number(HashingBuiltins::hash_number(
                self.number_arg(args, 0, name)?,
            ) as f64)),
            "SimpleRandom" => Some(Value::Number(HashingBuiltins::simple_random(
                self.u64_arg(args, 0, name)?,
            ) as f64)),
            "AreAnagrams" => Some(Value::Boolean(HashingBuiltins::are_anagrams(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            "IsPalindrome" => Some(Value::Boolean(HashingBuiltins::is_palindrome(
                &self.string_arg(args, 0, name)?,
            ))),
            "CountOccurrences" => Some(Value::Number(HashingBuiltins::count_occurrences(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ) as f64)),
            "RemoveDuplicates" => Some(Value::String(HashingBuiltins::remove_duplicates(
                &self.string_arg(args, 0, name)?,
            ))),
            "UniqueCharacters" => Some(Value::String(HashingBuiltins::unique_characters(
                &self.string_arg(args, 0, name)?,
            ))),
            "ReverseWords" => Some(Value::String(HashingBuiltins::reverse_words(
                &self.string_arg(args, 0, name)?,
            ))),
            "TitleCase" => Some(Value::String(HashingBuiltins::title_case(
                &self.string_arg(args, 0, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    fn call_statistics_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let numbers_primary = |this: &Self| -> Result<Vec<f64>, InterpreterError> {
            let array = this.array_arg(args, 0, name)?;
            this.values_to_numbers(&array, name)
        };

        let result = match name {
            "Mean" => Some(Value::Number(StatisticsBuiltins::calculate_mean(
                &numbers_primary(self)?,
            ))),
            "Median" => Some(Value::Number(StatisticsBuiltins::calculate_median(
                &numbers_primary(self)?,
            ))),
            "Mode" => Some(Value::Number(StatisticsBuiltins::calculate_mode(
                &numbers_primary(self)?,
            ))),
            "StandardDeviation" => Some(Value::Number(
                StatisticsBuiltins::calculate_standard_deviation(&numbers_primary(self)?),
            )),
            "Variance" => Some(Value::Number(StatisticsBuiltins::calculate_variance(
                &numbers_primary(self)?,
            ))),
            "Range" => Some(Value::Number(StatisticsBuiltins::calculate_range(
                &numbers_primary(self)?,
            ))),
            "Percentile" => Some(Value::Number(StatisticsBuiltins::calculate_percentile(
                &numbers_primary(self)?,
                self.number_arg(args, 1, name)?,
            ))),
            "Correlation" => {
                let x = self.values_to_numbers(&self.array_arg(args, 0, name)?, name)?;
                let y = self.values_to_numbers(&self.array_arg(args, 1, name)?, name)?;
                Some(Value::Number(StatisticsBuiltins::calculate_correlation(
                    &x, &y,
                )))
            }
            "LinearRegression" => {
                let x = self.values_to_numbers(&self.array_arg(args, 0, name)?, name)?;
                let y = self.values_to_numbers(&self.array_arg(args, 1, name)?, name)?;
                let (slope, intercept) = StatisticsBuiltins::linear_regression(&x, &y);
                Some(Value::Array(vec![
                    Value::Number(slope),
                    Value::Number(intercept),
                ]))
            }
            _ => None,
        };

        Ok(result)
    }

    fn call_system_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "GetCurrentDirectory" => Some(Value::String(SystemBuiltins::get_current_directory())),
            "GetEnv" => Some(self.option_string_to_value(SystemBuiltins::get_env_var(
                &self.string_arg(args, 0, name)?,
            ))),
            "SetEnv" => {
                SystemBuiltins::set_env_var(
                    &self.string_arg(args, 0, name)?,
                    &self.string_arg(args, 1, name)?,
                )
                .map_err(InterpreterError::Runtime)?;
                Some(Value::Null)
            }
            "GetOperatingSystem" => Some(Value::String(SystemBuiltins::get_operating_system())),
            "GetArchitecture" => Some(Value::String(SystemBuiltins::get_architecture())),
            "GetCpuCount" => Some(Value::Number(SystemBuiltins::get_cpu_count() as f64)),
            "GetHostname" => Some(Value::String(SystemBuiltins::get_hostname())),
            "GetUsername" => Some(Value::String(SystemBuiltins::get_username())),
            "GetHomeDirectory" => Some(Value::String(SystemBuiltins::get_home_directory())),
            "GetTempDirectory" => Some(Value::String(SystemBuiltins::get_temp_directory())),
            "GetArgs" => Some(Value::Array(
                SystemBuiltins::get_args()
                    .into_iter()
                    .map(Value::String)
                    .collect(),
            )),
            "Exit" => {
                // Exit mirrors the legacy runtime behavior by terminating the host process immediately.
                SystemBuiltins::exit(self.integer_arg(args, 0, name)? as i32);
            }
            _ => None,
        };

        Ok(result)
    }

    fn call_time_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "CurrentTimestamp" => Some(Value::Number(TimeBuiltins::get_current_time() as f64)),
            "CurrentDate" => Some(Value::String(TimeBuiltins::get_current_date())),
            "CurrentTime" => Some(Value::String(TimeBuiltins::get_current_time_string())),
            "CurrentDateTime" => Some(Value::String(TimeBuiltins::get_current_date_time())),
            "FormatDateTime" => Some(Value::String(TimeBuiltins::format_date_time(
                &self.string_arg(args, 0, name)?,
            ))),
            "DayOfWeek" => Some(Value::Number(TimeBuiltins::get_day_of_week() as f64)),
            "DayOfYear" => Some(Value::Number(TimeBuiltins::get_day_of_year() as f64)),
            "IsLeapYear" => Some(Value::Boolean(TimeBuiltins::is_leap_year(
                self.integer_arg(args, 0, name)? as i32,
            ))),
            "DaysInMonth" => Some(self.option_u32_to_value(TimeBuiltins::get_days_in_month(
                self.integer_arg(args, 0, name)? as i32,
                self.usize_arg(args, 1, name)? as u32,
            ))),
            "CurrentYear" => Some(Value::Number(TimeBuiltins::get_year() as f64)),
            "CurrentMonth" => Some(Value::Number(TimeBuiltins::get_month() as f64)),
            "CurrentDay" => Some(Value::Number(TimeBuiltins::get_day() as f64)),
            "CurrentHour" => Some(Value::Number(TimeBuiltins::get_hour() as f64)),
            "CurrentMinute" => Some(Value::Number(TimeBuiltins::get_minute() as f64)),
            "CurrentSecond" => Some(Value::Number(TimeBuiltins::get_second() as f64)),
            _ => None,
        };

        Ok(result)
    }

    fn call_validation_builtin(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, InterpreterError> {
        let result = match name {
            "IsValidEmail" => Some(Value::Boolean(ValidationBuiltins::is_valid_email(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsValidUrl" => Some(Value::Boolean(ValidationBuiltins::is_valid_url(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsValidPhoneNumber" => Some(Value::Boolean(
                ValidationBuiltins::is_valid_phone_number(&self.string_arg(args, 0, name)?),
            )),
            "IsAlphanumeric" => Some(Value::Boolean(ValidationBuiltins::is_alphanumeric(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsAlphabetic" => Some(Value::Boolean(ValidationBuiltins::is_alphabetic(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsNumeric" => Some(Value::Boolean(ValidationBuiltins::is_numeric(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsLowercase" => Some(Value::Boolean(ValidationBuiltins::is_lowercase(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsUppercase" => Some(Value::Boolean(ValidationBuiltins::is_uppercase(
                &self.string_arg(args, 0, name)?,
            ))),
            "IsInRange" => Some(Value::Boolean(ValidationBuiltins::is_in_range(
                self.number_arg(args, 0, name)?,
                self.number_arg(args, 1, name)?,
                self.number_arg(args, 2, name)?,
            ))),
            "MatchesPattern" => Some(Value::Boolean(ValidationBuiltins::matches_pattern(
                &self.string_arg(args, 0, name)?,
                &self.string_arg(args, 1, name)?,
            ))),
            _ => None,
        };

        Ok(result)
    }

    fn arg<'a>(
        &self,
        args: &'a [Value],
        index: usize,
        name: &str,
    ) -> Result<&'a Value, InterpreterError> {
        args.get(index).ok_or_else(|| {
            InterpreterError::Runtime(format!(
                "Builtin '{}' expected argument at position {}",
                name,
                index + 1
            ))
        })
    }

    fn number_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<f64, InterpreterError> {
        self.arg(args, index, name)?.to_number().map_err(|_| {
            InterpreterError::TypeError(format!(
                "Builtin '{}' expected numeric argument at position {}",
                name,
                index + 1
            ))
        })
    }

    fn integer_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<i64, InterpreterError> {
        let value = self.number_arg(args, index, name)?;
        Ok(value.round() as i64)
    }

    fn u64_arg(&self, args: &[Value], index: usize, name: &str) -> Result<u64, InterpreterError> {
        let value = self.number_arg(args, index, name)?;
        if value < 0.0 {
            return Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected non-negative number at position {}",
                name,
                index + 1
            )));
        }
        Ok(value.round() as u64)
    }

    fn usize_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<usize, InterpreterError> {
        let value = self.number_arg(args, index, name)?;
        if value < 0.0 {
            return Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected non-negative number at position {}",
                name,
                index + 1
            )));
        }
        Ok(value.round() as usize)
    }

    fn string_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<String, InterpreterError> {
        match self.arg(args, index, name)? {
            Value::String(s) => Ok(s.clone()),
            other => Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected string argument at position {}, got {:?}",
                name,
                index + 1,
                other
            ))),
        }
    }

    fn char_arg(&self, args: &[Value], index: usize, name: &str) -> Result<char, InterpreterError> {
        let text = self.string_arg(args, index, name)?;
        text.chars().next().ok_or_else(|| {
            InterpreterError::TypeError(format!(
                "Builtin '{}' expected non-empty string to derive character at position {}",
                name,
                index + 1
            ))
        })
    }

    fn array_arg(
        &self,
        args: &[Value],
        index: usize,
        name: &str,
    ) -> Result<Vec<Value>, InterpreterError> {
        match self.arg(args, index, name)? {
            Value::Array(items) => Ok(items.clone()),
            other => Err(InterpreterError::TypeError(format!(
                "Builtin '{}' expected array argument at position {}, got {:?}",
                name,
                index + 1,
                other
            ))),
        }
    }

    fn option_string_to_value(&self, input: Option<String>) -> Value {
        input.map(Value::String).unwrap_or(Value::Null)
    }

    fn option_u32_to_value(&self, input: Option<u32>) -> Value {
        input
            .map(|v| Value::Number(v as f64))
            .unwrap_or(Value::Null)
    }

    fn values_to_numbers(
        &self,
        values: &[Value],
        name: &str,
    ) -> Result<Vec<f64>, InterpreterError> {
        values
            .iter()
            .enumerate()
            .map(|(i, value)| {
                value.to_number().map_err(|_| {
                    InterpreterError::TypeError(format!(
                        "Builtin '{}' expected numeric array element at position {}",
                        name,
                        i + 1
                    ))
                })
            })
            .collect()
    }

    fn push_scope(&mut self) {
        self.locals.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.locals.pop();
    }

    fn set_variable(&mut self, name: String, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }

    fn get_variable(&self, name: &str) -> Result<Value, InterpreterError> {
        // Search in local scopes (from innermost to outermost)
        for scope in self.locals.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }

        // Search in global scope
        self.globals
            .get(name)
            .cloned()
            .ok_or_else(|| InterpreterError::UndefinedVariable(name.to_string()))
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
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

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
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

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
        interpreter.execute_program(ast).unwrap();

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
        interpreter.execute_program(ast).unwrap();

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
}
