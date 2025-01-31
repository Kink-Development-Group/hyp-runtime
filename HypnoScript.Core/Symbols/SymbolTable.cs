namespace HypnoScript.Core.Symbols
{
	public class SymbolTable
	{
		private Dictionary<string, Symbol> _symbols = new();

		public bool Define(Symbol sym)
		{
			if (_symbols.ContainsKey(sym.Name))
				return false;
			_symbols[sym.Name] = sym;
			return true;
		}

		public Symbol? Resolve(string name)
		{
			_symbols.TryGetValue(name, out var sym);
			return sym;
		}
	}
}
