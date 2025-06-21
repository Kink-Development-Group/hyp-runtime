using HypnoScript.LexerParser.AST;
using HypnoScript.Runtime;
using HypnoScript.Core.Symbols;
using HypnoScript.LexerParser.Parser;
using System.IO;

namespace HypnoScript.Compiler.Interpreter
{
	public class BreakException : Exception { }
	public class ContinueException : Exception { }

	public partial class HypnoInterpreter
	{
		private readonly SymbolTable _globals = new();

		private class SinkToLabelException : Exception
		{
			public string LabelName { get; }
			public SinkToLabelException(string labelName) { LabelName = labelName; }
		}

		public void ExecuteProgram(ProgramNode program)
		{
			// Führe entrance-Block (falls vorhanden) zuerst aus
			foreach (var stmt in program.Statements)
			{
				if (stmt is EntranceBlockNode entrance)
				{
					ExecuteBlockWithLabels(entrance.Statements);
				}
			}
			// Führe alle anderen Statements aus (außer EntranceBlockNode)
			var mainStatements = new List<IStatement>();
			foreach (var stmt in program.Statements)
			{
				if (stmt is not EntranceBlockNode)
				{
					mainStatements.Add(stmt);
				}
			}
			ExecuteBlockWithLabels(mainStatements);
		}

		private void ExecuteStatement(IStatement stmt)
		{
			switch (stmt)
			{
				case VarDeclNode varDecl:
					ExecuteVarDecl(varDecl);
					break;
				case IfStatementNode ifNode:
					ExecuteIf(ifNode);
					break;
				case WhileStatementNode whileNode:
					ExecuteWhile(whileNode);
					break;
				case LoopStatementNode loopNode:
					ExecuteLoop(loopNode);
					break;
				case ObserveStatementNode obs:
					var value = EvaluateExpression(obs.Expression);
					HypnoBuiltins.Observe(value);
					break;
				case DriftStatementNode drift:
					var ms = EvaluateExpression(drift.Milliseconds);
					if (ms is int intMs)
						HypnoBuiltins.Drift(intMs);
					else if (ms is double doubleMs)
						HypnoBuiltins.Drift((int)doubleMs);
					else
						throw new Exception("drift() expects a number");
					break;
				case ReturnStatementNode ret:
					// In einem vollwertigen System -> return from function
					// Hier ignorieren wir's (kein function context)
					break;
				case ExpressionStatementNode exprStmt:
					EvaluateExpression(exprStmt.Expression);
					break;
				case SnapStatementNode:
					throw new BreakException();
				case SinkStatementNode:
					throw new ContinueException();
				case SessionDeclNode sessionDecl:
					ExecuteSessionDeclaration(sessionDecl);
					break;
				case TranceifyDeclNode tranceifyDecl:
					ExecuteTranceifyDeclaration(tranceifyDecl);
					break;
				case FunctionDeclNode funcDecl:
					ExecuteFunctionDeclaration(funcDecl);
					break;
				case MindLinkNode mindLink:
					ImportMindLink(mindLink.FileName);
					break;
				case SharedTranceVarDeclNode shared:
					object? sharedVal = null;
					if (shared.Initializer != null)
						sharedVal = EvaluateExpression(shared.Initializer);
					var sharedSym = new Symbol(shared.Identifier, shared.TypeName, sharedVal);
					if (!_globals.Define(sharedSym))
						Console.Error.WriteLine($"[sharedTrance] Variable '{shared.Identifier}' already defined");
					break;
				case LabelNode label:
					// Label-Statement selbst tut nichts zur Laufzeit
					break;
				case SinkToNode sinkTo:
					throw new SinkToLabelException(sinkTo.LabelName);
				default:
					throw new NotSupportedException($"Unsupported statement type: {stmt.GetType().Name}");
			}
		}

		private void ExecuteVarDecl(VarDeclNode decl)
		{
			object? val = null;
			if (decl.FromExternal)
			{
				// Flexible Input-Quelle
				var input = HypnoBuiltins.InputProvider($"Input for {decl.Identifier}: ");
				val = input;
			}
			else if (decl.Initializer != null)
			{
				val = EvaluateExpression(decl.Initializer);
			}

			var sym = new Symbol(decl.Identifier, decl.TypeName, val);
			if (!_globals.Define(sym))
			{
				throw new Exception($"Variable {decl.Identifier} already defined");
			}
		}

