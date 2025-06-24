using System;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Types;
using HypnoScript.Compiler.Error;
using System.Collections.Generic;
using HypnoScript.Core.Symbols;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using System.IO;
using System.Linq;

namespace HypnoScript.Compiler.Analysis
{
    public class TypeChecker
    {
        private readonly Dictionary<string, SessionDeclNode> _sessions = new();
        private readonly Dictionary<string, TranceifyDeclNode> _tranceifies = new();
        private readonly SymbolTable _globals = new();
        private HashSet<string> _labelsInScope = new();
        private readonly Dictionary<string, string?> _typeCache = new();
        private readonly List<string> _importedFiles = new();

        // Runtime-Level: Neben der reinen Traversierung werden Typ-Inkonsistenzen protokolliert.
        public void Check(ProgramNode program)
        {
            // Sammle alle Sessions und tranceify-Definitionen
            foreach (var stmt in program.Statements)
            {
                if (stmt is SessionDeclNode session)
                    _sessions[session.Name] = session;
                if (stmt is TranceifyDeclNode trance)
                    _tranceifies[trance.Name] = trance;
            }
            // Check alle Statements
            foreach (var stmt in program.Statements)
            {
                CheckStatement(stmt);
            }
        }

        private void CheckStatement(IStatement stmt)
        {
            switch (stmt)
            {
                case VarDeclNode varDecl:
                    CheckVarDeclaration(varDecl);
                    break;
                case FunctionDeclNode funcDecl:
                    CheckFunctionDeclaration(funcDecl);
                    break;
                case SessionDeclNode session:
                    CheckSessionDeclaration(session);
                    break;
                case SessionMemberNode sessionMember:
                    CheckSessionMember(sessionMember);
                    break;
                case TranceifyDeclNode trance:
                    CheckTranceifyDeclaration(trance);
                    break;
                case ExpressionStatementNode exprStmt:
                    CheckExpression(exprStmt.Expression);
                    break;
                case ObserveStatementNode obs:
                    CheckExpression(obs.Expression);
                    break;
                case DriftStatementNode drift:
                    CheckDriftStatement(drift);
                    break;
                case ReturnStatementNode ret:
                    CheckReturnStatement(ret);
                    break;
                case IfStatementNode ifStmt:
                    CheckIfStatement(ifStmt);
                    break;
                case WhileStatementNode whileStmt:
                    CheckWhileStatement(whileStmt);
                    break;
                case LoopStatementNode loopStmt:
                    CheckLoopStatement(loopStmt);
                    break;
                case SnapStatementNode:
                case SinkStatementNode:
                    // Keine spezielle Typprüfung nötig
                    break;
                case MindLinkNode mindLink:
                    CheckMindLink(mindLink);
                    break;
                case SharedTranceVarDeclNode shared:
                    CheckSharedTranceVarDeclaration(shared);
                    break;
                case LabelNode label:
                    CheckLabelDeclaration(label);
                    break;
                case SinkToNode sinkTo:
                    CheckSinkToStatement(sinkTo);
                    break;
                case EntranceBlockNode entrance:
                    CheckEntranceBlock(entrance);
                    break;
                case AssertStatementNode assertStmt:
                    CheckAssertStatement(assertStmt);
                    break;
                default:
                    ErrorReporter.ReportWarning($"Unsupported statement type: {stmt.GetType().Name}", 0, 0, "TYPE999");
                    break;
            }
        }

        private void CheckVarDeclaration(VarDeclNode varDecl)
        {
            var initType = InferExpressionType(varDecl.Initializer);

            // Strengere Typprüfung
            if (varDecl.TypeName != null)
            {
                if (!IsValidType(varDecl.TypeName))
                {
                    ErrorReporter.Report($"Invalid type '{varDecl.TypeName}' for variable '{varDecl.Identifier}'", 0, 0, "TYPE001");
                    return;
                }

                if (initType != null && varDecl.TypeName != initType && !IsTypeCompatible(varDecl.TypeName, initType))
                {
                    ErrorReporter.Report($"Type mismatch: Variable '{varDecl.Identifier}' declared as '{varDecl.TypeName}' but initializer is '{initType}'", 0, 0, "TYPE002");
                }
            }
            else if (initType == null || initType == "unknown")
            {
                ErrorReporter.Report($"Type of variable '{varDecl.Identifier}' could not be inferred (unknown type)", 0, 0, "TYPE910");
            }

            if (!_globals.Define(new Symbol(varDecl.Identifier, varDecl.TypeName ?? initType)))
            {
                ErrorReporter.Report($"Variable '{varDecl.Identifier}' already defined", 0, 0, "TYPE003");
            }
        }

