using HypnoScript.LexerParser.AST;
using HypnoScript.Runtime;
using HypnoScript.Core.Symbols;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Runtime.Builtins;
using System.IO;
using System.Collections.Generic;

namespace HypnoScript.Compiler.Interpreter
{
	public class BreakException : Exception { }
	public class ContinueException : Exception { }

	public partial class HypnoInterpreter
	{
		private readonly SymbolTable _globals = new();
		private readonly List<string> _assertionFailures = new();

		private class SinkToLabelException : Exception
		{
			public string LabelName { get; }
			public SinkToLabelException(string labelName) { LabelName = labelName; }
		}

		private class ReturnFromFunctionException : Exception
		{
			public object? Value { get; }
			public ReturnFromFunctionException(object? value) { Value = value; }
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
					if (ret.Expression != null)
						throw new ReturnFromFunctionException(EvaluateExpression(ret.Expression));
					else
						throw new ReturnFromFunctionException(null);
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
				case AssertStatementNode assertStmt:
					var cond = EvaluateExpression(assertStmt.Condition);
					if (!IsTruthy(cond))
						_assertionFailures.Add(assertStmt.Message ?? "Assertion failed");
					break;
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
							return FileBuiltins.FileExists(filePath1);
						break;
					case "ReadFile":
						if (args.Length >= 1 && args[0] is string filePath2)
							return FileBuiltins.ReadFile(filePath2);
						break;
					case "WriteFile":
						if (args.Length >= 2 && args[0] is string filePath3 && args[1] is string fileContent3)
							FileBuiltins.WriteFile(filePath3, fileContent3);
						return null;
					case "AppendFile":
						if (args.Length >= 2 && args[0] is string filePath4 && args[1] is string fileContent4)
							FileBuiltins.AppendFile(filePath4, fileContent4);
						return null;
					case "ReadLines":
						if (args.Length >= 1 && args[0] is string filePath5)
							return FileBuiltins.ReadLines(filePath5);
						break;
					case "WriteLines":
						if (args.Length >= 2 && args[0] is string filePath6 && args[1] is string[] fileLines6)
							FileBuiltins.WriteLines(filePath6, fileLines6);
						return null;
					case "GetFileSize":
						if (args.Length >= 1 && args[0] is string filePath7)
							return FileBuiltins.GetFileSize(filePath7);
						break;
					case "GetFileExtension":
						if (args.Length >= 1 && args[0] is string filePath8)
							return FileBuiltins.GetFileExtension(filePath8);
						break;
					case "GetFileName":
						if (args.Length >= 1 && args[0] is string filePath9)
							return FileBuiltins.GetFileName(filePath9);
						break;
					case "GetDirectoryName":
						if (args.Length >= 1 && args[0] is string filePath10)
							return FileBuiltins.GetDirectoryName(filePath10);
						break;

					// Verzeichnis-Operationen
					case "DirectoryExists":
						if (args.Length >= 1 && args[0] is string dirPath1)
							return FileBuiltins.DirectoryExists(dirPath1);
						break;
					case "CreateDirectory":
						if (args.Length >= 1 && args[0] is string dirPath2)
							FileBuiltins.CreateDirectory(dirPath2);
						return null;
					case "GetFiles":
						if (args.Length >= 1 && args[0] is string dirPath3)
						{
							if (args.Length >= 2 && args[1] is string filePattern3)
								return FileBuiltins.GetFiles(dirPath3, filePattern3);
							else
								return FileBuiltins.GetFiles(dirPath3);
						}
						break;
					case "GetDirectories":
						if (args.Length >= 1 && args[0] is string dirPath4)
							return FileBuiltins.GetDirectories(dirPath4);
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
					case "GetMachineName":
						return SystemBuiltins.GetMachineName();
					case "GetUserName":
						return SystemBuiltins.GetUserName();
					case "GetOSVersion":
						return SystemBuiltins.GetOSVersion();
					case "GetProcessorCount":
						return SystemBuiltins.GetProcessorCount();
					case "GetWorkingSet":
						return SystemBuiltins.GetWorkingSet();
					case "PlaySound":
						if (args.Length >= 2 && args[0] is int sndFreq && args[1] is int sndDur)
							SystemBuiltins.PlaySound(sndFreq, sndDur);
						else
							SystemBuiltins.PlaySound();
						return null;
					case "Vibrate":
						if (args.Length >= 1 && args[0] is int vibDur)
							SystemBuiltins.Vibrate(vibDur);
						else
							SystemBuiltins.Vibrate();
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

