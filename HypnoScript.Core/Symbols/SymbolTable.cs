namespace HypnoScript.Core.Symbols
{
	// Erweiterung: Unterstützung für innere Scopes
	public class SymbolTable
	{
		private readonly SymbolTable? _enclosing;
		private Dictionary<string, Symbol> _symbols = new();

		public SymbolTable(SymbolTable? enclosing = null)
		{
			_enclosing = enclosing;
		}

		public bool Define(Symbol sym)
		{
			if (_symbols.ContainsKey(sym.Name))
				return false;
			_symbols[sym.Name] = sym;
			return true;
		}

		public Symbol? Resolve(string name)
		{
			if (_symbols.TryGetValue(name, out var sym))
				return sym;
			return _enclosing?.Resolve(name);
		}
	}
}