        private void CheckFunctionDeclaration(FunctionDeclNode funcDecl)
        {
            // Prüfe Parameter-Typen
            foreach (var param in funcDecl.Parameters)
            {
                if (!IsValidType(param.TypeName))
                {
                    ErrorReporter.Report($"Invalid parameter type '{param.TypeName}' in function '{funcDecl.Name}'", 0, 0, "TYPE004");
                }
            }

            // Prüfe Return-Typ
            if (funcDecl.ReturnType != null && !IsValidType(funcDecl.ReturnType))
            {
                ErrorReporter.Report($"Invalid return type '{funcDecl.ReturnType}' for function '{funcDecl.Name}'", 0, 0, "TYPE005");
            }

            // Funktionssymbol anlegen
            _globals.Define(new Symbol(funcDecl.Name, funcDecl.ReturnType ?? "unknown"));

            // Prüfe Funktionskörper
            foreach (var stmt in funcDecl.Body)
            {
                CheckStatement(stmt);
            }
        }

        private void CheckSessionDeclaration(SessionDeclNode session)
        {
            if (_sessions.ContainsKey(session.Name))
            {
                ErrorReporter.Report($"Session '{session.Name}' already defined", 0, 0, "TYPE006");
                return;
            }

            // Felder und Methoden prüfen
            foreach (var member in session.Members)
            {
                CheckSessionMember(member);
            }

            ValidateSession(session);
        }

        private void CheckTranceifyDeclaration(TranceifyDeclNode trance)
        {
            if (_tranceifies.ContainsKey(trance.Name))
            {
                ErrorReporter.Report($"Tranceify '{trance.Name}' already defined", 0, 0, "TYPE007");
                return;
            }

            // Felder prüfen
            foreach (var member in trance.Members)
            {
                CheckStatement(member);
            }

            ValidateTranceify(trance);
        }

        private void CheckDriftStatement(DriftStatementNode drift)
        {
            var driftType = InferExpressionType(drift.Milliseconds);
            if (driftType != "number" && driftType != "int")
            {
                ErrorReporter.Report($"drift() expects number, got '{driftType}'", 0, 0, "TYPE008");
            }
        }

        private void CheckReturnStatement(ReturnStatementNode ret)
        {
            if (ret.Expression != null)
            {
                var returnType = InferExpressionType(ret.Expression);
                // TODO: Prüfe gegen aktuellen Funktions-Return-Typ
            }
            CheckExpression(ret.Expression);
        }

        private void CheckIfStatement(IfStatementNode ifStmt)
        {
            var conditionType = InferExpressionType(ifStmt.Condition);
            if (conditionType != "boolean")
            {
                ErrorReporter.Report($"if condition must be boolean, got '{conditionType}'", 0, 0, "TYPE009");
            }
            foreach (var s in ifStmt.ThenBranch)
                CheckStatement(s);
            if (ifStmt.ElseBranch != null)
                foreach (var s in ifStmt.ElseBranch)
                    CheckStatement(s);
        }

        private void CheckWhileStatement(WhileStatementNode whileStmt)
        {
            var whileConditionType = InferExpressionType(whileStmt.Condition);
            if (whileConditionType != "boolean")
            {
                ErrorReporter.Report($"while condition must be boolean, got '{whileConditionType}'", 0, 0, "TYPE010");
            }
            foreach (var s in whileStmt.Body)
                CheckStatement(s);
        }

        private void CheckLoopStatement(LoopStatementNode loopStmt)
        {
            var loopConditionType = InferExpressionType(loopStmt.Condition);
            if (loopConditionType != "boolean")
            {
                ErrorReporter.Report($"loop condition must be boolean, got '{loopConditionType}'", 0, 0, "TYPE011");
            }
            if (loopStmt.Initializer != null)
                CheckStatement(loopStmt.Initializer);
            if (loopStmt.Iteration != null)
                CheckStatement(loopStmt.Iteration);
            foreach (var s in loopStmt.Body)
                CheckStatement(s);
        }

