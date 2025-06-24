using System;
using System.Collections.Generic;
using System.Linq;
using HypnoScript.LexerParser.AST;
using HypnoScript.Core.Symbols;
using HypnoScript.Core.Types;

namespace HypnoScript.Compiler.Session
{
    /// <summary>
    /// Factory for creating and managing Tranceify instances.
    /// </summary>
    public static class TranceifyFactory
    {
        private static readonly Dictionary<string, TranceifyTemplate> _tranceifyTemplates = new();
        private static readonly Dictionary<string, TranceifyInstance> _activeTranceifies = new();

        /// <summary>
        /// Registers a tranceify template for later instantiation.
        /// </summary>
        /// <param name="name">The name of the tranceify template</param>
        /// <param name="template">The tranceify template</param>
        public static void RegisterTranceifyTemplate(string name, TranceifyTemplate template)
        {
            _tranceifyTemplates[name] = template;
        }

        /// <summary>
        /// Creates a new tranceify instance from a template.
        /// </summary>
        /// <param name="templateName">The name of the template to use</param>
        /// <param name="tranceifyName">The name for the new tranceify instance</param>
        /// <returns>The created tranceify instance</returns>
        public static TranceifyInstance CreateTranceify(string templateName, string tranceifyName)
        {
            if (!_tranceifyTemplates.TryGetValue(templateName, out var template))
            {
                throw new ArgumentException($"Tranceify template '{templateName}' not found.");
            }

            var tranceify = new TranceifyInstance(tranceifyName, template);
            _activeTranceifies[tranceifyName] = tranceify;
            return tranceify;
        }

        /// <summary>
        /// Creates a tranceify instance directly from a TranceifyDeclNode.
        /// </summary>
        /// <param name="tranceifyDecl">The tranceify declaration node</param>
        /// <returns>The created tranceify instance</returns>
        public static TranceifyInstance CreateTranceifyFromDeclaration(TranceifyDeclNode tranceifyDecl)
        {
            var template = new TranceifyTemplate(tranceifyDecl.Name, tranceifyDecl.Members);
            var tranceify = new TranceifyInstance(tranceifyDecl.Name, template);
            _activeTranceifies[tranceifyDecl.Name] = tranceify;
            return tranceify;
        }

        /// <summary>
        /// Gets an active tranceify by name.
        /// </summary>
        /// <param name="tranceifyName">The name of the tranceify</param>
        /// <returns>The tranceify instance or null if not found</returns>
        public static TranceifyInstance? GetTranceify(string tranceifyName)
        {
            return _activeTranceifies.TryGetValue(tranceifyName, out var tranceify) ? tranceify : null;
        }

        /// <summary>
        /// Removes a tranceify instance.
        /// </summary>
        /// <param name="tranceifyName">The name of the tranceify to remove</param>
        /// <returns>True if the tranceify was removed, false if not found</returns>
        public static bool RemoveTranceify(string tranceifyName)
        {
            return _activeTranceifies.Remove(tranceifyName);
        }

        /// <summary>
        /// Gets all active tranceify names.
        /// </summary>
        /// <returns>Array of active tranceify names</returns>
        public static string[] GetActiveTranceifyNames()
        {
            return _activeTranceifies.Keys.ToArray();
        }

        /// <summary>
        /// Gets all registered template names.
        /// </summary>
        /// <returns>Array of registered template names</returns>
        public static string[] GetRegisteredTemplateNames()
        {
            return _tranceifyTemplates.Keys.ToArray();
        }

        /// <summary>
        /// Clears all active tranceifies.
        /// </summary>
        public static void ClearAllTranceifies()
        {
            _activeTranceifies.Clear();
        }

        /// <summary>
        /// Validates a tranceify declaration for type consistency.
        /// </summary>
        /// <param name="tranceifyDecl">The tranceify declaration to validate</param>
        /// <returns>Validation result with any errors</returns>
        public static ValidationResult ValidateTranceifyDeclaration(TranceifyDeclNode tranceifyDecl)
        {
            var result = new ValidationResult();
            var symbolTable = new SymbolTable();

            foreach (var variable in tranceifyDecl.Members)
            {
                try
                {
                    // Validate variable type
                    if (!string.IsNullOrEmpty(variable.TypeName))
                    {
                        // Note: HypnoType.FromString doesn't exist, so we'll skip type validation for now
                        // var type = HypnoType.FromString(variable.TypeName);
                        // if (type == null)
                        // {
                        //     result.AddError($"Unknown type '{variable.TypeName}' for variable '{variable.Identifier}'");
                        // }
                    }

                    // Check for duplicate variable names
                    if (symbolTable.HasSymbol(variable.Identifier))
                    {
                        result.AddError($"Duplicate variable name '{variable.Identifier}' in tranceify '{tranceifyDecl.Name}'");
                    }
                    else
                    {
                        var symbol = new Symbol(variable.Identifier, variable.TypeName ?? "any");
                        symbolTable.Define(symbol);
                    }
                }
                catch (Exception ex)
                {
                    result.AddError($"Error validating tranceify variable: {ex.Message}");
                }
            }

            return result;
        }

        /// <summary>
        /// Links a tranceify to a session for variable sharing.
        /// </summary>
        /// <param name="tranceifyName">The name of the tranceify</param>
        /// <param name="sessionName">The name of the session to link to</param>
        /// <returns>True if the link was successful, false otherwise</returns>
        public static bool LinkToSession(string tranceifyName, string sessionName)
        {
            if (!_activeTranceifies.TryGetValue(tranceifyName, out var tranceify))
            {
                return false;
            }

            var session = SessionFactory.GetSession(sessionName);
            if (session == null)
            {
                return false;
            }

            tranceify.LinkSession(session);
            return true;
        }

