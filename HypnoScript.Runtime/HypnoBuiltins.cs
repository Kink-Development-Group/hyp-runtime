namespace HypnoScript.Runtime
{
	public static class HypnoBuiltins
	{
		public static void Observe(object? value)
		{
			Console.WriteLine(value);
		}

		public static void Drift(int ms)
		{
			// Synchrone Variante:
			System.Threading.Thread.Sleep(ms);
			// Oder asynchron -> bräuchte async-Methoden
		}
	}
}