        private void CheckMindLink(MindLinkNode mindLink)
        {
            // Vollständige Symbolübernahme bei MindLink
            if (_importedFiles.Contains(mindLink.FileName))
            {
                ErrorReporter.ReportWarning($"File '{mindLink.FileName}' already imported", 0, 0, "TYPE012");
                return;
            }

            try
            {
                if (!File.Exists(mindLink.FileName))
                {
                    ErrorReporter.Report($"Import file '{mindLink.FileName}' not found", 0, 0, "TYPE013");
                    return;
                }

                var code = File.ReadAllText(mindLink.FileName);
                var lexer = new HypnoLexer(code);
                var tokens = lexer.Lex();
                var parser = new HypnoParser(tokens);
                var importedProgram = parser.ParseProgram();

                // Übernehme Sessions
                foreach (var stmt in importedProgram.Statements)
                {
                    if (stmt is SessionDeclNode session)
                    {
                        if (!_sessions.ContainsKey(session.Name))
                        {
                            _sessions[session.Name] = session;
                        }
                        else
                        {
                            ErrorReporter.ReportWarning($"Session '{session.Name}' from '{mindLink.FileName}' conflicts with existing definition", 0, 0, "TYPE014");
                        }
                    }
                }

                // Übernehme Tranceifies
                foreach (var stmt in importedProgram.Statements)
                {
                    if (stmt is TranceifyDeclNode trance)
                    {
                        if (!_tranceifies.ContainsKey(trance.Name))
                        {
                            _tranceifies[trance.Name] = trance;
                        }
                        else
                        {
                            ErrorReporter.ReportWarning($"Tranceify '{trance.Name}' from '{mindLink.FileName}' conflicts with existing definition", 0, 0, "TYPE015");
                        }
                    }
                }

                // Übernehme Funktionen
                foreach (var stmt in importedProgram.Statements)
                {
                    if (stmt is FunctionDeclNode func)
                    {
                        if (_globals.Resolve(func.Name) == null)
                        {
                            _globals.Define(new Symbol(func.Name, func.ReturnType ?? "unknown"));
                        }
                        else
                        {
                            ErrorReporter.ReportWarning($"Function '{func.Name}' from '{mindLink.FileName}' conflicts with existing definition", 0, 0, "TYPE016");
                        }
                    }
                }

                // Übernehme globale Variablen
                foreach (var stmt in importedProgram.Statements)
                {
                    if (stmt is VarDeclNode varDecl)
                    {
                        if (_globals.Resolve(varDecl.Identifier) == null)
                        {
                            _globals.Define(new Symbol(varDecl.Identifier, varDecl.TypeName ?? "unknown"));
                        }
                        else
                        {
                            ErrorReporter.ReportWarning($"Variable '{varDecl.Identifier}' from '{mindLink.FileName}' conflicts with existing definition", 0, 0, "TYPE017");
                        }
                    }
                }

                _importedFiles.Add(mindLink.FileName);
            }
            catch (Exception ex)
            {
                ErrorReporter.Report($"Failed to import '{mindLink.FileName}': {ex.Message}", 0, 0, "TYPE018");
            }
        }

        private void CheckSharedTranceVarDeclaration(SharedTranceVarDeclNode shared)
        {
            var sharedType = InferExpressionType(shared.Initializer);
            if (shared.TypeName != null && sharedType != null && shared.TypeName != sharedType && !IsTypeCompatible(shared.TypeName, sharedType))
            {
                ErrorReporter.Report($"Type mismatch: sharedTrance variable '{shared.Identifier}' declared as '{shared.TypeName}' but initializer is '{sharedType}'", 0, 0, "TYPE020");
            }
            if (!_globals.Define(new Symbol(shared.Identifier, shared.TypeName ?? sharedType)))
            {
                ErrorReporter.Report($"sharedTrance variable '{shared.Identifier}' already defined", 0, 0, "TYPE021");
            }
        }

        private void CheckLabelDeclaration(LabelNode label)
        {
            if (_labelsInScope.Contains(label.Name))
            {
                ErrorReporter.Report($"Label '{label.Name}' already defined in scope", 0, 0, "TYPE022");
            }
            _labelsInScope.Add(label.Name);
        }

