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
    Finale,      // Destructor/cleanup block
    DeepFocus,   // Deep focus block modifier
    DeeperStill, // Even deeper block modifier

    // Variables and declarations
    Induce,  // Variable declaration (standard)
    Implant, // Variable declaration (alternative)
    Embed,   // Variable declaration (deep memory)
    Freeze,  // Constant declaration
    From,
    External,
    Anchor, // Save state/create snapshot

    // Control structures
    If,
    Else,
    When,      // Pattern matching case
    Otherwise, // Pattern matching default
    Entrain,   // Pattern matching switch
    While,
    Loop,
    Pendulum,  // Bidirectional loop
    Snap,      // break
    Sink,      // continue
    SinkTo,    // goto
    Oscillate, // toggle boolean
    Suspend,   // Pause without fixed end

    // Functions
    Suggestion,           // Standard function
    Trigger,              // Event handler/callback function
    Imperative,           // Imperative modifier ('imperative suggestion ...')
    ImperativeSuggestion, // Imperative function modifier (one-word form)
    DominantSuggestion,   // Static function modifier
    Mesmerize,            // Async function modifier
    Awaken,               // return
    Await,                // await async
    SurrenderTo,          // await (synonym)
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
    Observe,        // Standard output with newline
    Whisper,        // Output without newline
    Command,        // Imperative output
    Murmur,         // Quiet output/debug level
    Drift,          // Sleep/delay
    PauseReality,   // Sleep/delay (synonym)
    AccelerateTime, // Speed up execution
    DecelerateTime, // Slow down execution
    Subconscious,   // Access to hidden memory

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
    LucidFallback,           // ?? (nullish coalescing)
    DreamReach,              // ?. (optional chaining)

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
    Bang,             // !
    AmpAmp,           // &&
    PipePipe,         // ||
    QuestionMark,     // ?
    QuestionDot,      // ?.
    QuestionQuestion, // ??
    Pipe,             // | (for union types)
    Ampersand,        // & (for intersection types)
    Arrow,            // => (for pattern matching)

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
    Lucid, // Optional type modifier

    // Boolean literals
    True,
    False,

    // Null literal
    Null,

    // Delimiters and brackets
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    LAngle,   // < (for generics)
    RAngle,   // > (for generics)
    Comma,
    Colon,     // :
    Semicolon, // ;
    Dot,       // .
    DotDotDot, // ... (spread operator)
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

/// Keyword table: `(normalized lowercase form, token type, canonical lexeme)`.
///
/// Lookups normalize the source lexeme to ASCII lowercase, so each keyword is
/// listed once here regardless of how it is capitalized in source code.
/// Plain-language aliases (`break`, `continue`, `return`) map to the same
/// token as their hypnotic counterparts.
#[rustfmt::skip]
const KEYWORDS: &[(&str, TokenType, &str)] = &[
    // Core structure keywords
    ("focus", TokenType::Focus, "Focus"),
    ("relax", TokenType::Relax, "Relax"),
    ("entrance", TokenType::Entrance, "entrance"),
    ("finale", TokenType::Finale, "finale"),
    ("deepfocus", TokenType::DeepFocus, "deepFocus"),
    ("deeperstill", TokenType::DeeperStill, "deeperStill"),
    // Variable declarations and sourcing
    ("induce", TokenType::Induce, "induce"),
    ("implant", TokenType::Implant, "implant"),
    ("embed", TokenType::Embed, "embed"),
    ("freeze", TokenType::Freeze, "freeze"),
    ("anchor", TokenType::Anchor, "anchor"),
    ("from", TokenType::From, "from"),
    ("external", TokenType::External, "external"),
    // Control flow constructs
    ("if", TokenType::If, "if"),
    ("else", TokenType::Else, "else"),
    ("when", TokenType::When, "when"),
    ("otherwise", TokenType::Otherwise, "otherwise"),
    ("entrain", TokenType::Entrain, "entrain"),
    ("while", TokenType::While, "while"),
    ("loop", TokenType::Loop, "loop"),
    ("pendulum", TokenType::Pendulum, "pendulum"),
    ("snap", TokenType::Snap, "snap"),
    ("break", TokenType::Snap, "snap"),
    ("sink", TokenType::Sink, "sink"),
    ("continue", TokenType::Sink, "sink"),
    ("sinkto", TokenType::SinkTo, "sinkTo"),
    ("oscillate", TokenType::Oscillate, "oscillate"),
    ("suspend", TokenType::Suspend, "suspend"),
    // Functions
    ("suggestion", TokenType::Suggestion, "suggestion"),
    ("trigger", TokenType::Trigger, "trigger"),
    ("imperative", TokenType::Imperative, "imperative"),
    ("imperativesuggestion", TokenType::ImperativeSuggestion, "imperativeSuggestion"),
    ("dominantsuggestion", TokenType::DominantSuggestion, "dominantSuggestion"),
    ("mesmerize", TokenType::Mesmerize, "mesmerize"),
    ("awaken", TokenType::Awaken, "awaken"),
    ("return", TokenType::Awaken, "awaken"),
    ("await", TokenType::Await, "await"),
    ("surrenderto", TokenType::SurrenderTo, "surrenderTo"),
    ("call", TokenType::Call, "call"),
    // Sessions (classes)
    ("session", TokenType::Session, "session"),
    ("constructor", TokenType::Constructor, "constructor"),
    ("expose", TokenType::Expose, "expose"),
    ("conceal", TokenType::Conceal, "conceal"),
    ("dominant", TokenType::Dominant, "dominant"),
    // Structures and observations
    ("tranceify", TokenType::Tranceify, "tranceify"),
    ("observe", TokenType::Observe, "observe"),
    ("whisper", TokenType::Whisper, "whisper"),
    ("command", TokenType::Command, "command"),
    ("murmur", TokenType::Murmur, "murmur"),
    ("drift", TokenType::Drift, "drift"),
    ("pausereality", TokenType::PauseReality, "pauseReality"),
    ("acceleratetime", TokenType::AccelerateTime, "accelerateTime"),
    ("deceleratetime", TokenType::DecelerateTime, "decelerateTime"),
    ("subconscious", TokenType::Subconscious, "subconscious"),
    // Modules and globals
    ("mindlink", TokenType::MindLink, "mindLink"),
    ("sharedtrance", TokenType::SharedTrance, "sharedTrance"),
    ("label", TokenType::Label, "label"),
    // Operator synonyms (equality)
    ("youarefeelingverysleepy", TokenType::YouAreFeelingVerySleepy, "youAreFeelingVerySleepy"),
    ("youcannotresist", TokenType::YouCannotResist, "youCannotResist"),
    ("notsodeep", TokenType::NotSoDeep, "notSoDeep"),
    // Operator synonyms (comparison)
    ("lookatthewatch", TokenType::LookAtTheWatch, "lookAtTheWatch"),
    ("fallundermyspell", TokenType::FallUnderMySpell, "fallUnderMySpell"),
    ("youreyesaregettingheavy", TokenType::YourEyesAreGettingHeavy, "yourEyesAreGettingHeavy"),
    ("goingdeeper", TokenType::GoingDeeper, "goingDeeper"),
    ("deeplygreater", TokenType::DeeplyGreater, "deeplyGreater"),
    ("deeplyless", TokenType::DeeplyLess, "deeplyLess"),
    // Logical operator synonyms
    ("undermycontrol", TokenType::UnderMyControl, "underMyControl"),
    ("resistanceisfutile", TokenType::ResistanceIsFutile, "resistanceIsFutile"),
    ("lucidfallback", TokenType::LucidFallback, "lucidFallback"),
    ("dreamreach", TokenType::DreamReach, "dreamReach"),
    // Primitive type aliases and literals
    ("number", TokenType::Number, "number"),
    ("string", TokenType::String, "string"),
    ("boolean", TokenType::Boolean, "boolean"),
    ("trance", TokenType::Trance, "trance"),
    ("lucid", TokenType::Lucid, "lucid"),
    ("true", TokenType::True, "true"),
    ("false", TokenType::False, "false"),
    ("null", TokenType::Null, "null"),
    // Assertions
    ("assert", TokenType::Assert, "assert"),
];

