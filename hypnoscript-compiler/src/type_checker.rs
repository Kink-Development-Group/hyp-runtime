use hypnoscript_core::{HypnoType, HypnoBaseType};
use hypnoscript_lexer_parser::ast::AstNode;
use std::collections::HashMap;

/// Type checker for HypnoScript programs
pub struct TypeChecker {
    // Type environment for variables
    type_env: HashMap<String, HypnoType>,
    // Function signatures
    function_types: HashMap<String, (Vec<HypnoType>, HypnoType)>,
    // Current function return type (for return statement checking)
    current_function_return_type: Option<HypnoType>,
    // Error messages
    errors: Vec<String>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        let mut checker = Self {
            type_env: HashMap::new(),
            function_types: HashMap::new(),
            current_function_return_type: None,
            errors: Vec::new(),
        };
        
        // Register builtin functions
        checker.register_builtins();
        
        checker
    }

    /// Register builtin function signatures
    fn register_builtins(&mut self) {
        let number = HypnoType::number();
        let string = HypnoType::string();
        let boolean = HypnoType::boolean();
        
        // Math builtins
        self.function_types.insert("Sin".to_string(), (vec![number.clone()], number.clone()));
        self.function_types.insert("Cos".to_string(), (vec![number.clone()], number.clone()));
        self.function_types.insert("Sqrt".to_string(), (vec![number.clone()], number.clone()));
        self.function_types.insert("Min".to_string(), (vec![number.clone(), number.clone()], number.clone()));
        self.function_types.insert("Max".to_string(), (vec![number.clone(), number.clone()], number.clone()));
        
        // String builtins
        self.function_types.insert("Length".to_string(), (vec![string.clone()], number.clone()));
        self.function_types.insert("ToUpper".to_string(), (vec![string.clone()], string.clone()));
        self.function_types.insert("Reverse".to_string(), (vec![string.clone()], string.clone()));
        
        // Validation
        self.function_types.insert("IsValidEmail".to_string(), (vec![string.clone()], boolean.clone()));
        
        // Conversions
        self.function_types.insert("ToInt".to_string(), (vec![number.clone()], number.clone()));
        self.function_types.insert("ToString".to_string(), (vec![number.clone()], string.clone()));
    }

    /// Parse type annotation string to HypnoType
    fn parse_type_annotation(&self, type_str: Option<&str>) -> HypnoType {
        match type_str {
            Some("number") => HypnoType::number(),
            Some("string") => HypnoType::string(),
            Some("boolean") => HypnoType::boolean(),
            Some("trance") => HypnoType::new(HypnoBaseType::Trance, None),
            _ => HypnoType::unknown(),
        }
    }

    /// Check a program and return errors
    pub fn check_program(&mut self, program: &AstNode) -> Vec<String> {
        self.errors.clear();
        
        if let AstNode::Program(statements) = program {
            // First pass: collect function declarations
            for stmt in statements {
                self.collect_function_signature(stmt);
            }
            
            // Second pass: type check all statements
            for stmt in statements {
                self.check_statement(stmt);
            }
        } else {
            self.errors.push("Expected program node".to_string());
        }
        
        self.errors.clone()
    }

    /// Collect function signatures
    fn collect_function_signature(&mut self, stmt: &AstNode) {
        if let AstNode::FunctionDeclaration { name, parameters, return_type, .. } = stmt {
            let param_types: Vec<HypnoType> = parameters.iter()
                .map(|p| self.parse_type_annotation(p.type_annotation.as_deref()))
                .collect();
            
            let ret_type = self.parse_type_annotation(return_type.as_deref());
            
            self.function_types.insert(name.clone(), (param_types, ret_type));
        }
    }

    /// Check a statement
    fn check_statement(&mut self, stmt: &AstNode) {
        match stmt {
            AstNode::VariableDeclaration { name, type_annotation, initializer } => {
                let expected_type = self.parse_type_annotation(type_annotation.as_deref());
                
                if let Some(init) = initializer {
                    let actual_type = self.infer_type(init);
                    
                    if !self.types_compatible(&expected_type, &actual_type) {
                        self.errors.push(format!(
                            "Type mismatch for variable '{}': expected {}, got {}",
                            name, expected_type, actual_type
                        ));
                    }
                }
                
                self.type_env.insert(name.clone(), expected_type);
            }

            AstNode::FunctionDeclaration { parameters, return_type, body, .. } => {
                let old_env = self.type_env.clone();
                let ret_type = self.parse_type_annotation(return_type.as_deref());
                self.current_function_return_type = Some(ret_type);
                
                for param in parameters {
                    let param_type = self.parse_type_annotation(param.type_annotation.as_deref());
                    self.type_env.insert(param.name.clone(), param_type);
                }
                
                for stmt in body {
                    self.check_statement(stmt);
                }
                
                self.type_env = old_env;
                self.current_function_return_type = None;
            }

            AstNode::IfStatement { condition, then_branch, else_branch } => {
                let cond_type = self.infer_type(condition);
                if cond_type.base_type != HypnoBaseType::Boolean {
                    self.errors.push(format!(
                        "If condition must be boolean, got {}",
                        cond_type
                    ));
                }
                
                for stmt in then_branch {
                    self.check_statement(stmt);
                }
                
                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.check_statement(stmt);
                    }
                }
            }

            AstNode::WhileStatement { condition, body } => {
                let cond_type = self.infer_type(condition);
                if cond_type.base_type != HypnoBaseType::Boolean {
                    self.errors.push(format!(
                        "While condition must be boolean, got {}",
                        cond_type
                    ));
                }
                
                for stmt in body {
                    self.check_statement(stmt);
                }
            }

            AstNode::LoopStatement { body } => {
                for stmt in body {
                    self.check_statement(stmt);
                }
            }

            AstNode::ReturnStatement(value) => {
                if let Some(val) = value {
                    let actual_type = self.infer_type(val);
                    if let Some(ret_type) = &self.current_function_return_type {
                        if !self.types_compatible(ret_type, &actual_type) {
                            self.errors.push(format!(
                                "Return type mismatch: expected {}, got {}",
                                ret_type, actual_type
                            ));
                        }
                    }
                }
            }

            AstNode::ExpressionStatement(expr) |
            AstNode::ObserveStatement(expr) => {
                self.infer_type(expr);
            }

            _ => {}
        }
    }

    /// Infer the type of an expression
    fn infer_type(&mut self, expr: &AstNode) -> HypnoType {
        match expr {
            AstNode::NumberLiteral(_) => HypnoType::number(),
            AstNode::StringLiteral(_) => HypnoType::string(),
            AstNode::BooleanLiteral(_) => HypnoType::boolean(),
            
            AstNode::Identifier(name) => {
                self.type_env.get(name)
                    .cloned()
                    .unwrap_or_else(|| {
                        self.errors.push(format!("Undefined variable '{}'", name));
                        HypnoType::unknown()
                    })
            }

            AstNode::BinaryExpression { left, operator, right } => {
                let left_type = self.infer_type(left);
                let right_type = self.infer_type(right);
                
                match operator.as_str() {
                    "+" | "-" | "*" | "/" | "%" => {
                        if left_type.base_type != HypnoBaseType::Number || right_type.base_type != HypnoBaseType::Number {
                            self.errors.push(format!(
                                "Arithmetic operation requires numeric operands, got {} and {}",
                                left_type, right_type
                            ));
                        }
                        HypnoType::number()
                    }
                    "==" | "!=" | ">" | "<" | ">=" | "<=" | 
                    "YouAreFeelingVerySleepy" | "NotSoDeep" | 
                    "LookAtTheWatch" | "FallUnderMySpell" | 
                    "DeeplyGreater" | "DeeplyLess" => {
                        HypnoType::boolean()
                    }
                    "&&" | "||" => {
                        if left_type.base_type != HypnoBaseType::Boolean || right_type.base_type != HypnoBaseType::Boolean {
                            self.errors.push(format!(
                                "Logical operation requires boolean operands, got {} and {}",
                                left_type, right_type
                            ));
                        }
                        HypnoType::boolean()
                    }
                    _ => HypnoType::unknown()
                }
            }

            AstNode::UnaryExpression { operator, operand } => {
                let operand_type = self.infer_type(operand);
                
                match operator.as_str() {
                    "-" => {
                        if operand_type.base_type != HypnoBaseType::Number {
                            self.errors.push(format!(
                                "Unary minus requires numeric operand, got {}",
                                operand_type
                            ));
                        }
                        HypnoType::number()
                    }
                    "!" => {
                        if operand_type.base_type != HypnoBaseType::Boolean {
                            self.errors.push(format!(
                                "Logical not requires boolean operand, got {}",
                                operand_type
                            ));
                        }
                        HypnoType::boolean()
                    }
                    _ => HypnoType::unknown()
                }
            }

            AstNode::CallExpression { callee, arguments } => {
                if let AstNode::Identifier(func_name) = callee.as_ref() {
                    // Clone the function signature to avoid borrow conflicts
                    let func_sig = self.function_types.get(func_name).cloned();
                    
                    if let Some((param_types, return_type)) = func_sig {
                        if arguments.len() != param_types.len() {
                            self.errors.push(format!(
                                "Function '{}' expects {} arguments, got {}",
                                func_name, param_types.len(), arguments.len()
                            ));
                        } else {
                            for (i, (arg, expected_type)) in arguments.iter().zip(param_types.iter()).enumerate() {
                                let actual_type = self.infer_type(arg);
                                if !self.types_compatible(expected_type, &actual_type) {
                                    self.errors.push(format!(
                                        "Function '{}' argument {} type mismatch: expected {}, got {}",
                                        func_name, i + 1, expected_type, actual_type
                                    ));
                                }
                            }
                        }
                        
                        return return_type;
                    } else {
                        self.errors.push(format!("Undefined function '{}'", func_name));
                    }
                }
                
                HypnoType::unknown()
            }

            AstNode::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    HypnoType::create_array(HypnoType::unknown())
                } else {
                    let first_type = self.infer_type(&elements[0]);
                    for elem in &elements[1..] {
                        let elem_type = self.infer_type(elem);
                        if !self.types_compatible(&first_type, &elem_type) {
                            self.errors.push(format!(
                                "Array elements must have same type, got {} and {}",
                                first_type, elem_type
                            ));
                        }
                    }
                    HypnoType::create_array(first_type)
                }
            }

            _ => HypnoType::unknown()
        }
    }

    /// Check if two types are compatible
    fn types_compatible(&self, expected: &HypnoType, actual: &HypnoType) -> bool {
        if expected.base_type == HypnoBaseType::Unknown || actual.base_type == HypnoBaseType::Unknown {
            return true;
        }
        expected.is_compatible_with(actual)
    }

    /// Get all errors
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hypnoscript_lexer_parser::{Lexer, Parser};

    #[test]
    fn test_type_check_simple() {
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

        let mut checker = TypeChecker::new();
        let errors = checker.check_program(&ast);
        assert!(errors.is_empty(), "Errors: {:?}", errors);
    }

    #[test]
    fn test_type_check_mismatch() {
        let source = r#"
Focus {
    induce x: number = "hello";
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut checker = TypeChecker::new();
        let errors = checker.check_program(&ast);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("Type mismatch"));
    }
}
