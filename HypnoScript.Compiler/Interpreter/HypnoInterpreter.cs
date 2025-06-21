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
					// String-Konkatenation oder arithmetische Addition
					if (leftVal is string || rightVal is string)
					{
						return leftVal?.ToString() + rightVal?.ToString();
					}
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

			// ===== MATHEMATISCHE BUILTINS =====
			if (funcNameNode.Name == "Sin" || funcNameNode.Name == "Cos" || funcNameNode.Name == "Tan" ||
				funcNameNode.Name == "Sqrt" || funcNameNode.Name == "Pow" || funcNameNode.Name == "Abs" ||
				funcNameNode.Name == "Floor" || funcNameNode.Name == "Ceiling" || funcNameNode.Name == "Round" ||
				funcNameNode.Name == "Log" || funcNameNode.Name == "Log10" || funcNameNode.Name == "Exp" ||
				funcNameNode.Name == "Max" || funcNameNode.Name == "Min" || funcNameNode.Name == "Random" ||
				funcNameNode.Name == "RandomInt")
			{
				if (call.Arguments.Count < 1)
					throw new Exception($"{funcNameNode.Name} expects at least 1 argument");

				var arg1 = Convert.ToDouble(EvaluateExpression(call.Arguments[0]));
				var arg2 = call.Arguments.Count > 1 ? Convert.ToDouble(EvaluateExpression(call.Arguments[1])) : 0.0;
				var arg3 = call.Arguments.Count > 2 ? Convert.ToInt32(EvaluateExpression(call.Arguments[2])) : 0;

				return funcNameNode.Name switch
				{
					"Sin" => HypnoBuiltins.Sin(arg1),
					"Cos" => HypnoBuiltins.Cos(arg1),
					"Tan" => HypnoBuiltins.Tan(arg1),
					"Sqrt" => HypnoBuiltins.Sqrt(arg1),
					"Pow" => HypnoBuiltins.Pow(arg1, arg2),
					"Abs" => HypnoBuiltins.Abs(arg1),
					"Floor" => HypnoBuiltins.Floor(arg1),
					"Ceiling" => HypnoBuiltins.Ceiling(arg1),
					"Round" => HypnoBuiltins.Round(arg1),
					"Log" => HypnoBuiltins.Log(arg1),
					"Log10" => HypnoBuiltins.Log10(arg1),
					"Exp" => HypnoBuiltins.Exp(arg1),
					"Max" => HypnoBuiltins.Max(arg1, arg2),
					"Min" => HypnoBuiltins.Min(arg1, arg2),
					"Random" => HypnoBuiltins.Random(),
					"RandomInt" => HypnoBuiltins.RandomInt((int)arg1, (int)arg2),
					_ => throw new Exception($"Unknown mathematical function: {funcNameNode.Name}")
				};
			}

			// ===== STRING BUILTINS =====
			if (funcNameNode.Name == "Length" || funcNameNode.Name == "ToUpper" || funcNameNode.Name == "ToLower" ||
				funcNameNode.Name == "Substring" || funcNameNode.Name == "Contains" || funcNameNode.Name == "Replace" ||
				funcNameNode.Name == "Trim" || funcNameNode.Name == "TrimStart" || funcNameNode.Name == "TrimEnd" ||
				funcNameNode.Name == "IndexOf" || funcNameNode.Name == "LastIndexOf" || funcNameNode.Name == "Split" ||
				funcNameNode.Name == "Join" || funcNameNode.Name == "StartsWith" || funcNameNode.Name == "EndsWith" ||
				funcNameNode.Name == "PadLeft" || funcNameNode.Name == "PadRight")
			{
				if (call.Arguments.Count < 1)
					throw new Exception($"{funcNameNode.Name} expects at least 1 argument");

				var str = EvaluateExpression(call.Arguments[0])?.ToString() ?? "";

				switch (funcNameNode.Name)
				{
					case "Length": return HypnoBuiltins.Length(str);
					case "ToUpper": return HypnoBuiltins.ToUpper(str);
					case "ToLower": return HypnoBuiltins.ToLower(str);
					case "Substring":
						if (call.Arguments.Count >= 3)
							return HypnoBuiltins.Substring(str, Convert.ToInt32(EvaluateExpression(call.Arguments[1])), Convert.ToInt32(EvaluateExpression(call.Arguments[2])));
						throw new Exception("Substring expects 3 arguments: string, start, length");
					case "Contains":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.Contains(str, EvaluateExpression(call.Arguments[1])?.ToString() ?? "");
						throw new Exception("Contains expects 2 arguments: string, substring");
					case "Replace":
						if (call.Arguments.Count >= 3)
							return HypnoBuiltins.Replace(str, EvaluateExpression(call.Arguments[1])?.ToString() ?? "", EvaluateExpression(call.Arguments[2])?.ToString() ?? "");
						throw new Exception("Replace expects 3 arguments: string, oldValue, newValue");
					case "Trim": return HypnoBuiltins.Trim(str);
					case "TrimStart": return HypnoBuiltins.TrimStart(str);
					case "TrimEnd": return HypnoBuiltins.TrimEnd(str);
					case "IndexOf":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.IndexOf(str, EvaluateExpression(call.Arguments[1])?.ToString() ?? "");
						throw new Exception("IndexOf expects 2 arguments: string, substring");
					case "LastIndexOf":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.LastIndexOf(str, EvaluateExpression(call.Arguments[1])?.ToString() ?? "");
						throw new Exception("LastIndexOf expects 2 arguments: string, substring");
					case "Split":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.Split(str, EvaluateExpression(call.Arguments[1])?.ToString() ?? "");
						throw new Exception("Split expects 2 arguments: string, separator");
					case "Join":
						if (call.Arguments.Count >= 2)
						{
							var arrObj = EvaluateExpression(call.Arguments[0]) as object[] ?? new object[0];
							var sep = EvaluateExpression(call.Arguments[1])?.ToString() ?? "";
							var strArr = arrObj.Select(o => o?.ToString() ?? "").ToArray();
							return HypnoBuiltins.Join(strArr, sep);
						}
						throw new Exception("Join expects 2 arguments: array, separator");
					case "StartsWith":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.StartsWith(str, EvaluateExpression(call.Arguments[1])?.ToString() ?? "");
						throw new Exception("StartsWith expects 2 arguments: string, prefix");
					case "EndsWith":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.EndsWith(str, EvaluateExpression(call.Arguments[1])?.ToString() ?? "");
						throw new Exception("EndsWith expects 2 arguments: string, suffix");
					case "PadLeft":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.PadLeft(str, Convert.ToInt32(EvaluateExpression(call.Arguments[1])), call.Arguments.Count >= 3 ? Convert.ToChar(EvaluateExpression(call.Arguments[2])) : ' ');
						throw new Exception("PadLeft expects at least 2 arguments: string, width");
					case "PadRight":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.PadRight(str, Convert.ToInt32(EvaluateExpression(call.Arguments[1])), call.Arguments.Count >= 3 ? Convert.ToChar(EvaluateExpression(call.Arguments[2])) : ' ');
						throw new Exception("PadRight expects at least 2 arguments: string, width");
					default:
						throw new Exception($"Unknown string function: {funcNameNode.Name}");
				}
			}

			// ===== ARRAY BUILTINS =====
			if (funcNameNode.Name == "ArrayLength" || funcNameNode.Name == "ArrayGet" || funcNameNode.Name == "ArraySet" ||
				funcNameNode.Name == "ArraySlice" || funcNameNode.Name == "ArrayConcat" || funcNameNode.Name == "ArrayIndexOf" ||
				funcNameNode.Name == "ArrayContains")
			{
				if (call.Arguments.Count < 1)
					throw new Exception($"{funcNameNode.Name} expects at least 1 argument");

				var arr = EvaluateExpression(call.Arguments[0]) as object[] ?? new object[0];

				switch (funcNameNode.Name)
				{
					case "ArrayLength":
						return HypnoBuiltins.ArrayLength(arr);
					case "ArrayGet":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.ArrayGet(arr, Convert.ToInt32(EvaluateExpression(call.Arguments[1])));
						throw new Exception("ArrayGet expects 2 arguments: array, index");
					case "ArraySet":
						if (call.Arguments.Count >= 3)
						{
							HypnoBuiltins.ArraySet(arr, Convert.ToInt32(EvaluateExpression(call.Arguments[1])), EvaluateExpression(call.Arguments[2]));
							return null;
						}
						throw new Exception("ArraySet expects 3 arguments: array, index, value");
					case "ArraySlice":
						if (call.Arguments.Count >= 3)
							return HypnoBuiltins.ArraySlice(arr, Convert.ToInt32(EvaluateExpression(call.Arguments[1])), Convert.ToInt32(EvaluateExpression(call.Arguments[2])));
						throw new Exception("ArraySlice expects 3 arguments: array, start, length");
					case "ArrayConcat":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.ArrayConcat(arr, EvaluateExpression(call.Arguments[1]) as object[] ?? new object[0]);
						throw new Exception("ArrayConcat expects 2 arguments: array1, array2");
					case "ArrayIndexOf":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.ArrayIndexOf(arr, EvaluateExpression(call.Arguments[1]));
						throw new Exception("ArrayIndexOf expects 2 arguments: array, value");
					case "ArrayContains":
						if (call.Arguments.Count >= 2)
							return HypnoBuiltins.ArrayContains(arr, EvaluateExpression(call.Arguments[1]));
						throw new Exception("ArrayContains expects 2 arguments: array, value");
					default:
						throw new Exception($"Unknown array function: {funcNameNode.Name}");
				}
			}

			// ===== KONVERTIERUNGSFUNKTIONEN =====
			if (funcNameNode.Name == "ToInt" || funcNameNode.Name == "ToDouble" || funcNameNode.Name == "ToString" ||
				funcNameNode.Name == "ToBoolean" || funcNameNode.Name == "ToChar")
			{
				if (call.Arguments.Count != 1)
					throw new Exception($"{funcNameNode.Name} expects 1 argument");

				var value = EvaluateExpression(call.Arguments[0]);

				return funcNameNode.Name switch
				{
					"ToInt" => HypnoBuiltins.ToInt(value),
					"ToDouble" => HypnoBuiltins.ToDouble(value),
					"ToString" => HypnoBuiltins.ToString(value),
					"ToBoolean" => HypnoBuiltins.ToBoolean(value),
					"ToChar" => HypnoBuiltins.ToChar(value),
					_ => throw new Exception($"Unknown conversion function: {funcNameNode.Name}")
				};
			}

			// ===== HYPNOTISCHE SPEZIALFUNKTIONEN =====
			if (funcNameNode.Name == "DeepTrance" || funcNameNode.Name == "HypnoticCountdown" || funcNameNode.Name == "TranceInduction" ||
				funcNameNode.Name == "HypnoticVisualization" || funcNameNode.Name == "ProgressiveRelaxation" ||
				funcNameNode.Name == "HypnoticSuggestion" || funcNameNode.Name == "TranceDeepening")
			{
				var arg1 = call.Arguments.Count > 0 ? EvaluateExpression(call.Arguments[0]) : null;
				var arg2 = call.Arguments.Count > 1 ? EvaluateExpression(call.Arguments[1]) : null;

				switch (funcNameNode.Name)
				{
					case "DeepTrance":
						var duration = arg1 != null ? Convert.ToInt32(arg1) : 5000;
						HypnoBuiltins.DeepTrance(duration);
						break;
					case "HypnoticCountdown":
						var from = arg1 != null ? Convert.ToInt32(arg1) : 10;
						HypnoBuiltins.HypnoticCountdown(from);
						break;
					case "TranceInduction":
						var subjectName = arg1?.ToString() ?? "Subject";
						HypnoBuiltins.TranceInduction(subjectName);
						break;
					case "HypnoticVisualization":
						var scene = arg1?.ToString() ?? "a peaceful garden";
						HypnoBuiltins.HypnoticVisualization(scene);
						break;
					case "ProgressiveRelaxation":
						var steps = arg1 != null ? Convert.ToInt32(arg1) : 5;
						HypnoBuiltins.ProgressiveRelaxation(steps);
						break;
					case "HypnoticSuggestion":
						var suggestion = arg1?.ToString() ?? "You are feeling very relaxed";
						HypnoBuiltins.HypnoticSuggestion(suggestion);
						break;
					case "TranceDeepening":
						var levels = arg1 != null ? Convert.ToInt32(arg1) : 3;
						HypnoBuiltins.TranceDeepening(levels);
						break;
				}
				return null;
			}

			// ===== ZEIT- UND DATUMSFUNKTIONEN =====
			if (funcNameNode.Name == "GetCurrentTime" || funcNameNode.Name == "GetCurrentDate" ||
				funcNameNode.Name == "GetCurrentTimeString" || funcNameNode.Name == "GetCurrentDateTime")
			{
				return funcNameNode.Name switch
				{
					"GetCurrentTime" => HypnoBuiltins.GetCurrentTime(),
					"GetCurrentDate" => HypnoBuiltins.GetCurrentDate(),
					"GetCurrentTimeString" => HypnoBuiltins.GetCurrentTimeString(),
					"GetCurrentDateTime" => HypnoBuiltins.GetCurrentDateTime(),
					_ => throw new Exception($"Unknown time function: {funcNameNode.Name}")
				};
			}

			// ===== SYSTEM-FUNKTIONEN =====
			if (funcNameNode.Name == "ClearScreen" || funcNameNode.Name == "Beep" || funcNameNode.Name == "GetEnvironmentVariable" ||
				funcNameNode.Name == "Exit" || funcNameNode.Name == "DebugPrint" || funcNameNode.Name == "DebugPrintType")
			{
				switch (funcNameNode.Name)
				{
					case "ClearScreen":
						HypnoBuiltins.ClearScreen();
						return null;
					case "Beep":
						var frequency = call.Arguments.Count > 0 ? Convert.ToInt32(EvaluateExpression(call.Arguments[0])) : 800;
						var duration = call.Arguments.Count > 1 ? Convert.ToInt32(EvaluateExpression(call.Arguments[1])) : 200;
						HypnoBuiltins.Beep(frequency, duration);
						return null;
					case "GetEnvironmentVariable":
						var name = call.Arguments.Count > 0 ? EvaluateExpression(call.Arguments[0])?.ToString() ?? "" : "";
						return HypnoBuiltins.GetEnvironmentVariable(name);
					case "Exit":
						var code = call.Arguments.Count > 0 ? Convert.ToInt32(EvaluateExpression(call.Arguments[0])) : 0;
						HypnoBuiltins.Exit(code);
						return null;
					case "DebugPrint":
						var value = call.Arguments.Count > 0 ? EvaluateExpression(call.Arguments[0]) : null;
						HypnoBuiltins.DebugPrint(value);
						return null;
					case "DebugPrintType":
						var typeValue = call.Arguments.Count > 0 ? EvaluateExpression(call.Arguments[0]) : null;
						HypnoBuiltins.DebugPrintType(typeValue);
						return null;
				}
			}

			// Normal function calls -> not implemented in this minimal example
			throw new Exception($"Unknown function {funcNameNode.Name}");
		}

		private object? EvaluateMethodCall(MethodCallExpressionNode methodCall)
		{
			var target = EvaluateExpression(methodCall.Target);

			if (target is SessionInstance session)
			{
				// Methodenaufruf auf Session-Instanz
				var arguments = new List<object?>();
				foreach (var arg in methodCall.Arguments)
				{
					arguments.Add(EvaluateExpression(arg));
				}
				return EvaluateSessionMemberCall(session, methodCall.MethodName, arguments);
			}

			throw new Exception($"Method call on non-session value: {methodCall.MethodName}");
		}

		private object? EvaluateSessionInstantiation(SessionInstantiationNode sessionInst)
		{
			// Session-Instanz erstellen
			var sessionSymbol = _globals.Resolve(sessionInst.SessionName);
			if (sessionSymbol?.Value is SessionDeclNode sessionDecl)
			{
				var arguments = new List<object?>();
				foreach (var arg in sessionInst.Arguments)
				{
					arguments.Add(EvaluateExpression(arg));
				}
				return InstantiateSession(sessionDecl, arguments);
			}

			throw new Exception($"Session '{sessionInst.SessionName}' not found");
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
			var array = EvaluateExpression(arrayAccess.Array);
			var index = EvaluateExpression(arrayAccess.Index);

			if (array is List<object?> list && index is int intIndex)
			{
				if (intIndex >= 0 && intIndex < list.Count)
					return list[intIndex];
				throw new Exception($"Array index {intIndex} out of bounds (array length: {list.Count})");
			}

			throw new Exception("Array access requires a list and integer index");
		}

		private object? EvaluateArrayLiteral(ArrayLiteralExpressionNode arrayLit)
		{
			var elements = new List<object?>();
			foreach (var element in arrayLit.Elements)
			{
				elements.Add(EvaluateExpression(element));
			}
			return elements;
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
