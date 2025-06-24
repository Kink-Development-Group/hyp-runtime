using System;
using System.Collections.Generic;
using System.Linq;

namespace HypnoScript.Core.Symbols
{
	// Runtime-Level: Erweiterte SymbolTable mit Debugging und Scope-Analyse
	public class SymbolTable
	{
		private readonly SymbolTable? _enclosing;
		private readonly Dictionary<string, Symbol> _symbols = new();
		private readonly List<SymbolTable> _childScopes = new();
		public string ScopeName { get; set; } = "Global";
		public int ScopeLevel { get; }

		public SymbolTable(SymbolTable? enclosing = null, string scopeName = "Global")
		{
			_enclosing = enclosing;
			ScopeName = scopeName;
			ScopeLevel = enclosing?.ScopeLevel + 1 ?? 0;
			enclosing?._childScopes.Add(this);
		}

		public bool Define(Symbol sym)
		{
			if (_symbols.ContainsKey(sym.Name))
			{
				Console.Error.WriteLine($"[SymbolTable] Symbol '{sym.Name}' is already defined in scope '{ScopeName}'.");
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

		public Symbol? ResolveLocal(string name)
		{
			_symbols.TryGetValue(name, out var sym);
			return sym;
		}

		public bool Assign(string name, object? value)
		{
			var symbol = Resolve(name);
			if (symbol == null)
			{
				Console.Error.WriteLine($"[SymbolTable] Cannot assign to undefined symbol '{name}'.");
				return false;
			}
			if (symbol.IsConstant)
			{
				Console.Error.WriteLine($"[SymbolTable] Cannot assign to constant symbol '{name}'.");
				return false;
			}
			symbol.Value = value;
			return true;
		}

		// Runtime-Level: Methode, um den aktuellen Scope-Stack als String auszugeben
		public string DebugScope()
		{
			var result = $"Scope '{ScopeName}' (Level {ScopeLevel}):\n";
			foreach (var kvp in _symbols.OrderBy(x => x.Key))
			{
				var symbol = kvp.Value;
				var valueInfo = symbol.Value != null ? $" = {symbol.Value}" : "";
				var constInfo = symbol.IsConstant ? " (const)" : "";
				var exportInfo = symbol.IsExported ? " (exported)" : "";
				result += $"  {symbol.Kind} {kvp.Key}: {symbol.TypeName}{valueInfo}{constInfo}{exportInfo}\n";
			}
			if (_enclosing != null)
			{
				result += "\nEnclosing Scope:\n" + _enclosing.DebugScope();
			}
			return result;
		}

		// Neue Runtime-Features
		public IEnumerable<Symbol> GetAllSymbols()
		{
			return _symbols.Values.OrderBy(s => s.Name);
		}

		public IEnumerable<Symbol> GetSymbolsByKind(SymbolKind kind)
		{
			return _symbols.Values.Where(s => s.Kind == kind).OrderBy(s => s.Name);
		}

		public IEnumerable<Symbol> GetExportedSymbols()
		{
			return _symbols.Values.Where(s => s.IsExported).OrderBy(s => s.Name);
		}

		public IEnumerable<Symbol> GetConstants()
		{
			return _symbols.Values.Where(s => s.IsConstant).OrderBy(s => s.Name);
		}

		public int SymbolCount => _symbols.Count;

		public bool HasSymbol(string name)
		{
			return _symbols.ContainsKey(name);
		}

		public bool RemoveSymbol(string name)
		{
			return _symbols.Remove(name);
		}

		public void Clear()
		{
			_symbols.Clear();
		}

		// Scope-Hierarchie-Management
		public SymbolTable? GetEnclosingScope() => _enclosing;
		public IEnumerable<SymbolTable> GetChildScopes() => _childScopes;

		public SymbolTable GetRootScope()
		{
			var current = this;
			while (current._enclosing != null)
			{
				current = current._enclosing;
			}
			return current;
		}

		public int GetScopeDepth()
		{
			var depth = 0;
			var current = this;
			while (current._enclosing != null)
			{
				depth++;
				current = current._enclosing;
			}
			return depth;
		}

		// Symbol-Statistiken
		public Dictionary<SymbolKind, int> GetSymbolStatistics()
		{
			return _symbols.Values
				.GroupBy(s => s.Kind)
				.ToDictionary(g => g.Key, g => g.Count());
		}

		public string GetScopeSummary()
		{
			var stats = GetSymbolStatistics();
			var summary = $"Scope '{ScopeName}' (Level {ScopeLevel}): {SymbolCount} symbols\n";
			foreach (var stat in stats.OrderBy(s => s.Key))
			{
				summary += $"  {stat.Key}: {stat.Value}\n";
			}
			return summary;
		}

		// Symbol-Suche mit Filter
		public IEnumerable<Symbol> SearchSymbols(string pattern, SymbolKind? kind = null)
		{
			var query = _symbols.Values.AsEnumerable();

			if (kind.HasValue)
				query = query.Where(s => s.Kind == kind.Value);

			return query.Where(s => s.Name.Contains(pattern, StringComparison.OrdinalIgnoreCase))
					   .OrderBy(s => s.Name);
		}

		// Symbol-Validierung
		public List<string> ValidateSymbols()
		{
			var errors = new List<string>();

			foreach (var symbol in _symbols.Values)
			{
				if (string.IsNullOrWhiteSpace(symbol.Name))
					errors.Add($"Symbol has empty name in scope '{ScopeName}'");

				if (symbol.Kind == SymbolKind.Function && string.IsNullOrEmpty(symbol.TypeName))
					errors.Add($"Function '{symbol.Name}' has no return type");

				if (symbol.IsConstant && symbol.Value == null)
					errors.Add($"Constant '{symbol.Name}' has no initial value");
			}

			return errors;
		}

		// Scope-Merging (für Module/Imports)
		public void MergeFrom(SymbolTable other, bool overwrite = false)
		{
			foreach (var kvp in other._symbols)
			{
				if (overwrite || !_symbols.ContainsKey(kvp.Key))
				{
					_symbols[kvp.Key] = kvp.Value;
				}
			}
		}

		// Scope-Export (für Module)
		public SymbolTable ExportScope()
		{
			var exported = new SymbolTable(null, $"{ScopeName}_Exported");
			foreach (var symbol in _symbols.Values.Where(s => s.IsExported))
			{
				exported.Define(symbol);
			}
			return exported;
		}
	}
}
