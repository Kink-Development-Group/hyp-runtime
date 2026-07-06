use crate::ast::{
    AstNode, EntrainCase, Parameter, Pattern, RecordFieldInit, RecordFieldPattern, SessionField,
    SessionMember, SessionMethod, SessionVisibility, TranceifyField, VariableStorage,
};
use crate::error::SyntaxError;
use crate::token::{Token, TokenType};

/// Result alias for parsing operations.
pub type ParseResult<T> = Result<T, SyntaxError>;

/// Parser for HypnoScript language.
///
/// Converts a stream of tokens into an Abstract Syntax Tree (AST).
/// Uses recursive descent parsing with operator precedence for expressions.
/// All errors are reported as [`SyntaxError`]s carrying the line and column
/// of the offending token.
///
/// # Supported Language Constructs
///
/// - **Program structure**: `Focus { ... } Relax`
/// - **Variables**: `induce`, `implant`, `embed`, `freeze`
/// - **Functions**: `suggestion`, `trigger`, `imperative suggestion`
/// - **Sessions (OOP)**: `session`, `constructor`, `expose`, `conceal`, `dominant`
/// - **Records**: `tranceify` declarations
/// - **Control flow**: `if`/`else`, `while`, `loop`, `pendulum`, `snap`, `sink`
/// - **Pattern matching**: `entrain`/`when`/`otherwise`
/// - **Async**: `mesmerize`, `await`, `surrenderTo`
/// - **Operators**: Standard + hypnotic synonyms
/// - **Nullish operators**: `lucidFallback` (`??`), `dreamReach` (`?.`)
///
/// # Examples
///
/// ```rust
/// use hypnoscript_lexer_parser::{Parser, Lexer};
///
/// let source = r#"
///     Focus {
///         entrance {
///             induce x = 42;
///             observe x;
///         }
///     } Relax;
/// "#;
///
/// let mut lexer = Lexer::new(source);
/// let tokens = lexer.lex().unwrap();
/// let mut parser = Parser::new(tokens);
/// let ast = parser.parse_program().unwrap();
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BlockContext {
    Program,
    Regular,
}

type LoopHeaderComponents = (
    Option<Box<AstNode>>,
    Option<Box<AstNode>>,
    Option<Box<AstNode>>,
);