/// All reserved words and hypnotic operator synonyms mapped by their normalized form.
static KEYWORD_DEFINITIONS: Lazy<HashMap<&'static str, KeywordDefinition>> = Lazy::new(|| {
    KEYWORDS
        .iter()
        .map(|&(normalized, token, canonical_lexeme)| {
            (
                normalized,
                KeywordDefinition {
                    token,
                    canonical_lexeme,
                },
            )
        })
        .collect()
});

impl TokenType {
    /// Check if token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::Focus
                | TokenType::Relax
                | TokenType::Entrance
                | TokenType::Finale
                | TokenType::DeepFocus
                | TokenType::DeeperStill
                | TokenType::Induce
                | TokenType::Implant
                | TokenType::Embed
                | TokenType::Freeze
                | TokenType::Anchor
                | TokenType::From
                | TokenType::External
                | TokenType::If
                | TokenType::Else
                | TokenType::When
                | TokenType::Otherwise
                | TokenType::Entrain
                | TokenType::While
                | TokenType::Loop
                | TokenType::Pendulum
                | TokenType::Snap
                | TokenType::Sink
                | TokenType::SinkTo
                | TokenType::Oscillate
                | TokenType::Suspend
                | TokenType::Suggestion
                | TokenType::Trigger
                | TokenType::Imperative
                | TokenType::ImperativeSuggestion
                | TokenType::DominantSuggestion
                | TokenType::Mesmerize
                | TokenType::Awaken
                | TokenType::Await
                | TokenType::SurrenderTo
                | TokenType::Call
                | TokenType::Session
                | TokenType::Constructor
                | TokenType::Expose
                | TokenType::Conceal
                | TokenType::Dominant
                | TokenType::Tranceify
                | TokenType::Observe
                | TokenType::Whisper
                | TokenType::Command
                | TokenType::Murmur
                | TokenType::Drift
                | TokenType::PauseReality
                | TokenType::AccelerateTime
                | TokenType::DecelerateTime
                | TokenType::Subconscious
                | TokenType::MindLink
                | TokenType::SharedTrance
                | TokenType::Label
                | TokenType::Assert
                | TokenType::True
                | TokenType::False
                | TokenType::Null
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
                | TokenType::LucidFallback
                | TokenType::DreamReach
                | TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::Bang
                | TokenType::AmpAmp
                | TokenType::PipePipe
                | TokenType::QuestionMark
                | TokenType::QuestionDot
                | TokenType::QuestionQuestion
                | TokenType::Pipe
                | TokenType::Ampersand
                | TokenType::Arrow
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
                | TokenType::Null
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
