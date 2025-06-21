namespace HypnoScript.LexerParser.Lexer
{
	public record Token(TokenType Type, string Lexeme, int Line, int Column);
}
