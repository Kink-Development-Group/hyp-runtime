use crate::ast::{AstNode, Parameter};
use crate::token::{Token, TokenType};

/// Parser for HypnoScript language
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse a complete program
    pub fn parse_program(&mut self) -> Result<AstNode, String> {
        // Program must start with Focus
        if !self.check(&TokenType::Focus) {
            return Err("Program must start with 'Focus'".to_string());
        }
        self.advance();

        // Expect opening brace
        if !self.match_token(&TokenType::LBrace) {
            return Err("Expected '{' after 'Focus'".to_string());
        }

        // Parse program body
        let statements = self.parse_block_statements()?;

        // Expect closing brace
        if !self.match_token(&TokenType::RBrace) {
            return Err("Expected '}' before 'Relax'".to_string());
        }

        // Program must end with Relax
        if !self.check(&TokenType::Relax) {
            return Err("Program must end with 'Relax'".to_string());
        }
        self.advance();

        Ok(AstNode::Program(statements))
    }

    /// Parse block statements
    fn parse_block_statements(&mut self) -> Result<Vec<AstNode>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() && !self.check(&TokenType::RBrace) && !self.check(&TokenType::Relax) {
            // Skip entrance blocks
            if self.match_token(&TokenType::Entrance) {
                if !self.match_token(&TokenType::LBrace) {
                    return Err("Expected '{' after 'entrance'".to_string());
                }
                while !self.is_at_end() && !self.check(&TokenType::RBrace) {
                    statements.push(self.parse_statement()?);
                }
                if !self.match_token(&TokenType::RBrace) {
                    return Err("Expected '}' after entrance block".to_string());
                }
                continue;
            }

            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> Result<AstNode, String> {
        // Variable declaration
        if self.match_token(&TokenType::Induce) {
            return self.parse_var_declaration();
        }

        // If statement
        if self.match_token(&TokenType::If) {
            return self.parse_if_statement();
        }

        // While loop
        if self.match_token(&TokenType::While) {
            return self.parse_while_statement();
        }

        // Loop
        if self.match_token(&TokenType::Loop) {
            return self.parse_loop_statement();
        }

        // Function declaration
        if self.match_token(&TokenType::Suggestion) {
            return self.parse_function_declaration();
        }

        // Session declaration
        if self.match_token(&TokenType::Session) {
            return self.parse_session_declaration();
        }

        // Observe statement
        if self.match_token(&TokenType::Observe) {
            return self.parse_observe_statement();
        }

        // Return statement
        if self.match_token(&TokenType::Awaken) {
            return self.parse_return_statement();
        }

        // Break
        if self.match_token(&TokenType::Snap) {
            self.consume(&TokenType::Semicolon, "Expected ';' after 'snap'")?;
            return Ok(AstNode::BreakStatement);
        }

        // Continue
        if self.match_token(&TokenType::Sink) {
            self.consume(&TokenType::Semicolon, "Expected ';' after 'sink'")?;
            return Ok(AstNode::ContinueStatement);
        }

        // Expression statement
        let expr = self.parse_expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(AstNode::ExpressionStatement(Box::new(expr)))
    }

    /// Parse variable declaration
    fn parse_var_declaration(&mut self) -> Result<AstNode, String> {
        let name = self.consume(&TokenType::Identifier, "Expected variable name")?.lexeme.clone();

        let type_annotation = if self.match_token(&TokenType::Colon) {
            let type_token = self.advance();
            Some(type_token.lexeme.clone())
        } else {
            None
        };

        let initializer = if self.match_token(&TokenType::Equals) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume(&TokenType::Semicolon, "Expected ';' after variable declaration")?;

        Ok(AstNode::VariableDeclaration {
            name,
            type_annotation,
            initializer,
        })
    }

    /// Parse if statement
    fn parse_if_statement(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::LParen, "Expected '(' after 'if'")?;
        let condition = Box::new(self.parse_expression()?);
        self.consume(&TokenType::RParen, "Expected ')' after if condition")?;

        // Check for deepFocus keyword or just a block
        self.match_token(&TokenType::DeepFocus);

        self.consume(&TokenType::LBrace, "Expected '{' after if condition")?;
        let then_branch = self.parse_block_statements()?;
        self.consume(&TokenType::RBrace, "Expected '}' after if block")?;

        let else_branch = if self.match_token(&TokenType::Else) {
            if self.match_token(&TokenType::If) {
                // else if
                Some(vec![self.parse_if_statement()?])
            } else {
                self.consume(&TokenType::LBrace, "Expected '{' after 'else'")?;
                let else_statements = self.parse_block_statements()?;
                self.consume(&TokenType::RBrace, "Expected '}' after else block")?;
                Some(else_statements)
            }
        } else {
            None
        };

        Ok(AstNode::IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parse while statement
    fn parse_while_statement(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::LParen, "Expected '(' after 'while'")?;
        let condition = Box::new(self.parse_expression()?);
        self.consume(&TokenType::RParen, "Expected ')' after while condition")?;

        self.consume(&TokenType::LBrace, "Expected '{' after while condition")?;
        let body = self.parse_block_statements()?;
        self.consume(&TokenType::RBrace, "Expected '}' after while block")?;

        Ok(AstNode::WhileStatement { condition, body })
    }

    /// Parse loop statement
    fn parse_loop_statement(&mut self) -> Result<AstNode, String> {
        self.consume(&TokenType::LBrace, "Expected '{' after 'loop'")?;
        let body = self.parse_block_statements()?;
        self.consume(&TokenType::RBrace, "Expected '}' after loop block")?;

        Ok(AstNode::LoopStatement { body })
    }

    /// Parse function declaration
    fn parse_function_declaration(&mut self) -> Result<AstNode, String> {
        let name = self.consume(&TokenType::Identifier, "Expected function name")?.lexeme.clone();

        self.consume(&TokenType::LParen, "Expected '(' after function name")?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RParen) {
            loop {
                let param_name = self.consume(&TokenType::Identifier, "Expected parameter name")?.lexeme.clone();
                let type_annotation = if self.match_token(&TokenType::Colon) {
                    let type_token = self.advance();
                    Some(type_token.lexeme.clone())
                } else {
                    None
                };
                parameters.push(Parameter::new(param_name, type_annotation));

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RParen, "Expected ')' after parameters")?;

        let return_type = if self.match_token(&TokenType::Colon) {
            let type_token = self.advance();
            Some(type_token.lexeme.clone())
        } else {
            None
        };

        self.consume(&TokenType::LBrace, "Expected '{' after function signature")?;
        let body = self.parse_block_statements()?;
        self.consume(&TokenType::RBrace, "Expected '}' after function body")?;

        Ok(AstNode::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    /// Parse session declaration
    fn parse_session_declaration(&mut self) -> Result<AstNode, String> {
        let name = self.consume(&TokenType::Identifier, "Expected session name")?.lexeme.clone();

        self.consume(&TokenType::LBrace, "Expected '{' after session name")?;
        let members = self.parse_block_statements()?;
        self.consume(&TokenType::RBrace, "Expected '}' after session body")?;

        Ok(AstNode::SessionDeclaration { name, members })
    }

    /// Parse observe statement
    fn parse_observe_statement(&mut self) -> Result<AstNode, String> {
        let expr = Box::new(self.parse_expression()?);
        self.consume(&TokenType::Semicolon, "Expected ';' after observe statement")?;
        Ok(AstNode::ObserveStatement(expr))
    }

    /// Parse return statement
    fn parse_return_statement(&mut self) -> Result<AstNode, String> {
        let value = if !self.check(&TokenType::Semicolon) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        self.consume(&TokenType::Semicolon, "Expected ';' after return statement")?;
        Ok(AstNode::ReturnStatement(value))
    }

    /// Parse expression
    fn parse_expression(&mut self) -> Result<AstNode, String> {
        self.parse_assignment()
    }

    /// Parse assignment
    fn parse_assignment(&mut self) -> Result<AstNode, String> {
        let expr = self.parse_logical_or()?;

        if self.match_token(&TokenType::Equals) {
            let value = Box::new(self.parse_assignment()?);
            return Ok(AstNode::AssignmentExpression {
                target: Box::new(expr),
                value,
            });
        }

        Ok(expr)
    }

    /// Parse logical OR
    fn parse_logical_or(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_logical_and()?;

        while self.match_token(&TokenType::PipePipe) {
            let operator = self.previous().lexeme.clone();
            let right = Box::new(self.parse_logical_and()?);
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right,
            };
        }

        Ok(left)
    }

    /// Parse logical AND
    fn parse_logical_and(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_equality()?;

        while self.match_token(&TokenType::AmpAmp) {
            let operator = self.previous().lexeme.clone();
            let right = Box::new(self.parse_equality()?);
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right,
            };
        }

        Ok(left)
    }

    /// Parse equality
    fn parse_equality(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_comparison()?;

        while self.match_tokens(&[TokenType::DoubleEquals, TokenType::NotEquals, 
                                   TokenType::YouAreFeelingVerySleepy, TokenType::NotSoDeep]) {
            let operator = self.previous().lexeme.clone();
            let right = Box::new(self.parse_comparison()?);
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right,
            };
        }

        Ok(left)
    }

    /// Parse comparison
    fn parse_comparison(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::LookAtTheWatch,
            TokenType::FallUnderMySpell,
            TokenType::DeeplyGreater,
            TokenType::DeeplyLess,
        ]) {
            let operator = self.previous().lexeme.clone();
            let right = Box::new(self.parse_term()?);
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right,
            };
        }

        Ok(left)
    }

    /// Parse term (addition/subtraction)
    fn parse_term(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().lexeme.clone();
            let right = Box::new(self.parse_factor()?);
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right,
            };
        }

        Ok(left)
    }

    /// Parse factor (multiplication/division/modulo)
    fn parse_factor(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_unary()?;

        while self.match_tokens(&[TokenType::Asterisk, TokenType::Slash, TokenType::Percent]) {
            let operator = self.previous().lexeme.clone();
            let right = Box::new(self.parse_unary()?);
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right,
            };
        }

        Ok(left)
    }

    /// Parse unary
    fn parse_unary(&mut self) -> Result<AstNode, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().lexeme.clone();
            let operand = Box::new(self.parse_unary()?);
            return Ok(AstNode::UnaryExpression { operator, operand });
        }

        self.parse_call()
    }

    /// Parse call expression
    fn parse_call(&mut self) -> Result<AstNode, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&TokenType::LParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&TokenType::Dot) {
                let property = self.consume(&TokenType::Identifier, "Expected property name after '.'")?.lexeme.clone();
                expr = AstNode::MemberExpression {
                    object: Box::new(expr),
                    property,
                };
            } else if self.match_token(&TokenType::LBracket) {
                let index = Box::new(self.parse_expression()?);
                self.consume(&TokenType::RBracket, "Expected ']' after array index")?;
                expr = AstNode::IndexExpression {
                    object: Box::new(expr),
                    index,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Finish parsing a call expression
    fn finish_call(&mut self, callee: AstNode) -> Result<AstNode, String> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RParen, "Expected ')' after arguments")?;

        Ok(AstNode::CallExpression {
            callee: Box::new(callee),
            arguments,
        })
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> Result<AstNode, String> {
        // Number literal
        if self.check(&TokenType::NumberLiteral) {
            let token = self.advance();
            let value = token.lexeme.parse::<f64>()
                .map_err(|_| format!("Invalid number: {}", token.lexeme))?;
            return Ok(AstNode::NumberLiteral(value));
        }

        // String literal
        if self.check(&TokenType::StringLiteral) {
            let token = self.advance();
            return Ok(AstNode::StringLiteral(token.lexeme.clone()));
        }

        // Boolean literals
        if self.match_token(&TokenType::True) {
            return Ok(AstNode::BooleanLiteral(true));
        }
        if self.match_token(&TokenType::False) {
            return Ok(AstNode::BooleanLiteral(false));
        }

        // Identifier
        if self.check(&TokenType::Identifier) {
            let token = self.advance();
            return Ok(AstNode::Identifier(token.lexeme.clone()));
        }

        // Array literal
        if self.match_token(&TokenType::LBracket) {
            let mut elements = Vec::new();
            if !self.check(&TokenType::RBracket) {
                loop {
                    elements.push(self.parse_expression()?);
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }
            self.consume(&TokenType::RBracket, "Expected ']' after array elements")?;
            return Ok(AstNode::ArrayLiteral(elements));
        }

        // Grouped expression
        if self.match_token(&TokenType::LParen) {
            let expr = self.parse_expression()?;
            self.consume(&TokenType::RParen, "Expected ')' after expression")?;
            return Ok(expr);
        }

        Err(format!("Unexpected token: {:?}", self.peek()))
    }

    // Helper methods
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} at line {}", message, self.peek().line))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_program() {
        let source = r#"
Focus {
    induce x: number = 42;
    observe x;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_if_statement() {
        let source = r#"
Focus {
    induce x: number = 10;
    if (x > 5) deepFocus {
        observe "Greater";
    }
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program();
        assert!(ast.is_ok());
    }
}