					// Array-Funktionen
					case "ArrayLength":
						if (args.Length >= 1 && args[0] is object[] arrLen)
							return ArrayBuiltins.ArrayLength(arrLen);
						break;
					case "ArrayGet":
						if (args.Length >= 2 && args[0] is object[] arrGet && args[1] is int indexGet)
							return ArrayBuiltins.ArrayGet(arrGet, indexGet);
						break;
					case "ArraySet":
						if (args.Length >= 3 && args[0] is object[] arrSet && args[1] is int indexSet)
						{
							ArrayBuiltins.ArraySet(arrSet, indexSet, args[2] ?? new object());
							return null;
						}
						break;
					case "ArraySlice":
						if (args.Length >= 3 && args[0] is object[] arrSlice && args[1] is int startSlice && args[2] is int length)
							return ArrayBuiltins.ArraySlice(arrSlice, startSlice, length);
						break;
					case "ArrayConcat":
						if (args.Length >= 2 && args[0] is object[] arr1 && args[1] is object[] arr2)
							return ArrayBuiltins.ArrayConcat(arr1, arr2);
						break;
					case "ArrayIndexOf":
						if (args.Length >= 2 && args[0] is object[] arrIdx)
							return ArrayBuiltins.ArrayIndexOf(arrIdx, args[1] ?? new object());
						break;
					case "ArrayContains":
						if (args.Length >= 2 && args[0] is object[] arrCont)
							return ArrayBuiltins.ArrayContains(arrCont, args[1] ?? new object());
						break;
					case "ArrayMap":
						if (args.Length >= 1 && args[0] is object[] arrMap)
							return ArrayBuiltins.ArrayMap(arrMap, item => item); // Einfache Implementierung
						break;
					case "ArrayReduce":
						if (args.Length >= 2 && args[0] is object[] arrRed)
							return ArrayBuiltins.ArrayReduce(arrRed, (acc, item) => item, args[1] ?? new object());
						break;
					case "ArrayFlatten":
						if (args.Length >= 1 && args[0] is object[] arrFlat)
							return ArrayBuiltins.ArrayFlatten(arrFlat);
						break;

					// Mathematische Funktionen
					case "Abs":
						if (args.Length >= 1 && args[0] is double absVal)
							return MathBuiltins.Abs(absVal);
						break;
					case "Sin":
						if (args.Length >= 1 && args[0] is double sinVal)
							return MathBuiltins.Sin(sinVal);
						break;
					case "Cos":
						if (args.Length >= 1 && args[0] is double cosVal)
							return MathBuiltins.Cos(cosVal);
						break;
					case "Tan":
						if (args.Length >= 1 && args[0] is double tanVal)
							return MathBuiltins.Tan(tanVal);
						break;
					case "Sqrt":
						if (args.Length >= 1 && args[0] is double sqrtVal)
							return MathBuiltins.Sqrt(sqrtVal);
						break;
					case "Pow":
						if (args.Length >= 2 && args[0] is double powX && args[1] is double powY)
							return MathBuiltins.Pow(powX, powY);
						break;
					case "Floor":
						if (args.Length >= 1 && args[0] is double floorVal)
							return MathBuiltins.Floor(floorVal);
						break;
					case "Ceiling":
						if (args.Length >= 1 && args[0] is double ceilVal)
							return MathBuiltins.Ceiling(ceilVal);
						break;
					case "Round":
						if (args.Length >= 1 && args[0] is double roundVal)
							return MathBuiltins.Round(roundVal);
						break;
					case "Log":
						if (args.Length >= 1 && args[0] is double logVal)
							return MathBuiltins.Log(logVal);
						break;
					case "Log10":
						if (args.Length >= 1 && args[0] is double log10Val)
							return MathBuiltins.Log10(log10Val);
						break;
					case "Exp":
						if (args.Length >= 1 && args[0] is double expVal)
							return MathBuiltins.Exp(expVal);
						break;
					case "Max":
						if (args.Length >= 2 && args[0] is double maxX && args[1] is double maxY)
							return MathBuiltins.Max(maxX, maxY);
						break;
					case "Min":
						if (args.Length >= 2 && args[0] is double minX && args[1] is double minY)
							return MathBuiltins.Min(minX, minY);
						break;
					case "Random":
						return MathBuiltins.Random();
					case "RandomInt":
						if (args.Length >= 2 && args[0] is int randMin && args[1] is int randMax)
							return MathBuiltins.RandomInt(randMin, randMax);
						break;

