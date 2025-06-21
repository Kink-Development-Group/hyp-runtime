using System.Text;
using HypnoScript.LexerParser.AST;

namespace HypnoScript.Compiler.CodeGen
{
    // WebAssembly-Codegenerator im WAT-Format
    public class WasmCodeGenerator
    {
        private StringBuilder _wat = null!;
        private int _localCounter = 0;
        private int _labelCounter = 0;
        private Dictionary<string, int> _variableMap = new();
        private Dictionary<string, int> _functionMap = new();
        private List<string> _imports = new();
        private List<string> _functions = new();

        public string Generate(ProgramNode program)
        {
            _wat = new StringBuilder();
            _localCounter = 0;
            _labelCounter = 0;
            _variableMap.Clear();
            _functionMap.Clear();
            _imports.Clear();
            _functions.Clear();

            // Standard-Imports
            AddImport("env", "console_log", "(func $console_log (param i32))");
            AddImport("env", "console_log_str", "(func $console_log_str (param i32 i32))");
            AddImport("env", "drift", "(func $drift (param i32))");
            AddImport("env", "memory", "(memory (export \"memory\") 1)");

            _wat.AppendLine("(module");

            // Imports
            foreach (var import in _imports)
            {
                _wat.AppendLine($"  (import {import})");
            }

            // Globale Variablen für String-Speicher
            _wat.AppendLine("  (global $string_offset (mut i32) (i32.const 0))");
            _wat.AppendLine("  (global $heap_offset (mut i32) (i32.const 1024))");

            // Hilfsfunktionen
            EmitHelperFunctions();

            // Hauptfunktion
            _wat.AppendLine("  (func $HypnoMain (export \"main\")");
            _wat.AppendLine("    (local $temp i32)");
            _wat.AppendLine("    (local $temp_f64 f64)");
            _wat.AppendLine("    (local $temp_str i32)");

            // Entrance-Block zuerst ausführen
            foreach (var stmt in program.Statements)
            {
                if (stmt is EntranceBlockNode entrance)
                {
                    EmitStatements(entrance.Statements);
                }
            }

            // Dann alle anderen Statements
            foreach (var stmt in program.Statements)
            {
                if (stmt is not EntranceBlockNode)
                {
                    EmitStatement(stmt);
                }
            }

            _wat.AppendLine("  )");

            // Weitere Funktionen
            foreach (var function in _functions)
            {
                _wat.AppendLine(function);
            }

            _wat.AppendLine(")");
            return _wat.ToString();
        }

        private void AddImport(string module, string name, string signature)
        {
            _imports.Add($"\"{module}\" \"{name}\" {signature}");
        }

        private void EmitHelperFunctions()
        {
            // String-Hilfsfunktionen
            _wat.AppendLine("  ;; String-Hilfsfunktionen");
            _wat.AppendLine("  (func $store_string (param $str i32) (param $len i32) (result i32)");
            _wat.AppendLine("    (local $offset i32)");
            _wat.AppendLine("    global.get $string_offset");
            _wat.AppendLine("    local.tee $offset");
            _wat.AppendLine("    local.get $len");
            _wat.AppendLine("    i32.add");
            _wat.AppendLine("    global.set $string_offset");
            _wat.AppendLine("    local.get $offset");
            _wat.AppendLine("  )");

            _wat.AppendLine("  (func $print_string (param $str i32) (param $len i32)");
            _wat.AppendLine("    local.get $str");
            _wat.AppendLine("    local.get $len");
            _wat.AppendLine("    call $console_log_str");
            _wat.AppendLine("  )");

            _wat.AppendLine("  (func $print_number (param $num i32)");
            _wat.AppendLine("    local.get $num");
            _wat.AppendLine("    call $console_log");
            _wat.AppendLine("  )");
        }

