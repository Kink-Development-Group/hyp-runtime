using HypnoScript.LexerParser.AST;
using HypnoScript.LexerParser.Lexer;

namespace HypnoScript.LexerParser.Parser
{
	public class HypnoParser
	{
		private readonly List<Token> _tokens;
		private int _current;

		public HypnoParser(IEnumerable<Token> tokens)
		{
			_tokens = tokens.ToList();
		}

		public ProgramNode ParseProgram()
		{
			Console.WriteLine($"[DEBUG] Start ParseProgram, current token: {Peek().Type} '{Peek().Lexeme}'");
			// Sicherstellen, dass das Programm mit "Focus" beginnt
			if (!Check(TokenType.Focus))
				throw new Exception("Program must start with 'Focus'.");
			Advance(); // consume Focus

			var statements = ParseBlockStatements();

			Console.WriteLine($"[DEBUG] Nach Block, current token: {Peek().Type} '{Peek().Lexeme}'");
			if (!Check(TokenType.Relax))
				throw new Exception("Program must end with 'Relax'.");
			Advance(); // consume Relax

			return new ProgramNode(statements);
		}

		private IStatement ParseStatement()
		{
			if (Match(TokenType.Induce))
				return ParseVarDecl();

			if (Match(TokenType.If))
				return ParseIfStatement();

			if (Match(TokenType.While))
				return ParseWhileStatement();

			if (Match(TokenType.Loop))
				return ParseLoopStatement();

			if (Match(TokenType.Suggestion) || Match(TokenType.ImperativeSuggestion) || Match(TokenType.DominantSuggestion))
				return ParseFunctionDeclaration();

			if (Match(TokenType.Session))
				return ParseSessionDeclaration();

			if (Match(TokenType.Tranceify))
				return ParseTranceifyDeclaration();

			if (Match(TokenType.Observe))
				return ParseObserveStatement();

			if (Match(TokenType.Drift))
				return ParseDriftStatement();

			if (Match(TokenType.Awaken))
				return ParseReturnStatement();

			if (Match(TokenType.Snap))
			{
				Consume(TokenType.Semicolon, "Expect ';' after snap.");
				return new SnapStatementNode();
			}
			if (Match(TokenType.Sink))
			{
				Consume(TokenType.Semicolon, "Expect ';' after sink.");
				return new SinkStatementNode();
			}
			if (Match(TokenType.MindLink))
			{
				var fileToken = Consume(TokenType.StringLiteral, "Expected string literal after mindLink.");
				Consume(TokenType.Semicolon, "Expect ';' after mindLink statement.");
				return new MindLinkNode(fileToken.Lexeme);
			}
			if (Match(TokenType.SharedTrance))
			{
				var nameToken = Consume(TokenType.Identifier, "Expect identifier after 'sharedTrance'.");
				string? typeName = null;
				IExpression? initializer = null;
				if (Match(TokenType.Colon))
				{
					var typeToken = Consume(TokenType.Identifier, "Expect type name after ':' in sharedTrance.");
					typeName = typeToken.Lexeme;
				}
				if (Match(TokenType.Equals))
				{
					initializer = ParseExpression();
				}
				Consume(TokenType.Semicolon, "Expect ';' after sharedTrance declaration.");
				return new SharedTranceVarDeclNode(nameToken.Lexeme, typeName, initializer);
			}
			if (Match(TokenType.Label))
			{
				var labelName = Previous().Lexeme;
				return new LabelNode(labelName);
			}
			if (Match(TokenType.SinkTo))
			{
				var labelToken = Consume(TokenType.Identifier, "Expected label name after 'sinkTo'.");
				Consume(TokenType.Semicolon, "Expect ';' after sinkTo statement.");
				return new SinkToNode(labelToken.Lexeme);
			}
			// Fallback: Expression Statement
			var expr = ParseExpression();
			Consume(TokenType.Semicolon, "Expect ';' after expression.");
			return new ExpressionStatementNode(expr);
		}

		private IStatement ParseDriftStatement()
		{
			// drift(expression);
			Consume(TokenType.LParen, "Expect '(' after 'drift'.");
			var milliseconds = ParseExpression();
			Consume(TokenType.RParen, "Expect ')' after drift expression.");
			Consume(TokenType.Semicolon, "Expect ';' after drift statement.");
			return new DriftStatementNode(milliseconds);
		}