		private void ExecuteIf(IfStatementNode ifNode)
		{
			var condValue = EvaluateExpression(ifNode.Condition);
			if (IsTruthy(condValue))
			{
				foreach (var st in ifNode.ThenBranch)
					ExecuteStatement(st);
			}
			else if (ifNode.ElseBranch != null)
			{
				foreach (var st in ifNode.ElseBranch)
					ExecuteStatement(st);
			}
		}

		private void ExecuteWhile(WhileStatementNode whileNode)
		{
			while (true)
			{
				var cond = EvaluateExpression(whileNode.Condition);
				if (!IsTruthy(cond)) break;

				foreach (var st in whileNode.Body)
				{
					ExecuteStatement(st);
				}
			}
		}

		private void ExecuteLoop(LoopStatementNode loopNode)
		{
			// Execute initializer
			if (loopNode.Initializer != null)
			{
				ExecuteStatement(loopNode.Initializer);
			}

			while (true)
			{
				var cond = EvaluateExpression(loopNode.Condition);
				if (!IsTruthy(cond)) break;

				try
				{
					foreach (var st in loopNode.Body)
					{
						ExecuteStatement(st);
					}
				}
				catch (BreakException)
				{
					break;
				}
				catch (ContinueException)
				{
					// Continue - skip to iteration
				}

				// Execute iteration
				if (loopNode.Iteration != null)
				{
					ExecuteStatement(loopNode.Iteration);
				}
			}
		}

		private void ExecuteSessionDeclaration(SessionDeclNode sessionDecl)
		{
			// Store session definition in globals for later instantiation
			var sessionSymbol = new Symbol(sessionDecl.Name, "session", sessionDecl);
			_globals.Define(sessionSymbol);
		}

		private void ExecuteTranceifyDeclaration(TranceifyDeclNode tranceifyDecl)
		{
			// Store tranceify definition in globals for later instantiation
			var tranceifySymbol = new Symbol(tranceifyDecl.Name, "tranceify", tranceifyDecl);
			_globals.Define(tranceifySymbol);
		}

		private void ExecuteFunctionDeclaration(FunctionDeclNode funcDecl)
		{
			// Store function definition in globals for later calls
			var funcSymbol = new Symbol(funcDecl.Name, "function", funcDecl);
			_globals.Define(funcSymbol);
		}

		private object? EvaluateExpression(IExpression expr)
		{
			switch (expr)
			{
				case LiteralExpressionNode lit:
					return ParseLiteral(lit);
				case IdentifierExpressionNode id:
					var s = _globals.Resolve(id.Name);
					if (s == null) throw new Exception($"Unknown identifier {id.Name}");
					return s.Value;
				case BinaryExpressionNode bin:
					return EvaluateBinary(bin);
				case UnaryExpressionNode unary:
					return EvaluateUnary(unary);
				case ParenthesizedExpressionNode paren:
					return EvaluateExpression(paren.Expression);
				case AssignmentExpressionNode assign:
					return EvaluateAssignment(assign);
				case CallExpressionNode call:
					return EvaluateCall(call);
				case MethodCallExpressionNode methodCall:
					return EvaluateMethodCall(methodCall);
				case SessionInstantiationNode sessionInst:
					return EvaluateSessionInstantiation(sessionInst);
				case FieldAccessExpressionNode field:
					return EvaluateFieldAccess(field);
				case RecordLiteralExpressionNode rec:
					return EvaluateRecordLiteral(rec);
				case ArrayAccessExpressionNode arrayAccess:
					return EvaluateArrayAccess(arrayAccess);
				case ArrayLiteralExpressionNode arrayLit:
					return EvaluateArrayLiteral(arrayLit);
				default:
					throw new NotSupportedException($"Unsupported expression type: {expr.GetType().Name}");
			}
		}

		private object? ParseLiteral(LiteralExpressionNode lit)
		{
			if (lit.LiteralType == "number")
			{
				if (lit.Value.Contains("."))
					return double.Parse(lit.Value);
				else
					return int.Parse(lit.Value);
			}
			else if (lit.LiteralType == "boolean")
			{
				return (lit.Value == "true");
			}
			else
			{
				// string
				return lit.Value;
			}
		}

		private bool IsTruthy(object? val)
		{
			if (val == null) return false;
			if (val is bool b) return b;
			// everything else treat as true
			return true;
		}

