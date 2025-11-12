use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    Snap,   // break
    Sink,   // continue
    SinkTo, // goto

    // Functions
    Suggestion,
    ImperativeSuggestion,
    DominantSuggestion,
    Awaken, // return
    Call,

    // Object-oriented programming
    Session,
    Constructor,
    Expose,   // public
    Conceal,  // private
    Dominant, // static

    // Structures
    Tranceify,

    // I/O
    Observe,
    Drift,

    // Hypnotic operators
    YouAreFeelingVerySleepy, // ==
    YouCannotResist,         // !=
    LookAtTheWatch,          // >
    FallUnderMySpell,        // <
    YourEyesAreGettingHeavy, // >=
    GoingDeeper,             // <=
    NotSoDeep,               // != (legacy)
    DeeplyGreater,           // >= (legacy)
    DeeplyLess,              // <= (legacy)
    UnderMyControl,          // &&
    ResistanceIsFutile,      // ||

    // Modules and globals
    MindLink,     // import
    SharedTrance, // global

    // Labels
    Label,

    // Standard operators
    DoubleEquals, // ==
    NotEquals,    // !=
    Greater,
    GreaterEqual, // >=
    Less,
    LessEqual, // <=
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Bang,     // !
    AmpAmp,   // &&
    PipePipe, // ||

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
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Comma,
    Colon,     // :
    Semicolon, // ;
    Dot,       // .
    Equals,    // =

    // End of file
    Eof,

    // Assert statement
    Assert,
}

/// Metadata describing a keyword, including its canonical lexeme for normalization.
#[derive(Clone, Copy)]
pub struct KeywordDefinition {
    pub token: TokenType,
    pub canonical_lexeme: &'static str,
}