        private void CheckSinkToStatement(SinkToNode sinkTo)
        {
            if (!_labelsInScope.Contains(sinkTo.LabelName))
            {
                ErrorReporter.Report($"sinkTo label '{sinkTo.LabelName}' not found in scope", 0, 0, "TYPE030");
            }
        }

        private void CheckEntranceBlock(EntranceBlockNode entrance)
        {
            foreach (var s in entrance.Statements)
                CheckStatement(s);
        }

        private void CheckAssertStatement(AssertStatementNode assertStmt)
        {
            var conditionType = InferExpressionType(assertStmt.Condition);
            if (conditionType != "boolean")
            {
                ErrorReporter.Report($"Assert condition must be boolean, got '{conditionType}'", 0, 0, "TYPE031");
            }
        }

        private void CheckSessionMember(SessionMemberNode member)
        {
            // Prüfe die eigentliche Deklaration
            CheckStatement(member.Declaration);
        }

        private void CheckExpression(IExpression? expr)
        {
            if (expr == null) return;
            switch (expr)
            {
                case LiteralExpressionNode lit:
                    CheckLiteralExpression(lit);
                    break;
                case BinaryExpressionNode bin:
                    CheckBinaryExpression(bin);
                    break;
                case UnaryExpressionNode unary:
                    CheckUnaryExpression(unary);
                    break;
                case ParenthesizedExpressionNode paren:
                    CheckExpression(paren.Expression);
                    break;
                case AssignmentExpressionNode assign:
                    CheckAssignmentExpression(assign);
                    break;
                case CallExpressionNode call:
                    CheckCallExpression(call);
                    break;
                case IdentifierExpressionNode id:
                    CheckIdentifierExpression(id);
                    break;
                case ArrayAccessExpressionNode arrayAccess:
                    CheckArrayAccessExpression(arrayAccess);
                    break;
                case ArrayLiteralExpressionNode arrayLit:
                    CheckArrayLiteralExpression(arrayLit);
                    break;
                case FieldAccessExpressionNode fieldAccess:
                    CheckFieldAccessExpression(fieldAccess);
                    break;
                case RecordLiteralExpressionNode recordLit:
                    CheckRecordLiteralExpression(recordLit);
                    break;
                case SessionInstantiationNode sessionInst:
                    CheckSessionInstantiation(sessionInst);
                    break;
                case MethodCallExpressionNode methodCall:
                    CheckMethodCallExpression(methodCall);
                    break;
                default:
                    ErrorReporter.ReportWarning($"Unsupported expression type: {expr.GetType().Name}", 0, 0, "TYPE999");
                    break;
            }
        }

        private void CheckLiteralExpression(LiteralExpressionNode lit)
        {
            switch (lit.LiteralType)
            {
                case "number":
                    if (!double.TryParse(lit.Value, out _))
                    {
                        ErrorReporter.Report($"Invalid numeric literal: {lit.Value}", 0, 0, "TYPE032");
                    }
                    break;
                case "string":
                    // String-Literale sind immer gültig
                    break;
                case "boolean":
                    if (lit.Value != "true" && lit.Value != "false")
                    {
                        ErrorReporter.Report($"Invalid boolean literal: {lit.Value}", 0, 0, "TYPE033");
                    }
                    break;
                default:
                    ErrorReporter.Report($"Unknown literal type: {lit.LiteralType}", 0, 0, "TYPE034");
                    break;
            }
        }

        private void CheckBinaryExpression(BinaryExpressionNode bin)
        {
            CheckExpression(bin.Left);
            CheckExpression(bin.Right);

            var leftType = InferExpressionType(bin.Left);
            var rightType = InferExpressionType(bin.Right);

            // Prüfe Operator-Kompatibilität
            if (!IsOperatorCompatible(bin.Operator, leftType, rightType))
            {
                ErrorReporter.Report($"Operator '{bin.Operator}' not compatible with types '{leftType}' and '{rightType}'", 0, 0, "TYPE035");
            }
        }

        private void CheckUnaryExpression(UnaryExpressionNode unary)
        {
            CheckExpression(unary.Operand);

            var operandType = InferExpressionType(unary.Operand);
            if (!IsUnaryOperatorCompatible(unary.Operator, operandType))
            {
                ErrorReporter.Report($"Unary operator '{unary.Operator}' not compatible with type '{operandType}'", 0, 0, "TYPE036");
            }
        }

