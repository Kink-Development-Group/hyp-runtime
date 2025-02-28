using System.Reflection;
using System.Reflection.Emit;
using HypnoScript.LexerParser.AST;
using HypnoScript.Runtime;

namespace HypnoScript.Compiler.CodeGen
{
	public class ILCodeGenerator
	{
		private ILGenerator _il;
		private readonly Dictionary<string, LocalBuilder> _locals = new();

		public Action Generate(ProgramNode program)
		{
			var method = new DynamicMethod("HypnoMain", typeof(void), Type.EmptyTypes);
			_il = method.GetILGenerator();

			foreach (var stmt in program.Statements)
			{
				EmitStatement(stmt);
			}

			_il.Emit(OpCodes.Ret);

			var action = (Action)method.CreateDelegate(typeof(Action));
			return action;
		}

		private void EmitStatement(IStatement stmt)
		{
			switch (stmt)
			{
				case VarDeclNode varDecl:
					EmitVarDecl(varDecl);
					break;
				case ObserveStatementNode obs:
					EmitExpression(obs.Expression);
					// Call HypnoBuiltins.Observe for enterprise-grade logging/output handling
					_il.Emit(OpCodes.Call, typeof(HypnoBuiltins).GetMethod(nameof(HypnoBuiltins.Observe))!);
					break;
				case ExpressionStatementNode exprStmt:
					EmitExpression(exprStmt.Expression);
					// Discard the result to maintain stack integrity
					_il.Emit(OpCodes.Pop);
					break;
				case IfStatementNode ifStmt:
					EmitIfStatement(ifStmt);
					break;
				case WhileStatementNode whileStmt:
					EmitWhileStatement(whileStmt);
					break;
				case BlockStatementNode block:
					// Process each statement in the block
					foreach (var s in block.Statements)
					{
						EmitStatement(s);
					}
					break;
				case FunctionDeclNode funcDecl:
					EmitFunction(funcDecl);
					break;
				default:
					// Centralized error handling for unsupported statement types
					throw new NotSupportedException($"Unsupported statement type: {stmt.GetType().Name}");
			}
		}