					// String-Funktionen
					case "Length":
						if (args.Length >= 1 && args[0] is string lenStr)
							return StringBuiltins.Length(lenStr);
						break;
					case "Substring":
						if (args.Length >= 3 && args[0] is string subStr && args[1] is int subStart && args[2] is int subLen)
							return StringBuiltins.Substring(subStr, subStart, subLen);
						break;
					case "ToUpper":
						if (args.Length >= 1 && args[0] is string upperStr)
							return StringBuiltins.ToUpper(upperStr);
						break;
					case "ToLower":
						if (args.Length >= 1 && args[0] is string lowerStr)
							return StringBuiltins.ToLower(lowerStr);
						break;
					case "Contains":
						if (args.Length >= 2 && args[0] is string contStr && args[1] is string contSub)
							return StringBuiltins.Contains(contStr, contSub);
						break;
					case "Replace":
						if (args.Length >= 3 && args[0] is string repStrReplace && args[1] is string repOld && args[2] is string repNew)
							return StringBuiltins.Replace(repStrReplace, repOld, repNew);
						break;
					case "Trim":
						if (args.Length >= 1 && args[0] is string trimStr)
							return StringBuiltins.Trim(trimStr);
						break;
					case "IndexOf":
						if (args.Length >= 2 && args[0] is string idxStr && args[1] is string idxSub)
							return StringBuiltins.IndexOf(idxStr, idxSub);
						break;
					case "Split":
						if (args.Length >= 2 && args[0] is string splitStr && args[1] is string splitSep)
							return StringBuiltins.Split(splitStr, splitSep);
						break;
					case "Join":
						if (args.Length >= 2 && args[0] is string[] joinArr && args[1] is string joinSep)
							return StringBuiltins.Join(joinArr, joinSep);
						break;

					// Konvertierungsfunktionen
					case "ToInt":
						if (args.Length >= 1)
							return HypnoBuiltins.ToInt(args[0]);
						break;
					case "ToDouble":
						if (args.Length >= 1)
							return HypnoBuiltins.ToDouble(args[0]);
						break;
					case "ToString":
						if (args.Length >= 1)
							return HypnoBuiltins.ToString(args[0]);
						break;
					case "ToBoolean":
						if (args.Length >= 1)
							return HypnoBuiltins.ToBoolean(args[0]);
						break;

					// Zeit- und Datumsfunktionen
					case "GetCurrentTime":
						return HypnoBuiltins.GetCurrentTime();
					case "GetCurrentDate":
						return HypnoBuiltins.GetCurrentDate();
					case "GetCurrentTimeString":
						return HypnoBuiltins.GetCurrentTimeString();
					case "GetCurrentDateTime":
						return HypnoBuiltins.GetCurrentDateTime();

					// System-Funktionen
					case "ClearScreen":
						SystemBuiltins.ClearScreen();
						return null;
					case "Beep":
						if (args.Length >= 2 && args[0] is int beepFreq && args[1] is int beepDur)
							SystemBuiltins.Beep(beepFreq, beepDur);
						else
							SystemBuiltins.Beep();
						return null;
					case "GetEnvironmentVariable":
						if (args.Length >= 1 && args[0] is string envVar)
							return SystemBuiltins.GetEnvironmentVariable(envVar);
						break;