        private void CheckAssignmentExpression(AssignmentExpressionNode assign)
        {
            CheckExpression(assign.Value);

            // Prüfe ob Variable existiert
            var symbol = _globals.Resolve(assign.Identifier);
            if (symbol == null)
            {
                ErrorReporter.Report($"Cannot assign to undefined variable '{assign.Identifier}'", 0, 0, "TYPE037");
                return;
            }

            var valueType = InferExpressionType(assign.Value);
            var symbolTypeName = symbol.Type?.ToString() ?? symbol.TypeName;

            if (symbolTypeName != null && valueType != null && symbolTypeName != valueType && !IsTypeCompatible(symbolTypeName, valueType))
            {
                ErrorReporter.Report($"Cannot assign value of type '{valueType}' to variable '{assign.Identifier}' of type '{symbolTypeName}'", 0, 0, "TYPE038");
            }
        }

        private void CheckCallExpression(CallExpressionNode call)
        {
            CheckExpression(call.Callee);

            foreach (var arg in call.Arguments)
            {
                CheckExpression(arg);
            }

            // Prüfe Builtin-Funktionen
            if (call.Callee is IdentifierExpressionNode id)
            {
                var returnType = InferBuiltinReturnType(id.Name);
                if (returnType == null)
                {
                    ErrorReporter.ReportWarning($"Unknown function '{id.Name}'", 0, 0, "TYPE039");
                }
            }
        }

        private void CheckIdentifierExpression(IdentifierExpressionNode id)
        {
            var symbol = _globals.Resolve(id.Name);
            if (symbol == null)
            {
                ErrorReporter.Report($"Undefined variable '{id.Name}'", 0, 0, "TYPE040");
            }
        }

        private void CheckArrayAccessExpression(ArrayAccessExpressionNode arrayAccess)
        {
            CheckExpression(arrayAccess.Array);
            CheckExpression(arrayAccess.Index);

            var arrayType = InferExpressionType(arrayAccess.Array);
            var indexType = InferExpressionType(arrayAccess.Index);

            if (arrayType != "array")
            {
                ErrorReporter.Report($"Cannot access index on non-array type '{arrayType}'", 0, 0, "TYPE041");
            }

            if (indexType != "number" && indexType != "int")
            {
                ErrorReporter.Report($"Array index must be number, got '{indexType}'", 0, 0, "TYPE042");
            }
        }

        private void CheckArrayLiteralExpression(ArrayLiteralExpressionNode arrayLit)
        {
            foreach (var element in arrayLit.Elements)
            {
                CheckExpression(element);
            }
        }

        private void CheckFieldAccessExpression(FieldAccessExpressionNode fieldAccess)
        {
            CheckExpression(fieldAccess.Target);

            var targetType = InferExpressionType(fieldAccess.Target);
            if (targetType != "record" && targetType != "session")
            {
                ErrorReporter.Report($"Cannot access field on non-record/session type '{targetType}'", 0, 0, "TYPE043");
            }
        }

        private void CheckRecordLiteralExpression(RecordLiteralExpressionNode recordLit)
        {
            foreach (var field in recordLit.Fields)
            {
                CheckExpression(field.Value);
            }
        }

        private void CheckSessionInstantiation(SessionInstantiationNode sessionInst)
        {
            if (!_sessions.ContainsKey(sessionInst.SessionName))
            {
                ErrorReporter.Report($"Undefined session '{sessionInst.SessionName}'", 0, 0, "TYPE044");
            }
        }

        private void CheckMethodCallExpression(MethodCallExpressionNode methodCall)
        {
            CheckExpression(methodCall.Target);

            foreach (var arg in methodCall.Arguments)
            {
                CheckExpression(arg);
            }
        }

        // Hilfsmethoden für Typprüfung
        private bool IsValidType(string? type)
        {
            if (string.IsNullOrEmpty(type)) return true; // null bedeutet "infer"

            return type switch
            {
                "string" => true,
                "number" => true,
                "int" => true,
                "boolean" => true,
                "array" => true,
                "record" => true,
                "session" => true,
                "tranceify" => true,
                "unknown" => true,
                _ => false
            };
        }

