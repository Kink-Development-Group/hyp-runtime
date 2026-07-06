use crate::error::SyntaxError;
use crate::token::{Token, TokenType};

/// Result alias for lexing operations.
pub type LexResult<T> = Result<T, SyntaxError>;

/// Lexer for the HypnoScript language.
///
/// Converts source text into a stream of [`Token`]s. Supports:
///
/// - All HypnoScript keywords and their plain-language aliases
/// - Numeric literals with decimal points, exponents (`1.5e3`) and digit
///   separators (`1_000_000`)
/// - String literals with escape sequences (`\n`, `\t`, `\uXXXX`, `\xXX`, ...)
/// - String interpolation: `"Hello, ${name}!"` is desugared into a
///   parenthesized string concatenation, so every later pipeline stage sees
///   ordinary expressions
/// - Line (`//`) and block (`/* ... */`) comments
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

/// A piece of a string literal: either literal text or an interpolated
/// expression that was lexed into its own token stream.
enum StringPart {
    Text(String),
    Expression(Vec<Token>),
}

impl Lexer {
    /// Create a new lexer
    pub fn new(source: &str) -> Self {
        Self::new_at(source, 1, 1)
    }

    /// Create a lexer whose position reporting starts at the given
    /// line/column. Used to lex interpolated expressions inside string
    /// literals while keeping positions relative to the whole file.
    fn new_at(source: &str, line: usize, column: usize) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line,
            column,
        }
    }

    /// Tokenize the source code
    pub fn lex(&mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments()?;
            if self.is_at_end() {
                break;
            }

            let start_line = self.line;
            let start_column = self.column;
            let c = self.advance();

            if c.is_alphabetic() || c == '_' {
                let ident = self.read_identifier(c);
                let (token_type, lexeme) = self.keyword_or_identifier(&ident);
                tokens.push(Token::new(token_type, lexeme, start_line, start_column));
            } else if c.is_ascii_digit() {
                let number = self.read_number(c)?;
                tokens.push(Token::new(
                    TokenType::NumberLiteral,
                    number,
                    start_line,
                    start_column,
                ));
            } else if c == '"' {
                self.read_string_tokens(start_line, start_column, &mut tokens)?;
            } else {
                let (token_type, lexeme) = self.read_operator(c, start_line, start_column)?;
                tokens.push(Token::new(
                    token_type,
                    lexeme.to_string(),
                    start_line,
                    start_column,
                ));
            }
        }

        tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            self.line,
            self.column,
        ));
        Ok(tokens)
    }

    /// Lex a single- or multi-character operator/delimiter starting at `c`.
    fn read_operator(
        &mut self,
        c: char,
        line: usize,
        column: usize,
    ) -> LexResult<(TokenType, &'static str)> {
        use TokenType::*;

        let token = match c {
            '=' => {
                if self.match_char('=') {
                    (DoubleEquals, "==")
                } else if self.match_char('>') {
                    (Arrow, "=>")
                } else {
                    (Equals, "=")
                }
            }
            '+' => (Plus, "+"),
            '-' => (Minus, "-"),
            '*' => (Asterisk, "*"),
            '/' => (Slash, "/"),
            '%' => (Percent, "%"),
            '>' => self.one_or_two('=', (GreaterEqual, ">="), (Greater, ">")),
            '<' => self.one_or_two('=', (LessEqual, "<="), (Less, "<")),
            '!' => self.one_or_two('=', (NotEquals, "!="), (Bang, "!")),
            '&' => self.one_or_two('&', (AmpAmp, "&&"), (Ampersand, "&")),
            '|' => self.one_or_two('|', (PipePipe, "||"), (Pipe, "|")),
            '?' => {
                if self.match_char('?') {
                    (QuestionQuestion, "??")
                } else if self.match_char('.') {
                    (QuestionDot, "?.")
                } else {
                    (QuestionMark, "?")
                }
            }
            ';' => (Semicolon, ";"),
            ',' => (Comma, ","),
            '(' => (LParen, "("),
            ')' => (RParen, ")"),
            '{' => (LBrace, "{"),
            '}' => (RBrace, "}"),
            '[' => (LBracket, "["),
            ']' => (RBracket, "]"),
            ':' => (Colon, ":"),
            '.' => {
                if self.match_char('.') {
                    if self.match_char('.') {
                        (DotDotDot, "...")
                    } else {
                        return Err(SyntaxError::new(
                            "Unexpected '..' (did you mean '.' or the spread operator '...'?)",
                            line,
                            column,
                        ));
                    }
                } else {
                    (Dot, ".")
                }
            }
            _ => {
                return Err(SyntaxError::new(
                    format!("Unexpected character '{}'", c),
                    line,
                    column,
                ));
            }
        };

        Ok(token)
    }

    /// If the next character matches `expected`, consume it and return `two`,
    /// otherwise return `one`.
    fn one_or_two(
        &mut self,
        expected: char,
        two: (TokenType, &'static str),
        one: (TokenType, &'static str),
    ) -> (TokenType, &'static str) {
        if self.match_char(expected) { two } else { one }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.pos];
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.pos]
        }
    }

    fn peek_next(&self) -> char {
        if self.pos + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.pos + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    /// Skip whitespace, line comments and block comments. Reports an error
    /// for unterminated block comments instead of silently accepting them.
    fn skip_whitespace_and_comments(&mut self) -> LexResult<()> {
        loop {
            while !self.is_at_end() && self.peek().is_whitespace() {
                self.advance();
            }

            if self.peek() == '/' && self.peek_next() == '/' {
                while !self.is_at_end() && self.peek() != '\n' {
                    self.advance();
                }
                continue;
            }

            if self.peek() == '/' && self.peek_next() == '*' {
                let start_line = self.line;
                let start_column = self.column;
                self.advance(); // '/'
                self.advance(); // '*'

                let mut terminated = false;
                while !self.is_at_end() {
                    if self.peek() == '*' && self.peek_next() == '/' {
                        self.advance();
                        self.advance();
                        terminated = true;
                        break;
                    }
                    self.advance();
                }

                if !terminated {
                    return Err(SyntaxError::new(
                        "Unterminated block comment",
                        start_line,
                        start_column,
                    ));
                }
                continue;
            }

            return Ok(());
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while !self.is_at_end() {
            let c = self.peek();
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    /// Read a numeric literal. Accepts at most one decimal point (only when
    /// followed by a digit, so `1.method()` lexes as member access), an
    /// optional exponent (`1e5`, `2.5E-3`) and `_` digit separators, which are
    /// stripped from the stored lexeme.
    fn read_number(&mut self, first: char) -> LexResult<String> {
        let mut number = String::new();
        number.push(first);

        let mut seen_dot = false;
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_ascii_digit() {
                number.push(c);
                self.advance();
            } else if c == '_' {
                if !self.peek_next().is_ascii_digit() {
                    return Err(SyntaxError::new(
                        "Digit separator '_' must be followed by a digit",
                        self.line,
                        self.column,
                    ));
                }
                self.advance();
            } else if c == '.' && !seen_dot && self.peek_next().is_ascii_digit() {
                seen_dot = true;
                number.push(c);
                self.advance();
            } else if c == 'e' || c == 'E' {
                let next = self.peek_next();
                let has_signed_exponent = (next == '+' || next == '-')
                    && self.pos + 2 < self.source.len()
                    && self.source[self.pos + 2].is_ascii_digit();
                if next.is_ascii_digit() || has_signed_exponent {
                    number.push('e');
                    self.advance();
                    if has_signed_exponent {
                        number.push(self.advance());
                    }
                    while !self.is_at_end() && self.peek().is_ascii_digit() {
                        number.push(self.advance());
                    }
                }
                break;
            } else {
                break;
            }
        }

        Ok(number)
    }

    /// Read a string literal starting after the opening quote and push the
    /// resulting token(s) onto `tokens`.
    ///
    /// Plain strings produce a single [`TokenType::StringLiteral`]. Strings
    /// containing `${expr}` interpolations are desugared into a parenthesized
    /// concatenation, e.g. `"a ${x} b"` becomes `("a " + (x) + " b")`. The
    /// leading string segment is always emitted (even when empty) so that the
    /// whole expression is string-typed and string coercion applies.
    fn read_string_tokens(
        &mut self,
        start_line: usize,
        start_column: usize,
        tokens: &mut Vec<Token>,
    ) -> LexResult<()> {
        let parts = self.read_string_parts(start_line, start_column)?;

        // Fast path: no interpolation, emit a single string literal token.
        if parts.len() == 1
            && let StringPart::Text(text) = &parts[0]
        {
            tokens.push(Token::new(
                TokenType::StringLiteral,
                text.clone(),
                start_line,
                start_column,
            ));
            return Ok(());
        }

        let synthetic = |token_type: TokenType, lexeme: &str| {
            Token::new(token_type, lexeme.to_string(), start_line, start_column)
        };

        tokens.push(synthetic(TokenType::LParen, "("));

        let mut first = true;
        for part in parts {
            match part {
                StringPart::Text(text) => {
                    // The leading segment anchors the expression as a string;
                    // later empty segments add nothing and are skipped.
                    if first || !text.is_empty() {
                        if !first {
                            tokens.push(synthetic(TokenType::Plus, "+"));
                        }
                        tokens.push(synthetic(TokenType::StringLiteral, &text));
                    }
                }
                StringPart::Expression(expr_tokens) => {
                    tokens.push(synthetic(TokenType::Plus, "+"));
                    tokens.push(synthetic(TokenType::LParen, "("));
                    tokens.extend(expr_tokens);
                    tokens.push(synthetic(TokenType::RParen, ")"));
                }
            }
            first = false;
        }

        tokens.push(synthetic(TokenType::RParen, ")"));
        Ok(())
    }

    /// Split a string literal into text segments and interpolated expressions.
    /// The first part is always a text segment (possibly empty).
    fn read_string_parts(
        &mut self,
        start_line: usize,
        start_column: usize,
    ) -> LexResult<Vec<StringPart>> {
        let mut parts = Vec::new();
        let mut current = String::new();

        loop {
            if self.is_at_end() {
                return Err(SyntaxError::new(
                    "Unterminated string",
                    start_line,
                    start_column,
                ));
            }

            let c = self.peek();
            if c == '"' {
                self.advance();
                parts.push(StringPart::Text(current));
                return Ok(parts);
            }

            if c == '\\' {
                self.advance();
                if !self.is_at_end() {
                    let escaped = self.advance();
                    match escaped {
                        'n' => current.push('\n'),
                        't' => current.push('\t'),
                        'r' => current.push('\r'),
                        '\\' => current.push('\\'),
                        '"' => current.push('"'),
                        '$' => current.push('$'),
                        'u' => current.push(self.read_hex_escape(4, "\\u")?),
                        'x' => current.push(self.read_hex_escape(2, "\\x")?),
                        _ => current.push(escaped),
                    }
                }
                continue;
            }

            if c == '$' && self.peek_next() == '{' {
                let expr_line = self.line;
                // +2 to point at the first character inside `${`.
                let expr_column = self.column + 2;
                self.advance(); // '$'
                self.advance(); // '{'
                let expression_source = self.read_interpolation_source(expr_line, expr_column)?;

                if expression_source.trim().is_empty() {
                    return Err(SyntaxError::new(
                        "Empty interpolation '${}' in string literal",
                        expr_line,
                        expr_column,
                    ));
                }

                let mut sub_lexer = Lexer::new_at(&expression_source, expr_line, expr_column);
                let mut expr_tokens = sub_lexer.lex()?;
                expr_tokens.pop(); // drop the EOF token

                parts.push(StringPart::Text(std::mem::take(&mut current)));
                parts.push(StringPart::Expression(expr_tokens));
                continue;
            }

            current.push(c);
            self.advance();
        }
    }

    /// Consume the source text of an interpolated expression up to (and
    /// including) the matching `}`. Handles nested braces and nested string
    /// literals so `${entrain x { otherwise => "}" }}` stays intact.
    fn read_interpolation_source(&mut self, line: usize, column: usize) -> LexResult<String> {
        let mut source = String::new();
        let mut depth = 1usize;

        while !self.is_at_end() {
            let c = self.peek();
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        self.advance();
                        return Ok(source);
                    }
                }
                '"' => {
                    // Copy a nested string literal verbatim, respecting
                    // escapes so an escaped quote does not end it early.
                    source.push(self.advance());
                    while !self.is_at_end() && self.peek() != '"' {
                        let inner = self.advance();
                        source.push(inner);
                        if inner == '\\' && !self.is_at_end() {
                            source.push(self.advance());
                        }
                    }
                    if self.is_at_end() {
                        break;
                    }
                }
                _ => {}
            }
            source.push(self.advance());
        }

        Err(SyntaxError::new(
            "Unterminated interpolation '${...}' in string literal",
            line,
            column,
        ))
    }

    /// Reads a fixed-width hexadecimal escape sequence from the current position.
    ///
    /// `digits` controls how many hexadecimal digits are consumed after the
    /// escape prefix (for example 4 for `\uXXXX` and 2 for `\xXX`). The method
    /// returns the decoded Unicode scalar value or an error if the escape is
    /// truncated, contains non-hex digits, or decodes to an invalid scalar such
    /// as a UTF-16 surrogate.
    fn read_hex_escape(&mut self, digits: usize, escape_prefix: &str) -> LexResult<char> {
        let mut hex = String::with_capacity(digits);

        for _ in 0..digits {
            if self.is_at_end() {
                return Err(SyntaxError::new(
                    format!("Unterminated {} escape", escape_prefix),
                    self.line,
                    self.column,
                ));
            }

            let digit = self.advance();
            if !digit.is_ascii_hexdigit() {
                return Err(SyntaxError::new(
                    format!("Invalid {} escape digit '{}'", escape_prefix, digit),
                    self.line,
                    self.column.saturating_sub(1),
                ));
            }

            hex.push(digit);
        }

        // Safe because each digit was already validated with `is_ascii_hexdigit`.
        let value = u32::from_str_radix(&hex, 16).unwrap();

        if (0xD800..=0xDFFF).contains(&value) {
            return Err(SyntaxError::new(
                format!(
                    "Invalid Unicode scalar value for {} escape '{}': surrogate code points (U+D800 to U+DFFF) are not valid scalar values",
                    escape_prefix, hex
                ),
                self.line,
                self.column.saturating_sub(digits),
            ));
        }

        char::from_u32(value).ok_or_else(|| {
            SyntaxError::new(
                format!(
                    "Invalid Unicode scalar value for {} escape '{}'",
                    escape_prefix, hex
                ),
                self.line,
                self.column.saturating_sub(digits),
            )
        })
    }

    fn keyword_or_identifier(&self, s: &str) -> (TokenType, String) {
        if let Some(definition) = TokenType::keyword_definition(s) {
            (definition.token, definition.canonical_lexeme.to_string())
        } else {
            (TokenType::Identifier, s.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_types(source: &str) -> Vec<TokenType> {
        let mut lexer = Lexer::new(source);
        lexer
            .lex()
            .unwrap()
            .into_iter()
            .map(|token| token.token_type)
            .collect()
    }

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("induce x: number = 42;");
        let tokens = lexer.lex().unwrap();
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].token_type, TokenType::Induce);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new(r#""Hello, World!""#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[0].lexeme, "Hello, World!");
    }

    #[test]
    fn test_string_literal_unicode_escapes() {
        let mut unicode_lexer = Lexer::new("\"\\u0041\\u0042\\u0043\"");
        let unicode_tokens = unicode_lexer.lex().unwrap();
        assert_eq!(unicode_tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(unicode_tokens[0].lexeme, "ABC");

        let mut hex_lexer = Lexer::new(r#""Hello\x20World\x21""#);
        let hex_tokens = hex_lexer.lex().unwrap();
        assert_eq!(hex_tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(hex_tokens[0].lexeme, "Hello World!");
    }

    #[test]
    fn test_string_literal_invalid_unicode_escape() {
        let mut lexer = Lexer::new(r#""\u12G4""#);
        let error = lexer.lex().unwrap_err();
        assert!(error.to_string().contains("Invalid \\u escape digit"));
    }

    #[test]
    fn test_string_literal_unterminated_unicode_escape() {
        let mut unicode_lexer = Lexer::new("\"\\u12");
        let unicode_error = unicode_lexer.lex().unwrap_err();
        assert!(
            unicode_error
                .to_string()
                .contains("Unterminated \\u escape")
        );

        let mut hex_lexer = Lexer::new("\"\\x4");
        let hex_error = hex_lexer.lex().unwrap_err();
        assert!(hex_error.to_string().contains("Unterminated \\x escape"));
    }

    #[test]
    fn test_string_literal_invalid_unicode_scalar_escape() {
        let mut lexer = Lexer::new(r#""\uD800""#);
        let error = lexer.lex().unwrap_err();
        assert!(error.to_string().contains("Invalid Unicode scalar value"));
    }

    #[test]
    fn test_operator_synonym_tokenization() {
        let mut lexer = Lexer::new("if (a youAreFeelingVerySleepy b) { }");
        let tokens = lexer.lex().unwrap();
        let synonym = tokens
            .iter()
            .find(|token| token.token_type == TokenType::YouAreFeelingVerySleepy)
            .expect("synonym token not found");
        assert_eq!(synonym.lexeme, "youAreFeelingVerySleepy");
    }

    #[test]
    fn test_unterminated_string_reports_start_position() {
        let mut lexer = Lexer::new("\"never closed");
        let error = lexer.lex().unwrap_err();
        assert!(error.to_string().contains("Unterminated string"));
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 1);
    }

    #[test]
    fn test_unterminated_block_comment_is_an_error() {
        let mut lexer = Lexer::new("induce x = 1; /* comment");
        let error = lexer.lex().unwrap_err();
        assert!(error.to_string().contains("Unterminated block comment"));
    }

    #[test]
    fn test_double_dot_is_rejected_not_swallowed() {
        let mut lexer = Lexer::new("a..b");
        let error = lexer.lex().unwrap_err();
        assert!(error.to_string().contains("'..'"));
    }

    #[test]
    fn test_number_with_digit_separators() {
        let mut lexer = Lexer::new("1_000_000");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::NumberLiteral);
        assert_eq!(tokens[0].lexeme, "1000000");
    }

    #[test]
    fn test_number_with_exponent() {
        let mut lexer = Lexer::new("2.5e3 1E2 7e-2");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].lexeme, "2.5e3");
        assert_eq!(tokens[1].lexeme, "1e2");
        assert_eq!(tokens[2].lexeme, "7e-2");
        assert!(tokens[0].lexeme.parse::<f64>().is_ok());
    }

    #[test]
    fn test_number_stops_at_second_dot() {
        // `1.2.3` must not produce a single bogus number literal.
        let mut lexer = Lexer::new("1.2.toFixed");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::NumberLiteral);
        assert_eq!(tokens[0].lexeme, "1.2");
        assert_eq!(tokens[1].token_type, TokenType::Dot);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
    }

    #[test]
    fn test_invalid_digit_separator_is_rejected() {
        let mut lexer = Lexer::new("1__2");
        assert!(lexer.lex().is_err());
        let mut trailing = Lexer::new("1_;");
        assert!(trailing.lex().is_err());
    }

    #[test]
    fn test_multiline_string_token_reports_start_line() {
        let mut lexer = Lexer::new("\"line1\nline2\" induce");
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].token_type, TokenType::Induce);
        assert_eq!(tokens[1].line, 2);
    }

    #[test]
    fn test_string_interpolation_desugars_to_concatenation() {
        // "a ${x} b" => ( "a " + ( x ) + " b" )
        let types = token_types(r#""a ${x} b""#);
        assert_eq!(
            types,
            vec![
                TokenType::LParen,
                TokenType::StringLiteral,
                TokenType::Plus,
                TokenType::LParen,
                TokenType::Identifier,
                TokenType::RParen,
                TokenType::Plus,
                TokenType::StringLiteral,
                TokenType::RParen,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn test_string_interpolation_expression_only() {
        // "${x}" => ( "" + ( x ) )
        let mut lexer = Lexer::new(r#""${x}""#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LParen);
        assert_eq!(tokens[1].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[1].lexeme, "");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[4].lexeme, "x");
    }

    #[test]
    fn test_string_interpolation_with_nested_expression() {
        // Nested braces and strings inside the interpolation stay intact.
        let mut lexer = Lexer::new(r#""v: ${format("{}", value)}""#);
        let tokens = lexer.lex().unwrap();
        let lexemes: Vec<&str> = tokens.iter().map(|t| t.lexeme.as_str()).collect();
        assert!(lexemes.contains(&"format"));
        assert!(lexemes.contains(&"{}"));
        assert!(lexemes.contains(&"value"));
    }

    #[test]
    fn test_string_interpolation_escape_opts_out() {
        let mut lexer = Lexer::new(r#""costs \${price}""#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[0].lexeme, "costs ${price}");
    }

    #[test]
    fn test_string_interpolation_empty_is_rejected() {
        let mut lexer = Lexer::new(r#""${}""#);
        let error = lexer.lex().unwrap_err();
        assert!(error.to_string().contains("Empty interpolation"));
    }

    #[test]
    fn test_string_interpolation_unterminated_is_rejected() {
        let mut lexer = Lexer::new(r#""${x + 1"#);
        let error = lexer.lex().unwrap_err();
        assert!(error.to_string().contains("Unterminated interpolation"));
    }

    #[test]
    fn test_plain_dollar_without_brace_is_literal() {
        let mut lexer = Lexer::new(r#""price: $5""#);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens[0].lexeme, "price: $5");
    }
}