		// Enterprise-level extension: handling If statements
		private void EmitIfStatement(IfStatementNode ifStmt)
		{
			// Evaluate the condition
			EmitExpression(ifStmt.Condition);
			// Unbox to boolean for condition evaluation; assuming a helper exists
			_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToBool), BindingFlags.Static | BindingFlags.NonPublic)!);

			// Emit branch instructions with labels for true and end segments
			Label elseLabel = _il.DefineLabel();
			Label endLabel = _il.DefineLabel();

			_il.Emit(OpCodes.Brfalse, elseLabel);
				// Handle then branch
			foreach (var stmt in ifStmt.ThenBranch)
			{
				EmitStatement(stmt);
			}
			_il.Emit(OpCodes.Br, endLabel);

			// Else branch, if provided
			_il.MarkLabel(elseLabel);
			if (ifStmt.ElseBranch != null)
			{
				foreach (var stmt in ifStmt.ElseBranch)
				{
					EmitStatement(stmt);
				}
			}
			_il.MarkLabel(endLabel);
		}

		// Enterprise-level extension: handling While loops
		private void EmitWhileStatement(WhileStatementNode whileStmt)
		{
			Label loopStart = _il.DefineLabel();
			Label loopEnd = _il.DefineLabel();

			_il.MarkLabel(loopStart);
			EmitExpression(whileStmt.Condition);
			_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToBool), BindingFlags.Static | BindingFlags.NonPublic)!);
			_il.Emit(OpCodes.Brfalse, loopEnd);

			foreach (var stmt in whileStmt.Body)
			{
				EmitStatement(stmt);
			}
			_il.Emit(OpCodes.Br, loopStart);
			_il.MarkLabel(loopEnd);
		}

		// Helper method to unbox an object to a boolean value
		private static bool UnboxToBool(object obj)
		{
			if (obj is bool b)
			{
				return b;
			}
			// Fallback: attempt to convert common types if necessary
			if (obj is int i)
			{
				return i != 0;
			}
			return false;
		}

		private void EmitVarDecl(VarDeclNode decl)
		{
			// Deklariere Local
			var local = _il.DeclareLocal(typeof(object));
			_locals[decl.Identifier] = local;

			if (decl.FromExternal)
			{
				// Konsoleingabe
				_il.Emit(OpCodes.Ldstr, $"Input for {decl.Identifier}: ");
				_il.Emit(OpCodes.Call, typeof(Console).GetMethod("Write", new[] { typeof(string) })!);
				_il.Emit(OpCodes.Call, typeof(Console).GetMethod(nameof(Console.ReadLine), Type.EmptyTypes)!);
			}
			else if (decl.Initializer != null)
			{
				EmitExpression(decl.Initializer);
			}
			else
			{
				// null
				_il.Emit(OpCodes.Ldnull);
			}

			_il.Emit(OpCodes.Stloc, local);
		}

		private void EmitExpression(IExpression expr)
		{
			switch (expr)
			{
				case LiteralExpressionNode lit:
					EmitLiteral(lit);
					break;
				case IdentifierExpressionNode id:
					if (_locals.TryGetValue(id.Name, out var local))
					{
						_il.Emit(OpCodes.Ldloc, local);
					}
					else
					{
						// fallback: push null
						_il.Emit(OpCodes.Ldnull);
					}
					break;
				case BinaryExpressionNode bin:
					EmitBinary(bin);
					break;
				case CallExpressionNode call:
					EmitCall(call);
					break;
			}
		}

		private void EmitLiteral(LiteralExpressionNode lit)
		{
			// Alles als object -> Boxen
			if (lit.LiteralType == "number")
			{
				if (lit.Value.Contains("."))
				{
					if (double.TryParse(lit.Value, out double d))
					{
						_il.Emit(OpCodes.Ldc_R8, d);
						_il.Emit(OpCodes.Box, typeof(double));
					}
				}
				else
				{
					if (int.TryParse(lit.Value, out int i))
					{
						_il.Emit(OpCodes.Ldc_I4, i);
						_il.Emit(OpCodes.Box, typeof(int));
					}
				}
			}
			else if (lit.LiteralType == "boolean")
			{
				bool b = (lit.Value == "true");
				_il.Emit(b ? OpCodes.Ldc_I4_1 : OpCodes.Ldc_I4_0);
				_il.Emit(OpCodes.Box, typeof(bool));
			}
			else
			{
				// string
				_il.Emit(OpCodes.Ldstr, lit.Value);
			}
		}

		private void EmitBinary(BinaryExpressionNode bin)
		{
			EmitExpression(bin.Left);
			EmitExpression(bin.Right);

			// wir haben 2 x object auf dem Stack -> wir konvertieren (double) für +, -, etc
			switch (bin.Operator)
			{
				case "+":
				case "-":
				case "*":
				case "/":
					// Unbox als double -> Rechenoperation -> Box
					_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToDouble), BindingFlags.Static | BindingFlags.NonPublic)!);
					_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToDouble), BindingFlags.Static | BindingFlags.NonPublic)!);

					switch (bin.Operator)
					{
						case "+": _il.Emit(OpCodes.Add); break;
						case "-": _il.Emit(OpCodes.Sub); break;
						case "*": _il.Emit(OpCodes.Mul); break;
						case "/": _il.Emit(OpCodes.Div); break;
					}

					_il.Emit(OpCodes.Box, typeof(double));
					break;

				case "==":
					// call Equals
					_il.Emit(OpCodes.Call, typeof(object).GetMethod(nameof(object.Equals), new[] { typeof(object), typeof(object) })!);
					break;
			}
		}

		private void EmitCall(CallExpressionNode call)
		{
			if (call.Callee is IdentifierExpressionNode id)
			{
				switch (id.Name)
				{
					case "drift":
						// drift(x)
						if (call.Arguments.Count != 1)
							throw new Exception("drift benötigt genau 1 Argument");

						EmitExpression(call.Arguments[0]);
						// -> Unbox to int
						_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToInt), BindingFlags.Static | BindingFlags.NonPublic)!);

						// Call HypnoBuiltins.Drift(int)
						_il.Emit(OpCodes.Call, typeof(HypnoBuiltins).GetMethod(nameof(HypnoBuiltins.Drift))!);
						break;

					default:
						// Enterprise-Level: Dynamische Funktionsaufrufe unterstützen
						// Suche nach einer statischen Methode in HypnoBuiltins mit dem Namen der Funktion
						var candidates = typeof(HypnoBuiltins).GetMethods()
							.Where(m => m.Name == id.Name && m.IsStatic)
							.ToList();

						if (!candidates.Any())
							throw new NotSupportedException($"Unbekannte Funktion: {id.Name}");

						// Wähle die Methode, die zur Anzahl der Parameter passt
						var targetMethod = candidates.FirstOrDefault(m => m.GetParameters().Length == call.Arguments.Count);
						if (targetMethod == null)
							throw new Exception($"Funktion {id.Name} mit {call.Arguments.Count} Argument(en) wurde nicht gefunden.");

						// Argumente evaluieren und auf den erwarteten Typ casten, falls nötig
						var parameters = targetMethod.GetParameters();
						for (int i = 0; i < call.Arguments.Count; i++)
						{
							EmitExpression(call.Arguments[i]);

							// Enterprise-Level: Falls der Parameter nicht vom Typ object ist, erfolgt eine Unboxing-Konvertierung
							if (parameters[i].ParameterType != typeof(object))
							{
								var paramType = parameters[i].ParameterType;
								// Es erfolgt hier eine einfache Fallunterscheidung für int und double
								if (paramType == typeof(int))
								{
									_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToInt), BindingFlags.Static | BindingFlags.NonPublic)!);
								}
								else if (paramType == typeof(double))
								{
									_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToDouble), BindingFlags.Static | BindingFlags.NonPublic)!);
								}
								// Weitere Typkonvertierungen können hier hinzugefügt werden
							}
						}

						_il.Emit(OpCodes.Call, targetMethod);
						break;
				}
			}
			else
			{
				throw new NotSupportedException("Nur Funktionsaufrufe über Bezeichner werden unterstützt.");
			}
		}

		private static double UnboxToDouble(object obj)
		{
			if (obj is int i) return i;
			if (obj is double d) return d;
			return 0.0;
		}

		private static int UnboxToInt(object obj)
		{
			if (obj is int i) return i;
			if (obj is double d) return (int)d;
			return 0;
		}

		private void EmitFunction(FunctionDeclNode funcDecl)
		{
			// Erweiterung: Dynamische Methoden für Funktionen erstellen
			// Parameter-Handling und Lokale Variablen initialisieren
			// ...implementierung...
		}
	}
}