        private void EmitStatements(List<IStatement> statements)
        {
            foreach (var stmt in statements)
            {
                EmitStatement(stmt);
            }
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
                    _wat.AppendLine("    drop  ;; Verwerfe Ergebnis");
                    break;
                case ObserveStatementNode observe:
                    EmitObserve(observe);
                    break;
                case IfStatementNode ifStmt:
                    EmitIf(ifStmt);
                    break;
                case WhileStatementNode whileStmt:
                    EmitWhile(whileStmt);
                    break;
                case LoopStatementNode loopStmt:
                    EmitLoop(loopStmt);
                    break;
                case SnapStatementNode:
                    EmitSnap();
                    break;
                case SinkStatementNode:
                    EmitSink();
                    break;
                case FunctionDeclNode funcDecl:
                    EmitFunctionDeclaration(funcDecl);
                    break;
                case SessionDeclNode sessionDecl:
                    EmitSessionDeclaration(sessionDecl);
                    break;
                case TranceifyDeclNode tranceifyDecl:
                    EmitTranceifyDeclaration(tranceifyDecl);
                    break;
                case DriftStatementNode drift:
                    EmitDrift(drift);
                    break;
                case BlockStatementNode block:
                    EmitStatements(block.Statements);
                    break;
                default:
                    _wat.AppendLine($"    ;; Unsupported statement: {stmt.GetType().Name}");
                    break;
            }
        }

        private void EmitVarDecl(VarDeclNode decl)
        {
            var varIndex = _localCounter++;
            _variableMap[decl.Identifier] = varIndex;

            if (decl.Initializer != null)
            {
                EmitExpression(decl.Initializer);
            }
            else
            {
                _wat.AppendLine("    i32.const 0");
            }

            _wat.AppendLine($"    local.set ${varIndex}  ;; {decl.Identifier}");
        }

        private void EmitObserve(ObserveStatementNode observe)
        {
            EmitExpression(observe.Expression);
            _wat.AppendLine("    call $print_number");
        }

        private void EmitIf(IfStatementNode ifStmt)
        {
            var elseLabel = $"else_{_labelCounter++}";
            var endLabel = $"endif_{_labelCounter++}";

            EmitExpression(ifStmt.Condition);
            _wat.AppendLine("    i32.eqz");
            _wat.AppendLine($"    br_if ${elseLabel}");

            // Then-Block
            EmitStatements(ifStmt.ThenBranch);
            _wat.AppendLine($"    br ${endLabel}");

            // Else-Block
            if (ifStmt.ElseBranch != null)
            {
                _wat.AppendLine($"  ${elseLabel}:");
                EmitStatements(ifStmt.ElseBranch);
            }
            else
            {
                _wat.AppendLine($"  ${elseLabel}:");
            }

            _wat.AppendLine($"  ${endLabel}:");
        }

        private void EmitWhile(WhileStatementNode whileStmt)
        {
            var loopLabel = $"while_loop_{_labelCounter++}";
            var endLabel = $"while_end_{_labelCounter++}";

            _wat.AppendLine($"  ${loopLabel}:");
            EmitExpression(whileStmt.Condition);
            _wat.AppendLine("    i32.eqz");
            _wat.AppendLine($"    br_if ${endLabel}");

            EmitStatements(whileStmt.Body);
            _wat.AppendLine($"    br ${loopLabel}");

            _wat.AppendLine($"  ${endLabel}:");
        }

        private void EmitLoop(LoopStatementNode loopStmt)
        {
            var loopLabel = $"for_loop_{_labelCounter++}";
            var endLabel = $"for_end_{_labelCounter++}";

            // Initializer
            if (loopStmt.Initializer != null)
            {
                EmitStatement(loopStmt.Initializer);
            }

            _wat.AppendLine($"  ${loopLabel}:");
            EmitExpression(loopStmt.Condition);
            _wat.AppendLine("    i32.eqz");
            _wat.AppendLine($"    br_if ${endLabel}");

            EmitStatements(loopStmt.Body);

            // Iteration
            if (loopStmt.Iteration != null)
            {
                EmitStatement(loopStmt.Iteration);
            }

            _wat.AppendLine($"    br ${loopLabel}");
            _wat.AppendLine($"  ${endLabel}:");
        }

        private void EmitSnap()
        {
            _wat.AppendLine("    ;; snap (break) - würde Schleife verlassen");
        }

        private void EmitSink()
        {
            _wat.AppendLine("    ;; sink (continue) - würde zum Schleifenanfang springen");
        }

