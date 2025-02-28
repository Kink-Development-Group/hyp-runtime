using System;
using System.Collections.Generic;

namespace HypnoScript.Core.Symbols
{
	// Enterprise-Level: Erweiterte SymbolTable mit Debugging und Scope-Analyse
	public class SymbolTable
	{
		private readonly SymbolTable? _enclosing;
		private readonly Dictionary<string, Symbol> _symbols = new();

		public SymbolTable(SymbolTable? enclosing = null)
		{
			_enclosing = enclosing;
		}

		public bool Define(Symbol sym)
		{
			if (_symbols.ContainsKey(sym.Name))
			{
				Console.Error.WriteLine($"[SymbolTable] Symbol '{sym.Name}' is already defined in this scope.");
				return false;
			}
			_symbols[sym.Name] = sym;
			return true;
		}

		public Symbol? Resolve(string name)
		{
			if (_symbols.TryGetValue(name, out var sym))
				return sym;
			return _enclosing?.Resolve(name);
		}

		// Enterprise-Level: Methode, um den aktuellen Scope-Stack als String auszugeben
		public string DebugScope()
		{
			var result = "Current Scope:\n";
			foreach (var kvp in _symbols)
			{
				result += $" - {kvp.Key}: {kvp.Value.TypeName}\n";
			}
			if (_enclosing != null)
			{
				result += "Enclosing Scope:\n" + _enclosing.DebugScope();
			}
			return result;
		}
	}
}
