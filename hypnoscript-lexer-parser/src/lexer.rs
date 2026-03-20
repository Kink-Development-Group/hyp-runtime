use crate::token::{Token, TokenType};

/// Lexer for the HypnoScript language
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the source code
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let start_column = self.column;
            let c = self.advance();

            if c.is_alphabetic() || c == '_' {
                let ident = self.read_identifier(c);
                let (token_type, lexeme) = self.keyword_or_identifier(&ident);
                tokens.push(Token::new(token_type, lexeme, self.line, start_column));
            } else if c.is_numeric() {
                let number = self.read_number(c);
                tokens.push(Token::new(
                    TokenType::NumberLiteral,
                    number,
                    self.line,
                    start_column,
                ));
            } else {
                match c {
                    '=' => {
                        if self.match_char('=') {
                            tokens.push(Token::new(
                                TokenType::DoubleEquals,
                                "==".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else if self.match_char('>') {
                            tokens.push(Token::new(
                                TokenType::Arrow,
                                "=>".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Equals,
                                "=".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '+' => tokens.push(Token::new(
                        TokenType::Plus,
                        "+".to_string(),
                        self.line,
                        start_column,
                    )),
                    '-' => tokens.push(Token::new(
                        TokenType::Minus,
                        "-".to_string(),
                        self.line,
                        start_column,
                    )),
                    '*' => tokens.push(Token::new(
                        TokenType::Asterisk,
                        "*".to_string(),
                        self.line,
                        start_column,
                    )),
                    '/' => {
                        if self.match_char('/') {
                            self.skip_line_comment();
                        } else if self.match_char('*') {
                            self.skip_block_comment();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Slash,
                                "/".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '%' => tokens.push(Token::new(
                        TokenType::Percent,
                        "%".to_string(),
                        self.line,
                        start_column,
                    )),
                    '>' => {
                        if self.match_char('=') {
                            tokens.push(Token::new(
                                TokenType::GreaterEqual,
                                ">=".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Greater,
                                ">".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '<' => {
                        if self.match_char('=') {
                            tokens.push(Token::new(
                                TokenType::LessEqual,
                                "<=".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Less,
                                "<".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '!' => {
                        if self.match_char('=') {
                            tokens.push(Token::new(
                                TokenType::NotEquals,
                                "!=".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Bang,
                                "!".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '&' => {
                        if self.match_char('&') {
                            tokens.push(Token::new(
                                TokenType::AmpAmp,
                                "&&".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Ampersand,
                                "&".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '|' => {
                        if self.match_char('|') {
                            tokens.push(Token::new(
                                TokenType::PipePipe,
                                "||".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Pipe,
                                "|".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '?' => {
                        if self.match_char('?') {
                            tokens.push(Token::new(
                                TokenType::QuestionQuestion,
                                "??".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else if self.match_char('.') {
                            tokens.push(Token::new(
                                TokenType::QuestionDot,
                                "?.".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::QuestionMark,
                                "?".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    ';' => tokens.push(Token::new(
                        TokenType::Semicolon,
                        ";".to_string(),
                        self.line,
                        start_column,
                    )),
                    ',' => tokens.push(Token::new(
                        TokenType::Comma,
                        ",".to_string(),
                        self.line,
                        start_column,
                    )),
                    '(' => tokens.push(Token::new(
                        TokenType::LParen,
                        "(".to_string(),
                        self.line,
                        start_column,
                    )),
                    ')' => tokens.push(Token::new(
                        TokenType::RParen,
                        ")".to_string(),
                        self.line,
                        start_column,
                    )),
                    '{' => tokens.push(Token::new(
                        TokenType::LBrace,
                        "{".to_string(),
                        self.line,
                        start_column,
                    )),
                    '}' => tokens.push(Token::new(
                        TokenType::RBrace,
                        "}".to_string(),
                        self.line,
                        start_column,
                    )),
                    '[' => tokens.push(Token::new(
                        TokenType::LBracket,
                        "[".to_string(),
                        self.line,
                        start_column,
                    )),
                    ']' => tokens.push(Token::new(
                        TokenType::RBracket,
                        "]".to_string(),
                        self.line,
                        start_column,
                    )),
                    ':' => tokens.push(Token::new(
                        TokenType::Colon,
                        ":".to_string(),
                        self.line,
                        start_column,
                    )),
                    '.' => {
                        if self.match_char('.') && self.match_char('.') {
                            tokens.push(Token::new(
                                TokenType::DotDotDot,
                                "...".to_string(),
                                self.line,
                                start_column,
                            ));
                        } else {
                            tokens.push(Token::new(
                                TokenType::Dot,
                                ".".to_string(),
                                self.line,
                                start_column,
                            ));
                        }
                    }
                    '"' => {
                        let string_val = self.read_string()?;
                        tokens.push(Token::new(
                            TokenType::StringLiteral,
                            string_val,
                            self.line,
                            start_column,
                        ));
                    }
                    _ => {
                        return Err(format!(
                            "Unexpected character '{}' at line {}, column {}",
                            c, self.line, self.column
                        ));
                    }
                }
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

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.pos];
        self.pos += 1;
        self.column += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.pos]
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

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        while !self.is_at_end() {
            if self.peek() == '*' {
                self.advance();
                if !self.is_at_end() && self.peek() == '/' {
                    self.advance();
                    break;
                }
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.advance();
            }
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

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while !self.is_at_end() {
            let c = self.peek();
            if c.is_numeric() || c == '.' {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        number
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut string = String::new();

        while !self.is_at_end() {
            let c = self.peek();
            if c == '"' {
                self.advance();
                return Ok(string);
            } else if c == '\\' {
                self.advance();
                if !self.is_at_end() {
                    let escaped = self.advance();
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        'u' => string.push(self.read_hex_escape(4, "\\u")?),
                        'x' => string.push(self.read_hex_escape(2, "\\x")?),
                        _ => string.push(escaped),
                    }
                }
            } else {
                if c == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                string.push(c);
                self.advance();
            }
        }

        Err(format!("Unterminated string at line {}", self.line))
    }

    fn read_hex_escape(&mut self, digits: usize, escape_prefix: &str) -> Result<char, String> {
        let mut hex = String::with_capacity(digits);

        for _ in 0..digits {
            if self.is_at_end() {
                return Err(format!(
                    "Unterminated {} escape at line {}, column {}",
                    escape_prefix, self.line, self.column
                ));
            }

            let digit = self.advance();
            if !digit.is_ascii_hexdigit() {
                return Err(format!(
                    "Invalid {} escape digit '{}' at line {}, column {}",
                    escape_prefix,
                    digit,
                    self.line,
                    self.column.saturating_sub(1)
                ));
            }

            hex.push(digit);
        }

        let value = u32::from_str_radix(&hex, 16).map_err(|_| {
            format!(
                "Invalid {} escape '{}' at line {}, column {}",
                escape_prefix,
                hex,
                self.line,
                self.column.saturating_sub(digits)
            )
        })?;

        char::from_u32(value).ok_or_else(|| {
            format!(
                "Invalid Unicode scalar value for {} escape '{}' at line {}, column {}",
                escape_prefix,
                hex,
                self.line,
                self.column.saturating_sub(digits)
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
        let mut unicode_lexer = Lexer::new(r#""\u0041\u0042\u0043""#);
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
        assert!(error.contains("Invalid \\u escape digit"));
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
}
