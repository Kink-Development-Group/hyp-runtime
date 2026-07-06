//! Runtime values: primitives, functions, promises and records.

use super::error::InterpreterError;
use super::session::{SessionDefinition, SessionInstance, SessionMethodDefinition};
use hypnoscript_lexer_parser::ast::AstNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Represents a callable suggestion (function) within the interpreter.
///
/// HypnoScript functions can be:
/// - Global suggestions (top-level functions)
/// - Session methods (instance methods)
/// - Static session methods (`dominant` keyword)
/// - Constructors (special session methods)
/// - Triggers (event-driven callbacks)
///
/// # Examples
///
/// ```hyp
/// // Global suggestion
/// suggestion greet(name: string) {
///     awaken "Hello, " + name;
/// }
///
/// // Session method
/// session Calculator {
///     suggestion add(a: number, b: number) {
///         awaken a + b;
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub(crate) name: String,
    pub(crate) parameters: Vec<String>,
    /// Shared so that cloning a function value (variable lookups, argument
    /// passing) never deep-copies the body AST.
    pub(crate) body: Rc<Vec<AstNode>>,
    pub(crate) this_binding: Option<Rc<RefCell<SessionInstance>>>,
    pub(crate) session_name: Option<String>,
    pub(crate) is_static: bool,
    pub(crate) is_constructor: bool,
    /// Lexical environment captured when the function was declared inside
    /// another function (closure by-value snapshot). Installed into the
    /// activation scope on every call; parameters shadow captures.
    pub(crate) captured: Rc<HashMap<String, Value>>,
}

impl FunctionValue {
    /// Creates a function that captures the given lexical environment.
    /// Top-level functions simply capture an empty environment.
    pub(crate) fn new_closure(
        name: String,
        parameters: Vec<String>,
        body: Vec<AstNode>,
        captured: HashMap<String, Value>,
    ) -> Self {
        Self {
            name,
            parameters,
            body: Rc::new(body),
            this_binding: None,
            session_name: None,
            is_static: false,
            is_constructor: false,
            captured: Rc::new(captured),
        }
    }

    pub(crate) fn new_session_member(
        session_name: String,
        method: &SessionMethodDefinition,
        this_binding: Option<Rc<RefCell<SessionInstance>>>,
    ) -> Self {
        Self {
            name: format!("{}::{}", session_name, method.name),
            parameters: method.parameters.clone(),
            body: Rc::clone(&method.body),
            this_binding,
            session_name: Some(session_name),
            is_static: method.is_static,
            is_constructor: method.is_constructor,
            captured: Rc::new(HashMap::new()),
        }
    }

    pub(crate) fn this_binding(&self) -> Option<Rc<RefCell<SessionInstance>>> {
        self.this_binding.as_ref().map(Rc::clone)
    }

    pub(crate) fn session_name(&self) -> Option<&str> {
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

/// Simple Promise/Future wrapper for async operations.
///
/// Promises represent asynchronous computations in HypnoScript.
/// They are created by `mesmerize` suggestions and resolved with `await` or `surrenderTo`.
///
/// # Examples
///
/// ```hyp
/// // Async suggestion returns a Promise
/// mesmerize suggestion fetchData() {
///     induce data = "some data";
///     awaken data;
/// }
///
/// entrance {
///     induce result = await fetchData();
///     observe result;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Promise {
    /// The value this promise resolves to.
    value: Option<Value>,
    /// Whether the promise is resolved
    resolved: bool,
    /// Remaining simulated delay (in milliseconds) before the promise
    /// resolves. `await` waits this long (honouring `HYPNO_TIME_SCALE`)
    /// and then resolves the promise with `value`.
    delay_ms: Option<u64>,
}

impl Promise {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            value: None,
            resolved: false,
            delay_ms: None,
        }
    }

    /// Creates an already-resolved promise.
    pub(crate) fn resolve(value: Value) -> Self {
        Self {
            value: Some(value),
            resolved: true,
            delay_ms: None,
        }
    }

    /// Creates a promise that resolves to `value` after `delay_ms`
    /// simulated milliseconds (elapsed when the promise is awaited).
    pub(crate) fn delayed(delay_ms: u64, value: Value) -> Self {
        Self {
            value: Some(value),
            resolved: false,
            delay_ms: Some(delay_ms),
        }
    }

    pub(crate) fn is_resolved(&self) -> bool {
        self.resolved
    }

    /// Remaining simulated delay before this promise resolves.
    pub(crate) fn pending_delay_ms(&self) -> Option<u64> {
        if self.resolved { None } else { self.delay_ms }
    }

    /// Marks the promise as resolved (its delay has elapsed) and returns
    /// the resolved value.
    pub(crate) fn mark_resolved(&mut self) -> Value {
        self.resolved = true;
        self.delay_ms = None;
        self.value.clone().unwrap_or(Value::Null)
    }
}

