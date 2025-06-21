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

		// ===== ERWEITERTE HYPNOTISCHE FUNKTIONEN =====
		public static void HypnoticBreathing(int cycles = 5)
		{
			Observe("Let's practice hypnotic breathing...");
			for (int i = 1; i <= cycles; i++)
			{
				Observe($"Cycle {i}: Breathe in deeply...");
				Drift(2000);
				Observe("Hold...");
				Drift(1000);
				Observe("Breathe out slowly...");
				Drift(2000);
			}
			Observe("You are now in a state of deep relaxation.");
		}

		public static void HypnoticAnchoring(string anchor = "peaceful")
		{
			Observe($"I will now create a powerful {anchor} anchor...");
			Drift(1500);
			Observe("Every time you hear this word, you will feel this calm...");
			Drift(2000);
			Observe($"Your {anchor} anchor is now established.");
		}

		public static void HypnoticRegression(int age = 10)
		{
			Observe("We will now travel back in time...");
			Drift(2000);
			Observe($"You are now {age} years old...");
			Drift(1500);
			Observe("Feel the memories and experiences of that time...");
			Drift(2000);
		}

		public static void HypnoticFutureProgression(int years = 5)
		{
			Observe("We will now travel into the future...");
			Drift(2000);
			Observe($"You are now {years} years in the future...");
			Drift(1500);
			Observe("See your goals achieved and dreams realized...");
			Drift(2000);
		}

		// ===== DATEI-OPERATIONEN =====
		public static bool FileExists(string path)
		{
			return File.Exists(path);
		}

		public static string ReadFile(string path)
		{
			try
			{
				return File.ReadAllText(path);
			}
			catch (Exception ex)
			{
				Observe($"Error reading file: {ex.Message}");
				return "";
			}
		}

		public static void WriteFile(string path, string content)
		{
			try
			{
				File.WriteAllText(path, content);
				Observe($"File written successfully: {path}");
			}
			catch (Exception ex)
			{
				Observe($"Error writing file: {ex.Message}");
			}
		}

		public static void AppendFile(string path, string content)
		{
			try
			{
				File.AppendAllText(path, content);
				Observe($"Content appended to: {path}");
			}
			catch (Exception ex)
			{
				Observe($"Error appending to file: {ex.Message}");
			}
		}

		public static string[] ReadLines(string path)
		{
			try
			{
				return File.ReadAllLines(path);
			}
			catch (Exception ex)
			{
				Observe($"Error reading file lines: {ex.Message}");
				return new string[0];
			}
		}

		public static void WriteLines(string path, string[] lines)
		{
			try
			{
				File.WriteAllLines(path, lines);
				Observe($"Lines written to: {path}");
			}
			catch (Exception ex)
			{
				Observe($"Error writing lines to file: {ex.Message}");
			}
		}

		public static long GetFileSize(string path)
		{
			try
			{
				var fileInfo = new FileInfo(path);
				return fileInfo.Length;
			}
			catch (Exception ex)
			{
				Observe($"Error getting file size: {ex.Message}");
				return -1;
			}
		}

		public static string GetFileExtension(string path)
		{
			return Path.GetExtension(path);
		}

		public static string GetFileName(string path)
		{
			return Path.GetFileName(path);
		}

		public static string GetDirectoryName(string path)
		{
			return Path.GetDirectoryName(path) ?? "";
		}

		// ===== VERZEICHNIS-OPERATIONEN =====
		public static bool DirectoryExists(string path)
		{
			return Directory.Exists(path);
		}

		public static void CreateDirectory(string path)
		{
			try
			{
				Directory.CreateDirectory(path);
				Observe($"Directory created: {path}");
			}
			catch (Exception ex)
			{
				Observe($"Error creating directory: {ex.Message}");
			}
		}

		public static string[] GetFiles(string path, string pattern = "*")
		{
			try
			{
				return Directory.GetFiles(path, pattern);
			}
			catch (Exception ex)
			{
				Observe($"Error getting files: {ex.Message}");
				return new string[0];
			}
		}

		public static string[] GetDirectories(string path)
		{
			try
			{
				return Directory.GetDirectories(path);
			}
			catch (Exception ex)
			{
				Observe($"Error getting directories: {ex.Message}");
				return new string[0];
			}
		}

		// ===== JSON-VERARBEITUNG =====
		public static string ToJson(object? obj)
		{
			try
			{
				return System.Text.Json.JsonSerializer.Serialize(obj);
			}
			catch (Exception ex)
			{
				Observe($"Error serializing to JSON: {ex.Message}");
				return "{}";
			}
		}

		public static object? FromJson(string json)
		{
			try
			{
				return System.Text.Json.JsonSerializer.Deserialize<object>(json);
			}
			catch (Exception ex)
			{
				Observe($"Error deserializing JSON: {ex.Message}");
				return null;
			}
		}

		// ===== ERWEITERTE MATHEMATISCHE FUNKTIONEN =====
		public static double Factorial(int n)
		{
			if (n <= 1) return 1;
			double result = 1;
			for (int i = 2; i <= n; i++)
			{
				result *= i;
			}
			return result;
		}

		public static double GCD(double a, double b)
		{
			while (b != 0)
			{
				double temp = b;
				b = a % b;
				a = temp;
			}
			return a;
		}

		public static double LCM(double a, double b)
		{
			return Math.Abs(a * b) / GCD(a, b);
		}

		public static double DegreesToRadians(double degrees)
		{
			return degrees * Math.PI / 180.0;
		}

		public static double RadiansToDegrees(double radians)
		{
			return radians * 180.0 / Math.PI;
		}

		public static double Asin(double x) => Math.Asin(x) * 180.0 / Math.PI;
		public static double Acos(double x) => Math.Acos(x) * 180.0 / Math.PI;
		public static double Atan(double x) => Math.Atan(x) * 180.0 / Math.PI;
		public static double Atan2(double y, double x) => Math.Atan2(y, x) * 180.0 / Math.PI;

		// ===== ERWEITERTE STRING-FUNKTIONEN =====
		public static string Reverse(string str)
		{
			char[] chars = str.ToCharArray();
			Array.Reverse(chars);
			return new string(chars);
		}

		public static string Capitalize(string str)
		{
			if (string.IsNullOrEmpty(str)) return str;
			return char.ToUpper(str[0]) + str.Substring(1).ToLower();
		}

		public static string TitleCase(string str)
		{
			if (string.IsNullOrEmpty(str)) return str;
			var words = str.Split(' ');
			for (int i = 0; i < words.Length; i++)
			{
				if (!string.IsNullOrEmpty(words[i]))
				{
					words[i] = Capitalize(words[i]);
				}
			}
			return string.Join(" ", words);
		}

		public static int CountOccurrences(string str, string substring)
		{
			int count = 0;
			int index = 0;
			while ((index = str.IndexOf(substring, index)) != -1)
			{
				count++;
				index += substring.Length;
			}
			return count;
		}

		public static string RemoveWhitespace(string str)
		{
			return str.Replace(" ", "").Replace("\t", "").Replace("\n", "").Replace("\r", "");
		}

		// ===== ERWEITERTE ARRAY-FUNKTIONEN =====
		public static object[] ArrayReverse(object[] arr)
		{
			var result = new object[arr.Length];
			Array.Copy(arr, result, arr.Length);
			Array.Reverse(result);
			return result;
		}

		public static object[] ArraySort(object[] arr)
		{
			var result = new object[arr.Length];
			Array.Copy(arr, result, arr.Length);
			Array.Sort(result);
			return result;
		}

		public static object[] ArrayUnique(object[] arr)
		{
			var unique = new List<object>();
			foreach (var item in arr)
			{
				if (!unique.Contains(item))
				{
					unique.Add(item);
				}
			}
			return unique.ToArray();
		}

		public static object[] ArrayFilter(object[] arr, Func<object, bool> predicate)
		{
			var filtered = new List<object>();
			foreach (var item in arr)
			{
				if (predicate(item))
				{
					filtered.Add(item);
				}
			}
			return filtered.ToArray();
		}

		// ===== KRYPTOLOGISCHE FUNKTIONEN =====
		public static string HashMD5(string input)
		{
			using (var md5 = System.Security.Cryptography.MD5.Create())
			{
				byte[] inputBytes = System.Text.Encoding.ASCII.GetBytes(input);
				byte[] hashBytes = md5.ComputeHash(inputBytes);
				return Convert.ToHexString(hashBytes).ToLower();
			}
		}

		public static string HashSHA256(string input)
		{
			using (var sha256 = System.Security.Cryptography.SHA256.Create())
			{
				byte[] inputBytes = System.Text.Encoding.ASCII.GetBytes(input);
				byte[] hashBytes = sha256.ComputeHash(inputBytes);
				return Convert.ToHexString(hashBytes).ToLower();
			}
		}

		public static string Base64Encode(string input)
		{
			byte[] bytes = System.Text.Encoding.UTF8.GetBytes(input);
			return Convert.ToBase64String(bytes);
		}

		public static string Base64Decode(string input)
		{
			try
			{
				byte[] bytes = Convert.FromBase64String(input);
				return System.Text.Encoding.UTF8.GetString(bytes);
			}
			catch (Exception ex)
			{
				Observe($"Error decoding Base64: {ex.Message}");
				return "";
			}
		}

		// ===== ERWEITERTE ZEIT-FUNKTIONEN =====
		public static string FormatDateTime(string format = "yyyy-MM-dd HH:mm:ss")
		{
			return DateTime.Now.ToString(format);
		}

		public static int GetDayOfWeek()
		{
			return (int)DateTime.Now.DayOfWeek;
		}

		public static int GetDayOfYear()
		{
			return DateTime.Now.DayOfYear;
		}

		public static bool IsLeapYear(int year)
		{
			return DateTime.IsLeapYear(year);
		}

		public static int GetDaysInMonth(int year, int month)
		{
			return DateTime.DaysInMonth(year, month);
		}

		// ===== ERWEITERTE SYSTEM-FUNKTIONEN =====
		public static string GetCurrentDirectory()
		{
			return Environment.CurrentDirectory;
		}

		public static string GetMachineName()
		{
			return Environment.MachineName;
		}

		public static string GetUserName()
		{
			return Environment.UserName;
		}

		public static string GetOSVersion()
		{
			return Environment.OSVersion.ToString();
		}

		public static int GetProcessorCount()
		{
			return Environment.ProcessorCount;
		}

		public static long GetWorkingSet()
		{
			return Environment.WorkingSet;
		}

		public static void PlaySound(int frequency = 800, int duration = 200)
		{
			Console.Beep(frequency, duration);
		}

		public static void Vibrate(int duration = 1000)
		{
			// Simulierte Vibration durch mehrere Beeps
			for (int i = 0; i < duration / 100; i++)
			{
				Console.Beep(400, 50);
				Thread.Sleep(50);
			}
		}

		// ===== ERWEITERTE DEBUGGING-FUNKTIONEN =====
		public static void DebugPrintMemory()
		{
			var process = System.Diagnostics.Process.GetCurrentProcess();
			Console.WriteLine($"[DEBUG] Memory Usage: {process.WorkingSet64 / 1024 / 1024} MB");
		}

		public static void DebugPrintStackTrace()
		{
			Console.WriteLine($"[DEBUG] Stack Trace: {Environment.StackTrace}");
		}

		public static void DebugPrintEnvironment()
		{
			Console.WriteLine($"[DEBUG] Current Directory: {Environment.CurrentDirectory}");
			Console.WriteLine($"[DEBUG] Machine Name: {Environment.MachineName}");
			Console.WriteLine($"[DEBUG] User Name: {Environment.UserName}");
			Console.WriteLine($"[DEBUG] OS Version: {Environment.OSVersion}");
		}
	}
}