		private object? EvaluateBinary(BinaryExpressionNode bin)
		{
			var leftVal = EvaluateExpression(bin.Left);
			var rightVal = EvaluateExpression(bin.Right);

			// Operator-Synonyme unterstützen
			switch (bin.Operator)
			{
				case "+":
					return Convert.ToDouble(leftVal) + Convert.ToDouble(rightVal);
				case "-":
					return Convert.ToDouble(leftVal) - Convert.ToDouble(rightVal);
				case "*":
					return Convert.ToDouble(leftVal) * Convert.ToDouble(rightVal);
				case "/":
					return Convert.ToDouble(leftVal) / Convert.ToDouble(rightVal);
				case ">":
				case "lookAtTheWatch":
					return Convert.ToDouble(leftVal) > Convert.ToDouble(rightVal);
				case "<":
				case "fallUnderMySpell":
					return Convert.ToDouble(leftVal) < Convert.ToDouble(rightVal);
				case ">=":
				case "deeplyGreater":
					return Convert.ToDouble(leftVal) >= Convert.ToDouble(rightVal);
				case "<=":
				case "deeplyLess":
					return Convert.ToDouble(leftVal) <= Convert.ToDouble(rightVal);
				case "==":
				case "youAreFeelingVerySleepy":
					return Equals(leftVal, rightVal);
				case "!=":
				case "notSoDeep":
					return !Equals(leftVal, rightVal);
				default:
					throw new Exception($"Unrecognized operator {bin.Operator}");
			}
		}

		private object? EvaluateUnary(UnaryExpressionNode unary)
		{
			var operand = EvaluateExpression(unary.Operand);
			switch (unary.Operator)
			{
				case "-":
					return -Convert.ToDouble(operand);
				case "+":
					return operand;
				default:
					throw new Exception($"Unrecognized unary operator: {unary.Operator}");
			}
		}

		private object? EvaluateAssignment(AssignmentExpressionNode assign)
		{
			var right = EvaluateExpression(assign.Value);

			// Für einfache Variablenzuweisungen
			if (_globals.Resolve(assign.Identifier) != null)
			{
				// Update existing variable
				// Note: This is a simplified implementation
				// In a full implementation, we'd need to update the symbol table
				return right;
			}

			throw new Exception($"Variable '{assign.Identifier}' not defined for assignment");
		}

		private object? EvaluateCall(CallExpressionNode call)
		{
			var funcNameNode = call.Callee as IdentifierExpressionNode;
			if (funcNameNode == null)
				throw new Exception("Unsupported callee type");

			// Erweiterung: Funktionen, Sessions und tranceify sollen hier unterstützt werden
			if (call.Callee is IdentifierExpressionNode func)
			{
				// Prüfe, ob es eine Funktion als Symbol gibt
				var symbol = _globals.Resolve(func.Name);
				if (symbol?.Value is FunctionDeclNode function)
				{
					// Erstelle neuen Scope, binde Parameter und führe Body aus
					var localScope = new SymbolTable(_globals);
					for (int i = 0; i < function.Parameters.Count; i++)
					{
						var param = function.Parameters[i];
						var argValue = EvaluateExpression(call.Arguments[i]);
						localScope.Define(new Symbol(param.Name, param.TypeName, argValue));
					}
					foreach (var stmt in function.Body)
					{
						ExecuteStatement(stmt);
					}
					return null; // Rückgabewert in dieser rudimentären Implementierung ignoriert
				}
				// Prüfe, ob es eine Session als Symbol gibt (für statische Methoden)
				var sessionSym = _globals.Resolve(func.Name);
				if (sessionSym?.Value is SessionDeclNode sessionDecl)
				{
					// Suche dominant suggestion
					foreach (var member in sessionDecl.Members)
					{
						if (member is SessionMemberNode sm && sm.Declaration is FunctionDeclNode f && f.Dominant)
						{
							// Erstelle neuen Scope, binde Parameter und führe Body aus
							var localScope = new SymbolTable(_globals);
							for (int i = 0; i < f.Parameters.Count; i++)
							{
								var param = f.Parameters[i];
								var argValue = EvaluateExpression(call.Arguments[i]);
								localScope.Define(new Symbol(param.Name, param.TypeName, argValue));
							}
							foreach (var stmt in f.Body)
							{
								ExecuteStatement(stmt);
							}
							return null;
						}
					}
					throw new Exception($"No dominant suggestion found in session '{func.Name}'");
				}
				// Weitere Builtins etc.
			}

