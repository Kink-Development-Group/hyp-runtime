using HypnoScript.Core.Types;

namespace HypnoScript.Core.Symbols
{
	public enum SymbolKind
	{
		Variable,
		Function,
		Session,
		Record,
		Parameter,
		Label,
		Builtin,
		Module
	}

	public class Symbol
	{
		public string Name { get; }
		public string? TypeName { get; }
		public object? Value { get; set; } // Falls wir Interpretieren
		public SymbolKind Kind { get; }
		public HypnoType? Type { get; set; }
		public bool IsConstant { get; set; }
		public bool IsExported { get; set; }
		public string? Documentation { get; set; }
		public int LineNumber { get; set; }
		public int ColumnNumber { get; set; }

		public Symbol(string name, string? typeName = null, object? value = null, SymbolKind kind = SymbolKind.Variable)
		{
			Name = name;
			TypeName = typeName;
			Value = value;
			Kind = kind;
		}

		// Erweiterte Konstruktoren
		public Symbol(string name, HypnoType type, SymbolKind kind = SymbolKind.Variable) : this(name, null, null, kind)
		{
			Type = type;
		}

		public Symbol(string name, string typeName, SymbolKind kind, string? documentation = null) : this(name, typeName, null, kind)
		{
			Documentation = documentation;
		}

		// Factory-Methoden
		public static Symbol CreateVariable(string name, string typeName, object? value = null)
			=> new Symbol(name, typeName, value, SymbolKind.Variable);

		public static Symbol CreateFunction(string name, string returnType, string? documentation = null)
			=> new Symbol(name, returnType, null, SymbolKind.Function) { Documentation = documentation };

		public static Symbol CreateSession(string name, string? documentation = null)
			=> new Symbol(name, "session", null, SymbolKind.Session) { Documentation = documentation };

		public static Symbol CreateRecord(string name, string? documentation = null)
			=> new Symbol(name, "record", null, SymbolKind.Record) { Documentation = documentation };

		public static Symbol CreateBuiltin(string name, string returnType, string? documentation = null)
			=> new Symbol(name, returnType, null, SymbolKind.Builtin) { Documentation = documentation };

		public static Symbol CreateLabel(string name)
			=> new Symbol(name, null, null, SymbolKind.Label);

		// Hilfsmethoden
		public bool IsFunction => Kind == SymbolKind.Function || Kind == SymbolKind.Builtin;
		public bool IsType => Kind == SymbolKind.Session || Kind == SymbolKind.Record;
		public bool IsVariable => Kind == SymbolKind.Variable || Kind == SymbolKind.Parameter;

		public override string ToString()
		{
			var typeInfo = Type?.ToString() ?? TypeName ?? "unknown";
			var kindInfo = Kind.ToString().ToLower();
			return $"{kindInfo} {Name}: {typeInfo}";
		}

		public string GetFullDescription()
		{
			var result = $"{Kind} '{Name}'";
			if (Type != null) result += $" of type {Type}";
			else if (TypeName != null) result += $" of type {TypeName}";

			if (IsConstant) result += " (constant)";
			if (IsExported) result += " (exported)";
			if (Documentation != null) result += $" - {Documentation}";

			return result;
		}
	}
}
