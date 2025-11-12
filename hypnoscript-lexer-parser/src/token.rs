use serde::{Deserialize, Serialize};

/// Token types in the HypnoScript language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    // Basic program structure
    Focus,
    Relax,
    Entrance,
    DeepFocus,

    // Variables and declarations
    Induce,
    From,
    External,

    // Control structures
    If,
    Else,
    While,
    Loop,
    Snap,      // break
    Sink,      // continue
    SinkTo,    // goto

    // Functions
    Suggestion,
    ImperativeSuggestion,
    DominantSuggestion,
    Awaken,    // return
    Call,

    // Object-oriented programming
    Session,
    Constructor,
    Expose,    // public
    Conceal,   // private
    Dominant,  // static

    // Structures
    Tranceify,

    // I/O
    Observe,
    Drift,

    // Hypnotic operators
    YouAreFeelingVerySleepy,  // ==
    LookAtTheWatch,           // >
    FallUnderMySpell,         // <
    NotSoDeep,                // !=
    DeeplyGreater,            // >=
    DeeplyLess,               // <=

    // Modules and globals
    MindLink,     // import
    SharedTrance, // global

    // Labels
    Label,

    // Standard operators
    DoubleEquals,     // ==
    NotEquals,        // !=
    Greater,
    GreaterEqual,     // >=
    Less,
    LessEqual,        // <=
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Bang,             // !
    AmpAmp,           // &&
    PipePipe,         // ||

    // Literals and identifiers
    Identifier,
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,

    // Types
    Number,
    String,
    Boolean,
    Trance,

    // Boolean literals
    True,
    False,

    // Delimiters and brackets
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Comma,
    Colon,      // :
    Semicolon,  // ;
    Dot,        // .
    Equals,     // =

    // End of file
    Eof,

    // Assert statement
    Assert,
}

impl TokenType {
    /// Check if token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::Focus
                | TokenType::Relax
                | TokenType::Entrance
                | TokenType::DeepFocus
                | TokenType::Induce
                | TokenType::From
                | TokenType::External
                | TokenType::If
                | TokenType::Else
                | TokenType::While
                | TokenType::Loop
                | TokenType::Snap
                | TokenType::Sink
                | TokenType::SinkTo
                | TokenType::Suggestion
                | TokenType::ImperativeSuggestion
                | TokenType::DominantSuggestion
                | TokenType::Awaken
                | TokenType::Call
                | TokenType::Session
                | TokenType::Constructor
                | TokenType::Expose
                | TokenType::Conceal
                | TokenType::Dominant
                | TokenType::Tranceify
                | TokenType::Observe
                | TokenType::Drift
                | TokenType::MindLink
                | TokenType::SharedTrance
                | TokenType::Label
                | TokenType::Assert
                | TokenType::True
                | TokenType::False
        )
    }

    /// Check if token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            TokenType::YouAreFeelingVerySleepy
                | TokenType::LookAtTheWatch
                | TokenType::FallUnderMySpell
                | TokenType::NotSoDeep
                | TokenType::DeeplyGreater
                | TokenType::DeeplyLess
                | TokenType::DoubleEquals
                | TokenType::NotEquals
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::Bang
                | TokenType::AmpAmp
                | TokenType::PipePipe
        )
    }

    /// Check if token is a literal
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            TokenType::NumberLiteral
                | TokenType::StringLiteral
                | TokenType::BooleanLiteral
                | TokenType::True
                | TokenType::False
        )
    }

    /// Get keyword from string
    pub fn from_keyword(s: &str) -> Option<TokenType> {
        match s {
            "Focus" => Some(TokenType::Focus),
            "Relax" => Some(TokenType::Relax),
            "entrance" => Some(TokenType::Entrance),
            "deepFocus" => Some(TokenType::DeepFocus),
            "induce" => Some(TokenType::Induce),
            "from" => Some(TokenType::From),
            "external" => Some(TokenType::External),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "while" => Some(TokenType::While),
            "loop" => Some(TokenType::Loop),
            "snap" => Some(TokenType::Snap),
            "sink" => Some(TokenType::Sink),
            "sinkTo" => Some(TokenType::SinkTo),
            "suggestion" => Some(TokenType::Suggestion),
            "imperativeSuggestion" => Some(TokenType::ImperativeSuggestion),
            "dominantSuggestion" => Some(TokenType::DominantSuggestion),
            "awaken" => Some(TokenType::Awaken),
            "call" => Some(TokenType::Call),
            "session" => Some(TokenType::Session),
            "constructor" => Some(TokenType::Constructor),
            "expose" => Some(TokenType::Expose),
            "conceal" => Some(TokenType::Conceal),
            "dominant" => Some(TokenType::Dominant),
            "tranceify" => Some(TokenType::Tranceify),
            "observe" => Some(TokenType::Observe),
            "drift" => Some(TokenType::Drift),
            "YouAreFeelingVerySleepy" => Some(TokenType::YouAreFeelingVerySleepy),
            "LookAtTheWatch" => Some(TokenType::LookAtTheWatch),
            "FallUnderMySpell" => Some(TokenType::FallUnderMySpell),
            "NotSoDeep" => Some(TokenType::NotSoDeep),
            "DeeplyGreater" => Some(TokenType::DeeplyGreater),
            "DeeplyLess" => Some(TokenType::DeeplyLess),
            "MindLink" => Some(TokenType::MindLink),
            "SharedTrance" => Some(TokenType::SharedTrance),
            "label" => Some(TokenType::Label),
            "number" => Some(TokenType::Number),
            "string" => Some(TokenType::String),
            "boolean" => Some(TokenType::Boolean),
            "trance" => Some(TokenType::Trance),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "assert" => Some(TokenType::Assert),
            _ => None,
        }
    }
}

/// A token in the HypnoScript language
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}
