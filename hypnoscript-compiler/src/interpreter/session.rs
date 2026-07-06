//! Session (class) definitions and instances: HypnoScript's OOP layer.

use super::error::{InterpreterError, localized};
use super::value::Value;
use hypnoscript_lexer_parser::ast::{AstNode, SessionVisibility};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Definition of a session field (instance scope).
///
/// Session fields represent instance-level variables in HypnoScript sessions (classes).
/// They can have visibility modifiers (`expose`/`conceal`) and optional type annotations.
///
/// # Examples
///
/// ```hyp
/// session Person {
///     expose name: string = "Unknown";
///     conceal age: number = 0;
/// }
/// ```
#[derive(Debug, Clone)]
pub(crate) struct SessionFieldDefinition {
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) type_annotation: Option<String>,
    pub(crate) visibility: SessionVisibility,
    pub(crate) initializer: Option<AstNode>,
}

/// Definition of a session method.
///
/// Session methods represent callable functions within HypnoScript sessions.
/// They can be:
/// - Instance methods (default)
/// - Static methods (`dominant` keyword)
/// - Constructors (special methods with `constructor` keyword)
///
/// # Examples
///
/// ```hyp
/// session Calculator {
///     // Constructor
///     constructor(initial: number) {
///         induce this.value = initial;
///     }
///
///     // Instance method
///     expose suggestion add(n: number) {
///         induce this.value = this.value + n;
///     }
///
///     // Static method
///     dominant suggestion createDefault() {
///         awaken Calculator(0);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub(crate) struct SessionMethodDefinition {
    pub(crate) name: String,
    pub(crate) parameters: Vec<String>,
    pub(crate) body: Vec<AstNode>,
    pub(crate) visibility: SessionVisibility,
    pub(crate) is_static: bool,
    pub(crate) is_constructor: bool,
}

/// Runtime data for a static field, including its initializer AST.
///
/// Static fields are initialized once and shared across all session instances.
/// They are declared with the `dominant` keyword in HypnoScript.
///
/// # Examples
///
/// ```hyp
/// session Counter {
///     dominant instanceCount: number = 0;
///
///     constructor() {
///         induce Counter.instanceCount = Counter.instanceCount + 1;
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub(crate) struct SessionStaticField {
    pub(crate) definition: SessionFieldDefinition,
    pub(crate) initializer: Option<AstNode>,
    pub(crate) value: Value,
}

/// Stores metadata and static members for a session (class-like construct).
///
/// Sessions are HypnoScript's OOP construct, similar to classes in other languages.
/// They support:
/// - Instance and static fields
/// - Instance and static methods
/// - Constructors
/// - Visibility modifiers (`expose`/`conceal`)
///
/// # Examples
///
/// ```hyp
/// session BankAccount {
///     conceal balance: number = 0;
///     dominant totalAccounts: number = 0;
///
///     constructor(initialBalance: number) {
///         induce this.balance = initialBalance;
///         induce BankAccount.totalAccounts = BankAccount.totalAccounts + 1;
///     }
///
///     expose suggestion deposit(amount: number) {
///         induce this.balance = this.balance + amount;
///     }
///
///     expose suggestion getBalance() {
///         awaken this.balance;
///     }
///
///     dominant suggestion getTotalAccounts() {
///         awaken BankAccount.totalAccounts;
///     }
/// }
/// ```
#[derive(Debug)]
pub struct SessionDefinition {
    pub(crate) name: String,
    fields: HashMap<String, SessionFieldDefinition>,
    field_order: Vec<String>,
    methods: HashMap<String, SessionMethodDefinition>,
    static_methods: HashMap<String, SessionMethodDefinition>,
    static_fields: RefCell<HashMap<String, SessionStaticField>>,
    static_field_order: Vec<String>,
    constructor: Option<SessionMethodDefinition>,
}