					// Utility-Funktionen
					case "IsValidEmail":
						if (args.Length >= 1 && args[0] is string email)
							return NetworkBuiltins.IsValidEmail(email);
						break;
					case "IsValidUrl":
						if (args.Length >= 1 && args[0] is string url)
							return NetworkBuiltins.IsValidUrl(url);
						break;
					case "IsValidJson":
						if (args.Length >= 1 && args[0] is string json)
							return HypnoBuiltins.IsValidJson(json);
						break;
					case "FormatNumber":
						if (args.Length >= 2 && args[0] is double num && args[1] is int dec)
							return HypnoBuiltins.FormatNumber(num, dec);
						else if (args.Length >= 1 && args[0] is double num2)
							return HypnoBuiltins.FormatNumber(num2);
						break;
					case "FormatCurrency":
						if (args.Length >= 2 && args[0] is double curr && args[1] is string currency)
							return HypnoBuiltins.FormatCurrency(curr, currency);
						else if (args.Length >= 1 && args[0] is double curr2)
							return HypnoBuiltins.FormatCurrency(curr2);
						break;
					case "FormatPercentage":
						if (args.Length >= 1 && args[0] is double perc)
							return HypnoBuiltins.FormatPercentage(perc);
						break;

					// HTTP-Funktionen
					case "HttpGet":
						if (args.Length >= 1 && args[0] is string httpUrl)
							return NetworkBuiltins.HttpGet(httpUrl);
						break;
					case "HttpPost":
						if (args.Length >= 2 && args[0] is string postUrl && args[1] is string postData)
							return NetworkBuiltins.HttpPost(postUrl, postData);
						break;

					// Statistik-Funktionen
					case "CalculateMean":
						if (args.Length >= 1 && args[0] is object[] meanArr)
							return HypnoBuiltins.CalculateMean(meanArr);
						break;
					case "CalculateStandardDeviation":
						if (args.Length >= 1 && args[0] is object[] stdArr)
							return HypnoBuiltins.CalculateStandardDeviation(stdArr);
						break;
					case "LinearRegression":
						if (args.Length >= 2 && args[0] is object[] lrX && args[1] is object[] lrY)
							return HypnoBuiltins.LinearRegression(lrX, lrY);
						break;

					// Performance-Funktionen
					case "GetPerformanceMetrics":
						return HypnoBuiltins.GetPerformanceMetrics();

					// Hypnotische Spezialfunktionen
					case "DeepTrance":
						if (args.Length >= 1 && args[0] is int deepDur)
							HypnoBuiltins.DeepTrance(deepDur);
						else
							HypnoBuiltins.DeepTrance();
						return null;
					case "HypnoticCountdown":
						if (args.Length >= 1 && args[0] is int countFrom)
							HypnoBuiltins.HypnoticCountdown(countFrom);
						else
							HypnoBuiltins.HypnoticCountdown();
						return null;
					case "TranceInduction":
						if (args.Length >= 1 && args[0] is string subject)
							HypnoBuiltins.TranceInduction(subject);
						else
							HypnoBuiltins.TranceInduction();
						return null;
					case "HypnoticVisualization":
						if (args.Length >= 1 && args[0] is string scene)
							HypnoBuiltins.HypnoticVisualization(scene);
						else
							HypnoBuiltins.HypnoticVisualization();
						return null;
					case "ProgressiveRelaxation":
						if (args.Length >= 1 && args[0] is int steps)
							HypnoBuiltins.ProgressiveRelaxation(steps);
						else
							HypnoBuiltins.ProgressiveRelaxation();
						return null;
					case "HypnoticSuggestion":
						if (args.Length >= 1 && args[0] is string suggestion)
							HypnoBuiltins.HypnoticSuggestion(suggestion);
						return null;
					case "TranceDeepening":
						if (args.Length >= 1 && args[0] is int levels)
							HypnoBuiltins.TranceDeepening(levels);
						else
							HypnoBuiltins.TranceDeepening();
						return null;
					case "HypnoticPatternMatching":
						if (args.Length >= 1 && args[0] is string pattern)
							HypnoBuiltins.HypnoticPatternMatching(pattern);
						return null;
					case "HypnoticTimeDilation":
						if (args.Length >= 1 && args[0] is double factor)
							HypnoBuiltins.HypnoticTimeDilation(factor);
						else
							HypnoBuiltins.HypnoticTimeDilation();
						return null;
					case "HypnoticMemoryEnhancement":
						HypnoBuiltins.HypnoticMemoryEnhancement();
						return null;
					case "HypnoticCreativityBoost":
						HypnoBuiltins.HypnoticCreativityBoost();
						return null;