		// Neue Methode: Loop-Statement parsen
		private IStatement ParseLoopStatement()
		{
			// Annahme: "loop" wurde bereits gematcht.
			// Erwarte: '(' [Initialisierung] ';' Expression ';' Expression ')' BlockStatement.
			Consume(TokenType.LParen, "Expected '(' after 'loop'.");

			IStatement? initializer = null;
			if (!Check(TokenType.Semicolon))
			{
				initializer = ParseVarDecl(); // Hier könnte ein spezialisierter Parser für eine VarDecl ohne abschließendes ';' nötig sein.
			}
			Consume(TokenType.Semicolon, "Expected ';' after loop initializer.");

			var condition = ParseExpression();
			Consume(TokenType.Semicolon, "Expected ';' after loop condition.");

			IExpression iteration = ParseExpression();
			Consume(TokenType.RParen, "Expected ')' after loop iteration.");

			var body = ParseBlockStatements();
			return new LoopStatementNode(initializer, condition, new ExpressionStatementNode(iteration), body);
		}

		// Neue Methode: Funktionsdeklaration parsen
		private IStatement ParseFunctionDeclaration()
		{
			// Erwartet: (suggestion | imperative suggestion | dominant suggestion) Identifier '(' [ParameterList] ')' [':' Type] BlockStatement.
			// Das Schlüsselwort wurde bereits gematcht, wir speichern es zur Unterscheidung.
			string funcKeyword = Previous().Lexeme;
			var nameToken = Consume(TokenType.Identifier, "Expected function name after suggestion keyword.");
			Consume(TokenType.LParen, "Expected '(' after function name.");
			var parameters = new List<ParameterNode>();
			if (!Check(TokenType.RParen))
			{
				do
				{
					var paramName = Consume(TokenType.Identifier, "Expected parameter name.").Lexeme;
					string? typeName = null;
					if (Match(TokenType.Colon))
					{
						var typeToken = Consume(TokenType.Identifier, "Expected type name after ':' in parameter list.");
						typeName = typeToken.Lexeme;
					}
					parameters.Add(new ParameterNode(paramName, typeName));
				} while (Match(TokenType.Comma));
			}
			Consume(TokenType.RParen, "Expected ')' after parameter list.");

			string? returnType = null;
			if (Match(TokenType.Colon))
			{
				var typeToken = Consume(TokenType.Identifier, "Expected return type following ':'.");
				returnType = typeToken.Lexeme;
			}

			var body = ParseBlockStatements();
			bool imperative = funcKeyword.Contains("imperative");
			bool dominant = funcKeyword.Contains("dominant");
			return new FunctionDeclNode(nameToken.Lexeme, parameters, returnType, body, imperative, dominant);
		}

		// Neue Methode: Session-Deklaration parsen
		private IStatement ParseSessionDeclaration()
		{
			// Erwartet: 'session' Identifier '{' { SessionMember } '}'
			var nameToken = Consume(TokenType.Identifier, "Expected session name after 'session'.").Lexeme;
			Consume(TokenType.LBrace, "Expected '{' after session name.");
			var members = new List<SessionMemberNode>();
			while (!Check(TokenType.RBrace) && !IsAtEnd())
			{
				members.Add(ParseSessionMember());
			}
			Consume(TokenType.RBrace, "Expected '}' to close session declaration.");
			return new SessionDeclNode(nameToken, members);
		}

		private SessionMemberNode ParseSessionMember()
		{
			bool isExposed = false;
			bool isDominant = false;

			// Parse expose/conceal
			if (Match(TokenType.Expose))
				isExposed = true;
			else if (Match(TokenType.Conceal))
				isExposed = false;

			// Parse dominant
			if (Match(TokenType.Dominant))
				isDominant = true;

			// Parse the actual declaration
			IStatement declaration;
			if (Match(TokenType.Induce))
			{
				declaration = ParseVarDecl();
			}
			else if (Match(TokenType.Suggestion))
			{
				declaration = ParseFunctionDeclaration();
			}
			else
			{
				throw new Exception("Expected 'induce' or 'suggestion' in session member.");
			}

			return new SessionMemberNode(isExposed, isDominant, declaration);
		}

