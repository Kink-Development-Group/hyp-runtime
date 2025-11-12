use hypnoscript_lexer_parser::ast::AstNode;
use std::collections::HashMap;

/// WASM code generator for HypnoScript
pub struct WasmCodeGenerator {
    output: String,
    local_counter: usize,
    label_counter: usize,
    variable_map: HashMap<String, usize>,
    function_map: HashMap<String, usize>,
    indent_level: usize,
}

impl WasmCodeGenerator {
    /// Create a new WASM code generator
    pub fn new() -> Self {
        Self {
            output: String::new(),
            local_counter: 0,
            label_counter: 0,
            variable_map: HashMap::new(),
            function_map: HashMap::new(),
            indent_level: 0,
        }
    }

    /// Generate WASM code from AST
    pub fn generate(&mut self, program: &AstNode) -> String {
        self.output.clear();
        self.local_counter = 0;
        self.label_counter = 0;
        self.variable_map.clear();
        self.function_map.clear();

        self.emit_line("(module");
        self.indent_level += 1;

        // Emit imports
        self.emit_imports();

        // Emit memory
        self.emit_line("(memory (export \"memory\") 1)");

        // Emit global variables
        self.emit_line("(global $string_offset (mut i32) (i32.const 0))");
        self.emit_line("(global $heap_offset (mut i32) (i32.const 1024))");

        // Emit main function
        if let AstNode::Program(statements) = program {
            self.emit_main_function(statements);
        }

        self.indent_level -= 1;
        self.emit_line(")");

        self.output.clone()
    }

    /// Emit standard imports
    fn emit_imports(&mut self) {
        self.emit_line(";; Imports");
        self.emit_line("(import \"env\" \"console_log\" (func $console_log (param i32)))");
        self.emit_line("(import \"env\" \"console_log_f64\" (func $console_log_f64 (param f64)))");
        self.emit_line("(import \"env\" \"console_log_str\" (func $console_log_str (param i32 i32)))");
        self.emit_line("(import \"env\" \"drift\" (func $drift (param i32)))");
        self.emit_line("");
    }

    /// Emit main function
    fn emit_main_function(&mut self, statements: &[AstNode]) {
        self.emit_line("(func $main (export \"main\")");
        self.indent_level += 1;

        // Emit local variables
        self.emit_line("(local $temp i32)");
        self.emit_line("(local $temp_f64 f64)");

        // Emit statements
        for stmt in statements {
            self.emit_statement(stmt);
        }

        self.indent_level -= 1;
        self.emit_line(")");
    }

    /// Emit a statement
    fn emit_statement(&mut self, stmt: &AstNode) {
        match stmt {
            AstNode::VariableDeclaration { name, initializer, .. } => {
                let var_idx = self.local_counter;
                self.variable_map.insert(name.clone(), var_idx);
                self.local_counter += 1;

                // Emit local declaration at function level (would need restructuring)
                if let Some(init) = initializer {
                    self.emit_expression(init);
                    self.emit_line(&format!("local.set ${}", var_idx));
                }
            }

            AstNode::ObserveStatement(expr) => {
                self.emit_line(";; observe statement");
                match expr.as_ref() {
                    AstNode::NumberLiteral(_) => {
                        self.emit_expression(expr);
                        self.emit_line("call $console_log_f64");
                    }
                    AstNode::StringLiteral(_) => {
                        self.emit_expression(expr);
                        self.emit_line("call $console_log");
                    }
                    _ => {
                        self.emit_expression(expr);
                        self.emit_line("call $console_log_f64");
                    }
                }
            }

            AstNode::IfStatement { condition, then_branch, else_branch } => {
                self.emit_expression(condition);
                self.emit_line("if");
                self.indent_level += 1;

                for stmt in then_branch {
                    self.emit_statement(stmt);
                }

                if let Some(else_stmts) = else_branch {
                    self.indent_level -= 1;
                    self.emit_line("else");
                    self.indent_level += 1;

                    for stmt in else_stmts {
                        self.emit_statement(stmt);
                    }
                }

                self.indent_level -= 1;
                self.emit_line("end");
            }

            AstNode::WhileStatement { condition, body } => {
                let loop_label = self.next_label();
                self.emit_line(&format!("(block ${}_end", loop_label));
                self.indent_level += 1;
                self.emit_line(&format!("(loop ${}_start", loop_label));
                self.indent_level += 1;

                // Check condition
                self.emit_expression(condition);
                self.emit_line("i32.eqz");
                self.emit_line(&format!("br_if ${}_end", loop_label));

                // Emit body
                for stmt in body {
                    self.emit_statement(stmt);
                }

                // Loop back
                self.emit_line(&format!("br ${}_start", loop_label));

                self.indent_level -= 1;
                self.emit_line(")");
                self.indent_level -= 1;
                self.emit_line(")");
            }

            AstNode::LoopStatement { body } => {
                let loop_label = self.next_label();
                self.emit_line(&format!("(block ${}_end", loop_label));
                self.indent_level += 1;
                self.emit_line(&format!("(loop ${}_start", loop_label));
                self.indent_level += 1;

                for stmt in body {
                    self.emit_statement(stmt);
                }

                self.emit_line(&format!("br ${}_start", loop_label));

                self.indent_level -= 1;
                self.emit_line(")");
                self.indent_level -= 1;
                self.emit_line(")");
            }

            AstNode::BreakStatement => {
                self.emit_line(";; break");
                self.emit_line("br 1");
            }

            AstNode::ContinueStatement => {
                self.emit_line(";; continue");
                self.emit_line("br 0");
            }

            AstNode::ExpressionStatement(expr) => {
                self.emit_expression(expr);
                self.emit_line("drop");
            }

            _ => {
                self.emit_line(&format!(";; Unsupported statement: {:?}", stmt));
            }
        }
    }