					// Weitere Utility-Funktionen
					case "Clamp":
						if (args.Length >= 3 && args[0] is double val && args[1] is double min && args[2] is double max)
							return HypnoBuiltins.Clamp(val, min, max);
						break;
					case "Sign":
						if (args.Length >= 1 && args[0] is double signVal)
							return HypnoBuiltins.Sign(signVal);
						break;
					case "IsEven":
						if (args.Length >= 1 && args[0] is int evenVal)
							return HypnoBuiltins.IsEven(evenVal);
						break;
					case "IsOdd":
						if (args.Length >= 1 && args[0] is int oddVal)
							return HypnoBuiltins.IsOdd(oddVal);
						break;
					case "ShuffleArray":
						if (args.Length >= 1 && args[0] is object[] arrShuf)
							return HypnoBuiltins.ShuffleArray(arrShuf);
						break;
					case "SumArray":
						if (args.Length >= 1 && args[0] is object[] arrSum)
							return HypnoBuiltins.SumArray(arrSum);
						break;
					case "AverageArray":
						if (args.Length >= 1 && args[0] is object[] arrAvg)
							return HypnoBuiltins.AverageArray(arrAvg);
						break;
					case "Range":
						if (args.Length >= 2 && args[0] is int startRange && args[1] is int count)
							return HypnoBuiltins.Range(startRange, count);
						break;
					case "Repeat":
						if (args.Length >= 2 && args[1] is int repCount)
							return HypnoBuiltins.Repeat(args[0] ?? "", repCount);
						break;
					case "Swap":
						if (args.Length >= 3 && args[0] is object[] arrSwap && args[1] is int i && args[2] is int j)
						{
							HypnoBuiltins.Swap(arrSwap, i, j);
							return null;
						}
						break;
					case "ChunkArray":
						if (args.Length >= 2 && args[0] is object[] arrChunk && args[1] is int chunkSize)
							return HypnoBuiltins.ChunkArray(arrChunk, chunkSize);
						break;
					case "ArraySum":
						if (args.Length >= 1 && args[0] is object[] arrSum2)
							return HypnoBuiltins.ArraySum(arrSum2);
						break;
					case "ArrayMin":
						if (args.Length >= 1 && args[0] is object[] arrMin)
							return HypnoBuiltins.ArrayMin(arrMin);
						break;
					case "ArrayMax":
						if (args.Length >= 1 && args[0] is object[] arrMax)
							return HypnoBuiltins.ArrayMax(arrMax);
						break;
					case "ArrayCount":
						if (args.Length >= 2 && args[0] is object[] arrCount)
							return HypnoBuiltins.ArrayCount(arrCount, args[1]);
						break;
					case "ArrayRemove":
						if (args.Length >= 2 && args[0] is object[] arrRem)
							return HypnoBuiltins.ArrayRemove(arrRem, args[1]);
						break;
					case "ArrayDistinct":
						if (args.Length >= 1 && args[0] is object[] arrDist)
							return HypnoBuiltins.ArrayDistinct(arrDist);
						break;
					case "IsNullOrEmpty":
						if (args.Length >= 1)
							return HypnoBuiltins.IsNullOrEmpty(args[0]?.ToString());
						break;
					case "RepeatString":
						if (args.Length >= 2 && args[0] is string repStrRepeat && args[1] is int repN)
							return HypnoBuiltins.RepeatString(repStrRepeat, repN);
						break;
					case "ReverseWords":
						if (args.Length >= 1 && args[0] is string revWords)
							return HypnoBuiltins.ReverseWords(revWords);
						break;
					case "Truncate":
						if (args.Length >= 2 && args[0] is string truncStr && args[1] is int truncLen)
							return HypnoBuiltins.Truncate(truncStr, truncLen);
						break;
					case "RemoveDigits":
						if (args.Length >= 1 && args[0] is string remDig)
							return HypnoBuiltins.RemoveDigits(remDig);
						break;
					case "IsPrime":
						if (args.Length >= 1 && args[0] is int nPrime)
							return HypnoBuiltins.IsPrime(nPrime);
						break;
					case "FactorialBig":
						if (args.Length >= 1 && args[0] is int nFact)
							return HypnoBuiltins.FactorialBig(nFact);
						break;
					case "ToHex":
						if (args.Length >= 1 && args[0] is long nHex)
							return HypnoBuiltins.ToHex(nHex);
						break;
					case "ToBinary":
						if (args.Length >= 1 && args[0] is long nBin)
							return HypnoBuiltins.ToBinary(nBin);
						break;
					case "ParseInt":
						if (args.Length >= 1 && args[0] is string strInt)
							return HypnoBuiltins.ParseInt(strInt);
						break;
					case "GetEnvVars":
						return HypnoBuiltins.GetEnvVars();
					case "GetTempPath":
						return HypnoBuiltins.GetTempPath();
					case "GetTickCount":
						return HypnoBuiltins.GetTickCount();
					case "Sleep":
						if (args.Length >= 1 && args[0] is int ms)
						{
							HypnoBuiltins.Sleep(ms);
							return null;
						}
						break;
					case "AddDays":
						if (args.Length >= 2 && args[0] is string date1 && args[1] is int days1)
							return HypnoBuiltins.AddDays(date1, days1);
						break;
					case "AddMonths":
						if (args.Length >= 2 && args[0] is string date2 && args[1] is int months)
							return HypnoBuiltins.AddMonths(date2, months);
						break;
					case "AddYears":
						if (args.Length >= 2 && args[0] is string date3 && args[1] is int years)
							return HypnoBuiltins.AddYears(date3, years);
						break;
					case "ParseDate":
						if (args.Length >= 1 && args[0] is string dateStr)
							return HypnoBuiltins.ParseDate(dateStr);
						break;
					case "IsArray":
						if (args.Length >= 1)
							return HypnoBuiltins.IsArray(args[0]);
						break;
					case "IsNumber":
						if (args.Length >= 1)
							return HypnoBuiltins.IsNumber(args[0]);
						break;
					case "IsString":
						if (args.Length >= 1)
							return HypnoBuiltins.IsString(args[0]);
						break;
					case "IsBoolean":
						if (args.Length >= 1)
							return HypnoBuiltins.IsBoolean(args[0]);
						break;