        private bool IsTypeCompatible(string targetType, string sourceType)
        {
            if (targetType == sourceType) return true;

            // Numerische Kompatibilität
            if ((targetType == "number" && sourceType == "int") ||
                (targetType == "int" && sourceType == "number"))
            {
                return true;
            }

            // Array-Kompatibilität
            if (targetType == "array" && sourceType == "array")
            {
                return true;
            }

            return false;
        }

        private bool IsOperatorCompatible(string op, string? leftType, string? rightType)
        {
            return op switch
            {
                "+" => IsNumericOrString(leftType) && IsNumericOrString(rightType),
                "-" => IsNumeric(leftType) && IsNumeric(rightType),
                "*" => IsNumeric(leftType) && IsNumeric(rightType),
                "/" => IsNumeric(leftType) && IsNumeric(rightType),
                "==" => true, // Alle Typen können verglichen werden
                "!=" => true,
                ">" => IsNumeric(leftType) && IsNumeric(rightType),
                "<" => IsNumeric(leftType) && IsNumeric(rightType),
                ">=" => IsNumeric(leftType) && IsNumeric(rightType),
                "<=" => IsNumeric(leftType) && IsNumeric(rightType),
                "&&" => leftType == "boolean" && rightType == "boolean",
                "||" => leftType == "boolean" && rightType == "boolean",
                _ => false
            };
        }

        private bool IsUnaryOperatorCompatible(string op, string? operandType)
        {
            return op switch
            {
                "-" => IsNumeric(operandType),
                "!" => operandType == "boolean",
                _ => false
            };
        }

        private bool IsNumeric(string? type)
        {
            return type == "number" || type == "int";
        }

        private bool IsNumericOrString(string? type)
        {
            return IsNumeric(type) || type == "string";
        }

        // Einfache Typinferenz für Literale, Record-Literale, Identifier
        private string? InferExpressionType(IExpression? expr)
        {
            if (expr == null) return null;
            switch (expr)
            {
                case LiteralExpressionNode lit:
                    return lit.LiteralType;
                case BinaryExpressionNode bin:
                    var leftType = InferExpressionType(bin.Left);
                    var rightType = InferExpressionType(bin.Right);
                    var binType = InferBinaryType(bin.Operator, leftType, rightType);
                    if (binType == "unknown")
                        ErrorReporter.Report($"Type of binary expression could not be inferred (unknown type)", 0, 0, "TYPE900");
                    return binType;
                case UnaryExpressionNode unary:
                    var operandType = InferExpressionType(unary.Operand);
                    var unaryType = InferUnaryType(unary.Operator, operandType);
                    if (unaryType == "unknown")
                        ErrorReporter.Report($"Type of unary expression could not be inferred (unknown type)", 0, 0, "TYPE901");
                    return unaryType;
                case ParenthesizedExpressionNode paren:
                    return InferExpressionType(paren.Expression);
                case AssignmentExpressionNode assign:
                    return InferExpressionType(assign.Value);
                case IdentifierExpressionNode id:
                    var sym = _globals.Resolve(id.Name);
                    if (sym?.TypeName == "unknown")
                        ErrorReporter.Report($"Type of variable '{id.Name}' is unknown", 0, 0, "TYPE902");
                    return sym?.TypeName;
                case CallExpressionNode call:
                    var callType = InferCallType(call);
                    if (callType == "unknown")
                        ErrorReporter.Report($"Type of function call could not be inferred (unknown type)", 0, 0, "TYPE903");
                    return callType;
                case ArrayLiteralExpressionNode arrayLit:
                    return "array";
                case ArrayAccessExpressionNode arrayAccess:
                    var arrType = InferArrayAccessType(arrayAccess);
                    if (arrType == "unknown")
                        ErrorReporter.Report($"Type of array access could not be inferred (unknown type)", 0, 0, "TYPE904");
                    return arrType;
                case FieldAccessExpressionNode fieldAccess:
                    var fieldType = InferFieldAccessType(fieldAccess);
                    if (fieldType == "unknown")
                        ErrorReporter.Report($"Type of field access could not be inferred (unknown type)", 0, 0, "TYPE905");
                    return fieldType;
                default:
                    ErrorReporter.Report($"Type could not be inferred (unknown type)", 0, 0, "TYPE999");
                    return "unknown";
            }
        }

