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

		// ===== MATHEMATISCHE FUNKTIONEN =====
		public static double Abs(double x) => Math.Abs(x);
		public static double Sin(double x) => Math.Sin(x * Math.PI / 180.0); // Grad zu Radiant
		public static double Cos(double x) => Math.Cos(x * Math.PI / 180.0);
		public static double Tan(double x) => Math.Tan(x * Math.PI / 180.0);
		public static double Sqrt(double x) => Math.Sqrt(x);
		public static double Pow(double x, double y) => Math.Pow(x, y);
		public static double Floor(double x) => Math.Floor(x);
		public static double Ceiling(double x) => Math.Ceiling(x);
		public static double Round(double x) => Math.Round(x);
		public static double Log(double x) => Math.Log(x);
		public static double Log10(double x) => Math.Log10(x);
		public static double Exp(double x) => Math.Exp(x);
		public static double Max(double x, double y) => Math.Max(x, y);
		public static double Min(double x, double y) => Math.Min(x, y);
		public static double Random() => new Random().NextDouble();
		public static int RandomInt(int min, int max) => new Random().Next(min, max + 1);

		// ===== STRING-FUNKTIONEN =====
		public static int Length(string str) => str.Length;
		public static string Substring(string str, int start, int length) => str.Substring(start, length);
		public static string ToUpper(string str) => str.ToUpper();
		public static string ToLower(string str) => str.ToLower();
		public static bool Contains(string str, string substring) => str.Contains(substring);
		public static string Replace(string str, string oldValue, string newValue) => str.Replace(oldValue, newValue);
		public static string Trim(string str) => str.Trim();
		public static string TrimStart(string str) => str.TrimStart();
		public static string TrimEnd(string str) => str.TrimEnd();
		public static int IndexOf(string str, string substring) => str.IndexOf(substring);
		public static int LastIndexOf(string str, string substring) => str.LastIndexOf(substring);
		public static string[] Split(string str, string separator) => str.Split(separator);
		public static string Join(string[] array, string separator) => string.Join(separator, array);
		public static bool StartsWith(string str, string prefix) => str.StartsWith(prefix);
		public static bool EndsWith(string str, string suffix) => str.EndsWith(suffix);
		public static string PadLeft(string str, int width, char paddingChar = ' ') => str.PadLeft(width, paddingChar);
		public static string PadRight(string str, int width, char paddingChar = ' ') => str.PadRight(width, paddingChar);

		// ===== ARRAY-FUNKTIONEN =====
		public static int ArrayLength(object[] arr) => arr.Length;
		public static object? ArrayGet(object[] arr, int index) => arr[index];
		public static void ArraySet(object[] arr, int index, object? value) => arr[index] = value;
		public static object[] ArraySlice(object[] arr, int start, int length)
		{
			var result = new object[length];
			Array.Copy(arr, start, result, 0, length);
			return result;
		}
		public static object[] ArrayConcat(object[] arr1, object[] arr2)
		{
			var result = new object[arr1.Length + arr2.Length];
			Array.Copy(arr1, 0, result, 0, arr1.Length);
			Array.Copy(arr2, 0, result, arr1.Length, arr2.Length);
			return result;
		}
		public static int ArrayIndexOf(object[] arr, object? value)
		{
			return Array.IndexOf(arr, value);
		}
		public static bool ArrayContains(object[] arr, object? value)
		{
			return Array.IndexOf(arr, value) >= 0;
		}

		// ===== KONVERTIERUNGSFUNKTIONEN =====
		public static int ToInt(object? value) => Convert.ToInt32(value);
		public static double ToDouble(object? value) => Convert.ToDouble(value);
		public static string ToString(object? value) => value?.ToString() ?? "";
		public static bool ToBoolean(object? value) => Convert.ToBoolean(value);
		public static char ToChar(object? value) => Convert.ToChar(value);

		// ===== HYPNOTISCHE SPEZIALFUNKTIONEN =====
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

		public static void HypnoticVisualization(string scene = "a peaceful garden")
		{
			Observe($"Imagine yourself in {scene}...");
			Drift(2000);
			Observe("Feel the tranquility surrounding you...");
			Drift(1500);
			Observe("Every detail becomes clearer and more vivid...");
			Drift(1500);
		}

		public static void ProgressiveRelaxation(int steps = 5)
		{
			Observe("Let's begin progressive relaxation...");
			for (int i = 1; i <= steps; i++)
			{
				Observe($"Step {i}: Relax your muscles deeper and deeper...");
				Drift(1500);
			}
			Observe("You are now completely relaxed and at peace.");
		}

		public static void HypnoticSuggestion(string suggestion)
		{
			Observe("I will now give you a powerful suggestion...");
			Drift(1000);
			Observe($"Remember this: {suggestion}");
			Drift(2000);
			Observe("This suggestion will become stronger with each passing moment.");
		}

		public static void TranceDeepening(int levels = 3)
		{
			Observe("We will now go deeper into trance...");
			for (int i = 1; i <= levels; i++)
			{
				Observe($"Level {i}: Going deeper...");
				Drift(2000);
			}
			Observe("You are now in the deepest level of trance.");
		}

		// ===== ZEIT- UND DATUMSFUNKTIONEN =====
		public static int GetCurrentTime() => (int)DateTimeOffset.UtcNow.ToUnixTimeSeconds();
		public static string GetCurrentDate() => DateTime.Now.ToString("yyyy-MM-dd");
		public static string GetCurrentTimeString() => DateTime.Now.ToString("HH:mm:ss");
		public static string GetCurrentDateTime() => DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss");

		// ===== SYSTEM-FUNKTIONEN =====
		public static void ClearScreen()
		{
			Console.Clear();
		}

		public static void Beep(int frequency = 800, int duration = 200)
		{
			Console.Beep(frequency, duration);
		}

		public static string GetEnvironmentVariable(string name)
		{
			return Environment.GetEnvironmentVariable(name) ?? "";
		}

		public static void Exit(int code = 0)
		{
			Environment.Exit(code);
		}

		// ===== DEBUGGING-FUNKTIONEN =====
		public static void DebugPrint(object? value)
		{
			Console.WriteLine($"[DEBUG] {value}");
		}

		public static void DebugPrintType(object? value)
		{
			Console.WriteLine($"[DEBUG] Type: {value?.GetType().Name ?? "null"}");
		}
	}
}