		// Neue Methode: Tranceify-Deklaration parsen
		private IStatement ParseTranceifyDeclaration()
		{
			// Erwartet: 'tranceify' Identifier '{' { VarDeclaration } '}'
			var nameToken = Consume(TokenType.Identifier, "Expected tranceify name after 'tranceify'.").Lexeme;
			Consume(TokenType.LBrace, "Expected '{' after tranceify name.");
			var members = new List<VarDeclNode>();
			while (!Check(TokenType.RBrace) && !IsAtEnd())
			{
				// Wir parsen jede VarDecl innerhalb des Tranceify-Blocks und casten explizit zu VarDeclNode.
				IStatement stmt = ParseVarDecl();
				if (stmt is VarDeclNode varDecl)
				{
					members.Add(varDecl);
				}
				else
				{
					throw new Exception("Expected variable declaration inside tranceify block.");
				}
			}
			Consume(TokenType.RBrace, "Expected '}' to close tranceify declaration.");
			return new TranceifyDeclNode(nameToken, members);
		}

		private IStatement ParseVarDecl()
		{
			// 'induce x: number = 5;' oder 'induce y from external;'
			var nameToken = Consume(TokenType.Identifier, "Expect identifier after 'induce'.");

			string? typeName = null;
			bool fromExternal = false;
			IExpression? initializer = null;

			if (Match(TokenType.Colon))
			{
				// parse type - akzeptiere Identifier oder Typ-Keywords
				if (Match(TokenType.Number) || Match(TokenType.String) || Match(TokenType.Boolean))
				{
					typeName = Previous().Lexeme;
				}
				else
				{
					var typeToken = Consume(TokenType.Identifier, "Expect type name after ':'.");
					typeName = typeToken.Lexeme;
				}
			}

			if (Match(TokenType.Equals))
			{
				// parse initializer
				initializer = ParseExpression();
			}
			else if (MatchKeyword("from"))
			{
				// parse 'from external'
				if (!MatchKeyword("external"))
					throw new Exception("Expected 'external' after 'from'.");
				fromExternal = true;
			}

			Consume(TokenType.Semicolon, "Expect ';' after variable declaration.");

			return new VarDeclNode(nameToken.Lexeme, typeName, initializer, fromExternal);
		}

		private IStatement ParseIfStatement()
		{
			// if ( expr ) { ... } else { ... }
			Consume(TokenType.LParen, "Expect '(' after 'if'.");
			var condition = ParseExpression();
			Consume(TokenType.RParen, "Expect ')' after if condition.");

			var thenBlock = ParseBlockStatements();

			List<IStatement>? elseBlock = null;
			if (Match(TokenType.Else))
			{
				elseBlock = ParseBlockStatements();
			}

			return new IfStatementNode(condition, thenBlock, elseBlock);
		}

		private IStatement ParseWhileStatement()
		{
			Consume(TokenType.LParen, "Expect '(' after 'while'.");
			var condition = ParseExpression();
			Consume(TokenType.RParen, "Expect ')' after condition.");

			var body = ParseBlockStatements();

			return new WhileStatementNode(condition, body);
		}

		private IStatement ParseObserveStatement()
		{
			// observe expression ;
			var expr = ParseExpression();
			Consume(TokenType.Semicolon, "Expect ';' after observe expression.");
			return new ObserveStatementNode(expr);
		}

		private IStatement ParseReturnStatement()
		{
			// awaken <expr> ;
			if (!Check(TokenType.Semicolon))
			{
				var expr = ParseExpression();
				Consume(TokenType.Semicolon, "Expect ';' after return expression.");
				return new ReturnStatementNode(expr);
			}
			else
			{
				// awaken ;
				Advance(); // consume semicolon
				return new ReturnStatementNode(null);
			}
		}

