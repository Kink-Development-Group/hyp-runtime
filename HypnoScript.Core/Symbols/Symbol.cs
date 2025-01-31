namespace HypnoScript.Core.Symbols
{
	public class Symbol
	{
		public string Name { get; }
		public string? TypeName { get; }
		public object? Value { get; set; } // Falls wir Interpretieren

		public Symbol(string name, string? typeName = null, object? value = null)
		{
			Name = name;
			TypeName = typeName;
			Value = value;
		}
	}
}