					// Dictionary-Utilities
					case "CreateDictionary":
						return HypnoBuiltins.CreateDictionary();
					case "DictionaryKeys":
						if (args.Length >= 1 && args[0] is Dictionary<string, object> dictKeys)
							return HypnoBuiltins.DictionaryKeys(dictKeys);
						break;
					case "DictionaryValues":
						if (args.Length >= 1 && args[0] is Dictionary<string, object> dictValues)
							return HypnoBuiltins.DictionaryValues(dictValues);
						break;
					case "DictionaryContainsKey":
						if (args.Length >= 2 && args[0] is Dictionary<string, object> dictCont && args[1] is string key)
							return HypnoBuiltins.DictionaryContainsKey(dictCont, key);
						break;
					case "DictionaryGet":
						if (args.Length >= 2 && args[0] is Dictionary<string, object> dictGet && args[1] is string keyGet)
						{
							var defaultValue = args.Length >= 3 ? args[2] : null;
							return HypnoBuiltins.DictionaryGet(dictGet, keyGet, defaultValue);
						}
						break;
					case "DictionarySet":
						if (args.Length >= 3 && args[0] is Dictionary<string, object> dictSet && args[1] is string keySet)
						{
							HypnoBuiltins.DictionarySet(dictSet, keySet, args[2] ?? "");
							return null;
						}
						break;
					case "DictionaryRemove":
						if (args.Length >= 2 && args[0] is Dictionary<string, object> dictRem && args[1] is string keyRem)
							return HypnoBuiltins.DictionaryRemove(dictRem, keyRem);
						break;
					case "DictionaryCount":
						if (args.Length >= 1 && args[0] is Dictionary<string, object> dictCount)
							return HypnoBuiltins.DictionaryCount(dictCount);
						break;

