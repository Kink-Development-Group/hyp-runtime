use hypnoscript_lexer_parser::ast::AstNode;
use hypnoscript_runtime::{
    ArrayBuiltins, CoreBuiltins, FileBuiltins, HashingBuiltins, MathBuiltins, StatisticsBuiltins,
    StringBuiltins, SystemBuiltins, TimeBuiltins, ValidationBuiltins,
};
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
                    .map_err(|e| InterpreterError::Runtime(e))?,
            )),
            "ToString" => Some(Value::String(
                args.get(0)
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
                );
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