        private string? InferBinaryType(string op, string? leftType, string? rightType)
        {
            // Erweiterte Typinferenz für binäre Operatoren
            switch (op)
            {
                case "+":
                    if (leftType == "string" || rightType == "string") return "string";
                    if (leftType == "number" && rightType == "number") return "number";
                    return "unknown";
                case "-":
                case "*":
                case "/":
                case "%":
                    if (leftType == "number" && rightType == "number") return "number";
                    return "unknown";
                case "==":
                case "!=":
                case "youAreFeelingVerySleepy":
                case "notSoDeep":
                    return "boolean";
                case ">":
                case "<":
                case ">=":
                case "<=":
                case "lookAtTheWatch":
                case "fallUnderMySpell":
                case "deeplyGreater":
                case "deeplyLess":
                    if (leftType == "number" && rightType == "number") return "boolean";
                    return "unknown";
                case "&&":
                case "||":
                    if (leftType == "boolean" && rightType == "boolean") return "boolean";
                    return "unknown";
                default:
                    return "unknown";
            }
        }

        private string? InferUnaryType(string op, string? operandType)
        {
            switch (op)
            {
                case "!":
                    if (operandType == "boolean") return "boolean";
                    return "unknown";
                case "+":
                case "-":
                    if (operandType == "number") return "number";
                    return "unknown";
                default:
                    return "unknown";
            }
        }

        private string? InferCallType(CallExpressionNode call)
        {
            // Builtin-Funktionen Typinferenz
            if (call.Callee is IdentifierExpressionNode id)
            {
                return InferBuiltinReturnType(id.Name);
            }
            return "unknown";
        }

        private string? InferBuiltinReturnType(string functionName)
        {
            // Umfassende Builtin-Funktionen Typinferenz
            switch (functionName)
            {
                // Mathematische Funktionen
                case "Abs":
                case "Sin":
                case "Cos":
                case "Tan":
                case "Sqrt":
                case "Pow":
                case "Floor":
                case "Ceiling":
                case "Round":
                case "Log":
                case "Log10":
                case "Exp":
                case "Max":
                case "Min":
                case "Random":
                case "Factorial":
                case "GCD":
                case "LCM":
                case "DegreesToRadians":
                case "RadiansToDegrees":
                case "Asin":
                case "Acos":
                case "Atan":
                case "Atan2":
                    return "number";

                // String-Funktionen
                case "Length":
                case "IndexOf":
                case "LastIndexOf":
                case "CountOccurrences":
                    return "number";
                case "Substring":
                case "ToUpper":
                case "ToLower":
                case "Trim":
                case "TrimStart":
                case "TrimEnd":
                case "Replace":
                case "PadLeft":
                case "PadRight":
                case "Reverse":
                case "Capitalize":
                case "TitleCase":
                case "RemoveWhitespace":
                case "ToString":
                case "Base64Encode":
                case "Base64Decode":
                case "HashMD5":
                case "HashSHA256":
                case "FormatDateTime":
                case "GetCurrentDate":
                case "GetCurrentTimeString":
                case "GetCurrentDateTime":
                case "GetCurrentDirectory":
                case "GetMachineName":
                case "GetUserName":
                case "GetOSVersion":
                case "GetFileExtension":
                case "GetFileName":
                case "GetDirectoryName":
                case "ToJson":
                    return "string";

                // Boolean-Funktionen
                case "Contains":
                case "StartsWith":
                case "EndsWith":
                case "ArrayContains":
                case "FileExists":
                case "DirectoryExists":
                case "IsLeapYear":
                case "ToBoolean":
                    return "boolean";

                // Array-Funktionen
                case "ArrayLength":
                    return "number";
                case "ArrayGet":
                case "ArraySlice":
                case "ArrayConcat":
                case "ArrayReverse":
                case "ArraySort":
                case "ArrayUnique":
                case "ArrayFilter":
                case "Split":
                    return "array";

                // Konvertierungsfunktionen
                case "ToInt":
                    return "number";
                case "ToDouble":
                    return "number";
                case "ToChar":
                    return "string";

                // Void-Funktionen (kein Rückgabewert)
                case "Observe":
                case "Drift":
                case "DeepTrance":
                case "HypnoticCountdown":
                case "TranceInduction":
                case "HypnoticVisualization":
                case "ProgressiveRelaxation":
                case "HypnoticSuggestion":
                case "TranceDeepening":
                case "HypnoticBreathing":
                case "HypnoticAnchoring":
                case "HypnoticRegression":
                case "HypnoticFutureProgression":
                case "WriteFile":
                case "AppendFile":
                case "WriteLines":
                case "CreateDirectory":
                case "ClearScreen":
                case "Beep":
                case "Exit":
                case "DebugPrint":
                case "DebugPrintType":
                case "DebugPrintMemory":
                case "DebugPrintStackTrace":
                case "DebugPrintEnvironment":
                case "PlaySound":
                case "Vibrate":
                    return "void";

                // Zeit-Funktionen
                case "GetCurrentTime":
                case "GetDayOfWeek":
                case "GetDayOfYear":
                case "GetDaysInMonth":
                case "GetFileSize":
                case "GetProcessorCount":
                case "GetWorkingSet":
                    return "number";

                default:
                    return "unknown";
            }
        }

