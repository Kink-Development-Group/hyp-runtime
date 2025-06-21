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
			// Builtin-Funktionen direkt evaluieren
			if (call.Callee is IdentifierExpressionNode id)
			{
				var functionName = id.Name;
				var args = call.Arguments.Select(EvaluateExpression).ToArray();

				// Erweiterte Builtin-Funktionen
				switch (functionName)
				{
					// Erweiterte hypnotische Funktionen
					case "HypnoticBreathing":
						if (args.Length >= 1 && args[0] is int cycles)
							HypnoBuiltins.HypnoticBreathing(cycles);
						else
							HypnoBuiltins.HypnoticBreathing();
						return null;
					case "HypnoticAnchoring":
						if (args.Length >= 1 && args[0] is string anchorStr)
							HypnoBuiltins.HypnoticAnchoring(anchorStr);
						else
							HypnoBuiltins.HypnoticAnchoring();
						return null;
					case "HypnoticRegression":
						if (args.Length >= 1 && args[0] is int regAge)
							HypnoBuiltins.HypnoticRegression(regAge);
						else
							HypnoBuiltins.HypnoticRegression();
						return null;
					case "HypnoticFutureProgression":
						if (args.Length >= 1 && args[0] is int futYears)
							HypnoBuiltins.HypnoticFutureProgression(futYears);
						else
							HypnoBuiltins.HypnoticFutureProgression();
						return null;

					// Datei-Operationen
					case "FileExists":
						if (args.Length >= 1 && args[0] is string filePath1)
							return HypnoBuiltins.FileExists(filePath1);
						break;
					case "ReadFile":
						if (args.Length >= 1 && args[0] is string filePath2)
							return HypnoBuiltins.ReadFile(filePath2);
						break;
					case "WriteFile":
						if (args.Length >= 2 && args[0] is string filePath3 && args[1] is string fileContent3)
							HypnoBuiltins.WriteFile(filePath3, fileContent3);
						return null;
					case "AppendFile":
						if (args.Length >= 2 && args[0] is string filePath4 && args[1] is string fileContent4)
							HypnoBuiltins.AppendFile(filePath4, fileContent4);
						return null;
					case "ReadLines":
						if (args.Length >= 1 && args[0] is string filePath5)
							return HypnoBuiltins.ReadLines(filePath5);
						break;
					case "WriteLines":
						if (args.Length >= 2 && args[0] is string filePath6 && args[1] is string[] fileLines6)
							HypnoBuiltins.WriteLines(filePath6, fileLines6);
						return null;
					case "GetFileSize":
						if (args.Length >= 1 && args[0] is string filePath7)
							return HypnoBuiltins.GetFileSize(filePath7);
						break;
					case "GetFileExtension":
						if (args.Length >= 1 && args[0] is string filePath8)
							return HypnoBuiltins.GetFileExtension(filePath8);
						break;
					case "GetFileName":
						if (args.Length >= 1 && args[0] is string filePath9)
							return HypnoBuiltins.GetFileName(filePath9);
						break;
					case "GetDirectoryName":
						if (args.Length >= 1 && args[0] is string filePath10)
							return HypnoBuiltins.GetDirectoryName(filePath10);
						break;

					// Verzeichnis-Operationen
					case "DirectoryExists":
						if (args.Length >= 1 && args[0] is string dirPath1)
							return HypnoBuiltins.DirectoryExists(dirPath1);
						break;
					case "CreateDirectory":
						if (args.Length >= 1 && args[0] is string dirPath2)
							HypnoBuiltins.CreateDirectory(dirPath2);
						return null;
					case "GetFiles":
						if (args.Length >= 1 && args[0] is string dirPath3)
						{
							if (args.Length >= 2 && args[1] is string filePattern3)
								return HypnoBuiltins.GetFiles(dirPath3, filePattern3);
							else
								return HypnoBuiltins.GetFiles(dirPath3);
						}
						break;
					case "GetDirectories":
						if (args.Length >= 1 && args[0] is string dirPath4)
							return HypnoBuiltins.GetDirectories(dirPath4);
						break;

					// JSON-Verarbeitung
					case "ToJson":
						if (args.Length >= 1)
							return HypnoBuiltins.ToJson(args[0]);
						break;
					case "FromJson":
						if (args.Length >= 1 && args[0] is string jsonStr)
							return HypnoBuiltins.FromJson(jsonStr);
						break;

					// Erweiterte mathematische Funktionen
					case "Factorial":
						if (args.Length >= 1 && args[0] is int factN)
							return HypnoBuiltins.Factorial(factN);
						break;
					case "GCD":
						if (args.Length >= 2 && args[0] is double gcdA && args[1] is double gcdB)
							return HypnoBuiltins.GCD(gcdA, gcdB);
						break;
					case "LCM":
						if (args.Length >= 2 && args[0] is double lcmA && args[1] is double lcmB)
							return HypnoBuiltins.LCM(lcmA, lcmB);
						break;
					case "DegreesToRadians":
						if (args.Length >= 1 && args[0] is double degVal)
							return HypnoBuiltins.DegreesToRadians(degVal);
						break;
					case "RadiansToDegrees":
						if (args.Length >= 1 && args[0] is double radVal)
							return HypnoBuiltins.RadiansToDegrees(radVal);
						break;
					case "Asin":
						if (args.Length >= 1 && args[0] is double asinX)
							return HypnoBuiltins.Asin(asinX);
						break;
					case "Acos":
						if (args.Length >= 1 && args[0] is double acosX)
							return HypnoBuiltins.Acos(acosX);
						break;
					case "Atan":
						if (args.Length >= 1 && args[0] is double atanX)
							return HypnoBuiltins.Atan(atanX);
						break;
					case "Atan2":
						if (args.Length >= 2 && args[0] is double atan2Y && args[1] is double atan2X)
							return HypnoBuiltins.Atan2(atan2Y, atan2X);
						break;

					// Erweiterte String-Funktionen
					case "Reverse":
						if (args.Length >= 1 && args[0] is string revStr)
							return HypnoBuiltins.Reverse(revStr);
						break;
					case "Capitalize":
						if (args.Length >= 1 && args[0] is string capStr)
							return HypnoBuiltins.Capitalize(capStr);
						break;
					case "TitleCase":
						if (args.Length >= 1 && args[0] is string titleStr)
							return HypnoBuiltins.TitleCase(titleStr);
						break;
					case "CountOccurrences":
						if (args.Length >= 2 && args[0] is string countStr && args[1] is string countSub)
							return HypnoBuiltins.CountOccurrences(countStr, countSub);
						break;
					case "RemoveWhitespace":
						if (args.Length >= 1 && args[0] is string wsStr)
							return HypnoBuiltins.RemoveWhitespace(wsStr);
						break;

					// Erweiterte Array-Funktionen
					case "ArrayReverse":
						if (args.Length >= 1 && args[0] is object[] arrRev)
							return HypnoBuiltins.ArrayReverse(arrRev);
						break;
					case "ArraySort":
						if (args.Length >= 1 && args[0] is object[] arrSort)
							return HypnoBuiltins.ArraySort(arrSort);
						break;
					case "ArrayUnique":
						if (args.Length >= 1 && args[0] is object[] arrUnique)
							return HypnoBuiltins.ArrayUnique(arrUnique);
						break;
					case "ArrayFilter":
						if (args.Length >= 1 && args[0] is object[] arrFilter)
						{
							// Einfache Implementierung - filtert nach nicht-null Werten
							return HypnoBuiltins.ArrayFilter(arrFilter, item => item != null);
						}
						break;

					// Kryptologische Funktionen
					case "HashMD5":
						if (args.Length >= 1 && args[0] is string hashInput1)
							return HypnoBuiltins.HashMD5(hashInput1);
						break;
					case "HashSHA256":
						if (args.Length >= 1 && args[0] is string hashInput2)
							return HypnoBuiltins.HashSHA256(hashInput2);
						break;
					case "Base64Encode":
						if (args.Length >= 1 && args[0] is string base64Input1)
							return HypnoBuiltins.Base64Encode(base64Input1);
						break;
					case "Base64Decode":
						if (args.Length >= 1 && args[0] is string base64Input2)
							return HypnoBuiltins.Base64Decode(base64Input2);
						break;

					// Erweiterte Zeit-Funktionen
					case "FormatDateTime":
						if (args.Length >= 1 && args[0] is string dtFormat)
							return HypnoBuiltins.FormatDateTime(dtFormat);
						else
							return HypnoBuiltins.FormatDateTime();
					case "GetDayOfWeek":
						return HypnoBuiltins.GetDayOfWeek();
					case "GetDayOfYear":
						return HypnoBuiltins.GetDayOfYear();
					case "IsLeapYear":
						if (args.Length >= 1 && args[0] is int leapYear)
							return HypnoBuiltins.IsLeapYear(leapYear);
						break;
					case "GetDaysInMonth":
						if (args.Length >= 2 && args[0] is int daysYear && args[1] is int daysMonth)
							return HypnoBuiltins.GetDaysInMonth(daysYear, daysMonth);
						break;

					// Erweiterte System-Funktionen
					case "GetCurrentDirectory":
						return HypnoBuiltins.GetCurrentDirectory();
					case "GetMachineName":
						return HypnoBuiltins.GetMachineName();
					case "GetUserName":
						return HypnoBuiltins.GetUserName();
					case "GetOSVersion":
						return HypnoBuiltins.GetOSVersion();
					case "GetProcessorCount":
						return HypnoBuiltins.GetProcessorCount();
					case "GetWorkingSet":
						return HypnoBuiltins.GetWorkingSet();
					case "PlaySound":
						if (args.Length >= 2 && args[0] is int sndFreq && args[1] is int sndDur)
							HypnoBuiltins.PlaySound(sndFreq, sndDur);
						else
							HypnoBuiltins.PlaySound();
						return null;
					case "Vibrate":
						if (args.Length >= 1 && args[0] is int vibDur)
							HypnoBuiltins.Vibrate(vibDur);
						else
							HypnoBuiltins.Vibrate();
						return null;

					// Erweiterte Debugging-Funktionen
					case "DebugPrint":
						if (args.Length >= 1)
							HypnoBuiltins.DebugPrint(args[0]);
						return null;
					case "DebugPrintType":
						if (args.Length >= 1)
							HypnoBuiltins.DebugPrintType(args[0]);
						return null;
					case "DebugPrintMemory":
						HypnoBuiltins.DebugPrintMemory();
						return null;
					case "DebugPrintStackTrace":
						HypnoBuiltins.DebugPrintStackTrace();
						return null;
					case "DebugPrintEnvironment":
						HypnoBuiltins.DebugPrintEnvironment();
						return null;
				}
			}

			// Fallback für andere Funktionen
			var callee = EvaluateExpression(call.Callee);
			if (callee is not FunctionDeclNode func)
			{
				throw new Exception($"Cannot call non-function: {callee}");
			}

			// Funktionsaufruf-Logik...
			return null;
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
