namespace HypnoScript.Runtime
{
	public static class HypnoBuiltins
	{
		// Flexible Input/Output für Interpreter
		public static Func<string, string> InputProvider = prompt => {
			Console.Write(prompt);
			return Console.ReadLine() ?? "";
		};
		public static Action<object?> OutputConsumer = val => Console.WriteLine(val);

		public static void Observe(object? value)
		{
			OutputConsumer(value);
		}

		public static void Drift(int ms)
		{
			// Synchrone Variante:
			System.Threading.Thread.Sleep(ms);
			// Oder asynchron -> bräuchte async-Methoden
		}

		// Mathematische Funktionen
		public static double Abs(double x) => Math.Abs(x);
		public static double Sin(double x) => Math.Sin(x);
		public static double Cos(double x) => Math.Cos(x);
		public static double Tan(double x) => Math.Tan(x);
		public static double Sqrt(double x) => Math.Sqrt(x);
		public static double Pow(double x, double y) => Math.Pow(x, y);
		public static double Floor(double x) => Math.Floor(x);
		public static double Ceiling(double x) => Math.Ceiling(x);
		public static double Round(double x) => Math.Round(x);

		// String-Funktionen
		public static int Length(string str) => str.Length;
		public static string Substring(string str, int start, int length) => str.Substring(start, length);
		public static string ToUpper(string str) => str.ToUpper();
		public static string ToLower(string str) => str.ToLower();
		public static bool Contains(string str, string substring) => str.Contains(substring);
		public static string Replace(string str, string oldValue, string newValue) => str.Replace(oldValue, newValue);

		// Array-Funktionen
		public static int ArrayLength(object[] arr) => arr.Length;
		public static object? ArrayGet(object[] arr, int index) => arr[index];
		public static void ArraySet(object[] arr, int index, object? value) => arr[index] = value;

		// Konvertierungsfunktionen
		public static int ToInt(object? value) => Convert.ToInt32(value);
		public static double ToDouble(object? value) => Convert.ToDouble(value);
		public static string ToString(object? value) => value?.ToString() ?? "";

		// Hypnotische Spezialfunktionen
		public static void DeepTrance(int duration = 5000)
		{
			Observe("Entering deep trance...");
			Drift(duration);
			Observe("Emerging from trance...");
		}

		public static void HypnoticCountdown(int from = 10)
		{
			for (int i = from; i > 0; i--)
			{
				Observe($"You are feeling very sleepy... {i}");
				Drift(1000);
			}
			Observe("You are now in a deep hypnotic state.");
		}

		public static void TranceInduction(string subjectName = "Subject")
		{
			Observe($"Welcome {subjectName}, you are about to enter a deep trance...");
			Drift(2000);
			Observe("Take a deep breath and relax...");
			Drift(1500);
			Observe("With each breath, you feel more and more relaxed...");
			Drift(1500);
			Observe("Your mind is becoming clear and focused...");
			Drift(1000);
		}
	}
}
