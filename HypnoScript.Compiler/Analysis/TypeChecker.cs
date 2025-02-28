using System;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Types;
using HypnoScript.Compiler.Error;

namespace HypnoScript.Compiler.Analysis
{
    public class TypeChecker
    {
        // Enterprise-Level: Neben der reinen Traversierung werden Typ-Inkonsistenzen protokolliert.
        public void Check(ProgramNode program)
        {
            // Beispielhafte Traversierung:
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
                    CheckExpression(varDecl.Initializer);
                    // Überprüfe, ob TypeName mit dem ermittelten Literaltyp übereinstimmt
                    // Wenn nicht: ErrorReporter.Report(...)
                    break;
                case FunctionDeclNode funcDecl:
                    // Überprüfe Parameter und Rückgabetypen
                    foreach (var param in funcDecl.Parameters)
                    {
                        // Überprüfe Parameter, evtl. Typ nicht null
                    }
                    foreach (var s in funcDecl.Body)
                    {
                        CheckStatement(s);
                    }
                    break;
                // Weitere Statement-Typen hinzufügen
                default:
                    // ...existing code...
                    break;
            }
        }

        private void CheckExpression(IExpression? expr)
        {
            if (expr == null) return;
            switch (expr)
            {
                case LiteralExpressionNode lit:
                    // Beispiel: Zahl-Literal sollte zu "number" passen
                    if(lit.LiteralType == "number" && !double.TryParse(lit.Value, out _))
                    {
                        ErrorReporter.Report("Invalid numeric literal", 0, 0, "TYPE001");
                    }
                    break;
                case BinaryExpressionNode bin:
                    CheckExpression(bin.Left);
                    CheckExpression(bin.Right);
                    // Beispiel: Prüfe typkompatible Operatoren
                    break;
                // Breite Unterstützung für weitere Ausdrucksarten
                default:
                    break;
            }
        }
    }
}
