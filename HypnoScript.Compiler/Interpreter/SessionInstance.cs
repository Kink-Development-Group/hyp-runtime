using System.Collections.Generic;
using HypnoScript.LexerParser.AST;

namespace HypnoScript.Compiler.Interpreter
{
    // Repräsentiert eine Instanz einer Session (ähnlich einer Klasse)
    public class SessionInstance(string name)
    {
        public string Name { get; set; } = name;
        public Dictionary<string, object?> Fields { get; } = [];
        public Dictionary<string, FunctionDeclNode> Methods { get; } = [];
    }
}