/// Keywords that introduce a variable declaration.
const DECLARATION_KEYWORDS: [TokenType; 4] = [
    TokenType::Induce,
    TokenType::Implant,
    TokenType::Embed,
    TokenType::Freeze,
];

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse a complete program
    pub fn parse_program(&mut self) -> ParseResult<AstNode> {
        // Program must start with Focus
        if !self.check(&TokenType::Focus) {
            return Err(self.error_here("Program must start with 'Focus'"));
        }
        self.advance();

        // Expect opening brace
        if !self.match_token(&TokenType::LBrace) {
            return Err(self.error_here("Expected '{' after 'Focus'"));
        }

        // Parse program body
        let statements = self.parse_block_statements(BlockContext::Program)?;

        // Expect closing brace
        if !self.match_token(&TokenType::RBrace) {
            return Err(self.error_here("Expected '}' before 'Relax'"));
        }

        // Program must end with Relax
        if !self.check(&TokenType::Relax) {
            return Err(self.error_here("Program must end with 'Relax'"));
        }
        self.advance();

        Ok(AstNode::Program(statements))
    }

    /// Parse block statements
    fn parse_block_statements(&mut self, context: BlockContext) -> ParseResult<Vec<AstNode>> {
        let mut statements = Vec::new();

        while !self.is_at_end() && !self.check(&TokenType::RBrace) && !self.check(&TokenType::Relax)
        {
            // entrance block (constructor/setup)
            if self.match_token(&TokenType::Entrance) {
                statements.push(self.parse_top_level_block(
                    context,
                    "entrance",
                    AstNode::EntranceBlock,
                )?);
                continue;
            }

            // finale block (destructor/cleanup)
            if self.match_token(&TokenType::Finale) {
                statements.push(self.parse_top_level_block(
                    context,
                    "finale",
                    AstNode::FinaleBlock,
                )?);
                continue;
            }

            statements.push(self.parse_statement(context)?);
        }

        Ok(statements)
    }

    /// Parse an `entrance`/`finale` block, which is only valid at the top
    /// level of a program.
    fn parse_top_level_block(
        &mut self,
        context: BlockContext,
        keyword: &str,
        constructor: fn(Vec<AstNode>) -> AstNode,
    ) -> ParseResult<AstNode> {
        if context != BlockContext::Program {
            return Err(self.error_here(format!(
                "'{}' blocks are only allowed at the top level",
                keyword
            )));
        }
        if !self.match_token(&TokenType::LBrace) {
            return Err(self.error_here(format!("Expected '{{' after '{}'", keyword)));
        }
        let mut block_statements = Vec::new();
        while !self.is_at_end() && !self.check(&TokenType::RBrace) {
            block_statements.push(self.parse_statement(BlockContext::Regular)?);
        }
        if !self.match_token(&TokenType::RBrace) {
            return Err(self.error_here(format!("Expected '}}' after {} block", keyword)));
        }
        Ok(constructor(block_statements))
    }

    /// Parse a single statement
    fn parse_statement(&mut self, context: BlockContext) -> ParseResult<AstNode> {
        // Variable declaration - induce, implant, embed, freeze
        if self.match_token(&TokenType::SharedTrance) {
            if self.match_tokens(&DECLARATION_KEYWORDS) {
                return self.parse_var_declaration(VariableStorage::SharedTrance);
            }

            return Err(
                self.error_here("'sharedTrance' must be followed by induce/implant/embed/freeze")
            );
        }

        if self.match_tokens(&DECLARATION_KEYWORDS) {
            return self.parse_var_declaration(VariableStorage::Local);
        }

        // Anchor declaration - saves variable state
        if self.match_token(&TokenType::Anchor) {
            return self.parse_anchor_declaration();
        }

        // If statement
        if self.match_token(&TokenType::If) {
            return self.parse_if_statement();
        }

        // While loop
        if self.match_token(&TokenType::While) {
            return self.parse_while_statement();
        }

        // Loop (modern for-loop syntax)
        if self.match_token(&TokenType::Loop) {
            return self.parse_loop_statement("loop", false, false);
        }

        // Pendulum loop (alias for loop syntax, header required)
        if self.match_token(&TokenType::Pendulum) {
            return self.parse_loop_statement("pendulum", true, true);
        }

        // Suspend statement (infinite pause)
        if self.match_token(&TokenType::Suspend) {
            self.consume(&TokenType::Semicolon, "Expected ';' after 'suspend'")?;
            return Ok(AstNode::SuspendStatement);
        }

        // Function declaration
        if self.match_token(&TokenType::Suggestion) {
            return self.parse_function_declaration();
        }

        // Trigger declaration (event handler/callback)
        if self.match_token(&TokenType::Trigger) {
            if context != BlockContext::Program {
                return Err(self.error_here("Triggers can only be declared at the top level"));
            }
            return self.parse_trigger_declaration();
        }

        // Session declaration
        if self.match_token(&TokenType::Session) {
            return self.parse_session_declaration();
        }

        // Tranceify declaration (record/struct type)
        if self.match_token(&TokenType::Tranceify) {
            return self.parse_tranceify_declaration();
        }

        // Output statements
        if self.match_token(&TokenType::Observe) {
            return self.parse_output_statement("observe", AstNode::ObserveStatement);
        }

        if self.match_token(&TokenType::Whisper) {
            return self.parse_output_statement("whisper", AstNode::WhisperStatement);
        }

        if self.match_token(&TokenType::Command) {
            return self.parse_output_statement("command", AstNode::CommandStatement);
        }

        // Murmur statement (quiet/debug output)
        if self.match_token(&TokenType::Murmur) {
            return self.parse_output_statement("murmur", AstNode::MurmurStatement);
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

        // Oscillate statement (toggle boolean)
        if self.match_token(&TokenType::Oscillate) {
            return self.parse_oscillate_statement();
        }

        // Expression statement
        let expr = self.parse_expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(AstNode::ExpressionStatement(Box::new(expr)))
    }

    /// Parse an output statement (`observe`, `whisper`, `command`, `murmur`).
    fn parse_output_statement(
        &mut self,
        keyword: &str,
        constructor: fn(Box<AstNode>) -> AstNode,
    ) -> ParseResult<AstNode> {
        let expr = Box::new(self.parse_expression()?);
        self.consume(
            &TokenType::Semicolon,
            &format!("Expected ';' after {}", keyword),
        )?;
        Ok(constructor(expr))
    }

    /// Parse variable declaration (induce/implant/embed/freeze).
    /// The declaration keyword has already been consumed.
    /// - induce/implant/embed: variables (like let/var)
    /// - freeze: constant (like const)
    fn parse_var_declaration(&mut self, storage: VariableStorage) -> ParseResult<AstNode> {
        let declaration = self.parse_var_declaration_body(storage)?;
        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;
        Ok(declaration)
    }

    /// Parse the body of a variable declaration (name, optional type
    /// annotation, optional initializer) without the trailing semicolon.
    /// Shared between statements and loop initializers.
    fn parse_var_declaration_body(&mut self, storage: VariableStorage) -> ParseResult<AstNode> {
        // Determine if this is a constant (freeze) or variable (induce/implant/embed)
        let is_constant = self.previous().token_type == TokenType::Freeze;

        let name = self
            .consume(&TokenType::Identifier, "Expected variable name")?
            .lexeme
            .clone();

        let type_annotation = self.parse_optional_type_annotation();

        let initializer = if self.match_token(&TokenType::Equals) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        Ok(AstNode::VariableDeclaration {
            name,
            type_annotation,
            initializer,
            is_constant,
            storage,
        })
    }

    /// Parse anchor declaration (saves variable state)
    /// Example: anchor savedValue = currentValue;
    fn parse_anchor_declaration(&mut self) -> ParseResult<AstNode> {
        let name = self
            .consume(&TokenType::Identifier, "Expected anchor name")?
            .lexeme
            .clone();

        self.consume(&TokenType::Equals, "Expected '=' after anchor name")?;

        let source = Box::new(self.parse_expression()?);

        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after anchor declaration",
        )?;

        Ok(AstNode::AnchorDeclaration { name, source })
    }

    /// Parse oscillate statement (toggle boolean)
    /// Example: oscillate myFlag;
    fn parse_oscillate_statement(&mut self) -> ParseResult<AstNode> {
        let target = Box::new(self.parse_primary()?);

        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after oscillate statement",
        )?;

        Ok(AstNode::OscillateStatement { target })
    }

    /// Parse trigger declaration (event handler/callback)
    fn parse_trigger_declaration(&mut self) -> ParseResult<AstNode> {
        let name = self
            .consume(&TokenType::Identifier, "Expected trigger name")?
            .lexeme
            .clone();

        self.consume(&TokenType::Equals, "Expected '=' after trigger name")?;

        // Expect 'suggestion' keyword for the function body
        self.consume(&TokenType::Suggestion, "Expected 'suggestion' after '='")?;

        self.consume(&TokenType::LParen, "Expected '(' after 'suggestion'")?;
        let parameters = self.parse_parameter_list()?;
        self.consume(&TokenType::RParen, "Expected ')' after parameters")?;

        let return_type = self.parse_optional_type_annotation();

        // Parse body
        self.consume(&TokenType::LBrace, "Expected '{' before trigger body")?;
        let body = self.parse_block_statements(BlockContext::Regular)?;
        self.consume(&TokenType::RBrace, "Expected '}' after trigger body")?;

        Ok(AstNode::TriggerDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    /// Parse if statement
    fn parse_if_statement(&mut self) -> ParseResult<AstNode> {
        self.consume(&TokenType::LParen, "Expected '(' after 'if'")?;
        let condition = Box::new(self.parse_expression()?);
        self.consume(&TokenType::RParen, "Expected ')' after if condition")?;

        // Check for deepFocus keyword or just a block
        self.match_token(&TokenType::DeepFocus);

        self.consume(&TokenType::LBrace, "Expected '{' after if condition")?;
        let then_branch = self.parse_block_statements(BlockContext::Regular)?;
        self.consume(&TokenType::RBrace, "Expected '}' after if block")?;

        let else_branch = if self.match_token(&TokenType::Else) {
            if self.match_token(&TokenType::If) {
                // else if
                Some(vec![self.parse_if_statement()?])
            } else {
                self.consume(&TokenType::LBrace, "Expected '{' after 'else'")?;
                let else_statements = self.parse_block_statements(BlockContext::Regular)?;
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
    fn parse_while_statement(&mut self) -> ParseResult<AstNode> {
        self.consume(&TokenType::LParen, "Expected '(' after 'while'")?;
        let condition = Box::new(self.parse_expression()?);
        self.consume(&TokenType::RParen, "Expected ')' after while condition")?;

        self.consume(&TokenType::LBrace, "Expected '{' after while condition")?;
        let body = self.parse_block_statements(BlockContext::Regular)?;
        self.consume(&TokenType::RBrace, "Expected '}' after while block")?;

        Ok(AstNode::WhileStatement { condition, body })
    }

    /// Parse loop/pendulum statements (C-style for loop)
    fn parse_loop_statement(
        &mut self,
        keyword: &str,
        require_header: bool,
        require_condition: bool,
    ) -> ParseResult<AstNode> {
        let has_header = if self.match_token(&TokenType::LParen) {
            true
        } else {
            if require_header {
                return Err(self.error_here(format!("Expected '(' after '{}'", keyword)));
            }
            false
        };

        let (init, condition, update) = if has_header {
            self.parse_loop_header(keyword, require_condition)?
        } else {
            (None, None, None)
        };

        self.consume(
            &TokenType::LBrace,
            &format!("Expected '{{' after '{}' loop header", keyword),
        )?;
        let body = self.parse_block_statements(BlockContext::Regular)?;
        self.consume(
            &TokenType::RBrace,
            &format!("Expected '}}' after '{}' loop block", keyword),
        )?;

        Ok(AstNode::LoopStatement {
            init,
            condition,
            update,
            body,
        })
    }

    fn parse_loop_header(
        &mut self,
        keyword: &str,
        require_condition: bool,
    ) -> ParseResult<LoopHeaderComponents> {
        // Parse init (variable declaration or expression)
        let init = if self.check(&TokenType::Semicolon) {
            None
        } else {
            self.parse_loop_init_statement()?
        };

        self.consume(
            &TokenType::Semicolon,
            &format!("Expected ';' after '{}' loop initializer", keyword),
        )?;

        // Parse condition (optional for legacy loop syntax)
        let condition = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        if require_condition && condition.is_none() {
            return Err(
                self.error_here(format!("{} loop requires a condition expression", keyword))
            );
        }

        self.consume(
            &TokenType::Semicolon,
            &format!("Expected ';' after '{}' loop condition", keyword),
        )?;

        // Parse update (optional expression)
        let update = if self.check(&TokenType::RParen) {
            None
        } else {
            let expr = self.parse_expression()?;
            Some(Box::new(AstNode::ExpressionStatement(Box::new(expr))))
        };

        self.consume(
            &TokenType::RParen,
            &format!("Expected ')' after '{}' loop clauses", keyword),
        )?;

        Ok((init, condition, update))
    }

    fn parse_loop_init_statement(&mut self) -> ParseResult<Option<Box<AstNode>>> {
        if self.match_tokens(&DECLARATION_KEYWORDS) {
            let declaration = self.parse_var_declaration_body(VariableStorage::Local)?;
            return Ok(Some(Box::new(declaration)));
        }

        if self.check(&TokenType::Semicolon) {
            return Ok(None);
        }

        let expr = self.parse_expression()?;
        Ok(Some(Box::new(AstNode::ExpressionStatement(Box::new(expr)))))
    }

    /// Parse function declaration
    fn parse_function_declaration(&mut self) -> ParseResult<AstNode> {
        let name = self
            .consume(&TokenType::Identifier, "Expected function name")?
            .lexeme
            .clone();

        self.consume(&TokenType::LParen, "Expected '(' after function name")?;
        let parameters = self.parse_parameter_list()?;
        self.consume(&TokenType::RParen, "Expected ')' after parameters")?;

        let return_type = self.parse_optional_type_annotation();

        self.consume(&TokenType::LBrace, "Expected '{' after function signature")?;
        let body = self.parse_block_statements(BlockContext::Regular)?;
        self.consume(&TokenType::RBrace, "Expected '}' after function body")?;

        Ok(AstNode::FunctionDeclaration {
            name,
            parameters,
            return_type,
            body,
        })
    }

    /// Parse a comma-separated parameter list (without the surrounding
    /// parentheses). Each parameter is a name with an optional `: type`
    /// annotation.
    fn parse_parameter_list(&mut self) -> ParseResult<Vec<Parameter>> {
        let mut parameters = Vec::new();

        if !self.check(&TokenType::RParen) {
            loop {
                let param_name = self
                    .consume(&TokenType::Identifier, "Expected parameter name")?
                    .lexeme
                    .clone();
                let type_annotation = self.parse_optional_type_annotation();
                parameters.push(Parameter::new(param_name, type_annotation));

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        Ok(parameters)
    }

    /// Parse an optional `: type` annotation. Returns `None` when no colon
    /// follows. Accepts any single token as the type name (identifiers as
    /// well as type keywords like `number`, `string`, `boolean`, `trance`).
    fn parse_optional_type_annotation(&mut self) -> Option<String> {
        if self.match_token(&TokenType::Colon) {
            Some(self.advance().lexeme.clone())
        } else {
            None
        }
    }

    /// Parse session declaration
    fn parse_session_declaration(&mut self) -> ParseResult<AstNode> {
        let name = self
            .consume(&TokenType::Identifier, "Expected session name")?
            .lexeme
            .clone();

        self.consume(&TokenType::LBrace, "Expected '{' after session name")?;

        let mut members = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() {
            members.push(self.parse_session_member()?);
        }

        self.consume(&TokenType::RBrace, "Expected '}' after session body")?;

        Ok(AstNode::SessionDeclaration { name, members })
    }

    /// Parse tranceify declaration (record/struct type definition)
    /// Example: tranceify Person { name: string; age: number; isInTrance: boolean; }
    fn parse_tranceify_declaration(&mut self) -> ParseResult<AstNode> {
        let name = self
            .consume(&TokenType::Identifier, "Expected tranceify type name")?
            .lexeme
            .clone();

        self.consume(&TokenType::LBrace, "Expected '{' after tranceify name")?;

        let mut fields = Vec::new();
        while !self.check(&TokenType::RBrace) && !self.is_at_end() {
            let field_name = self
                .consume(&TokenType::Identifier, "Expected field name")?
                .lexeme
                .clone();

            self.consume(&TokenType::Colon, "Expected ':' after field name")?;

            let type_annotation = self.parse_type_annotation()?;

            self.consume(
                &TokenType::Semicolon,
                "Expected ';' after field declaration",
            )?;

            fields.push(TranceifyField {
                name: field_name,
                type_annotation,
            });
        }

        self.consume(&TokenType::RBrace, "Expected '}' after tranceify body")?;

        Ok(AstNode::TranceifyDeclaration { name, fields })
    }

    /// Parse record literal (instance of a tranceify type)
    /// Example: Person { name: "Alice", age: 30, isInTrance: true }
    /// Note: The opening '{' has already been consumed
    fn parse_record_literal(&mut self, type_name: String) -> ParseResult<AstNode> {
        let mut fields = Vec::new();

        if !self.check(&TokenType::RBrace) {
            loop {
                let field_name = self
                    .consume(&TokenType::Identifier, "Expected field name")?
                    .lexeme
                    .clone();

                self.consume(&TokenType::Colon, "Expected ':' after field name")?;

                let value = Box::new(self.parse_expression()?);

                fields.push(RecordFieldInit {
                    name: field_name,
                    value,
                });

                if self.match_token(&TokenType::Comma) {
                    // Allow trailing comma
                    if self.check(&TokenType::RBrace) {
                        break;
                    }
                    continue;
                } else {
                    break;
                }
            }
        }

        self.consume(&TokenType::RBrace, "Expected '}' after record fields")?;

        Ok(AstNode::RecordLiteral { type_name, fields })
    }

    /// Parse an individual session member (field or method)
    fn parse_session_member(&mut self) -> ParseResult<SessionMember> {
        let is_static = self.match_token(&TokenType::Dominant);

        // Optional visibility modifiers
        if self.check(&TokenType::Expose) || self.check(&TokenType::Conceal) {
            let visibility = if self.advance().token_type == TokenType::Expose {
                SessionVisibility::Public
            } else {
                SessionVisibility::Private
            };

            if self.check(&TokenType::Suggestion)
                || self.check(&TokenType::ImperativeSuggestion)
                || self.check(&TokenType::DominantSuggestion)
            {
                return self.parse_session_method(is_static, Some(visibility));
            } else {
                return self.parse_session_field(is_static, visibility);
            }
        }

        // No explicit visibility modifier => default to public
        self.parse_session_method(is_static, Some(SessionVisibility::Public))
    }

    fn parse_session_field(
        &mut self,
        is_static: bool,
        visibility: SessionVisibility,
    ) -> ParseResult<SessionMember> {
        let name = self
            .consume(&TokenType::Identifier, "Expected field name in session")?
            .lexeme
            .clone();

        let type_annotation = self.parse_optional_type_annotation();

        let initializer = if self.match_token(&TokenType::Equals) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after session field declaration",
        )?;

        Ok(SessionMember::Field(SessionField {
            name,
            type_annotation,
            initializer,
            visibility,
            is_static,
        }))
    }

    fn parse_session_method(
        &mut self,
        mut is_static: bool,
        visibility: Option<SessionVisibility>,
    ) -> ParseResult<SessionMember> {
        let visibility = visibility.unwrap_or(SessionVisibility::Public);

        if self.match_token(&TokenType::DominantSuggestion) {
            is_static = true;
        } else if !self.match_token(&TokenType::Suggestion)
            && !self.match_token(&TokenType::ImperativeSuggestion)
        {
            return Err(self.error_here("Expected 'suggestion' inside session"));
        }

        let mut is_constructor = false;
        let name = if self.match_token(&TokenType::Constructor) {
            is_constructor = true;
            "constructor".to_string()
        } else {
            self.consume(&TokenType::Identifier, "Expected method name")?
                .lexeme
                .clone()
        };

        self.consume(&TokenType::LParen, "Expected '(' after method name")?;
        let parameters = self.parse_parameter_list()?;
        self.consume(&TokenType::RParen, "Expected ')' after parameters")?;

        let return_type = self.parse_optional_type_annotation();

        self.consume(&TokenType::LBrace, "Expected '{' after method signature")?;
        let body = self.parse_block_statements(BlockContext::Regular)?;
        self.consume(&TokenType::RBrace, "Expected '}' after method body")?;

        Ok(SessionMember::Method(SessionMethod {
            name,
            parameters,
            return_type,
            body,
            visibility,
            is_static,
            is_constructor,
        }))
    }

    /// Parse return statement
    fn parse_return_statement(&mut self) -> ParseResult<AstNode> {
        let value = if !self.check(&TokenType::Semicolon) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        self.consume(&TokenType::Semicolon, "Expected ';' after return statement")?;
        Ok(AstNode::ReturnStatement(value))
    }

    /// Parse expression
    fn parse_expression(&mut self) -> ParseResult<AstNode> {
        self.parse_assignment()
    }

    /// Parse assignment
    fn parse_assignment(&mut self) -> ParseResult<AstNode> {
        let expr = self.parse_nullish_coalescing()?;

        if self.match_token(&TokenType::Equals) {
            let value = Box::new(self.parse_assignment()?);
            return Ok(AstNode::AssignmentExpression {
                target: Box::new(expr),
                value,
            });
        }

        Ok(expr)
    }

    /// Parse nullish coalescing (?? or lucidFallback)
    fn parse_nullish_coalescing(&mut self) -> ParseResult<AstNode> {
        let mut left = self.parse_logical_or()?;

        while self.match_tokens(&[TokenType::QuestionQuestion, TokenType::LucidFallback]) {
            let right = Box::new(self.parse_logical_or()?);
            left = AstNode::NullishCoalescing {
                left: Box::new(left),
                right,
            };
        }

        Ok(left)
    }

    /// Parse a left-associative chain of binary operators, delegating to
    /// `next` for operands of the next-higher precedence level.
    fn parse_binary_level(
        &mut self,
        operators: &[TokenType],
        next: fn(&mut Self) -> ParseResult<AstNode>,
    ) -> ParseResult<AstNode> {
        let mut left = next(self)?;

        while self.match_tokens(operators) {
            let operator = self.previous().lexeme.clone();
            let right = Box::new(next(self)?);
            left = AstNode::BinaryExpression {
                left: Box::new(left),
                operator,
                right,
            };
        }

        Ok(left)
    }

    /// Parse logical OR
    fn parse_logical_or(&mut self) -> ParseResult<AstNode> {
        self.parse_binary_level(
            &[TokenType::PipePipe, TokenType::ResistanceIsFutile],
            Self::parse_logical_and,
        )
    }

    /// Parse logical AND
    fn parse_logical_and(&mut self) -> ParseResult<AstNode> {
        self.parse_binary_level(
            &[TokenType::AmpAmp, TokenType::UnderMyControl],
            Self::parse_equality,
        )
    }

    /// Parse equality
    fn parse_equality(&mut self) -> ParseResult<AstNode> {
        self.parse_binary_level(
            &[
                TokenType::DoubleEquals,
                TokenType::NotEquals,
                TokenType::YouAreFeelingVerySleepy,
                TokenType::YouCannotResist,
                TokenType::NotSoDeep,
            ],
            Self::parse_comparison,
        )
    }

    /// Parse comparison
    fn parse_comparison(&mut self) -> ParseResult<AstNode> {
        self.parse_binary_level(
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
                TokenType::LookAtTheWatch,
                TokenType::FallUnderMySpell,
                TokenType::YourEyesAreGettingHeavy,
                TokenType::GoingDeeper,
                TokenType::DeeplyGreater,
                TokenType::DeeplyLess,
            ],
            Self::parse_term,
        )
    }

    /// Parse term (addition/subtraction)
    fn parse_term(&mut self) -> ParseResult<AstNode> {
        self.parse_binary_level(&[TokenType::Plus, TokenType::Minus], Self::parse_factor)
    }

    /// Parse factor (multiplication/division/modulo)
    fn parse_factor(&mut self) -> ParseResult<AstNode> {
        self.parse_binary_level(
            &[TokenType::Asterisk, TokenType::Slash, TokenType::Percent],
            Self::parse_unary,
        )
    }

    /// Parse unary
    fn parse_unary(&mut self) -> ParseResult<AstNode> {
        // Handle await/surrenderTo
        if self.match_tokens(&[TokenType::Await, TokenType::SurrenderTo]) {
            let expression = Box::new(self.parse_unary()?);
            return Ok(AstNode::AwaitExpression { expression });
        }

        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().lexeme.clone();
            let operand = Box::new(self.parse_unary()?);
            return Ok(AstNode::UnaryExpression { operator, operand });
        }

        self.parse_call()
    }

    /// Parse call expression
    fn parse_call(&mut self) -> ParseResult<AstNode> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&TokenType::LParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_tokens(&[TokenType::QuestionDot, TokenType::DreamReach]) {
                // Optional chaining (?. or dreamReach)
                if self.check(&TokenType::Identifier) {
                    let property = self.advance().lexeme.clone();
                    expr = AstNode::OptionalChaining {
                        object: Box::new(expr),
                        property,
                    };
                } else if self.match_token(&TokenType::LBracket) {
                    // Optional indexing ?.[
                    let index = Box::new(self.parse_expression()?);
                    self.consume(&TokenType::RBracket, "Expected ']' after optional index")?;
                    expr = AstNode::OptionalIndexing {
                        object: Box::new(expr),
                        index,
                    };
                } else {
                    return Err(self.error_here("Expected property name or '[' after '?.'"));
                }
            } else if self.match_token(&TokenType::Dot) {
                let property = self
                    .consume(&TokenType::Identifier, "Expected property name after '.'")?
                    .lexeme
                    .clone();
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
    fn finish_call(&mut self, callee: AstNode) -> ParseResult<AstNode> {
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
    fn parse_primary(&mut self) -> ParseResult<AstNode> {
        // Entrain (pattern matching) expression
        if self.check(&TokenType::Entrain) {
            return self.parse_entrain_expression();
        }

        // Number literal
        if self.check(&TokenType::NumberLiteral) {
            return Ok(AstNode::NumberLiteral(self.parse_number_literal()?));
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

        // Identifier or Record Literal
        if self.check(&TokenType::Identifier) {
            let identifier = self.advance().lexeme.clone();

            // Check if this is a record literal (Type { field: value, ... })
            if self.check(&TokenType::LBrace) {
                let next_token_type = self.peek_next().map(|tok| &tok.token_type);

                if matches!(
                    next_token_type,
                    Some(TokenType::Identifier) | Some(TokenType::RBrace)
                ) {
                    self.advance(); // consume '{'
                    return self.parse_record_literal(identifier);
                }
            }

            return Ok(AstNode::Identifier(identifier));
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

        Err(self.error_here(format!("Unexpected token '{}'", self.describe_peek())))
    }

    /// Consume a number literal token and parse its numeric value.
    fn parse_number_literal(&mut self) -> ParseResult<f64> {
        let (line, column) = {
            let token = self.peek();
            (token.line, token.column)
        };
        let lexeme = self.advance().lexeme.clone();
        lexeme
            .parse::<f64>()
            .map_err(|_| SyntaxError::new(format!("Invalid number: {}", lexeme), line, column))
    }

    /// Parse entrain (pattern matching) expression
    fn parse_entrain_expression(&mut self) -> ParseResult<AstNode> {
        self.consume(&TokenType::Entrain, "Expected 'entrain'")?;
        let subject = Box::new(self.parse_expression()?);
        self.consume(&TokenType::LBrace, "Expected '{' after entrain subject")?;

        let mut cases = Vec::new();
        let mut default_case = None;

        while !self.check(&TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Otherwise) {
                self.consume(&TokenType::Arrow, "Expected '=>' after 'otherwise'")?;
                default_case = Some(self.parse_entrain_body()?);
                self.match_token(&TokenType::Comma);
                self.match_token(&TokenType::Semicolon);
                break;
            }

            self.consume(&TokenType::When, "Expected 'when' or 'otherwise'")?;
            let pattern = self.parse_pattern()?;

            let guard = if self.match_token(&TokenType::If) {
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };

            self.consume(&TokenType::Arrow, "Expected '=>' after pattern")?;
            let body = self.parse_entrain_body()?;

            cases.push(EntrainCase {
                pattern,
                guard,
                body,
            });

            // Optional comma or semicolon between cases
            self.match_token(&TokenType::Comma);
            self.match_token(&TokenType::Semicolon);
        }

        self.consume(&TokenType::RBrace, "Expected '}' after entrain cases")?;

        Ok(AstNode::EntrainExpression {
            subject,
            cases,
            default: default_case,
        })
    }

    /// Parse pattern for matching
    fn parse_pattern(&mut self) -> ParseResult<Pattern> {
        // Literal patterns
        if self.check(&TokenType::NumberLiteral) {
            let value = self.parse_number_literal()?;
            return Ok(Pattern::Literal(Box::new(AstNode::NumberLiteral(value))));
        }

        if self.check(&TokenType::StringLiteral) {
            let token = self.advance();
            return Ok(Pattern::Literal(Box::new(AstNode::StringLiteral(
                token.lexeme.clone(),
            ))));
        }

        if self.match_token(&TokenType::True) {
            return Ok(Pattern::Literal(Box::new(AstNode::BooleanLiteral(true))));
        }

        if self.match_token(&TokenType::False) {
            return Ok(Pattern::Literal(Box::new(AstNode::BooleanLiteral(false))));
        }

        // Array pattern: [first, second, ...rest]
        if self.match_token(&TokenType::LBracket) {
            let mut elements = Vec::new();
            let mut rest = None;

            if !self.check(&TokenType::RBracket) {
                loop {
                    if self.match_token(&TokenType::DotDotDot) {
                        // Rest pattern
                        if self.check(&TokenType::Identifier) {
                            rest = Some(self.advance().lexeme.clone());
                        }
                        break;
                    }

                    elements.push(self.parse_pattern()?);

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            self.consume(&TokenType::RBracket, "Expected ']' after array pattern")?;
            return Ok(Pattern::Array { elements, rest });
        }

        // Record pattern or identifier with type annotation
        if self.check(&TokenType::Identifier) {
            let name = self.advance().lexeme.clone();

            // Check for type annotation: name: Type
            if self.match_token(&TokenType::Colon) {
                let type_annotation = self.parse_type_annotation()?;
                return Ok(Pattern::Typed {
                    name: Some(name),
                    type_annotation,
                });
            }

            // Check for record pattern: TypeName { field1, field2 }
            if self.match_token(&TokenType::LBrace) {
                let type_name = name.clone();
                let mut fields = Vec::new();

                if !self.check(&TokenType::RBrace) {
                    loop {
                        let field_name = self
                            .consume(
                                &TokenType::Identifier,
                                "Expected field name in record pattern",
                            )?
                            .lexeme
                            .clone();

                        let pattern = if self.match_token(&TokenType::Colon) {
                            Some(Box::new(self.parse_pattern()?))
                        } else {
                            None
                        };

                        fields.push(RecordFieldPattern {
                            name: field_name,
                            pattern,
                        });

                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }

                self.consume(&TokenType::RBrace, "Expected '}' after record pattern")?;
                return Ok(Pattern::Record { type_name, fields });
            }

            // Simple identifier binding
            return Ok(Pattern::Identifier(name));
        }

        Err(self.error_here(format!("Expected pattern, got '{}'", self.describe_peek())))
    }

    /// Parse body of an entrain case (can be block or single expression)
    fn parse_entrain_body(&mut self) -> ParseResult<Vec<AstNode>> {
        if self.match_token(&TokenType::LBrace) {
            let mut statements = Vec::new();
            while !self.check(&TokenType::RBrace) && !self.is_at_end() {
                statements.push(self.parse_statement(BlockContext::Regular)?);
            }
            self.consume(&TokenType::RBrace, "Expected '}' after block")?;
            Ok(statements)
        } else {
            // Single expression
            Ok(vec![self.parse_expression()?])
        }
    }

    /// Parse type annotation (returns the type as a string)
    fn parse_type_annotation(&mut self) -> ParseResult<String> {
        // Accept identifiers and type keywords (number, string, boolean)
        let type_name = match self.peek().token_type {
            TokenType::Identifier => self.advance().lexeme.clone(),
            TokenType::Number => {
                self.advance();
                "number".to_string()
            }
            TokenType::String => {
                self.advance();
                "string".to_string()
            }
            TokenType::Boolean => {
                self.advance();
                "boolean".to_string()
            }
            _ => {
                return Err(self.error_here(format!(
                    "Expected type annotation, got '{}'",
                    self.describe_peek()
                )));
            }
        };
        Ok(type_name)
    }

    // Helper methods

    /// Build a [`SyntaxError`] pointing at the current token.
    fn error_here(&self, message: impl Into<String>) -> SyntaxError {
        let token = self.peek();
        SyntaxError::new(message, token.line, token.column)
    }

    /// Human-readable description of the current token for error messages.
    fn describe_peek(&self) -> String {
        let token = self.peek();
        if token.token_type == TokenType::Eof {
            "end of input".to_string()
        } else {
            token.lexeme.clone()
        }
    }

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

    fn advance(&mut self) -> &Token {
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

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek_next(&self) -> Option<&Token> {
        if self.current + 1 >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.current + 1])
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> ParseResult<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error_here(format!("{}, found '{}'", message, self.describe_peek())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> ParseResult<AstNode> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().expect("lexing failed");
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    #[test]
    fn test_parse_simple_program() {
        let ast = parse(
            r#"
Focus {
    induce x: number = 42;
    observe x;
} Relax
"#,
        );
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_if_statement() {
        let ast = parse(
            r#"
Focus {
    induce x: number = 10;
    if (x > 5) deepFocus {
        observe "Greater";
    }
} Relax
"#,
        );
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_hypnotic_operator_synonyms() {
        let ast = parse(
            r#"
Focus {
    induce x: number = 10;
    if (x youAreFeelingVerySleepy 10 resistanceIsFutile x youCannotResist 5) deepFocus {
        observe "Synonym branch";
    }
} Relax
"#,
        );
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_entrain_with_record_pattern() {
        let ast = parse(
            r#"
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
            depth: 7,
        };

        induce status: string = entrain guest {
            when HypnoGuest { name: alias } => alias;
            otherwise => "Unknown";
        };

        observe status;
    }
} Relax
"#,
        );
        assert!(ast.is_ok(), "parse failed: {:?}", ast.err());
    }

    #[test]
    fn test_parse_string_interpolation() {
        let ast = parse(
            r#"
Focus {
    entrance {
        induce name: string = "Luna";
        induce depth: number = 7;
        observe "Guest ${name} is at depth ${depth + 1}!";
    }
} Relax
"#,
        );
        assert!(ast.is_ok(), "parse failed: {:?}", ast.err());
    }

    #[test]
    fn test_trigger_inside_function_is_rejected() {
        let ast = parse(
            r#"
Focus {
    suggestion inner() {
        trigger localTrigger = suggestion() {
            observe "Nope";
        };
    }
} Relax
"#,
        );
        assert!(ast.is_err());
        let error = ast.err().unwrap();
        assert!(
            error
                .to_string()
                .contains("Triggers can only be declared at the top level")
        );
    }

    #[test]
    fn test_entrance_inside_function_is_rejected() {
        let ast = parse(
            r#"
Focus {
    suggestion wrong() {
        entrance {
            observe "Nope";
        }
    }
} Relax
"#,
        );
        assert!(ast.is_err());
        let error = ast.err().unwrap();
        assert!(
            error
                .to_string()
                .contains("'entrance' blocks are only allowed at the top level")
        );
    }

    #[test]
    fn test_parser_errors_carry_positions() {
        let ast = parse(
            r#"
Focus {
    induce x = 1
    observe x;
} Relax
"#,
        );
        let error = ast.expect_err("expected a parse error");
        // The missing semicolon is discovered at 'observe' on line 4.
        assert_eq!(error.line, 4);
        assert!(error.message.contains("Expected ';'"));
        assert!(error.message.contains("found 'observe'"));
    }

    #[test]
    fn test_loop_init_supports_declarations() {
        let ast = parse(
            r#"
Focus {
    loop (induce i: number = 0; i < 3; i = i + 1) {
        observe i;
    }
} Relax
"#,
        );
        assert!(ast.is_ok(), "parse failed: {:?}", ast.err());
    }
}
