using System;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Types;
using HypnoScript.Compiler.Error;
using System.Collections.Generic;
using HypnoScript.Core.Symbols;

namespace HypnoScript.Compiler.Analysis
{
    public class TypeChecker
    {
        private readonly Dictionary<string, SessionDeclNode> _sessions = new();
        private readonly Dictionary<string, TranceifyDeclNode> _tranceifies = new();
        private readonly SymbolTable _globals = new();
        private HashSet<string> _labelsInScope = new();

        // Enterprise-Level: Neben der reinen Traversierung werden Typ-Inkonsistenzen protokolliert.
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
                    var initType = InferExpressionType(varDecl.Initializer);
                    if (varDecl.TypeName != null && initType != null && varDecl.TypeName != initType)
                    {
                        ErrorReporter.Report($"Type mismatch: Variable '{varDecl.Identifier}' declared as '{varDecl.TypeName}' but initializer is '{initType}'", 0, 0, "TYPE002");
                    }
                    if (!_globals.Define(new HypnoScript.Core.Symbols.Symbol(varDecl.Identifier, varDecl.TypeName)))
                    {
                        ErrorReporter.Report($"Variable '{varDecl.Identifier}' already defined", 0, 0, "TYPE003");
                    }
                    break;
                case FunctionDeclNode funcDecl:
                    // Funktionssymbol anlegen
                    _globals.Define(new HypnoScript.Core.Symbols.Symbol(funcDecl.Name, funcDecl.ReturnType));
                    foreach (var s in funcDecl.Body)
                    {
                        CheckStatement(s);
                    }
                    break;
                case SessionDeclNode session:
                    // Felder und Methoden prüfen
                    foreach (var member in session.Members)
                    {
                        CheckSessionMember(member);
                    }
                    break;
                case SessionMemberNode sessionMember:
                    CheckSessionMember(sessionMember);
                    break;
                case TranceifyDeclNode trance:
                    // Felder prüfen
                    foreach (var member in trance.Members)
                    {
                        CheckStatement(member);
                    }
                    break;
                case ExpressionStatementNode exprStmt:
                    CheckExpression(exprStmt.Expression);
                    break;
                case ObserveStatementNode obs:
                    CheckExpression(obs.Expression);
                    break;
                case DriftStatementNode drift:
                    var driftType = InferExpressionType(drift.Milliseconds);
                    if (driftType != "number")
                    {
                        ErrorReporter.Report($"drift() expects number, got '{driftType}'", 0, 0, "TYPE004");
                    }
                    break;
                case ReturnStatementNode ret:
                    CheckExpression(ret.Expression);
                    break;
                case IfStatementNode ifStmt:
                    var conditionType = InferExpressionType(ifStmt.Condition);
                    if (conditionType != "boolean")
                    {
                        ErrorReporter.Report($"if condition must be boolean, got '{conditionType}'", 0, 0, "TYPE005");
                    }
                    foreach (var s in ifStmt.ThenBranch)
                        CheckStatement(s);
                    if (ifStmt.ElseBranch != null)
                        foreach (var s in ifStmt.ElseBranch)
                            CheckStatement(s);
                    break;
                case WhileStatementNode whileStmt:
                    var whileConditionType = InferExpressionType(whileStmt.Condition);
                    if (whileConditionType != "boolean")
                    {
                        ErrorReporter.Report($"while condition must be boolean, got '{whileConditionType}'", 0, 0, "TYPE006");
                    }
                    foreach (var s in whileStmt.Body)
                        CheckStatement(s);
                    break;
                case LoopStatementNode loopStmt:
                    var loopConditionType = InferExpressionType(loopStmt.Condition);
                    if (loopConditionType != "boolean")
                    {
                        ErrorReporter.Report($"loop condition must be boolean, got '{loopConditionType}'", 0, 0, "TYPE007");
                    }
                    if (loopStmt.Initializer != null)
                        CheckStatement(loopStmt.Initializer);
                    if (loopStmt.Iteration != null)
                        CheckStatement(loopStmt.Iteration);
                    foreach (var s in loopStmt.Body)
                        CheckStatement(s);
                    break;
                case SnapStatementNode:
                case SinkStatementNode:
                    // Keine spezielle Typprüfung nötig
                    break;
                case MindLinkNode mindLink:
                    // TODO: Später importierte Symbole in _sessions, _tranceifies, _globals übernehmen
                    break;
                case SharedTranceVarDeclNode shared:
                    var sharedType = InferExpressionType(shared.Initializer);
                    if (shared.TypeName != null && sharedType != null && shared.TypeName != sharedType)
                    {
                        ErrorReporter.Report($"Type mismatch: sharedTrance variable '{shared.Identifier}' declared as '{shared.TypeName}' but initializer is '{sharedType}'", 0, 0, "TYPE020");
                    }
                    if (!_globals.Define(new HypnoScript.Core.Symbols.Symbol(shared.Identifier, shared.TypeName)))
                    {
                        ErrorReporter.Report($"sharedTrance variable '{shared.Identifier}' already defined", 0, 0, "TYPE021");
                    }
                    break;
                case LabelNode label:
                    _labelsInScope.Add(label.Name);
                    break;
                case SinkToNode sinkTo:
                    if (!_labelsInScope.Contains(sinkTo.LabelName))
                    {
                        ErrorReporter.Report($"sinkTo label '{sinkTo.LabelName}' not found in scope", 0, 0, "TYPE030");
                    }
                    break;
                case EntranceBlockNode entrance:
                    foreach (var s in entrance.Statements)
                        CheckStatement(s);
                    break;
                default:
                    break;
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
                    if(lit.LiteralType == "number" && !double.TryParse(lit.Value, out _))
                    {
                        ErrorReporter.Report("Invalid numeric literal", 0, 0, "TYPE001");
                    }
                    break;
                case BinaryExpressionNode bin:
                    CheckExpression(bin.Left);
                    CheckExpression(bin.Right);
                    break;
                case UnaryExpressionNode unary:
                    CheckExpression(unary.Operand);
                    break;
                case ParenthesizedExpressionNode paren:
                    CheckExpression(paren.Expression);
                    break;
                case AssignmentExpressionNode assign:
                    CheckExpression(assign.Value);
                    break;
                case IdentifierExpressionNode id:
                    var sym = _globals.Resolve(id.Name);
                    if (sym == null)
                    {
                        ErrorReporter.Report($"Variable '{id.Name}' not defined", 0, 0, "TYPE008");
                    }
                    break;
                case CallExpressionNode call:
                    CheckExpression(call.Callee);
                    foreach (var arg in call.Arguments)
                        CheckExpression(arg);
                    break;
                case MethodCallExpressionNode methodCall:
                    CheckExpression(methodCall.Target);
                    foreach (var arg in methodCall.Arguments)
                        CheckExpression(arg);
                    break;
                case SessionInstantiationNode sessionInst:
                    if (!_sessions.ContainsKey(sessionInst.SessionName))
                    {
                        ErrorReporter.Report($"Session '{sessionInst.SessionName}' not defined", 0, 0, "TYPE009");
                    }
                    foreach (var arg in sessionInst.Arguments)
                        CheckExpression(arg);
                    break;
                case RecordLiteralExpressionNode rec:
                    // Prüfe, ob tranceify-Typ existiert
                    if (!_tranceifies.TryGetValue(rec.TypeName, out var tranceDef))
                    {
                        ErrorReporter.Report($"tranceify type '{rec.TypeName}' not defined", 0, 0, "TYPE010");
                        break;
                    }
                    // Prüfe Felder
                    var tranceFields = new HashSet<string>(tranceDef.Members.ConvertAll(f => f.Identifier));
                    foreach (var kv in rec.Fields)
                    {
                        if (!tranceFields.Contains(kv.Key))
                        {
                            ErrorReporter.Report($"Field '{kv.Key}' not in tranceify '{rec.TypeName}'", 0, 0, "TYPE011");
                        }
                        CheckExpression(kv.Value);
                    }
                    break;
                case FieldAccessExpressionNode field:
                    // Prüfe Typ des Targets
                    var targetType = InferExpressionType(field.Target);
                    if (targetType == null)
                        break;
                    // Prüfe tranceify-Feld
                    if (_tranceifies.TryGetValue(targetType, out var tranceType))
                    {
                        var found = tranceType.Members.Exists(f => f.Identifier == field.FieldName);
                        if (!found)
                        {
                            ErrorReporter.Report($"Field '{field.FieldName}' not found in tranceify '{targetType}'", 0, 0, "TYPE012");
                        }
                    }
                    // Prüfe Session-Feld
                    if (_sessions.TryGetValue(targetType, out var sessionType))
                    {
                        var found = sessionType.Members.Exists(f => f.Declaration is VarDeclNode v && v.Identifier == field.FieldName);
                        if (!found)
                        {
                            ErrorReporter.Report($"Field '{field.FieldName}' not found in session '{targetType}'", 0, 0, "TYPE013");
                        }
                    }
                    break;
                case ArrayAccessExpressionNode arrayAccess:
                    CheckExpression(arrayAccess.Array);
                    CheckExpression(arrayAccess.Index);
                    break;
                case ArrayLiteralExpressionNode arrayLit:
                    foreach (var element in arrayLit.Elements)
                        CheckExpression(element);
                    break;
                default:
                    break;
            }
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
                    return InferBinaryType(bin.Operator, leftType, rightType);
                case UnaryExpressionNode unary:
                    var operandType = InferExpressionType(unary.Operand);
                    return InferUnaryType(unary.Operator, operandType);
                case ParenthesizedExpressionNode paren:
                    return InferExpressionType(paren.Expression);
                case AssignmentExpressionNode assign:
                    return InferExpressionType(assign.Value);
                case IdentifierExpressionNode id:
                    var sym = _globals.Resolve(id.Name);
                    return sym?.TypeName;
                case CallExpressionNode call:
                    return InferCallType(call);
                case ArrayLiteralExpressionNode arrayLit:
                    return "array";
                case ArrayAccessExpressionNode arrayAccess:
                    return InferArrayAccessType(arrayAccess);
                case FieldAccessExpressionNode fieldAccess:
                    return InferFieldAccessType(fieldAccess);
                default:
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

        // Performance-Optimierung: Caching für wiederholte Typüberprüfungen
        private readonly Dictionary<string, string?> _typeCache = new();

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
