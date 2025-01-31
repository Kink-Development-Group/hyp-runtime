using System.Reflection;
using System.Reflection.Emit;
using HypnoScript.LexerParser.AST;
using HypnoScript.Runtime;

namespace HypnoScript.Compiler.CodeGen
{
	public class ILCodeGenerator
	{
		private ILGenerator _il;
		private Dictionary<string, LocalBuilder> _locals = new();

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
					// call HypnoBuiltins.Observe
					_il.Emit(OpCodes.Call, typeof(HypnoBuiltins).GetMethod(nameof(HypnoBuiltins.Observe))!);
					break;
				case ExpressionStatementNode exprStmt:
					EmitExpression(exprStmt.Expression);
					// Expressionsergebnis ignorieren
					_il.Emit(OpCodes.Pop);
					break;
					// ...
			}
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
				if (id.Name == "drift")
				{
					// drift(x)
					if (call.Arguments.Count != 1)
						throw new Exception("drift needs 1 argument");

					EmitExpression(call.Arguments[0]);
					// -> unbox int
					_il.Emit(OpCodes.Call, typeof(ILCodeGenerator).GetMethod(nameof(UnboxToInt), BindingFlags.Static | BindingFlags.NonPublic)!);

					// call HypnoBuiltins.Drift(int)
					_il.Emit(OpCodes.Call, typeof(HypnoBuiltins).GetMethod(nameof(HypnoBuiltins.Drift))!);
				}
				else
				{
					// Unbekannte function
					// ...
				}
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
	}
}
