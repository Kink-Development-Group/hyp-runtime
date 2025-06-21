using System.Text;
namespace HypnoScript.LexerParser.Lexer
{
	public class HypnoLexer
	{
		private readonly string _source;
		private int _pos;
		private int _line = 1;
		private int _column = 1;

		public HypnoLexer(string source)
		{
			_source = source;
		}

		public IEnumerable<Token> Lex()
		{
			Console.WriteLine("[DEBUG] Lex() aufgerufen");
			var tokens = new List<Token>();

			while (!IsAtEnd())
			{
				Console.WriteLine($"[DEBUG] Lexer-Schleife: pos={_pos}, char='{Peek()}'");
				var startPos = _pos;
				var c = Advance();

				if (char.IsWhiteSpace(c))
				{
					if (c == '\n')
					{
						_line++;
						_column = 1;
					}
					continue;
				}

				if (char.IsLetter(c) || c == '_')
				{
					// Identifier oder Keyword
					var ident = ReadIdentifier(c);
					var tokenType = KeywordOrIdentifier(ident);
					var token = new Token(tokenType, ident, _line, _column);
					Console.WriteLine($"[DEBUG][Lexer] Token: {tokenType} '{ident}' @ {_line}:{_column}");
					tokens.Add(token);
				}
				else if (char.IsDigit(c))
				{
					// Nummer
					var number = ReadNumber(c);
					var token = new Token(TokenType.NumberLiteral, number, _line, _column);
					Console.WriteLine($"[DEBUG][Lexer] Token: {TokenType.NumberLiteral} '{number}' @ {_line}:{_column}");
					tokens.Add(token);
				}
				else
				{
					switch (c)
					{
						case '=':
							if (Match('='))
								tokens.Add(NewToken(TokenType.DoubleEquals, "=="));
							else
								tokens.Add(NewToken(TokenType.Equals, "="));
							break;
						case '+':
							tokens.Add(NewToken(TokenType.Plus, "+"));
							break;
						case '-':
							tokens.Add(NewToken(TokenType.Minus, "-"));
							break;
						case '*':
							tokens.Add(NewToken(TokenType.Asterisk, "*"));
							break;
						case '/':
							if (Match('/'))
							{
								// Einzeiliger Kommentar
								SkipLineComment();
							}
							else if (Match('*'))
							{
								// Mehrzeiliger Kommentar
								SkipBlockComment();
							}
							else
							{
								tokens.Add(NewToken(TokenType.Slash, "/"));
							}
							break;
						case '%':
							tokens.Add(NewToken(TokenType.Percent, "%"));
							break;
						case '>':
							if (Match('='))
								tokens.Add(NewToken(TokenType.GreaterEqual, ">="));
							else
								tokens.Add(NewToken(TokenType.Greater, ">"));
							break;
						case '<':
							if (Match('='))
								tokens.Add(NewToken(TokenType.LessEqual, "<="));
							else
								tokens.Add(NewToken(TokenType.Less, "<"));
							break;
						case '!':
							if (Match('='))
								tokens.Add(NewToken(TokenType.NotEquals, "!="));
							else
								tokens.Add(NewToken(TokenType.Bang, "!"));
							break;
						case '&':
							if (Match('&'))
								tokens.Add(NewToken(TokenType.AmpAmp, "&&"));
							// ggf. else-Fehler
							break;
						case '|':
							if (Match('|'))
								tokens.Add(NewToken(TokenType.PipePipe, "||"));
							break;
						case ';':
							tokens.Add(NewToken(TokenType.Semicolon, ";"));
							break;
						case ',':
							tokens.Add(NewToken(TokenType.Comma, ","));
							break;
						case '(':
							tokens.Add(NewToken(TokenType.LParen, "("));
							break;
						case ')':
							tokens.Add(NewToken(TokenType.RParen, ")"));
							break;
						case '{':
							tokens.Add(NewToken(TokenType.LBrace, "{"));
							break;
						case '}':
							tokens.Add(NewToken(TokenType.RBrace, "}"));
							break;
						case '[':
							tokens.Add(NewToken(TokenType.LBracket, "["));
							break;
						case ']':
							tokens.Add(NewToken(TokenType.RBracket, "]"));
							break;
						case ':':
							tokens.Add(NewToken(TokenType.Colon, ":"));
							break;
						case '"':
							var strVal = ReadString();
							var strToken = new Token(TokenType.StringLiteral, strVal, _line, _column);
							Console.WriteLine($"[DEBUG][Lexer] Token: {TokenType.StringLiteral} '{strVal}' @ {_line}:{_column}");
							tokens.Add(strToken);
							break;
						case '.':
							tokens.Add(NewToken(TokenType.Dot, "."));
							break;
						default:
							// Unbekanntes Zeichen -> ignorieren oder Fehler
							break;
					}
				}
			}

			tokens.Add(NewToken(TokenType.Eof, ""));
			Console.WriteLine($"[DEBUG] Lex() fertig, {tokens.Count} Tokens");
			return tokens;
		}