					// Erweiterte String-Utilities
					case "StartsWith":
						if (args.Length >= 2 && args[0] is string strStart && args[1] is string prefix)
							return StringBuiltins.StartsWith(strStart, prefix);
						break;
					case "EndsWith":
						if (args.Length >= 2 && args[0] is string strEnd && args[1] is string suffix)
							return StringBuiltins.EndsWith(strEnd, suffix);
						break;
					case "PadLeft":
						if (args.Length >= 2 && args[0] is string strPadL && args[1] is int widthL)
						{
							var charL = args.Length >= 3 && args[2] is char cL ? cL : ' ';
							return StringBuiltins.PadLeft(strPadL, widthL, charL);
						}
						break;
					case "PadRight":
						if (args.Length >= 2 && args[0] is string strPadR && args[1] is int widthR)
						{
							var charR = args.Length >= 3 && args[2] is char cR ? cR : ' ';
							return StringBuiltins.PadRight(strPadR, widthR, charR);
						}
						break;
					case "Insert":
						if (args.Length >= 3 && args[0] is string strIns && args[1] is int indexIns && args[2] is string valueIns)
							return StringBuiltins.Insert(strIns, indexIns, valueIns);
						break;
					case "Remove":
						if (args.Length >= 3 && args[0] is string strRem && args[1] is int startRem && args[2] is int countRem)
							return StringBuiltins.Remove(strRem, startRem, countRem);
						break;
					case "Compare":
						if (args.Length >= 2 && args[0] is string str1 && args[1] is string str2)
							return StringBuiltins.Compare(str1, str2);
						break;
					case "EqualsIgnoreCase":
						if (args.Length >= 2 && args[0] is string strEq1 && args[1] is string strEq2)
							return StringBuiltins.EqualsIgnoreCase(strEq1, strEq2);
						break;
					case "IsPalindrome":
						if (args.Length >= 1 && args[0] is string strPal)
							return StringBuiltins.IsPalindrome(strPal);
						break;
					case "CountWords":
						if (args.Length >= 1 && args[0] is string strWords)
							return StringBuiltins.CountWords(strWords);
						break;
					case "ExtractNumbers":
						if (args.Length >= 1 && args[0] is string strNum)
							return StringBuiltins.ExtractNumbers(strNum);
						break;
					case "ExtractLetters":
						if (args.Length >= 1 && args[0] is string strLet)
							return StringBuiltins.ExtractLetters(strLet);
						break;
				}
			}

			// Fallback für andere Funktionen
			var callee = EvaluateExpression(call.Callee);
			if (callee is not FunctionDeclNode func)
			{
				throw new Exception($"Cannot call non-function: {callee}");
			}

			// Funktionsaufruf-Logik mit Rückgabewert
			var localScope = new SymbolTable(_globals);
			for (int i = 0; i < func.Parameters.Count; i++)
			{
				var param = func.Parameters[i];
				var argValue = i < call.Arguments.Count ? EvaluateExpression(call.Arguments[i]) : null;
				localScope.Define(new Symbol(param.Name, param.TypeName, argValue));
			}

			try
			{
				foreach (var stmt in func.Body)
				{
					if (stmt is ReturnStatementNode ret)
					{
						if (ret.Expression != null)
							return EvaluateExpression(ret.Expression);
						else
							return null;
					}
					ExecuteStatement(stmt);
				}
			}
			catch (ReturnFromFunctionException ex)
			{
				return ex.Value;
			}
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

		public IReadOnlyList<string> GetAssertionFailures() => _assertionFailures.AsReadOnly();
	}
}