        private string? InferArrayAccessType(ArrayAccessExpressionNode arrayAccess)
        {
            var arrayType = InferExpressionType(arrayAccess.Array);
            if (arrayType == "array")
            {
                // Für Arrays geben wir "unknown" zurück, da wir den Elementtyp nicht kennen
                return "unknown";
            }
            return "unknown";
        }

        private string? InferFieldAccessType(FieldAccessExpressionNode fieldAccess)
        {
            var objectType = InferExpressionType(fieldAccess.Target);

            // Session-Member-Zugriff
            if (!string.IsNullOrEmpty(objectType) && _sessions.ContainsKey(objectType))
            {
                var session = _sessions[objectType];
                foreach (var member in session.Members)
                {
                    if (member.Declaration is VarDeclNode varDecl && varDecl.Identifier == fieldAccess.FieldName)
                    {
                        return varDecl.TypeName;
                    }
                }
            }

            // Tranceify-Member-Zugriff
            if (!string.IsNullOrEmpty(objectType) && _tranceifies.ContainsKey(objectType))
            {
                var tranceify = _tranceifies[objectType];
                foreach (var member in tranceify.Members)
                {
                    if (member is VarDeclNode varDecl && varDecl.Identifier == fieldAccess.FieldName)
                    {
                        return varDecl.TypeName;
                    }
                }
            }

            return "unknown";
        }

        // Erweiterte Validierung für Session-Definitionen
        private void ValidateSession(SessionDeclNode session)
        {
            var sessionSymbols = new Dictionary<string, string>();

            foreach (var member in session.Members)
            {
                if (member.Declaration is VarDeclNode varDecl)
                {
                    if (sessionSymbols.ContainsKey(varDecl.Identifier))
                    {
                        ErrorReporter.Report($"Duplicate member '{varDecl.Identifier}' in session '{session.Name}'", 0, 0, "TYPE040");
                    }
                    else
                    {
                        sessionSymbols[varDecl.Identifier] = varDecl.TypeName ?? "unknown";
                    }
                }
                else if (member.Declaration is FunctionDeclNode funcDecl)
                {
                    if (sessionSymbols.ContainsKey(funcDecl.Name))
                    {
                        ErrorReporter.Report($"Duplicate method '{funcDecl.Name}' in session '{session.Name}'", 0, 0, "TYPE041");
                    }
                    else
                    {
                        sessionSymbols[funcDecl.Name] = funcDecl.ReturnType ?? "void";
                    }
                }
            }
        }

        // Erweiterte Validierung für Tranceify-Definitionen
        private void ValidateTranceify(TranceifyDeclNode tranceify)
        {
            var fieldNames = new HashSet<string>();

            foreach (var member in tranceify.Members)
            {
                if (member is VarDeclNode varDecl)
                {
                    if (fieldNames.Contains(varDecl.Identifier))
                    {
                        ErrorReporter.Report($"Duplicate field '{varDecl.Identifier}' in tranceify '{tranceify.Name}'", 0, 0, "TYPE050");
                    }
                    else
                    {
                        fieldNames.Add(varDecl.Identifier);
                    }
                }
            }
        }

        private string? GetCachedType(string key)
        {
            return _typeCache.TryGetValue(key, out var type) ? type : null;
        }

        private void CacheType(string key, string? type)
        {
            if (_typeCache.Count > 1000) // Cache-Größe begrenzen
            {
                _typeCache.Clear();
            }
            _typeCache[key] = type;
        }
    }
}