		private string ReadIdentifier(char firstChar)
		{
			Console.WriteLine($"[DEBUG] ReadIdentifier startet mit '{firstChar}'");
			var sb = new StringBuilder();
			sb.Append(firstChar);

			while (!IsAtEnd() && (char.IsLetterOrDigit(Peek()) || Peek() == '_'))
			{
				var nextChar = Peek();
				Console.WriteLine($"[DEBUG] ReadIdentifier: pos={_pos}, nextChar='{nextChar}'");
				sb.Append(Advance());
			}

			var result = sb.ToString();
			Console.WriteLine($"[DEBUG] ReadIdentifier fertig: '{result}'");
			return result;
		}

		private string ReadNumber(char firstChar)
		{
			var sb = new StringBuilder();
			sb.Append(firstChar);

			bool hasDot = false;

			while (!IsAtEnd())
			{
				if (char.IsDigit(Peek()))
				{
					sb.Append(Advance());
				}
				else if (Peek() == '.' && !hasDot)
				{
					hasDot = true;
					sb.Append(Advance());
				}
				else
				{
					break;
				}
			}

			return sb.ToString();
		}

		private string ReadString()
		{
			var sb = new StringBuilder();
			while (!IsAtEnd() && Peek() != '"')
			{
				sb.Append(Advance());
			}
			// Schluckendes " Ende
			if (!IsAtEnd())
			{
				Advance(); // Konsumiere das schließende Anführungszeichen
			}
			return sb.ToString();
		}

		private void SkipLineComment()
		{
			while (!IsAtEnd() && Peek() != '\n')
				Advance();
		}

		private void SkipBlockComment()
		{
			while (!IsAtEnd())
			{
				if (Peek() == '*' && PeekNext() == '/')
				{
					Advance();
					Advance();
					break;
				}
				else
				{
					Advance();
				}
			}
		}

		private TokenType KeywordOrIdentifier(string ident)
		{
			return ident switch
			{
				// Grundlegende Programmstruktur
				"Focus" => TokenType.Focus,
				"Relax" => TokenType.Relax,
				"entrance" => TokenType.Entrance,
				"deepFocus" => TokenType.DeepFocus,

				// Variablen und Deklarationen
				"induce" => TokenType.Induce,
				"from" => TokenType.From,
				"external" => TokenType.External,

				// Kontrollstrukturen
				"if" => TokenType.If,
				"else" => TokenType.Else,
				"while" => TokenType.While,
				"loop" => TokenType.Loop,
				"snap" => TokenType.Snap,
				"sink" => TokenType.Sink,
				"sinkTo" => TokenType.SinkTo,

				// Funktionen
				"suggestion" => TokenType.Suggestion,
				"imperative" => TokenType.ImperativeSuggestion,
				"dominant" => TokenType.Dominant,
				"awaken" => TokenType.Awaken,
				"call" => TokenType.Call,

				// Objektorientierung
				"session" => TokenType.Session,
				"constructor" => TokenType.Constructor,
				"expose" => TokenType.Expose,
				"conceal" => TokenType.Conceal,

				// Strukturen
				"tranceify" => TokenType.Tranceify,

				// Ein-/Ausgabe
				"observe" => TokenType.Observe,
				"drift" => TokenType.Drift,

				// Hypnotische Operatoren
				"youAreFeelingVerySleepy" => TokenType.YouAreFeelingVerySleepy,
				"lookAtTheWatch" => TokenType.LookAtTheWatch,
				"fallUnderMySpell" => TokenType.FallUnderMySpell,
				"notSoDeep" => TokenType.NotSoDeep,
				"deeplyGreater" => TokenType.DeeplyGreater,
				"deeplyLess" => TokenType.DeeplyLess,

				// Module und Globale
				"mindLink" => TokenType.MindLink,
				"sharedTrance" => TokenType.SharedTrance,

				// Typen
				"number" => TokenType.Number,
				"string" => TokenType.String,
				"boolean" => TokenType.Boolean,
				"trance" => TokenType.Trance,

				// Boolean Literale
				"true" => TokenType.True,
				"false" => TokenType.False,

				_ => TokenType.Identifier
			};
		}

		private char Advance()
		{
			var c = _source[_pos];
			_pos++;
			_column++;
			return c;
		}

		private bool Match(char expected)
		{
			if (IsAtEnd()) return false;
			if (_source[_pos] == expected)
			{
				_pos++;
				_column++;
				return true;
			}
			return false;
		}

		private char Peek() => IsAtEnd() ? '\0' : _source[_pos];
		private char PeekNext() => (_pos + 1 >= _source.Length) ? '\0' : _source[_pos + 1];

		private bool IsAtEnd() => _pos >= _source.Length;

		private Token NewToken(TokenType type, string lexeme)
		{
			var token = new Token(type, lexeme, _line, _column);
			Console.WriteLine($"[DEBUG][NewToken] Token: {type} '{lexeme}' @ {_line}:{_column}");
			return token;
		}

		// Hilfsmethode, um das nächste Wort zu peeken (ohne Whitespace zu überspringen)
		private string PeekWord()
		{
			int pos = _pos;
			while (pos < _source.Length && char.IsWhiteSpace(_source[pos])) pos++;
			var sb = new StringBuilder();
			while (pos < _source.Length && (char.IsLetter(_source[pos]) || _source[pos] == '_'))
				sb.Append(_source[pos++]);
			return sb.ToString();
		}
	}
}