			// Hardcode builtins: drift(...)?
			if (funcNameNode.Name == "drift")
			{
				if (call.Arguments.Count != 1)
					throw new Exception("drift(ms) expects 1 arg");
				var msVal = EvaluateExpression(call.Arguments[0]);
				HypnoBuiltins.Drift(Convert.ToInt32(msVal));
				return null;
			}

			// Normal function calls -> not implemented in this minimal example
			throw new Exception($"Unknown function {funcNameNode.Name}");
		}

		private object? EvaluateMethodCall(MethodCallExpressionNode methodCall)
		{
			// Implementation of method call evaluation
			throw new NotImplementedException();
		}

		private object? EvaluateSessionInstantiation(SessionInstantiationNode sessionInst)
		{
			// Implementation of session instantiation evaluation
			throw new NotImplementedException();
		}

		private object? EvaluateFieldAccess(FieldAccessExpressionNode field)
		{
			var target = EvaluateExpression(field.Target);
			if (target is Dictionary<string, object?> recordDict)
			{
				if (recordDict.TryGetValue(field.FieldName, out var value))
					return value;
				throw new Exception($"Field '{field.FieldName}' not found in record.");
			}
			if (target is SessionInstance session)
			{
				if (session.Fields.TryGetValue(field.FieldName, out var value))
					return value;
				throw new Exception($"Field '{field.FieldName}' not found in session '{session.Name}'.");
			}
			throw new Exception($"Field access on non-record/session value: {field.FieldName}");
		}

		private object? EvaluateRecordLiteral(RecordLiteralExpressionNode rec)
		{
			var dict = new Dictionary<string, object?>();
			foreach (var kv in rec.Fields)
			{
				dict[kv.Key] = EvaluateExpression(kv.Value);
			}
			// Optional: dict["__type"] = rec.TypeName;
			return dict;
		}

		private object? EvaluateArrayAccess(ArrayAccessExpressionNode arrayAccess)
		{
			// Implementation of array access evaluation
			throw new NotImplementedException();
		}

		private object? EvaluateArrayLiteral(ArrayLiteralExpressionNode arrayLit)
		{
			// Implementation of array literal evaluation
			throw new NotImplementedException();
		}

		private void ImportMindLink(string fileName)
		{
			// Annahme: relativer Pfad, .hyp-Datei
			if (!File.Exists(fileName))
			{
				Console.Error.WriteLine($"[mindLink] File not found: {fileName}");
				return;
			}
			var code = File.ReadAllText(fileName);
			var lexer = new HypnoScript.LexerParser.Lexer.HypnoLexer(code);
			var tokens = lexer.Lex();
			var parser = new HypnoParser(tokens);
			var importedProgram = parser.ParseProgram();
			// Übernehme nur globale Definitionen
			foreach (var stmt in importedProgram.Statements)
			{
				switch (stmt)
				{
					case SessionDeclNode session:
						_globals.Define(new HypnoScript.Core.Symbols.Symbol(session.Name, "session", session));
						break;
					case TranceifyDeclNode trance:
						_globals.Define(new HypnoScript.Core.Symbols.Symbol(trance.Name, "tranceify", trance));
						break;
					case FunctionDeclNode func:
						_globals.Define(new HypnoScript.Core.Symbols.Symbol(func.Name, func.ReturnType, func));
						break;
					case VarDeclNode varDecl:
						_globals.Define(new HypnoScript.Core.Symbols.Symbol(varDecl.Identifier, varDecl.TypeName));
						break;
				}
			}
		}

		private void ExecuteBlockWithLabels(List<IStatement> statements)
		{
			// Mappe Labelnamen auf Statement-Index
			var labelMap = new Dictionary<string, int>();
			for (int i = 0; i < statements.Count; i++)
			{
				if (statements[i] is HypnoScript.LexerParser.AST.LabelNode label)
					labelMap[label.Name] = i;
			}
			for (int i = 0; i < statements.Count; i++)
			{
				try
				{
					ExecuteStatement(statements[i]);
				}
				catch (SinkToLabelException ex)
				{
					if (labelMap.TryGetValue(ex.LabelName, out var targetIdx))
					{
						i = targetIdx - 1; // -1, da i++ im Loop
						continue;
					}
					else
					{
						throw; // Label nicht im Block gefunden -> Exception weiterwerfen
					}
				}
			}
		}
	}
}