        private void EmitDrift(DriftStatementNode drift)
        {
            EmitExpression(drift.Milliseconds);
            _wat.AppendLine("    call $drift");
        }

        private void EmitExpression(IExpression expr)
        {
            switch (expr)
            {
                case LiteralExpressionNode lit:
                    EmitLiteral(lit);
                    break;
                case BinaryExpressionNode bin:
                    EmitBinary(bin);
                    break;
                case UnaryExpressionNode unary:
                    EmitUnary(unary);
                    break;
                case IdentifierExpressionNode id:
                    EmitIdentifier(id);
                    break;
                case CallExpressionNode call:
                    EmitCall(call);
                    break;
                case AssignmentExpressionNode assign:
                    EmitAssignment(assign);
                    break;
                case ArrayLiteralExpressionNode arrayLit:
                    EmitArrayLiteral(arrayLit);
                    break;
                case ArrayAccessExpressionNode arrayAccess:
                    EmitArrayAccess(arrayAccess);
                    break;
                case ParenthesizedExpressionNode paren:
                    EmitExpression(paren.Expression);
                    break;
                default:
                    _wat.AppendLine($"    ;; Unsupported expression: {expr.GetType().Name}");
                    _wat.AppendLine("    i32.const 0");
                    break;
            }
        }

        private void EmitLiteral(LiteralExpressionNode lit)
        {
            switch (lit.LiteralType)
            {
                case "number":
                    if (double.TryParse(lit.Value, out double num))
                    {
                        if (num == (int)num)
                        {
                            _wat.AppendLine($"    i32.const {(int)num}");
                        }
                        else
                        {
                            _wat.AppendLine($"    f64.const {num}");
                        }
                    }
                    else
                    {
                        _wat.AppendLine("    i32.const 0");
                    }
                    break;
                case "boolean":
                    int boolVal = (lit.Value == "true") ? 1 : 0;
                    _wat.AppendLine($"    i32.const {boolVal}");
                    break;
                case "string":
                    EmitStringLiteral(lit.Value);
                    break;
                default:
                    _wat.AppendLine("    i32.const 0");
                    break;
            }
        }