/// All reserved words and hypnotic operator synonyms mapped by their normalized form.
static KEYWORD_DEFINITIONS: Lazy<HashMap<&'static str, KeywordDefinition>> = Lazy::new(|| {
    use TokenType::*;

    let mut map = HashMap::with_capacity(64);

    // Core structure keywords
    map.insert(
        "focus",
        KeywordDefinition {
            token: Focus,
            canonical_lexeme: "Focus",
        },
    );
    map.insert(
        "relax",
        KeywordDefinition {
            token: Relax,
            canonical_lexeme: "Relax",
        },
    );
    map.insert(
        "entrance",
        KeywordDefinition {
            token: Entrance,
            canonical_lexeme: "entrance",
        },
    );
    map.insert(
        "deepfocus",
        KeywordDefinition {
            token: DeepFocus,
            canonical_lexeme: "deepFocus",
        },
    );

    // Variable declarations and sourcing
    map.insert(
        "induce",
        KeywordDefinition {
            token: Induce,
            canonical_lexeme: "induce",
        },
    );
    map.insert(
        "freeze",
        KeywordDefinition {
            token: Induce,
            canonical_lexeme: "induce",
        },
    );
    map.insert(
        "from",
        KeywordDefinition {
            token: From,
            canonical_lexeme: "from",
        },
    );
    map.insert(
        "external",
        KeywordDefinition {
            token: External,
            canonical_lexeme: "external",
        },
    );

    // Control flow constructs
    map.insert(
        "if",
        KeywordDefinition {
            token: If,
            canonical_lexeme: "if",
        },
    );
    map.insert(
        "else",
        KeywordDefinition {
            token: Else,
            canonical_lexeme: "else",
        },
    );
    map.insert(
        "while",
        KeywordDefinition {
            token: While,
            canonical_lexeme: "while",
        },
    );
    map.insert(
        "loop",
        KeywordDefinition {
            token: Loop,
            canonical_lexeme: "loop",
        },
    );
    map.insert(
        "snap",
        KeywordDefinition {
            token: Snap,
            canonical_lexeme: "snap",
        },
    );
    map.insert(
        "break",
        KeywordDefinition {
            token: Snap,
            canonical_lexeme: "snap",
        },
    );
    map.insert(
        "sink",
        KeywordDefinition {
            token: Sink,
            canonical_lexeme: "sink",
        },
    );
    map.insert(
        "continue",
        KeywordDefinition {
            token: Sink,
            canonical_lexeme: "sink",
        },
    );
    map.insert(
        "sinkto",
        KeywordDefinition {
            token: SinkTo,
            canonical_lexeme: "sinkTo",
        },
    );

    // Functions
    map.insert(
        "suggestion",
        KeywordDefinition {
            token: Suggestion,
            canonical_lexeme: "suggestion",
        },
    );
    map.insert(
        "imperativesuggestion",
        KeywordDefinition {
            token: ImperativeSuggestion,
            canonical_lexeme: "imperativeSuggestion",
        },
    );
    map.insert(
        "dominantsuggestion",
        KeywordDefinition {
            token: DominantSuggestion,
            canonical_lexeme: "dominantSuggestion",
        },
    );
    map.insert(
        "awaken",
        KeywordDefinition {
            token: Awaken,
            canonical_lexeme: "awaken",
        },
    );
    map.insert(
        "return",
        KeywordDefinition {
            token: Awaken,
            canonical_lexeme: "awaken",
        },
    );
    map.insert(
        "call",
        KeywordDefinition {
            token: Call,
            canonical_lexeme: "call",
        },
    );

    // Sessions (classes)
    map.insert(
        "session",
        KeywordDefinition {
            token: Session,
            canonical_lexeme: "session",
        },
    );
    map.insert(
        "constructor",
        KeywordDefinition {
            token: Constructor,
            canonical_lexeme: "constructor",
        },
    );
    map.insert(
        "expose",
        KeywordDefinition {
            token: Expose,
            canonical_lexeme: "expose",
        },
    );
    map.insert(
        "conceal",
        KeywordDefinition {
            token: Conceal,
            canonical_lexeme: "conceal",
        },
    );
    map.insert(
        "dominant",
        KeywordDefinition {
            token: Dominant,
            canonical_lexeme: "dominant",
        },
    );

    // Structures and observations
    map.insert(
        "tranceify",
        KeywordDefinition {
            token: Tranceify,
            canonical_lexeme: "tranceify",
        },
    );
    map.insert(
        "observe",
        KeywordDefinition {
            token: Observe,
            canonical_lexeme: "observe",
        },
    );
    map.insert(
        "whisper",
        KeywordDefinition {
            token: Observe,
            canonical_lexeme: "observe",
        },
    );
    map.insert(
        "drift",
        KeywordDefinition {
            token: Drift,
            canonical_lexeme: "drift",
        },
    );

    // Modules and globals
    map.insert(
        "mindlink",
        KeywordDefinition {
            token: MindLink,
            canonical_lexeme: "mindLink",
        },
    );
    map.insert(
        "sharedtrance",
        KeywordDefinition {
            token: SharedTrance,
            canonical_lexeme: "sharedTrance",
        },
    );
    map.insert(
        "label",
        KeywordDefinition {
            token: Label,
            canonical_lexeme: "label",
        },
    );

    // Operator synonyms (equality)
    map.insert(
        "youarefeelingverysleepy",
        KeywordDefinition {
            token: YouAreFeelingVerySleepy,
            canonical_lexeme: "youAreFeelingVerySleepy",
        },
    );
    map.insert(
        "youcannotresist",
        KeywordDefinition {
            token: YouCannotResist,
            canonical_lexeme: "youCannotResist",
        },
    );
    map.insert(
        "notsodeep",
        KeywordDefinition {
            token: NotSoDeep,
            canonical_lexeme: "notSoDeep",
        },
    );

    // Operator synonyms (comparison)
    map.insert(
        "lookatthewatch",
        KeywordDefinition {
            token: LookAtTheWatch,
            canonical_lexeme: "lookAtTheWatch",
        },
    );
    map.insert(
        "fallundermyspell",
        KeywordDefinition {
            token: FallUnderMySpell,
            canonical_lexeme: "fallUnderMySpell",
        },
    );
    map.insert(
        "youreyesaregettingheavy",
        KeywordDefinition {
            token: YourEyesAreGettingHeavy,
            canonical_lexeme: "yourEyesAreGettingHeavy",
        },
    );
    map.insert(
        "goingdeeper",
        KeywordDefinition {
            token: GoingDeeper,
            canonical_lexeme: "goingDeeper",
        },
    );
    map.insert(
        "deeplygreater",
        KeywordDefinition {
            token: DeeplyGreater,
            canonical_lexeme: "deeplyGreater",
        },
    );
    map.insert(
        "deeplyless",
        KeywordDefinition {
            token: DeeplyLess,
            canonical_lexeme: "deeplyLess",
        },
    );

    // Logical operator synonyms
    map.insert(
        "undermycontrol",
        KeywordDefinition {
            token: UnderMyControl,
            canonical_lexeme: "underMyControl",
        },
    );
    map.insert(
        "resistanceisfutile",
        KeywordDefinition {
            token: ResistanceIsFutile,
            canonical_lexeme: "resistanceIsFutile",
        },
    );

    // Primitive type aliases and literals
    map.insert(
        "number",
        KeywordDefinition {
            token: Number,
            canonical_lexeme: "number",
        },
    );
    map.insert(
        "string",
        KeywordDefinition {
            token: String,
            canonical_lexeme: "string",
        },
    );
    map.insert(
        "boolean",
        KeywordDefinition {
            token: Boolean,
            canonical_lexeme: "boolean",
        },
    );
    map.insert(
        "trance",
        KeywordDefinition {
            token: Trance,
            canonical_lexeme: "trance",
        },
    );
    map.insert(
        "true",
        KeywordDefinition {
            token: True,
            canonical_lexeme: "true",
        },
    );
    map.insert(
        "false",
        KeywordDefinition {
            token: False,
            canonical_lexeme: "false",
        },
    );

    map.insert(
        "assert",
        KeywordDefinition {
            token: Assert,
            canonical_lexeme: "assert",
        },
    );

    map
});

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
                | TokenType::YouCannotResist
                | TokenType::LookAtTheWatch
                | TokenType::FallUnderMySpell
                | TokenType::YourEyesAreGettingHeavy
                | TokenType::GoingDeeper
                | TokenType::NotSoDeep
                | TokenType::DeeplyGreater
                | TokenType::DeeplyLess
                | TokenType::DoubleEquals
                | TokenType::NotEquals
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::UnderMyControl
                | TokenType::ResistanceIsFutile
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

    /// Lookup keyword definition by source lexeme.
    pub fn keyword_definition(s: &str) -> Option<KeywordDefinition> {
        let normalized = s.to_ascii_lowercase();
        KEYWORD_DEFINITIONS.get(normalized.as_str()).copied()
    }

    /// Get keyword from string.
    pub fn from_keyword(s: &str) -> Option<TokenType> {
        Self::keyword_definition(s).map(|definition| definition.token)
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