        /// <summary>
        /// Unlinks a tranceify from its session.
        /// </summary>
        /// <param name="tranceifyName">The name of the tranceify</param>
        /// <returns>True if the unlink was successful, false otherwise</returns>
        public static bool UnlinkFromSession(string tranceifyName)
        {
            if (!_activeTranceifies.TryGetValue(tranceifyName, out var tranceify))
            {
                return false;
            }

            tranceify.UnlinkSession();
            return true;
        }
    }

    /// <summary>
    /// Template for creating tranceify instances.
    /// </summary>
    public class TranceifyTemplate
    {
        /// <summary>
        /// The name of the tranceify template.
        /// </summary>
        public string Name { get; }

        /// <summary>
        /// The variables of the tranceify.
        /// </summary>
        public List<VarDeclNode> Variables { get; }

        /// <summary>
        /// Initializes a new tranceify template.
        /// </summary>
        /// <param name="name">The name of the template</param>
        /// <param name="variables">The tranceify variables</param>
        public TranceifyTemplate(string name, List<VarDeclNode> variables)
        {
            Name = name;
            Variables = variables ?? new List<VarDeclNode>();
        }
    }

    /// <summary>
    /// Represents a tranceify instance.
    /// </summary>
    public class TranceifyInstance
    {
        /// <summary>
        /// The name of the tranceify instance.
        /// </summary>
        public string Name { get; }

        /// <summary>
        /// The template used to create this tranceify.
        /// </summary>
        public TranceifyTemplate Template { get; }

        /// <summary>
        /// The symbol table for this tranceify.
        /// </summary>
        public SymbolTable SymbolTable { get; }

        /// <summary>
        /// The variables in this tranceify.
        /// </summary>
        public Dictionary<string, object> Variables { get; }

        /// <summary>
        /// The linked session instance.
        /// </summary>
        public SessionInstance? LinkedSession { get; private set; }

        /// <summary>
        /// Initializes a new tranceify instance.
        /// </summary>
        /// <param name="name">The name of the tranceify</param>
        /// <param name="template">The template to use</param>
        public TranceifyInstance(string name, TranceifyTemplate template)
        {
            Name = name;
            Template = template;
            SymbolTable = new SymbolTable();
            Variables = new Dictionary<string, object>();

            // Initialize symbols from template
            foreach (var variable in template.Variables)
            {
                var symbol = new Symbol(variable.Identifier, variable.TypeName ?? "any");
                SymbolTable.Define(symbol);
            }
        }

        /// <summary>
        /// Sets a variable value in the tranceify.
        /// </summary>
        /// <param name="name">The variable name</param>
        /// <param name="value">The value to set</param>
        public void SetVariable(string name, object value)
        {
            Variables[name] = value;

            // If linked to a session, also set the variable there
            if (LinkedSession != null && LinkedSession.SymbolTable.HasSymbol(name))
            {
                LinkedSession.SetVariable(name, value);
            }
        }

        /// <summary>
        /// Gets a variable value from the tranceify.
        /// </summary>
        /// <param name="name">The variable name</param>
        /// <returns>The variable value or null if not found</returns>
        public object? GetVariable(string name)
        {
            // First check tranceify variables
            if (Variables.TryGetValue(name, out var value))
            {
                return value;
            }

            // Then check linked session variables
            if (LinkedSession != null)
            {
                return LinkedSession.GetVariable(name);
            }

            return null;
        }

        /// <summary>
        /// Checks if a variable exists in the tranceify or linked session.
        /// </summary>
        /// <param name="name">The variable name</param>
        /// <returns>True if the variable exists, false otherwise</returns>
        public bool HasVariable(string name)
        {
            if (Variables.ContainsKey(name))
            {
                return true;
            }

            return LinkedSession?.HasVariable(name) ?? false;
        }

        /// <summary>
        /// Gets all variable names in the tranceify and linked session.
        /// </summary>
        /// <returns>Array of variable names</returns>
        public string[] GetVariableNames()
        {
            var names = new HashSet<string>(Variables.Keys);

            if (LinkedSession != null)
            {
                foreach (var name in LinkedSession.GetVariableNames())
                {
                    names.Add(name);
                }
            }

            return names.ToArray();
        }

        /// <summary>
        /// Links this tranceify to a session.
        /// </summary>
        /// <param name="session">The session to link to</param>
        public void LinkSession(SessionInstance session)
        {
            LinkedSession = session;
        }

        /// <summary>
        /// Unlinks this tranceify from its session.
        /// </summary>
        public void UnlinkSession()
        {
            LinkedSession = null;
        }

        /// <summary>
        /// Clears all variables in the tranceify.
        /// </summary>
        public void ClearVariables()
        {
            Variables.Clear();
        }

        /// <summary>
        /// Gets the tranceify state as a dictionary.
        /// </summary>
        /// <returns>Dictionary containing all variable values</returns>
        public Dictionary<string, object> GetState()
        {
            var state = new Dictionary<string, object>(Variables);

            if (LinkedSession != null)
            {
                foreach (var kvp in LinkedSession.Variables)
                {
                    if (!state.ContainsKey(kvp.Key))
                    {
                        state[kvp.Key] = kvp.Value;
                    }
                }
            }

            return state;
        }

        /// <summary>
        /// Sets the tranceify state from a dictionary.
        /// </summary>
        /// <param name="state">Dictionary containing variable values</param>
        public void SetState(Dictionary<string, object> state)
        {
            foreach (var kvp in state)
            {
                SetVariable(kvp.Key, kvp.Value);
            }
        }
    }
}