        private void EmitStringLiteral(string value)
        {
            // Vereinfachte String-Behandlung
            _wat.AppendLine($"    ;; String: \"{value}\"");
            _wat.AppendLine("    i32.const 0  ;; Platzhalter für String-Pointer");
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
                case "%":
                    _wat.AppendLine("    i32.rem_s");
                    break;
                case "==":
                case "youAreFeelingVerySleepy":
                    _wat.AppendLine("    i32.eq");
                    break;
                case "!=":
                case "notSoDeep":
                    _wat.AppendLine("    i32.ne");
                    break;
                case ">":
                case "lookAtTheWatch":
                    _wat.AppendLine("    i32.gt_s");
                    break;
                case "<":
                case "fallUnderMySpell":
                    _wat.AppendLine("    i32.lt_s");
                    break;
                case ">=":
                case "deeplyGreater":
                    _wat.AppendLine("    i32.ge_s");
                    break;
                case "<=":
                case "deeplyLess":
                    _wat.AppendLine("    i32.le_s");
                    break;
                case "&&":
                    _wat.AppendLine("    i32.and");
                    break;
                case "||":
                    _wat.AppendLine("    i32.or");
                    break;
                default:
                    _wat.AppendLine($"    ;; Unsupported operator: {bin.Operator}");
                    _wat.AppendLine("    i32.const 0");
                    break;
            }
        }

        private void EmitUnary(UnaryExpressionNode unary)
        {
            EmitExpression(unary.Operand);

            switch (unary.Operator)
            {
                case "!":
                    _wat.AppendLine("    i32.eqz");
                    break;
                case "-":
                    _wat.AppendLine("    i32.const 0");
                    _wat.AppendLine("    i32.sub");
                    break;
                case "+":
                    // Nichts zu tun, Wert bleibt unverändert
                    break;
                default:
                    _wat.AppendLine($"    ;; Unsupported unary operator: {unary.Operator}");
                    break;
            }
        }

        private void EmitIdentifier(IdentifierExpressionNode id)
        {
            if (_variableMap.TryGetValue(id.Name, out int varIndex))
            {
                _wat.AppendLine($"    local.get ${varIndex}  ;; {id.Name}");
            }
            else
            {
                _wat.AppendLine($"    ;; Variable {id.Name} nicht gefunden");
                _wat.AppendLine("    i32.const 0");
            }
        }

        private void EmitCall(CallExpressionNode call)
        {
            // Argumente auswerten
            foreach (var arg in call.Arguments)
            {
                EmitExpression(arg);
            }

            if (call.Callee is IdentifierExpressionNode funcId)
            {
                // Builtin-Funktionen
                switch (funcId.Name)
                {
                    case "drift":
                        _wat.AppendLine("    call $drift");
                        break;
                    case "Sin":
                    case "Cos":
                    case "Tan":
                    case "Sqrt":
                    case "Pow":
                    case "Abs":
                    case "Floor":
                    case "Ceiling":
                    case "Round":
                        _wat.AppendLine($"    ;; Mathematische Funktion: {funcId.Name}");
                        _wat.AppendLine("    f64.const 0.0  ;; Platzhalter");
                        break;
                    default:
                        _wat.AppendLine($"    ;; Funktionsaufruf: {funcId.Name}");
                        _wat.AppendLine("    i32.const 0  ;; Platzhalter");
                        break;
                }
            }
            else
            {
                _wat.AppendLine("    ;; Komplexer Funktionsaufruf");
                _wat.AppendLine("    i32.const 0  ;; Platzhalter");
            }
        }

        private void EmitAssignment(AssignmentExpressionNode assign)
        {
            EmitExpression(assign.Value);

            if (_variableMap.TryGetValue(assign.Identifier, out int varIndex))
            {
                _wat.AppendLine($"    local.set ${varIndex}  ;; {assign.Identifier}");
            }
            else
            {
                _wat.AppendLine($"    ;; Variable {assign.Identifier} nicht gefunden");
            }
        }

        private void EmitArrayLiteral(ArrayLiteralExpressionNode arrayLit)
        {
            _wat.AppendLine("    ;; Array-Literal");
            foreach (var element in arrayLit.Elements)
            {
                EmitExpression(element);
            }
            _wat.AppendLine("    i32.const 0  ;; Platzhalter für Array");
        }

        private void EmitArrayAccess(ArrayAccessExpressionNode arrayAccess)
        {
            EmitExpression(arrayAccess.Array);
            EmitExpression(arrayAccess.Index);
            _wat.AppendLine("    ;; Array-Zugriff");
            _wat.AppendLine("    i32.const 0  ;; Platzhalter");
        }

        private void EmitFunctionDeclaration(FunctionDeclNode funcDecl)
        {
            _functionMap[funcDecl.Name] = _functions.Count;

            var funcCode = new StringBuilder();
            funcCode.AppendLine($"  (func ${funcDecl.Name}");

            // Parameter
            for (int i = 0; i < funcDecl.Parameters.Count; i++)
            {
                funcCode.AppendLine($"    (param ${i} i32)");
            }

            // Rückgabetyp
            if (funcDecl.ReturnType != null)
            {
                funcCode.AppendLine($"    (result i32)");
            }

            // Lokale Variablen
            funcCode.AppendLine("    (local $temp i32)");

            // Body
            foreach (var stmt in funcDecl.Body)
            {
                // Vereinfachte Statement-Emission
                funcCode.AppendLine("    ;; Statement");
            }

            funcCode.AppendLine("  )");
            _functions.Add(funcCode.ToString());
        }

        private void EmitSessionDeclaration(SessionDeclNode sessionDecl)
        {
            _wat.AppendLine($"    ;; Session-Deklaration: {sessionDecl.Name}");
            foreach (var member in sessionDecl.Members)
            {
                _wat.AppendLine("    ;; Session-Member");
            }
        }

        private void EmitTranceifyDeclaration(TranceifyDeclNode tranceifyDecl)
        {
            _wat.AppendLine($"    ;; Tranceify-Deklaration: {tranceifyDecl.Name}");
            foreach (var member in tranceifyDecl.Members)
            {
                _wat.AppendLine("    ;; Tranceify-Member");
            }
        }
    }
}
