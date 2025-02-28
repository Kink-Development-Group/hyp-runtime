using System.Text;
using HypnoScript.LexerParser.AST;

namespace HypnoScript.Compiler.CodeGen
{
    // WebAssembly-Codegenerator im WAT-Format
    public class WasmCodeGenerator
    {
        private StringBuilder _wat;

        public string Generate(ProgramNode program)
        {
            _wat = new StringBuilder();

            _wat.AppendLine("(module");
            // Importe (z.B. Konsolenausgabe und drift als Verzögerung)
            _wat.AppendLine("  (import \"env\" \"console_log\" (func $console_log (param i32)))");
            _wat.AppendLine("  (import \"env\" \"drift\" (func $drift (param i32)))");
            _wat.AppendLine("  (memory (export \"memory\") 1)");
            // Hauptfunktion; in einer echten Implementierung sind lokale Variablen vorher zu deklarieren.
            _wat.AppendLine("  (func $HypnoMain (export \"main\")");
            _wat.AppendLine("    ;; ... Lokale Variablen initialisieren ...");

            foreach (var stmt in program.Statements)
            {
                EmitStatement(stmt);
            }

            _wat.AppendLine("  )");
            _wat.AppendLine(")");
            return _wat.ToString();
        }

        private void EmitStatement(IStatement stmt)
        {
            switch (stmt)
            {
                case VarDeclNode varDecl:
                    EmitVarDecl(varDecl);
                    break;
                case ExpressionStatementNode exprStmt:
                    EmitExpression(exprStmt.Expression);
                    // Ergebnis verwerfen
                    break;
                case ObserveStatementNode observe:
                    EmitExpression(observe.Expression);
                    _wat.AppendLine("    call $console_log");
                    break;
                case IfStatementNode ifStmt:
                    EmitIf(ifStmt);
                    break;
                case WhileStatementNode whileStmt:
                    EmitWhile(whileStmt);
                    break;
                // Weitere Statement-Typen können hier ergänzt werden.
                default:
                    _wat.AppendLine("    ;; Unsupported statement");
                    break;
            }
        }

        private void EmitVarDecl(VarDeclNode decl)
        {
            // Vereinfachte Annahme: Locals werden nicht dynamisch verwaltet.
            _wat.AppendLine($"    ;; Deklariere Variable {decl.Identifier}");
            if (decl.Initializer != null)
            {
                EmitExpression(decl.Initializer);
            }
            else
            {
                _wat.AppendLine("    i32.const 0");
            }
            _wat.AppendLine($"    ;; local.set für {decl.Identifier}");
        }

        private void EmitIf(IfStatementNode ifStmt)
        {
            EmitExpression(ifStmt.Condition);
            _wat.AppendLine("    if");
            _wat.AppendLine("      ;; then-Block");
            foreach (var stmt in ifStmt.ThenBranch)
            {
                EmitStatement(stmt);
            }
            if (ifStmt.ElseBranch != null)
            {
                _wat.AppendLine("    else");
                foreach (var stmt in ifStmt.ElseBranch)
                {
                    EmitStatement(stmt);
                }
            }
            _wat.AppendLine("    end");
        }

        private void EmitWhile(WhileStatementNode whileStmt)
        {
            _wat.AppendLine("    block");
            _wat.AppendLine("      loop");
            EmitExpression(whileStmt.Condition);
            _wat.AppendLine("      i32.eqz");
            _wat.AppendLine("      br_if 1  ;; Schleife verlassen, wenn Bedingung false");
            foreach (var stmt in whileStmt.Body)
            {
                EmitStatement(stmt);
            }
            _wat.AppendLine("      br 0  ;; Wiederholung der Schleife");
            _wat.AppendLine("      end");
            _wat.AppendLine("    end");
        }

        private void EmitExpression(IExpression expr)
        {
            switch (expr)
            {
                case LiteralExpressionNode lit:
                    if (lit.LiteralType == "string")
                    {
                        _wat.AppendLine($"    ;; String Handling is not implemented - placeholder for \"{lit.Value}\"");
                    }
                    else
                    {
                        EmitLiteral(lit);
                    }
                    break;
                case BinaryExpressionNode bin:
                    EmitBinary(bin);
                    break;
                case IdentifierExpressionNode id:
                    _wat.AppendLine($"    ;; local.get für {id.Name}");
                    break;
                case CallExpressionNode call:
                    EmitCall(call);
                    break;
                default:
                    _wat.AppendLine("    ;; Unsupported expression");
                    break;
            }
        }

        private void EmitLiteral(LiteralExpressionNode lit)
        {
            if (lit.LiteralType == "number")
            {
                _wat.AppendLine($"    i32.const {lit.Value}");
            }
            else if (lit.LiteralType == "boolean")
            {
                int boolVal = (lit.Value == "true") ? 1 : 0;
                _wat.AppendLine($"    i32.const {boolVal}");
            }
            else
            {
                // Bei Strings: Platzhalter, da komplexe Speicherverwaltung erforderlich wäre.
                _wat.AppendLine($"    ;; push string \"{lit.Value}\"");
            }
        }

        private void EmitBinary(BinaryExpressionNode bin)
        {
            EmitExpression(bin.Left);
            EmitExpression(bin.Right);
            switch (bin.Operator)
            {
                case "+":
                    _wat.AppendLine("    i32.add");
                    break;
                case "-":
                    _wat.AppendLine("    i32.sub");
                    break;
                case "*":
                    _wat.AppendLine("    i32.mul");
                    break;
                case "/":
                    _wat.AppendLine("    i32.div_s");
                    break;
                case "==":
                    _wat.AppendLine("    i32.eq");
                    break;
                case "!=":
                    _wat.AppendLine("    i32.ne");
                    break;
                case ">":
                    _wat.AppendLine("    i32.gt_s");
                    break;
                case "<":
                    _wat.AppendLine("    i32.lt_s");
                    break;
                default:
                    _wat.AppendLine($"    ;; Unsupported operator {bin.Operator}");
                    break;
            }
        }

        private void EmitCall(CallExpressionNode call)
        {
            if (call.Callee is IdentifierExpressionNode id)
            {
                if (id.Name == "drift")
                {
                    foreach (var arg in call.Arguments)
                    {
                        EmitExpression(arg);
                    }
                    _wat.AppendLine("    call $drift");
                }
                else
                {
                    // Annahme: Weitere Funktionen werden entweder importiert oder innerhalb des Moduls definiert.
                    foreach (var arg in call.Arguments)
                    {
                        EmitExpression(arg);
                    }
                    _wat.AppendLine($"    call ${id.Name}");
                }
            }
        }
    }
}
