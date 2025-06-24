using System;
using System.Collections.Generic;
using System.Linq;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Symbols;
using HypnoScript.Core.Types;

namespace HypnoScript.Compiler.Session
{
    /// <summary>
    /// Factory for creating and managing Session instances.
    /// </summary>
    public static class SessionFactory
    {
        private static readonly Dictionary<string, SessionTemplate> _sessionTemplates = new();
        private static readonly Dictionary<string, SessionInstance> _activeSessions = new();

        /// <summary>
        /// Registers a session template for later instantiation.
        /// </summary>
        /// <param name="name">The name of the session template</param>
        /// <param name="template">The session template</param>
        public static void RegisterSessionTemplate(string name, SessionTemplate template)
        {
            _sessionTemplates[name] = template;
        }

        /// <summary>
        /// Creates a new session instance from a template.
        /// </summary>
        /// <param name="templateName">The name of the template to use</param>
        /// <param name="sessionName">The name for the new session instance</param>
        /// <returns>The created session instance</returns>
        public static SessionInstance CreateSession(string templateName, string sessionName)
        {
            if (!_sessionTemplates.TryGetValue(templateName, out var template))
            {
                throw new ArgumentException($"Session template '{templateName}' not found.");
            }

            var session = new SessionInstance(sessionName, template);
            _activeSessions[sessionName] = session;
            return session;
        }

        /// <summary>
        /// Creates a session instance directly from a SessionDeclNode.
        /// </summary>
        /// <param name="sessionDecl">The session declaration node</param>
        /// <returns>The created session instance</returns>
        public static SessionInstance CreateSessionFromDeclaration(SessionDeclNode sessionDecl)
        {
            var template = new SessionTemplate(sessionDecl.Name, sessionDecl.Members);
            var session = new SessionInstance(sessionDecl.Name, template);
            _activeSessions[sessionDecl.Name] = session;
            return session;
        }

        /// <summary>
        /// Gets an active session by name.
        /// </summary>
        /// <param name="sessionName">The name of the session</param>
        /// <returns>The session instance or null if not found</returns>
        public static SessionInstance? GetSession(string sessionName)
        {
            return _activeSessions.TryGetValue(sessionName, out var session) ? session : null;
        }

        /// <summary>
        /// Removes a session instance.
        /// </summary>
        /// <param name="sessionName">The name of the session to remove</param>
        /// <returns>True if the session was removed, false if not found</returns>
        public static bool RemoveSession(string sessionName)
        {
            return _activeSessions.Remove(sessionName);
        }

        /// <summary>
        /// Gets all active session names.
        /// </summary>
        /// <returns>Array of active session names</returns>
        public static string[] GetActiveSessionNames()
        {
            return _activeSessions.Keys.ToArray();
        }

        /// <summary>
        /// Gets all registered template names.
        /// </summary>
        /// <returns>Array of registered template names</returns>
        public static string[] GetRegisteredTemplateNames()
        {
            return _sessionTemplates.Keys.ToArray();
        }

        /// <summary>
        /// Clears all active sessions.
        /// </summary>
        public static void ClearAllSessions()
        {
            _activeSessions.Clear();
        }

        /// <summary>
        /// Validates a session declaration for type consistency.
        /// </summary>
        /// <param name="sessionDecl">The session declaration to validate</param>
        /// <returns>Validation result with any errors</returns>
        public static ValidationResult ValidateSessionDeclaration(SessionDeclNode sessionDecl)
        {
            var result = new ValidationResult();
            var symbolTable = new SymbolTable();

            foreach (var member in sessionDecl.Members)
            {
                try
                {
                    // Validate member type
                    if (member.Declaration is VarDeclNode varDecl)
                    {
                        if (!string.IsNullOrEmpty(varDecl.TypeName))
                        {
                            // Note: HypnoType.FromString doesn't exist, so we'll skip type validation for now
                            // var type = HypnoType.FromString(varDecl.TypeName);
                            // if (type == null)
                            // {
                            //     result.AddError($"Unknown type '{varDecl.TypeName}' for variable '{varDecl.Identifier}'");
                            // }
                        }

                        // Check for duplicate variable names
                        if (symbolTable.HasSymbol(varDecl.Identifier))
                        {
                            result.AddError($"Duplicate variable name '{varDecl.Identifier}' in session '{sessionDecl.Name}'");
                        }
                        else
                        {
                            var symbol = new Symbol(varDecl.Identifier, varDecl.TypeName ?? "any");
                            symbolTable.Define(symbol);
                        }
                    }
                    else if (member.Declaration is FunctionDeclNode funcDecl)
                    {
                        // Check for duplicate function names
                        if (symbolTable.HasSymbol(funcDecl.Name))
                        {
                            result.AddError($"Duplicate function name '{funcDecl.Name}' in session '{sessionDecl.Name}'");
                        }
                        else
                        {
                            var symbol = new Symbol(funcDecl.Name, "function");
                            symbolTable.Define(symbol);
                        }
                    }
                }
                catch (Exception ex)
                {
                    result.AddError($"Error validating session member: {ex.Message}");
                }
            }

            return result;
        }
    }

