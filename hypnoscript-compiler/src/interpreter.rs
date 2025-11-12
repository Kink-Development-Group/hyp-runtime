use hypnoscript_lexer_parser::ast::AstNode;
use hypnoscript_runtime::{CoreBuiltins, MathBuiltins, StringBuiltins};
use std::collections::HashMap;
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

/// Runtime value in HypnoScript
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Function {
        name: String,
        parameters: Vec<String>,
        body: Vec<AstNode>,
    },
    Null,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            _ => true,
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
            Value::Function { name, .. } => write!(f, "<function {}>", name),
        }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
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
        }
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
            } => {
                let value = if let Some(init) = initializer {
                    self.evaluate_expression(init)?
                } else {
                    Value::Null
                };
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
                let func = Value::Function {
                    name: name.clone(),
                    parameters: param_names,
                    body: body.clone(),
                };
                self.set_variable(name.clone(), func);
                Ok(())
            }

            AstNode::SessionDeclaration {
                name: _,
                members: _,
            } => {
                // Sessions not yet fully implemented
                Ok(())
            }

            AstNode::ObserveStatement(expr) => {
                let value = self.evaluate_expression(expr)?;
                CoreBuiltins::observe(&value.to_string());
                Ok(())
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

            AstNode::AssignmentExpression { target, value } => {
                if let AstNode::Identifier(name) = target.as_ref() {
                    let val = self.evaluate_expression(value)?;
                    self.set_variable(name.clone(), val.clone());
                    Ok(val)
                } else {
                    Err(InterpreterError::Runtime(
                        "Invalid assignment target".to_string(),
                    ))
                }
            }

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

            _ => Err(InterpreterError::Runtime(format!(
                "Unsupported expression: {:?}",
                expr
            ))),
        }
    }

    fn evaluate_binary_op(
        &self,
        left: &Value,
        op: &str,
        right: &Value,
    ) -> Result<Value, InterpreterError> {
        match op {
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
            "==" | "YouAreFeelingVerySleepy" => Ok(Value::Boolean(self.values_equal(left, right))),
            "!=" | "NotSoDeep" => Ok(Value::Boolean(!self.values_equal(left, right))),
            ">" | "LookAtTheWatch" => Ok(Value::Boolean(left.to_number()? > right.to_number()?)),
            "<" | "FallUnderMySpell" => Ok(Value::Boolean(left.to_number()? < right.to_number()?)),
            ">=" | "DeeplyGreater" => Ok(Value::Boolean(left.to_number()? >= right.to_number()?)),
            "<=" | "DeeplyLess" => Ok(Value::Boolean(left.to_number()? <= right.to_number()?)),
            "&&" => Ok(Value::Boolean(left.is_truthy() && right.is_truthy())),
            "||" => Ok(Value::Boolean(left.is_truthy() || right.is_truthy())),
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
        if let AstNode::Identifier(name) = callee {
            // Evaluate arguments
            let mut args = Vec::new();
            for arg in arguments {
                args.push(self.evaluate_expression(arg)?);
            }

            // Try builtin functions first
            if let Some(result) = self.call_builtin(name, &args)? {
                return Ok(result);
            }

            // Try user-defined functions
            if let Ok(Value::Function {
                parameters, body, ..
            }) = self.get_variable(name)
            {
                return self.call_user_function(&parameters, &body, &args);
            }

            Err(InterpreterError::UndefinedVariable(name.clone()))
        } else {
            Err(InterpreterError::Runtime(
                "Cannot call non-identifier".to_string(),
            ))
        }
    }

    fn call_builtin(&self, name: &str, args: &[Value]) -> Result<Option<Value>, InterpreterError> {
        match name {
            // Math builtins
            "Sin" => Ok(Some(Value::Number(MathBuiltins::sin(args[0].to_number()?)))),
            "Cos" => Ok(Some(Value::Number(MathBuiltins::cos(args[0].to_number()?)))),
            "Tan" => Ok(Some(Value::Number(MathBuiltins::tan(args[0].to_number()?)))),
            "Sqrt" => Ok(Some(Value::Number(MathBuiltins::sqrt(
                args[0].to_number()?,
            )))),
            "Abs" => Ok(Some(Value::Number(MathBuiltins::abs(args[0].to_number()?)))),
            "Floor" => Ok(Some(Value::Number(MathBuiltins::floor(
                args[0].to_number()?,
            )))),
            "Ceil" => Ok(Some(Value::Number(MathBuiltins::ceil(
                args[0].to_number()?,
            )))),
            "Round" => Ok(Some(Value::Number(MathBuiltins::round(
                args[0].to_number()?,
            )))),
            "Min" => Ok(Some(Value::Number(MathBuiltins::min(
                args[0].to_number()?,
                args[1].to_number()?,
            )))),
            "Max" => Ok(Some(Value::Number(MathBuiltins::max(
                args[0].to_number()?,
                args[1].to_number()?,
            )))),
            "Pow" => Ok(Some(Value::Number(MathBuiltins::pow(
                args[0].to_number()?,
                args[1].to_number()?,
            )))),
            "Factorial" => Ok(Some(Value::Number(
                MathBuiltins::factorial(args[0].to_number()? as i64) as f64,
            ))),

            // String builtins
            "Length" if args.len() == 1 => {
                if let Value::String(s) = &args[0] {
                    Ok(Some(Value::Number(StringBuiltins::length(s) as f64)))
                } else {
                    Ok(None)
                }
            }
            "ToUpper" => {
                if let Value::String(s) = &args[0] {
                    Ok(Some(Value::String(StringBuiltins::to_upper(s))))
                } else {
                    Ok(None)
                }
            }
            "ToLower" => {
                if let Value::String(s) = &args[0] {
                    Ok(Some(Value::String(StringBuiltins::to_lower(s))))
                } else {
                    Ok(None)
                }
            }
            "Reverse" => {
                if let Value::String(s) = &args[0] {
                    Ok(Some(Value::String(StringBuiltins::reverse(s))))
                } else {
                    Ok(None)
                }
            }

            // Core builtins
            "ToInt" => Ok(Some(Value::Number(
                CoreBuiltins::to_int(args[0].to_number()?) as f64,
            ))),
            "ToString" => Ok(Some(Value::String(args[0].to_string()))),

            _ => Ok(None),
        }
    }

    fn call_user_function(
        &mut self,
        parameters: &[String],
        body: &[AstNode],
        args: &[Value],
    ) -> Result<Value, InterpreterError> {
        if parameters.len() != args.len() {
            return Err(InterpreterError::Runtime(format!(
                "Expected {} arguments, got {}",
                parameters.len(),
                args.len()
            )));
        }

        self.push_scope();

        // Bind parameters
        for (param, arg) in parameters.iter().zip(args.iter()) {
            self.set_variable(param.clone(), arg.clone());
        }

        // Execute function body
        let result = (|| {
            for stmt in body {
                self.execute_statement(stmt)?;
            }
            Ok(Value::Null)
        })();

        self.pop_scope();

        match result {
            Err(InterpreterError::Return(val)) => Ok(val),
            Err(e) => Err(e),
            Ok(val) => Ok(val),
        }
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
}