    /// Emit an expression
    fn emit_expression(&mut self, expr: &AstNode) {
        match expr {
            AstNode::NumberLiteral(n) => {
                self.emit_line(&format!("f64.const {}", n));
            }

            AstNode::StringLiteral(s) => {
                // For simplicity, emit string length (would need proper string handling)
                self.emit_line(&format!("i32.const {} ;; string: {}", s.len(), s.escape_default()));
            }

            AstNode::BooleanLiteral(b) => {
                self.emit_line(&format!("i32.const {}", if *b { 1 } else { 0 }));
            }

            AstNode::Identifier(name) => {
                if let Some(&idx) = self.variable_map.get(name) {
                    self.emit_line(&format!("local.get ${}", idx));
                } else {
                    self.emit_line(&format!(";; undefined variable: {}", name));
                    self.emit_line("f64.const 0");
                }
            }

            AstNode::BinaryExpression { left, operator, right } => {
                self.emit_expression(left);
                self.emit_expression(right);

                match operator.as_str() {
                    "+" => self.emit_line("f64.add"),
                    "-" => self.emit_line("f64.sub"),
                    "*" => self.emit_line("f64.mul"),
                    "/" => self.emit_line("f64.div"),
                    ">" | "LookAtTheWatch" => {
                        self.emit_line("f64.gt");
                    }
                    "<" | "FallUnderMySpell" => {
                        self.emit_line("f64.lt");
                    }
                    ">=" | "DeeplyGreater" => {
                        self.emit_line("f64.ge");
                    }
                    "<=" | "DeeplyLess" => {
                        self.emit_line("f64.le");
                    }
                    "==" | "YouAreFeelingVerySleepy" => {
                        self.emit_line("f64.eq");
                    }
                    "!=" | "NotSoDeep" => {
                        self.emit_line("f64.ne");
                    }
                    "&&" => {
                        self.emit_line("i32.and");
                    }
                    "||" => {
                        self.emit_line("i32.or");
                    }
                    _ => {
                        self.emit_line(&format!(";; unknown operator: {}", operator));
                    }
                }
            }

            AstNode::UnaryExpression { operator, operand } => {
                self.emit_expression(operand);

                match operator.as_str() {
                    "-" => self.emit_line("f64.neg"),
                    "!" => {
                        self.emit_line("i32.eqz");
                    }
                    _ => {
                        self.emit_line(&format!(";; unknown unary operator: {}", operator));
                    }
                }
            }

            AstNode::AssignmentExpression { target, value } => {
                if let AstNode::Identifier(name) = target.as_ref() {
                    self.emit_expression(value);
                    if let Some(&idx) = self.variable_map.get(name) {
                        self.emit_line(&format!("local.tee ${}", idx));
                    }
                }
            }

            _ => {
                self.emit_line(&format!(";; Unsupported expression: {:?}", expr));
                self.emit_line("f64.const 0");
            }
        }
    }

    /// Emit a line with proper indentation
    fn emit_line(&mut self, line: &str) {
        let indent = "  ".repeat(self.indent_level);
        self.output.push_str(&format!("{}{}\n", indent, line));
    }

    /// Get next label
    fn next_label(&mut self) -> String {
        let label = format!("label{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Get generated WASM code
    pub fn get_output(&self) -> &str {
        &self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hypnoscript_lexer_parser::{Lexer, Parser};

    #[test]
    fn test_wasm_generation_simple() {
        let source = r#"
Focus {
    induce x: number = 42;
    induce y: number = 10;
    observe x;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut generator = WasmCodeGenerator::new();
        let wasm = generator.generate(&ast);

        assert!(wasm.contains("(module"));
        assert!(wasm.contains("func $main"));
        assert!(wasm.contains("f64.const 42"));
    }

    #[test]
    fn test_wasm_generation_arithmetic() {
        let source = r#"
Focus {
    induce result: number = 10 + 20;
    observe result;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut generator = WasmCodeGenerator::new();
        let wasm = generator.generate(&ast);

        assert!(wasm.contains("f64.add"));
    }
}