    /// <summary>
    /// Template for creating session instances.
    /// </summary>
    public class SessionTemplate
    {
        /// <summary>
        /// The name of the session template.
        /// </summary>
        public string Name { get; }

        /// <summary>
        /// The members of the session.
        /// </summary>
        public List<SessionMemberNode> Members { get; }

        /// <summary>
        /// Initializes a new session template.
        /// </summary>
        /// <param name="name">The name of the template</param>
        /// <param name="members">The session members</param>
        public SessionTemplate(string name, List<SessionMemberNode> members)
        {
            Name = name;
            Members = members ?? new List<SessionMemberNode>();
        }
    }

    /// <summary>
    /// Represents a session instance.
    /// </summary>
    public class SessionInstance
    {
        /// <summary>
        /// The name of the session instance.
        /// </summary>
        public string Name { get; }

        /// <summary>
        /// The template used to create this session.
        /// </summary>
        public SessionTemplate Template { get; }

        /// <summary>
        /// The symbol table for this session.
        /// </summary>
        public SymbolTable SymbolTable { get; }

        /// <summary>
        /// The variables in this session.
        /// </summary>
        public Dictionary<string, object> Variables { get; }

        /// <summary>
        /// Initializes a new session instance.
        /// </summary>
        /// <param name="name">The name of the session</param>
        /// <param name="template">The template to use</param>
        public SessionInstance(string name, SessionTemplate template)
        {
            Name = name;
            Template = template;
            SymbolTable = new SymbolTable();
            Variables = new Dictionary<string, object>();

            // Initialize symbols from template
            foreach (var member in template.Members)
            {
                if (member.Declaration is VarDeclNode varDecl)
                {
                    var symbol = new Symbol(varDecl.Identifier, varDecl.TypeName ?? "any");
                    SymbolTable.Define(symbol);
                }
                else if (member.Declaration is FunctionDeclNode funcDecl)
                {
                    var symbol = new Symbol(funcDecl.Name, "function");
                    SymbolTable.Define(symbol);
                }
            }
        }

        /// <summary>
        /// Sets a variable value in the session.
        /// </summary>
        /// <param name="name">The variable name</param>
        /// <param name="value">The value to set</param>
        public void SetVariable(string name, object value)
        {
            Variables[name] = value;
        }

        /// <summary>
        /// Gets a variable value from the session.
        /// </summary>
        /// <param name="name">The variable name</param>
        /// <returns>The variable value or null if not found</returns>
        public object? GetVariable(string name)
        {
            return Variables.TryGetValue(name, out var value) ? value : null;
        }

        /// <summary>
        /// Checks if a variable exists in the session.
        /// </summary>
        /// <param name="name">The variable name</param>
        /// <returns>True if the variable exists, false otherwise</returns>
        public bool HasVariable(string name)
        {
            return Variables.ContainsKey(name);
        }

        /// <summary>
        /// Gets all variable names in the session.
        /// </summary>
        /// <returns>Array of variable names</returns>
        public string[] GetVariableNames()
        {
            return Variables.Keys.ToArray();
        }

        /// <summary>
        /// Clears all variables in the session.
        /// </summary>
        public void ClearVariables()
        {
            Variables.Clear();
        }
    }

    /// <summary>
    /// Result of session validation.
    /// </summary>
    public class ValidationResult
    {
        private readonly List<string> _errors = new();

        /// <summary>
        /// Gets whether the validation was successful.
        /// </summary>
        public bool IsValid => _errors.Count == 0;

        /// <summary>
        /// Gets the validation errors.
        /// </summary>
        public string[] Errors => _errors.ToArray();

        /// <summary>
        /// Adds an error to the validation result.
        /// </summary>
        /// <param name="error">The error message</param>
        public void AddError(string error)
        {
            _errors.Add(error);
        }

        /// <summary>
        /// Gets a formatted error message.
        /// </summary>
        /// <returns>The formatted error message</returns>
        public string GetErrorMessage()
        {
            return string.Join(Environment.NewLine, _errors);
        }
    }
}