		private List<IStatement> ParseBlockStatements()
		{
			Console.WriteLine($"[DEBUG] Enter Block, current token: {Peek().Type} '{Peek().Lexeme}'");
			if (Match(TokenType.DeepFocus))
			{
				Consume(TokenType.LBrace, "Expect '{' after 'deepFocus'.");
			}
			else if (!Match(TokenType.LBrace))
			{
				throw new Exception("Expect '{' to start block.");
			}

			var stmts = new List<IStatement>();
			while (!Check(TokenType.RBrace) && !IsAtEnd())
			{
				Console.WriteLine($"[DEBUG] Block loop, current token: {Peek().Type} '{Peek().Lexeme}'");
				if (Match(TokenType.Entrance))
				{
					stmts.Add(ParseEntranceBlock());
				}
				else
				{
					stmts.Add(ParseStatement());
				}
			}

			Console.WriteLine($"[DEBUG] Leave Block, current token: {Peek().Type} '{Peek().Lexeme}'");
			Consume(TokenType.RBrace, "Expect '}' to end block.");
			return stmts;
		}

		// ---------------------
		// Expressions
		// ---------------------

		private IExpression ParseExpression()
		{
			return ParseAssignment();
		}

		private IExpression ParseAssignment()
		{
			var expr = ParseEquality();

			if (Match(TokenType.Equals))
			{
				var equals = Previous();
				var value = ParseAssignment();

				if (expr is IdentifierExpressionNode)
				{
					var name = ((IdentifierExpressionNode)expr).Name;
					return new AssignmentExpressionNode(name, value);
				}

				throw new Exception("Invalid assignment target.");
			}

			return expr;
		}

		private IExpression ParseEquality()
		{
			var expr = ParseComparison();

			while (Match(TokenType.DoubleEquals) || Match(TokenType.NotEquals) ||
				   Match(TokenType.YouAreFeelingVerySleepy) || Match(TokenType.NotSoDeep))
			{
				var op = Previous().Lexeme;

				// Map Synonyme
				if (Previous().Type.Equals(TokenType.YouAreFeelingVerySleepy))
					op = "==";
				if (Previous().Type.Equals(TokenType.NotSoDeep))
					op = "!=";

				var right = ParseComparison();
				expr = new BinaryExpressionNode(expr, op, right);
			}

			return expr;
		}

		private IExpression ParseComparison()
		{
			var expr = ParseTerm();

			while (Match(TokenType.Greater) || Match(TokenType.GreaterEqual) ||
				   Match(TokenType.Less) || Match(TokenType.LessEqual) ||
				   Match(TokenType.LookAtTheWatch) || Match(TokenType.FallUnderMySpell) ||
				   Match(TokenType.DeeplyGreater) || Match(TokenType.DeeplyLess))
			{
				var op = Previous().Lexeme;
				if (Previous().Type.Equals(TokenType.LookAtTheWatch))
					op = ">";
				if (Previous().Type.Equals(TokenType.FallUnderMySpell))
					op = "<";
				if (Previous().Type.Equals(TokenType.DeeplyGreater))
					op = ">=";
				if (Previous().Type.Equals(TokenType.DeeplyLess))
					op = "<=";

				var right = ParseTerm();
				expr = new BinaryExpressionNode(expr, op, right);
			}

			return expr;
		}

		private IExpression ParseTerm()
		{
			var expr = ParseFactor();

			while (Match(TokenType.Plus) || Match(TokenType.Minus))
			{
				var op = Previous().Lexeme;
				var right = ParseFactor();
				expr = new BinaryExpressionNode(expr, op, right);
			}
			return expr;
		}

		private IExpression ParseFactor()
		{
			var expr = ParseUnary();

			while (Match(TokenType.Asterisk) || Match(TokenType.Slash) || Match(TokenType.Percent))
			{
				var op = Previous().Lexeme;
				var right = ParseUnary();
				expr = new BinaryExpressionNode(expr, op, right);
			}
			return expr;
		}

		private IExpression ParseUnary()
		{
			if (Match(TokenType.Bang) || Match(TokenType.Minus) || Match(TokenType.Plus))
			{
				var op = Previous().Lexeme;
				var right = ParseUnary();
				// in unserem AST nicht extra, wir machen es als "BinaryExpressionNode(null, op, right)"
				// -> oder ein "UnaryExpressionNode"
				return new BinaryExpressionNode(
					new LiteralExpressionNode("0", "number"), op, right);
				// unsauber, aber symbolisch
			}
			return ParsePrimary();
		}