/// Runtime value in HypnoScript.
///
/// Represents all possible runtime values in the HypnoScript interpreter.
/// This includes primitives, collections, functions, sessions, and async values.
///
/// # Variants
///
/// - `Number(f64)` - Numeric values (e.g., `42`, `3.14`)
/// - `String(String)` - Text values (e.g., `"Hello"`)
/// - `Boolean(bool)` - Boolean values (`true`/`false`)
/// - `Array(Vec<Value>)` - Arrays (e.g., `[1, 2, 3]`)
/// - `Function(FunctionValue)` - Callable suggestions
/// - `Session(Rc<SessionDefinition>)` - Session type (class constructor)
/// - `Instance(Rc<RefCell<SessionInstance>>)` - Session instance
/// - `Promise(Rc<RefCell<Promise>>)` - Async promise from `mesmerize`
/// - `Record(RecordValue)` - Record/struct from `tranceify`
/// - `Null` - Null value
///
/// # Examples
///
/// ```hyp
/// induce num: number = 42;                    // Value::Number
/// induce text: string = "Hello";              // Value::String
/// induce flag: boolean = true;                // Value::Boolean
/// induce list: number[] = [1, 2, 3];          // Value::Array
/// induce account = BankAccount(100);          // Value::Instance
/// induce promise = mesmerize getData();       // Value::Promise
/// induce nothing: null = null;                // Value::Null
/// ```
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    /// Arrays are immutable values; sharing them via `Rc` makes cloning
    /// (assignments, function arguments, ...) O(1) instead of deep copies.
    Array(Rc<Vec<Value>>),
    Function(FunctionValue),
    Session(Rc<SessionDefinition>),
    Instance(Rc<RefCell<SessionInstance>>),
    Promise(Rc<RefCell<Promise>>),
    /// Records are immutable values; shared via `Rc` like arrays.
    Record(Rc<RecordValue>),
    Null,
}

impl Value {
    /// Creates an array value (shared, cheap to clone).
    pub fn array(values: Vec<Value>) -> Self {
        Value::Array(Rc::new(values))
    }

    /// Creates a record value (shared, cheap to clone).
    pub fn record(record: RecordValue) -> Self {
        Value::Record(Rc::new(record))
    }
}

/// A record instance (from tranceify declarations).
///
/// Records are user-defined structured data types in HypnoScript,
/// similar to structs in other languages.
///
/// # Examples
///
/// ```hyp
/// tranceify Point {
///     x: number,
///     y: number
/// }
///
/// entrance {
///     induce p = Point { x: 10, y: 20 };
///     observe p.x;  // 10
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RecordValue {
    pub type_name: String,
    pub fields: HashMap<String, Value>,
}

impl PartialEq for RecordValue {
    fn eq(&self, other: &Self) -> bool {
        self.type_name == other.type_name && self.fields == other.fields
    }
}

impl Eq for RecordValue {}

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
            (Value::Record(ra), Value::Record(rb)) => ra == rb,
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
            Value::Function(_)
            | Value::Session(_)
            | Value::Instance(_)
            | Value::Promise(_)
            | Value::Record(_) => true,
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
            Value::Record(record) => {
                write!(f, "<record {}>", record.type_name)
            }
        }
    }
}
