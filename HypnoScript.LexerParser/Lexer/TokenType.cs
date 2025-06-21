public enum TokenType
{
    // Grundlegende Programmstruktur
    Focus,
    Relax,
    Entrance,
    DeepFocus,

    // Variablen und Deklarationen
    Induce,
    From,
    External,

    // Kontrollstrukturen
    If,
    Else,
    While,
    Loop,
    Snap,      // break
    Sink,      // continue
    SinkTo,    // goto

    // Funktionen
    Suggestion,
    ImperativeSuggestion,
    DominantSuggestion,
    Awaken,    // return
    Call,

    // Objektorientierung
    Session,
    Constructor,
    Expose,    // public
    Conceal,   // private
    Dominant,  // static

    // Strukturen
    Tranceify,

    // Ein-/Ausgabe
    Observe,
    Drift,

    // Hypnotische Operatoren
    YouAreFeelingVerySleepy,  // ==
    LookAtTheWatch,           // >
    FallUnderMySpell,         // <
    NotSoDeep,                // !=
    DeeplyGreater,            // >=
    DeeplyLess,               // <=

    // Module und Globale
    MindLink,     // import
    SharedTrance, // global

    // Labels
    Label,

    // Standard Operatoren
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

    // Literale und Bezeichner
    Identifier,
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,

    // Typen
    Number,
    String,
    Boolean,
    Trance,

    // Boolean Literale
    True,
    False,

    // Trennzeichen und Klammern
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

    // Ende der Datei
    Eof,

    // Assert-Statement
    Assert
}