		private IExpression ParsePrimary()
		{
			if (Match(TokenType.NumberLiteral))
				return new LiteralExpressionNode(Previous().Lexeme, "number");

			if (Match(TokenType.StringLiteral))
				return new LiteralExpressionNode(Previous().Lexeme, "string");

			if (Match(TokenType.True) || Match(TokenType.False))
				return new LiteralExpressionNode(Previous().Lexeme, "boolean");

			if (Match(TokenType.Identifier))
			{
				var name = Previous().Lexeme;

				// Session-Instanziierung: Identifier( ... )
				if (Match(TokenType.LParen))
				{
					var args = new List<IExpression>();
					if (!Check(TokenType.RParen))
					{
						do
						{
							args.Add(ParseExpression());
						} while (Match(TokenType.Comma));
					}
					Consume(TokenType.RParen, "Expect ')' after session arguments.");
					return new SessionInstantiationNode(name, args);
				}

				// Record-Literal: Identifier gefolgt von '{'
				if (Match(TokenType.LBrace))
				{
					var fields = new Dictionary<string, IExpression>();
					while (!Check(TokenType.RBrace) && !IsAtEnd())
					{
						var fieldName = Consume(TokenType.Identifier, $"Expected field name in record literal for {name}.").Lexeme;
						Consume(TokenType.Colon, "Expected ':' after field name in record literal.");
						var fieldExpr = ParseExpression();
						fields[fieldName] = fieldExpr;
						if (!Check(TokenType.RBrace))
						{
							Consume(TokenType.Comma, "Expected ',' between fields in record literal.");
						}
					}
					Consume(TokenType.RBrace, "Expected '}' to close record literal.");
					return new RecordLiteralExpressionNode(name, fields);
				}

				// Normale Identifier
				IExpression currentExpr = new IdentifierExpressionNode(name);

				// Feldzugriff und Methodenaufrufe: .field oder .method( ... )
				while (Match(TokenType.Dot))
				{
					var memberName = Consume(TokenType.Identifier, "Expected member name after '.'").Lexeme;

					// Methodenaufruf: .method( ... )
					if (Match(TokenType.LParen))
					{
						var args = new List<IExpression>();
						if (!Check(TokenType.RParen))
						{
							do
							{
								args.Add(ParseExpression());
							} while (Match(TokenType.Comma));
						}
						Consume(TokenType.RParen, "Expect ')' after method arguments.");
						currentExpr = new MethodCallExpressionNode(currentExpr, memberName, args);
					}
					else
					{
						// Feldzugriff: .field
						currentExpr = new FieldAccessExpressionNode(currentExpr, memberName);
					}
				}

				return currentExpr;
			}

			if (Match(TokenType.LParen))
			{
				var parenExpr = ParseExpression();
				Consume(TokenType.RParen, "Expect ')' after group expression.");
				return new ParenthesizedExpressionNode(parenExpr);
			}

			throw new Exception($"Unexpected token {Peek().Type} at line {Peek().Line}.");
		}

		// Hilfsfunktionen:
		private bool Match(params TokenType[] types)
		{
			foreach (var t in types)
			{
				if (Check(t))
				{
					Advance();
					return true;
				}
			}
			return false;
		}

		private bool MatchKeyword(string keyword)
		{
			if (Check(TokenType.Identifier) && Peek().Lexeme == keyword)
			{
				Advance();
				return true;
			}
			return false;
		}

		private Token Consume(TokenType type, string errorMessage)
		{
			if (Check(type)) return Advance();
			throw new Exception(errorMessage + $" Found {Peek().Type}.");
		}

		private bool Check(TokenType type)
		{
			if (IsAtEnd()) return false;
			return Peek().Type.Equals(type);
		}

		private Token Advance()
		{
			if (!IsAtEnd()) _current++;
			return Previous();
		}

		private bool IsAtEnd() => Peek().Type.Equals(TokenType.Eof);

		private Token Peek() => _tokens[_current];
		private Token Previous() => _tokens[_current - 1];

		private EntranceBlockNode ParseEntranceBlock()
		{
			// Erwartet: entrance { ... }
			Consume(TokenType.LBrace, "Expected '{' after 'entrance'.");
			var stmts = new List<IStatement>();
			while (!Check(TokenType.RBrace) && !IsAtEnd())
			{
				stmts.Add(ParseStatement());
			}
			Consume(TokenType.RBrace, "Expected '}' to close entrance block.");
			return new EntranceBlockNode(stmts);
		}
	}
}
