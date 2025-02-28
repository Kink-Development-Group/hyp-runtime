using HypnoScript.LexerParser.AST;
using HypnoScript.Runtime;
using HypnoScript.Core.Symbols;

namespace HypnoScript.Compiler.Interpreter
{
	public class HypnoInterpreter
	{
		private readonly SymbolTable _globals = new();

		public void ExecuteProgram(ProgramNode program)
		{
			foreach (var stmt in program.Statements)
			{
				ExecuteStatement(stmt);
			}
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
				case ObserveStatementNode obs:
					var value = EvaluateExpression(obs.Expression);
					HypnoBuiltins.Observe(value);
					break;
				case ReturnStatementNode ret:
					// In einem vollwertigen System -> return from function
					// Hier ignorieren wir's (kein function context)
					break;
				case ExpressionStatementNode exprStmt:
					EvaluateExpression(exprStmt.Expression);
					break;
			}
		}

		private void ExecuteVarDecl(VarDeclNode decl)
		{
			object? val = null;
			if (decl.FromExternal)
			{
				// "Eingabe" -> hier Hardcode oder Konsole
				// z.B.:
				Console.Write($"Input for {decl.Identifier}: ");
				var input = Console.ReadLine();
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
				case CallExpressionNode call:
					return EvaluateCall(call);
			}
			return null;
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

			// For simplicity: assume number or bool
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
					return Convert.ToDouble(leftVal) > Convert.ToDouble(rightVal);
				case "<":
					return Convert.ToDouble(leftVal) < Convert.ToDouble(rightVal);
				case "==":
					// "youAreFeelingVerySleepy" gemappt
					return Equals(leftVal, rightVal);
				case "!=":
					return !Equals(leftVal, rightVal);
				default:
					throw new Exception($"Unrecognized operator {bin.Operator}");
			}
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
	}
}
