using System;
using System.Collections.Generic;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Symbols;

namespace HypnoScript.Compiler.Interpreter
{
    public partial class HypnoInterpreter
    {
        // Erzeugt eine Session-Instanz basierend auf einer SessionDeclNode und übergibt die Argumente an den Konstruktor.
        private SessionInstance InstantiateSession(SessionDeclNode sessionDecl, List<object?> constructorArgs)
        {
            var instance = new SessionInstance(sessionDecl.Name);

            // Initialisierung der Felder und Registrierung von Methoden.
            foreach (var member in sessionDecl.Members)
            {
                switch (member)
                {
                    case VarDeclNode varDecl:
                        // Setze das Feld auf den Wert des Initializers (falls vorhanden) oder auf null.
                        instance.Fields[varDecl.Identifier] = varDecl.Initializer != null ? EvaluateExpression(varDecl.Initializer) : null;
                        break;
                    case FunctionDeclNode funcDecl:
                        if (funcDecl.Name != "constructor")
                        {
                            instance.Methods[funcDecl.Name] = funcDecl;
                        }
                        // Den Konstruktor behandeln wir später.
                        break;
                }
            }

            // Falls ein Konstruktor definiert ist, führe ihn aus.
            var constructor = sessionDecl.Members.Find(m => m is FunctionDeclNode fd && fd.Name == "constructor") as FunctionDeclNode;
            if (constructor != null)
            {
                // Erstelle einen separaten Scope für den Konstruktor.
                var localScope = new SymbolTable(_globals);
                for (int i = 0; i < constructor.Parameters.Count; i++)
                {
                    var param = constructor.Parameters[i];
                    var argValue = i < constructorArgs.Count ? constructorArgs[i] : null;
                    localScope.Define(new Symbol(param.Name, param.TypeName, argValue));
                }
                // Führe den Konstruktor-Body aus. (Rückgabewert wird ignoriert.)
                foreach (var stmt in constructor.Body)
                {
                    ExecuteStatement(stmt);
                }
            }
            return instance;
        }

        // Führt einen Methodenaufruf auf einer Session-Instanz aus.
        private object? EvaluateSessionMemberCall(SessionInstance instance, string memberName, List<object?> arguments)
        {
            if (instance.Methods.TryGetValue(memberName, out var method))
            {
                // Erstelle einen neuen Scope für den Methodenaufruf und binde Parameter.
                var localScope = new SymbolTable(_globals);
                for (int i = 0; i < method.Parameters.Count; i++)
                {
                    var param = method.Parameters[i];
                    var argValue = i < arguments.Count ? arguments[i] : null;
                    localScope.Define(new Symbol(param.Name, param.TypeName, argValue));
                }
                // Führe den Methoden-Body aus.
                foreach (var stmt in method.Body)
                {
                    ExecuteStatement(stmt);
                }
                return null; // Rückgabewert ignoriert – Erweiterungen möglich.
            }
            throw new Exception($"Member '{memberName}' nicht in Session '{instance.Name}' gefunden.");
        }
    }
}
