namespace HypnoScript.LexerParser.Lexer
{
	public enum TokenType
	{
		// Schlüsselwörter
		Focus, Relax,
		If, Else,
		While, Loop,
		Suggestion, Awaken,
		Induce, Observe,
		Drift,
		Session, Constructor,
		Tranceify,
		Entrance,
		Snap, Sink,

		// Hypnotische Operator-Synonyme
		YouAreFeelingVerySleepy, // statt ==
		LookAtTheWatch,          // statt >
		FallUnderMySpell,        // statt <
								 // ... ggf. mehr Synonyme

		// Operatoren/Symbole
		Equals,     // =
		Plus,       // +
		Minus,      // -
		Asterisk,   // *
		Slash,      // /
		Percent,    // %
		DoubleEquals, // ==
		NotEquals,
		Greater, GreaterEqual,
		Less, LessEqual,
		AmpAmp,   // &&
		PipePipe, // ||
		Bang,     // !
		Semicolon, Comma,
		LParen, RParen,
		LBrace, RBrace,
		LBracket, RBracket,
		Colon,

		// Literale
		NumberLiteral,
		StringLiteral,
		BooleanLiteral,

		// Identifizierer
		Identifier,

		// Sonstiges
		Eof
	}

	public record Token(TokenType Type, string Lexeme, int Line, int Column);
}
