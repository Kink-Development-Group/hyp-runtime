public enum TokenType
{
    // ...existing token types...
    Focus,
    Relax,
    Induce,
    If,
    Else,
    While,
    Loop,
    Observe,
    Awaken,
    // Neue Tokens für Funktionen
    Suggestion,
    ImperativeSuggestion,
    DominantSuggestion,
    Session,      // Neuer Token für Session-Deklarationen
    Tranceify,    // Neuer Token für Tranceify-Deklarationen
    // Weitere operatorische Tokens
    DoubleEquals,
    NotEquals,
    YouAreFeelingVerySleepy,  // Hinzugefügt für '=='-Synonym
    LookAtTheWatch,
    FallUnderMySpell,
    // ...weitere Token...
    Identifier,
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Equals,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Bang,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Eof,
    MindLink,
    SharedTrance,
    SinkTo,
    Label,
    NotSoDeep,
    DeeplyGreater,
    DeeplyLess
}