impl SessionDefinition {
    pub(crate) fn new(name: String) -> Self {
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

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn push_field(
        &mut self,
        field: SessionFieldDefinition,
    ) -> Result<(), InterpreterError> {
        if self.fields.contains_key(&field.name)
            || self.static_fields.borrow().contains_key(&field.name)
        {
            return Err(InterpreterError::Runtime(localized(
                &format!(
                    "Duplicate session field '{}' in session '{}'",
                    field.name, self.name
                ),
                &format!(
                    "Duplicate field '{}' in session '{}'",
                    field.name, self.name
                ),
            )));
        }
        self.field_order.push(field.name.clone());
        self.fields.insert(field.name.clone(), field);
        Ok(())
    }

    pub(crate) fn push_static_field(
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
                &format!(
                    "Duplicate field '{}' in session '{}'",
                    field.name, self.name
                ),
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

    pub(crate) fn push_method(
        &mut self,
        method: SessionMethodDefinition,
    ) -> Result<(), InterpreterError> {
        if method.is_constructor {
            if self.constructor.is_some() {
                return Err(InterpreterError::Runtime(localized(
                    &format!("Multiple constructors declared in session '{}'", self.name),
                    &format!("Multiple constructors declared in session '{}'", self.name),
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
                        "Duplicate static method '{}' in session '{}'",
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
                        "Duplicate method '{}' in session '{}'",
                        method.name, self.name
                    ),
                )));
            }
            self.methods.insert(method.name.clone(), method);
        }
        Ok(())
    }

    pub(crate) fn get_field_definition(&self, name: &str) -> Option<&SessionFieldDefinition> {
        self.fields.get(name)
    }

    pub(crate) fn get_method_definition(&self, name: &str) -> Option<&SessionMethodDefinition> {
        self.methods.get(name)
    }

    pub(crate) fn get_static_method_definition(
        &self,
        name: &str,
    ) -> Option<&SessionMethodDefinition> {
        self.static_methods.get(name)
    }

    pub(crate) fn get_static_field_snapshot(&self, name: &str) -> Option<SessionStaticField> {
        self.static_fields.borrow().get(name).cloned()
    }

    pub(crate) fn set_static_field_value(
        &self,
        name: &str,
        value: Value,
    ) -> Result<(), InterpreterError> {
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
                    "Static field '{}' not found on session '{}'",
                    name, self.name
                ),
            ))),
        }
    }

    pub(crate) fn take_static_field_initializer(&self, name: &str) -> Option<AstNode> {
        self.static_fields
            .borrow()
            .get(name)
            .and_then(|field| field.initializer.clone())
    }

    pub(crate) fn field_order(&self) -> &[String] {
        &self.field_order
    }

    pub(crate) fn static_field_order(&self) -> &[String] {
        &self.static_field_order
    }

    pub(crate) fn constructor(&self) -> Option<&SessionMethodDefinition> {
        self.constructor.as_ref()
    }
}

/// Runtime representation of a session instance.
///
/// Each instantiated session creates a `SessionInstance` that holds:
/// - A reference to the session definition (metadata)
/// - Instance-specific field values
///
/// # Examples
///
/// ```hyp
/// session Person {
///     expose name: string = "Unknown";
///     expose age: number = 0;
///
///     constructor(n: string, a: number) {
///         induce this.name = n;
///         induce this.age = a;
///     }
/// }
///
/// // Creates a SessionInstance
/// induce person = Person("Alice", 30);
/// ```
#[derive(Debug)]
pub struct SessionInstance {
    definition: Rc<SessionDefinition>,
    field_values: HashMap<String, Value>,
}

impl SessionInstance {
    pub(crate) fn new(definition: Rc<SessionDefinition>) -> Self {
        let mut field_values = HashMap::new();
        for name in definition.field_order() {
            field_values.insert(name.clone(), Value::Null);
        }
        Self {
            definition,
            field_values,
        }
    }

    pub(crate) fn definition(&self) -> Rc<SessionDefinition> {
        Rc::clone(&self.definition)
    }

    pub(crate) fn definition_name(&self) -> &str {
        self.definition.name()
    }

    pub(crate) fn get_field(&self, name: &str) -> Option<Value> {
        self.field_values.get(name).cloned()
    }

    pub(crate) fn set_field(&mut self, name: &str, value: Value) {
        self.field_values.insert(name.to_string(), value);
    }
}
