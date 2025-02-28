namespace HypnoScript.Core.Types
{
	public enum HypnoBaseType
	{
		Number,
		String,
		Boolean,
		Trance,      // Neuer Basistyp
		Unknown,
		// ...
	}

	public class HypnoType
	{
		public HypnoBaseType BaseType { get; }
		public string? Name { get; }

		public HypnoType(HypnoBaseType baseType, string? name = null)
		{
			BaseType = baseType;
			Name = name;
		}

		public static readonly HypnoType Number = new HypnoType(HypnoBaseType.Number);
		public static readonly HypnoType String = new HypnoType(HypnoBaseType.String);
		public static readonly HypnoType Boolean = new HypnoType(HypnoBaseType.Boolean);
		public static readonly HypnoType Unknown = new HypnoType(HypnoBaseType.Unknown);

		public override string ToString()
			=> Name ?? BaseType.ToString();
	}
}
