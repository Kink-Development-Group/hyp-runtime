namespace HypnoScript.Core.Types
{
	public enum HypnoBaseType
	{
		Number,
		String,
		Boolean,
		Trance,      // Neuer Basistyp
		Array,       // Array-Typ
		Object,      // Objekt-Typ
		Function,    // Funktions-Typ
		Session,     // Session-Typ
		Record,      // Record/Struct-Typ
		Unknown,
		// ...
	}

	public class HypnoType
	{
		public HypnoBaseType BaseType { get; }
		public string? Name { get; }
		public HypnoType? ElementType { get; }  // Für Arrays
		public Dictionary<string, HypnoType>? Fields { get; }  // Für Records/Objects
		public List<HypnoType>? ParameterTypes { get; }  // Für Functions
		public HypnoType? ReturnType { get; }  // Für Functions

		public HypnoType(HypnoBaseType baseType, string? name = null)
		{
			BaseType = baseType;
			Name = name;
		}

		// Konstruktor für Array-Typen
		public HypnoType(HypnoType elementType) : this(HypnoBaseType.Array)
		{
			ElementType = elementType;
		}

		// Konstruktor für Record-Typen
		public HypnoType(string name, Dictionary<string, HypnoType> fields) : this(HypnoBaseType.Record, name)
		{
			Fields = fields;
		}

		// Konstruktor für Funktions-Typen
		public HypnoType(List<HypnoType> parameterTypes, HypnoType returnType) : this(HypnoBaseType.Function)
		{
			ParameterTypes = parameterTypes;
			ReturnType = returnType;
		}

		public static readonly HypnoType Number = new HypnoType(HypnoBaseType.Number);
		public static readonly HypnoType String = new HypnoType(HypnoBaseType.String);
		public static readonly HypnoType Boolean = new HypnoType(HypnoBaseType.Boolean);
		public static readonly HypnoType Unknown = new HypnoType(HypnoBaseType.Unknown);

		// Factory-Methoden für komplexe Typen
		public static HypnoType CreateArray(HypnoType elementType) => new HypnoType(elementType);
		public static HypnoType CreateRecord(string name, Dictionary<string, HypnoType> fields) => new HypnoType(name, fields);
		public static HypnoType CreateFunction(List<HypnoType> parameterTypes, HypnoType returnType) => new HypnoType(parameterTypes, returnType);

		// Typprüfungs-Methoden
		public bool IsArray => BaseType == HypnoBaseType.Array;
		public bool IsRecord => BaseType == HypnoBaseType.Record;
		public bool IsFunction => BaseType == HypnoBaseType.Function;
		public bool IsPrimitive => BaseType == HypnoBaseType.Number || BaseType == HypnoBaseType.String || BaseType == HypnoBaseType.Boolean;

		// Kompatibilitätsprüfung
		public bool IsCompatibleWith(HypnoType other)
		{
			if (BaseType != other.BaseType) return false;

			switch (BaseType)
			{
				case HypnoBaseType.Array:
					return ElementType?.IsCompatibleWith(other.ElementType!) ?? false;
				case HypnoBaseType.Record:
					if (Fields == null || other.Fields == null) return false;
					if (Fields.Count != other.Fields.Count) return false;
					foreach (var field in Fields)
					{
						if (!other.Fields.ContainsKey(field.Key)) return false;
						if (!field.Value.IsCompatibleWith(other.Fields[field.Key])) return false;
					}
					return true;
				case HypnoBaseType.Function:
					if (ParameterTypes?.Count != other.ParameterTypes?.Count) return false;
					if (!ReturnType?.IsCompatibleWith(other.ReturnType!) ?? false) return false;
					for (int i = 0; i < ParameterTypes?.Count; i++)
					{
						if (!ParameterTypes![i].IsCompatibleWith(other.ParameterTypes![i])) return false;
					}
					return true;
				default:
					return true;
			}
		}

		public override string ToString()
		{
			return BaseType switch
			{
				HypnoBaseType.Array => $"[{ElementType}]",
				HypnoBaseType.Record => $"Record<{Name}>",
				HypnoBaseType.Function => $"Function<{string.Join(",", ParameterTypes ?? new List<HypnoType>())} -> {ReturnType}>",
				_ => Name ?? BaseType.ToString()
			};
		}

		public override bool Equals(object? obj)
		{
			if (obj is not HypnoType other) return false;
			return BaseType == other.BaseType &&
				   Name == other.Name &&
				   (ElementType?.Equals(other.ElementType) ?? other.ElementType == null) &&
				   (Fields?.Count == other.Fields?.Count) &&
				   (ParameterTypes?.Count == other.ParameterTypes?.Count) &&
				   (ReturnType?.Equals(other.ReturnType) ?? other.ReturnType == null);
		}

		public override int GetHashCode()
		{
			return HashCode.Combine(BaseType, Name, ElementType, Fields, ParameterTypes, ReturnType);
		}
	}
}
