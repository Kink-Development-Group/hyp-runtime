using System;
using System.Collections.Generic;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Symbols;

namespace HypnoScript.Compiler.Interpreter
{
    // Eigene Exceptions für break (snap) und continue (sink)
    public class BreakException : Exception { }
    public class ContinueException : Exception { }

    public partial class HypnoInterpreter
    {
        // Enterprise-Level: Erweiterte Implementierung des "loop"-Statements.
        // Wir nehmen an, dass LoopStatementNode folgende Struktur hat:
        // IStatement Initializer, IExpression Condition, IStatement Iteration, List<IStatement> Body.
        private void ExecuteLoop(LoopStatementNode loopStmt)
        {
            // Neuer lokaler Scope für die Schleifenvariablen
            var loopScope = new SymbolTable(_globals);
            
            // Führe Initialisierungs-Anweisung aus, falls vorhanden
            if (loopStmt.Initializer != null)
                ExecuteStatement(loopStmt.Initializer);
            
            while (true)
            {
                // Überprüfe die Bedingung
                var cond = EvaluateExpression(loopStmt.Condition);
                if (!IsTruthy(cond))
                    break;
                
                try
                {
                    // Führe jeden Statement im Schleifen-Körper aus
                    foreach (var stmt in loopStmt.Body)
                    {
                        ExecuteStatement(stmt);
                    }
                }
                catch (BreakException)
                {
                    // "snap" wurde aufgerufen – Schleife verlassen
                    break;
                }
                catch (ContinueException)
                {
                    // "sink" wurde aufgerufen – überspringe den restlichen Körper und gehe zur Iteration
                }
                
                // Führe Iterations-Anweisung (z.B. i = i + 1) aus, falls vorhanden
                if (loopStmt.Iteration != null)
                    ExecuteStatement(loopStmt.Iteration);
            }
        }
    }
}
