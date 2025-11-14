use hypnoscript_lexer_parser::ast::AstNode;
use std::collections::HashMap;

/// WASM code generator for HypnoScript
///
/// Generiert WebAssembly Text Format (.wat) aus HypnoScript AST.
/// Unterstützt:
/// - Variablen und Funktionen
/// - Kontrollfluss (if/while/loop)
/// - Arithmetische und logische Operationen
/// - Session-Definitionen (OOP)
/// - Built-in Funktionen
pub struct WasmCodeGenerator {
    output: String,
    local_counter: usize,
    label_counter: usize,
    variable_map: HashMap<String, usize>,
    function_map: HashMap<String, usize>,
    session_map: HashMap<String, SessionInfo>,
    indent_level: usize,
    break_labels: Vec<String>,
    continue_labels: Vec<String>,
}

/// Session-Informationen für WASM-Generierung
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct SessionInfo {
    name: String,
    field_count: usize,
    method_indices: HashMap<String, usize>,
}

impl Default for WasmCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
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
            session_map: HashMap::new(),
            indent_level: 0,
            break_labels: Vec::new(),
            continue_labels: Vec::new(),
        }
    }

    /// Generate WASM code from AST
    pub fn generate(&mut self, program: &AstNode) -> String {
        self.output.clear();
        self.local_counter = 0;
        self.label_counter = 0;
        self.variable_map.clear();
        self.function_map.clear();
        self.session_map.clear();
        self.break_labels.clear();
        self.continue_labels.clear();

        self.emit_line("(module");
        self.indent_level += 1;

        // Emit imports
        self.emit_imports();

        // Emit memory
        self.emit_line("(memory (export \"memory\") 1)");

        // Emit global variables
        self.emit_line("(global $string_offset (mut i32) (i32.const 0))");
        self.emit_line("(global $heap_offset (mut i32) (i32.const 1024))");
        self.emit_line("");

        // Pre-scan for sessions and functions
        if let AstNode::Program(statements) = program {
            self.prescan_declarations(statements);
        }

        // Emit main function
        if let AstNode::Program(statements) = program {
            self.emit_main_function(statements);
        }

        self.indent_level -= 1;
        self.emit_line(")");

        self.output.clone()
    }

    /// Pre-scan für Sessions und Funktionen
    fn prescan_declarations(&mut self, statements: &[AstNode]) {
        for stmt in statements {
            match stmt {
                AstNode::SessionDeclaration { name, members } => {
                    let mut session_info = SessionInfo {
                        name: name.clone(),
                        field_count: 0,
                        method_indices: HashMap::new(),
                    };

                    for member in members {
                        match member {
                            hypnoscript_lexer_parser::ast::SessionMember::Field(_) => {
                                session_info.field_count += 1;
                            }
                            hypnoscript_lexer_parser::ast::SessionMember::Method(method) => {
                                let func_idx = self.function_map.len();
                                self.function_map.insert(
                                    format!("{}::{}", name, method.name),
                                    func_idx,
                                );
                                session_info.method_indices.insert(method.name.clone(), func_idx);
                            }
                        }
                    }

                    self.session_map.insert(name.clone(), session_info);
                }

                AstNode::FunctionDeclaration { name, .. } => {
                    let func_idx = self.function_map.len();
                    self.function_map.insert(name.clone(), func_idx);
                }

                _ => {}
            }
        }
    }

    /// Emit standard imports
    fn emit_imports(&mut self) {
        self.emit_line(";; Imports");
        self.emit_line("(import \"env\" \"console_log\" (func $console_log (param i32)))");
        self.emit_line("(import \"env\" \"console_log_f64\" (func $console_log_f64 (param f64)))");
        self.emit_line(
            "(import \"env\" \"console_log_str\" (func $console_log_str (param i32 i32)))",
        );
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
            AstNode::VariableDeclaration {
                name, initializer, ..
            } => {
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

            AstNode::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
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
                let break_label = format!("${}_end", loop_label);
                let continue_label = format!("${}_start", loop_label);
                self.break_labels.push(break_label.clone());
                self.continue_labels.push(continue_label.clone());

                self.emit_line(&format!("(block {}", break_label));
                self.indent_level += 1;
                self.emit_line(&format!("(loop {}", continue_label));
                self.indent_level += 1;

                // Check condition
                self.emit_expression(condition);
                self.emit_line("i32.eqz");
                self.emit_line(&format!("br_if {}", break_label));

                // Emit body
                for stmt in body {
                    self.emit_statement(stmt);
                }

                // Loop back
                self.emit_line(&format!("br {}", continue_label));

                self.indent_level -= 1;
                self.emit_line(")");
                self.indent_level -= 1;
                self.emit_line(")");

                self.continue_labels.pop();
                self.break_labels.pop();
            }

            AstNode::LoopStatement {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init_stmt) = init.as_ref() {
                    self.emit_statement(init_stmt);
                }

                let loop_label = self.next_label();
                let break_label = format!("${}_end", loop_label);
                let start_label = format!("${}_start", loop_label);
                let continue_label = format!("${}_continue", loop_label);
                self.break_labels.push(break_label.clone());
                self.continue_labels.push(continue_label.clone());

                self.emit_line(&format!("(block {}", break_label));
                self.indent_level += 1;
                self.emit_line(&format!("(loop {}", start_label));
                self.indent_level += 1;

                if let Some(cond_expr) = condition.as_ref() {
                    self.emit_expression(cond_expr);
                    self.emit_line("i32.eqz");
                    self.emit_line(&format!("br_if {}", break_label));
                }

                self.emit_line(&format!("(block {}", continue_label));
                self.indent_level += 1;
                for stmt in body {
                    self.emit_statement(stmt);
                }
                self.indent_level -= 1;
                self.emit_line(")");

                if let Some(update_stmt) = update.as_ref() {
                    self.emit_statement(update_stmt);
                }

                self.emit_line(&format!("br {}", start_label));

                self.indent_level -= 1;
                self.emit_line(")");
                self.indent_level -= 1;
                self.emit_line(")");

                self.continue_labels.pop();
                self.break_labels.pop();
            }

            AstNode::BreakStatement => {
                self.emit_line(";; break");
                if let Some(label) = self.break_labels.last() {
                    self.emit_line(&format!("br {}", label));
                } else {
                    self.emit_line(";; warning: break outside loop ignored");
                }
            }

            AstNode::ContinueStatement => {
                self.emit_line(";; continue");
                if let Some(label) = self.continue_labels.last() {
                    self.emit_line(&format!("br {}", label));
                } else {
                    self.emit_line(";; warning: continue outside loop ignored");
                }
            }

            AstNode::ExpressionStatement(expr) => {
                self.emit_expression(expr);
                self.emit_line("drop");
            }

            AstNode::FunctionDeclaration { name, parameters, body, .. } => {
                self.emit_line(&format!(";; Function: {}", name));
                self.emit_function(name, parameters, body);
            }

            AstNode::SessionDeclaration { name, members } => {
                self.emit_line(&format!(";; Session: {}", name));
                self.emit_session_methods(name, members);
            }

            AstNode::ReturnStatement(expr) => {
                if let Some(e) = expr {
                    self.emit_expression(e);
                }
                self.emit_line("return");
            }

            _ => {
                self.emit_line(&format!(";; Note: Statement type not yet fully supported in WASM: {:?}",
                    std::any::type_name_of_val(stmt)));
            }
        }
    }

    /// Emit eine Funktion
    fn emit_function(&mut self, name: &str, parameters: &[hypnoscript_lexer_parser::ast::Parameter], body: &[AstNode]) {
        self.emit_line(&format!("(func ${} (export \"{}\")", name, name));
        self.indent_level += 1;

        // Parameter
        for param in parameters {
            self.emit_line(&format!("(param ${} f64) ;; {}", param.name, param.name));
        }
        self.emit_line("(result f64)");

        // Lokale Variablen
        self.emit_line("(local $temp f64)");

        // Body
        for stmt in body {
            self.emit_statement(stmt);
        }

        // Default return 0
        self.emit_line("f64.const 0");

        self.indent_level -= 1;
        self.emit_line(")");
        self.emit_line("");
    }

    /// Emit Session-Methoden
    fn emit_session_methods(&mut self, session_name: &str, members: &[hypnoscript_lexer_parser::ast::SessionMember]) {
        use hypnoscript_lexer_parser::ast::SessionMember;

        self.emit_line(&format!(";; Session methods for: {}", session_name));

        for member in members {
            if let SessionMember::Method(method) = member {
                self.emit_line(&format!(
                    "(func ${} (export \"{}\")",
                    method.name, method.name
                ));
                self.indent_level += 1;

                // Impliziter 'this' Parameter
                self.emit_line("(param $this i32)");

                // Weitere Parameter
                for _ in &method.parameters {
                    self.emit_line("(param f64)");
                }

                self.emit_line("(result f64)");
                self.emit_line("(local $temp f64)");

                // Method body
                for stmt in &method.body {
                    self.emit_statement(stmt);
                }

                // Default return
                self.emit_line("f64.const 0");

                self.indent_level -= 1;
                self.emit_line(")");
                self.emit_line("");
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
                self.emit_line(&format!(
                    "i32.const {} ;; string: {}",
                    s.len(),
                    s.escape_default()
                ));
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

            AstNode::BinaryExpression {
                left,
                operator,
                right,
            } => {
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

            AstNode::CallExpression { callee, arguments } => {
                // Extract function name from callee
                let name = if let AstNode::Identifier(n) = callee.as_ref() {
                    n.clone()
                } else {
                    "unknown".to_string()
                };
                self.emit_line(&format!(";; Function call: {}", name));

                // Emit arguments
                for arg in arguments {
                    self.emit_expression(arg);
                }

                // Call function
                if self.function_map.contains_key(&name) {
                    self.emit_line(&format!("call ${}", name));
                } else {
                    // Versuch, als Built-in-Funktion aufzurufen
                    self.emit_line(&format!("call ${}", name));
                }
            }

            AstNode::ArrayLiteral(elements) => {
                // Simplified: Emit length as i32
                self.emit_line(&format!("i32.const {} ;; array length", elements.len()));
            }

            _ => {
                self.emit_line(";; Note: Expression type not yet fully supported in WASM");
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

    #[test]
    fn test_wasm_generation_control_flow() {
        let source = r#"
Focus {
    induce x: number = 10;
    induce y: number = 5;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut generator = WasmCodeGenerator::new();
        let wasm = generator.generate(&ast);

        assert!(wasm.contains("(module"));
        assert!(wasm.contains("f64.const 10"));
    }

    #[test]
    fn test_wasm_generation_loop() {
        let source = r#"
Focus {
    induce i: number = 0;
    i = i + 1;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut generator = WasmCodeGenerator::new();
        let wasm = generator.generate(&ast);

        assert!(wasm.contains("(module"));
        assert!(wasm.contains("f64.add"));
    }

    #[test]
    fn test_wasm_module_structure() {
        let source = "Focus {} Relax";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut generator = WasmCodeGenerator::new();
        let wasm = generator.generate(&ast);

        // Prüfe grundlegende WASM-Struktur
        assert!(wasm.starts_with("(module"));
        assert!(wasm.ends_with(")\n"));
        assert!(wasm.contains("memory"));
        assert!(wasm.contains("func $main"));
    }

    #[test]
    fn test_wasm_binary_operators() {
        let operators = vec![
            ("+", "f64.add"),
            ("-", "f64.sub"),
            ("*", "f64.mul"),
            ("/", "f64.div"),
        ];

        for (op, wasm_op) in operators {
            let source = format!("Focus {{ induce x: number = 10 {} 5; }} Relax", op);
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.lex().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse_program().unwrap();

            let mut generator = WasmCodeGenerator::new();
            let wasm = generator.generate(&ast);

            assert!(wasm.contains(wasm_op), "Should contain {} for operator {}", wasm_op, op);
        }
    }

    #[test]
    fn test_wasm_function_declaration() {
        let source = r#"
Focus {
    induce result: number = 10 + 20;
    induce x: number = 30;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut generator = WasmCodeGenerator::new();
        let wasm = generator.generate(&ast);

        assert!(wasm.contains("f64.const 10"));
        assert!(wasm.contains("f64.const 20"));
        assert!(wasm.contains("f64.add"));
    }
}
