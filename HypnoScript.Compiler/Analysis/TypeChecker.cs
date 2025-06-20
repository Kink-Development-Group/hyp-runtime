using System;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Types;
using HypnoScript.Compiler.Error;
using System.Collections.Generic;

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
                        CheckStatement(member);
                    }
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
                case ReturnStatementNode ret:
                    CheckExpression(ret.Expression);
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
                default:
                    break;
            }
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
                        var found = sessionType.Members.Exists(f => f is VarDeclNode v && v.Identifier == field.FieldName);
                        if (!found)
                        {
                            ErrorReporter.Report($"Field '{field.FieldName}' not found in session '{targetType}'", 0, 0, "TYPE013");
                        }
                    }
                    break;
                case CallExpressionNode call:
                    // Methodenaufruf auf Session oder tranceify
                    if (call.Callee is FieldAccessExpressionNode fieldCall)
                    {
                        var calleeType = InferExpressionType(fieldCall.Target);
                        if (calleeType != null && _sessions.TryGetValue(calleeType, out var sessionType))
                        {
                            var method = sessionType.Members.Find(m => m is FunctionDeclNode f && f.Name == fieldCall.FieldName) as FunctionDeclNode;
                            if (method == null)
                            {
                                ErrorReporter.Report($"Method '{fieldCall.FieldName}' not found in session '{calleeType}'", 0, 0, "TYPE014");
                            }
                            else if (call.Arguments.Count != method.Parameters.Count)
                            {
                                ErrorReporter.Report($"Method '{fieldCall.FieldName}' expects {method.Parameters.Count} arguments, got {call.Arguments.Count}", 0, 0, "TYPE015");
                            }
                            else
                            {
                                for (int i = 0; i < call.Arguments.Count; i++)
                                {
                                    CheckExpression(call.Arguments[i]);
                                }
                            }
                        }
                    }
                    else if (call.Callee is IdentifierExpressionNode id)
                    {
                        // Funktionsaufruf im globalen Scope
                        var sym = _globals.Resolve(id.Name);
                        if (sym == null)
                        {
                            ErrorReporter.Report($"Function '{id.Name}' not defined", 0, 0, "TYPE016");
                        }
                    }
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
                case IdentifierExpressionNode id:
                    var sym = _globals.Resolve(id.Name);
                    return sym?.TypeName;
                case RecordLiteralExpressionNode rec:
                    return rec.TypeName;
                default:
                    return null;
            }
        }
    }
}
