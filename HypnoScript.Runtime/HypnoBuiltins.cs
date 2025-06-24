using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Security.Cryptography;
using System.Text.Json;
using System.Net.Http;
using System.Threading.Tasks;
using System.Threading;
using HypnoScript.Runtime.Builtins;

namespace HypnoScript.Runtime
{
	/// <summary>
	/// Stellt zentrale Builtins für HypnoScript bereit (z.B. IO, Hypnose, System, Debug).
	/// </summary>
	public static class HypnoBuiltins
	{
		/// <summary>
		/// Thread-sicherer Zufallsgenerator für nicht-kryptografische Zwecke.
		/// </summary>
		internal static readonly Random _random = new Random();
		// For cryptographic randomness, use System.Security.Cryptography.RandomNumberGenerator

		/// <summary>
		/// Eingabe-Provider für den Interpreter (kann überschrieben werden).
		/// </summary>
		public static Func<string, string> InputProvider = prompt => {
			Console.Write(prompt);
			return Console.ReadLine() ?? "";
		};
		/// <summary>
		/// Ausgabe-Consumer für den Interpreter (kann überschrieben werden).
		/// </summary>
		public static Action<object?> OutputConsumer = val => Console.WriteLine(val);

		/// <summary>
		/// Gibt einen Wert an den OutputConsumer aus.
		/// </summary>
		public static void Observe(object? value)
		{
			OutputConsumer(value);
		}

		/// <summary>
		/// Wartet synchron für die angegebene Zeit in Millisekunden.
		/// </summary>
		public static void Drift(int ms)
		{
			System.Threading.Thread.Sleep(ms);
		}

		// ===== MATHEMATISCHE FUNKTIONEN =====
		// (Moved to Builtins/MathBuiltins.cs)

		// ===== STRING-FUNKTIONEN =====
		// (Moved to Builtins/StringBuiltins.cs)

		// ===== ARRAY-FUNKTIONEN =====
		// (Moved to Builtins/ArrayBuiltins.cs)

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
#if WINDOWS
			Console.Beep(frequency, duration);
#else
			// Fallback für nicht-Windows Plattformen
			System.Threading.Thread.Sleep(duration);
#endif
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

		// ===== ERWEITERTE HYPNOTISCHE FUNKTIONEN (Runtime) =====
		public static void HypnoticBreathing(int cycles = 5)
		{
			Observe("Let's practice hypnotic breathing...");
			for (int i = 1; i <= cycles; i++)
			{
				Observe($"Cycle {i}: Breathe in deeply...");
				Drift(2000);
				Observe("Hold your breath...");
				Drift(1000);
				Observe("Now exhale slowly...");
				Drift(2000);
			}
			Observe("You are now in a state of perfect calm.");
		}

		public static void HypnoticAnchoring(string anchor = "peaceful")
		{
			Observe($"I will now create a powerful anchor for '{anchor}'...");
			Drift(1500);
			Observe("Every time you think of this anchor, you will feel this way...");
			Drift(2000);
			Observe($"Your '{anchor}' anchor is now established.");
		}

		public static void HypnoticRegression(int age = 10)
		{
			Observe($"We will now travel back in time to when you were {age} years old...");
			Drift(3000);
			Observe("You can see yourself as a child...");
			Drift(2000);
			Observe("Feel the memories and emotions of that time...");
			Drift(2000);
			Observe("You are now experiencing your past self.");
		}

		public static void HypnoticFutureProgression(int years = 5)
		{
			Observe($"Let's travel forward {years} years into your future...");
			Drift(3000);
			Observe("You can see your future self...");
			Drift(2000);
			Observe("Feel the wisdom and experience of your future...");
			Drift(2000);
			Observe("You are now connected to your future potential.");
		}

		// ===== DATEI- UND VERZEICHNIS-OPERATIONEN =====
		// (Moved to Builtins/FileBuiltins.cs)

		// ===== JSON-VERARBEITUNG =====
		public static string ToJson(object? obj)
		{
			try
			{
				return JsonSerializer.Serialize(obj, new JsonSerializerOptions { WriteIndented = true });
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
				return JsonSerializer.Deserialize<object>(json);
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
			if (n < 0) return double.NaN;
			if (n <= 1) return 1;
			double result = 1;
			for (int i = 2; i <= n; i++)
				result *= i;
			return result;
		}

		public static double GCD(double a, double b)
		{
			a = Math.Abs(a);
			b = Math.Abs(b);
			while (b != 0)
			{
				var temp = b;
				b = a % b;
				a = temp;
			}
			return a;
		}

		public static double LCM(double a, double b)
		{
			return Math.Abs(a * b) / GCD(a, b);
		}

		public static double DegreesToRadians(double degrees) => degrees * Math.PI / 180.0;
		public static double RadiansToDegrees(double radians) => radians * 180.0 / Math.PI;

		public static double Asin(double x) => Math.Asin(x) * 180.0 / Math.PI;
		public static double Acos(double x) => Math.Acos(x) * 180.0 / Math.PI;
		public static double Atan(double x) => Math.Atan(x) * 180.0 / Math.PI;
		public static double Atan2(double y, double x) => Math.Atan2(y, x) * 180.0 / Math.PI;

		// ===== ERWEITERTE STRING-FUNKTIONEN =====
		public static string Reverse(string str)
		{
			var chars = str.ToCharArray();
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
					words[i] = Capitalize(words[i]);
			}
			return string.Join(" ", words);
		}

		public static int CountOccurrences(string str, string substring)
		{
			if (string.IsNullOrEmpty(str) || string.IsNullOrEmpty(substring))
				return 0;

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
			return string.Join("", str.Where(c => !char.IsWhiteSpace(c)));
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
			return arr.Distinct().ToArray();
		}

		public static object[] ArrayFilter(object[] arr, Func<object, bool> predicate)
		{
			return arr.Where(predicate).ToArray();
		}

		// ===== KRYPTOLOGISCHE FUNKTIONEN =====
		public static string HashMD5(string input)
		{
			using (var md5 = MD5.Create())
			{
				var hash = md5.ComputeHash(Encoding.UTF8.GetBytes(input));
				return Convert.ToHexString(hash).ToLower();
			}
		}

		public static string HashSHA256(string input)
		{
			using (var sha256 = SHA256.Create())
			{
				var hash = sha256.ComputeHash(Encoding.UTF8.GetBytes(input));
				return Convert.ToHexString(hash).ToLower();
			}
		}

		public static string Base64Encode(string input)
		{
			var bytes = Encoding.UTF8.GetBytes(input);
			return Convert.ToBase64String(bytes);
		}

		public static string Base64Decode(string input)
		{
			try
			{
				var bytes = Convert.FromBase64String(input);
				return Encoding.UTF8.GetString(bytes);
			}
			catch
			{
				return "";
			}
		}

		// ===== ERWEITERTE ZEIT- UND DATUMSFUNKTIONEN =====
		public static string FormatDateTime(string format = "yyyy-MM-dd HH:mm:ss")
		{
			return DateTime.Now.ToString(format);
		}

		public static int GetDayOfWeek() => (int)DateTime.Now.DayOfWeek;
		public static int GetDayOfYear() => DateTime.Now.DayOfYear;
		public static bool IsLeapYear(int year) => DateTime.IsLeapYear(year);
		public static int GetDaysInMonth(int year, int month) => DateTime.DaysInMonth(year, month);

		// ===== ERWEITERTE SYSTEM-FUNKTIONEN =====
		public static string GetCurrentDirectory() => Environment.CurrentDirectory;
		public static string GetMachineName() => Environment.MachineName;
		public static string GetUserName() => Environment.UserName;
		public static string GetOSVersion() => Environment.OSVersion.ToString();
		public static int GetProcessorCount() => Environment.ProcessorCount;
		public static long GetWorkingSet() => Environment.WorkingSet;

		public static void PlaySound(int frequency = 800, int duration = 200)
		{
#if WINDOWS
			Console.Beep(frequency, duration);
#else
			// Fallback für nicht-Windows Plattformen
			System.Threading.Thread.Sleep(duration);
#endif
		}

		public static void Vibrate(int duration = 1000)
		{
			// Simuliere Vibration durch mehrere Beeps
			var startTime = DateTime.Now;
			while ((DateTime.Now - startTime).TotalMilliseconds < duration)
			{
#if WINDOWS
				Console.Beep(200, 50);
#else
				// Fallback für nicht-Windows Plattformen
				System.Threading.Thread.Sleep(50);
#endif
				System.Threading.Thread.Sleep(50);
			}
		}

		public static void DebugPrintMemory()
		{
			var process = System.Diagnostics.Process.GetCurrentProcess();
			Observe($"Memory Usage: {process.WorkingSet64 / 1024 / 1024} MB");
		}

		public static void DebugPrintStackTrace()
		{
			Observe("Stack Trace:");
			Observe(Environment.StackTrace);
		}

		public static void DebugPrintEnvironment()
		{
			Observe("Environment Variables:");
			foreach (var env in Environment.GetEnvironmentVariables().Cast<System.Collections.DictionaryEntry>().Take(10))
			{
				Observe($"  {env.Key} = {env.Value}");
			}
		}

		// ===== NEUE ENTERPRISE-FEATURES =====

		// Machine Learning Funktionen
		public static double LinearRegression(object[] x, object[] y)
		{
			if (x.Length != y.Length || x.Length < 2) return double.NaN;

			var n = x.Length;
			var sumX = 0.0;
			var sumY = 0.0;
			var sumXY = 0.0;
			var sumX2 = 0.0;

			for (int i = 0; i < n; i++)
			{
				var xi = Convert.ToDouble(x[i]);
				var yi = Convert.ToDouble(y[i]);
				sumX += xi;
				sumY += yi;
				sumXY += xi * yi;
				sumX2 += xi * xi;
			}

			var slope = (n * sumXY - sumX * sumY) / (n * sumX2 - sumX * sumX);
			return slope;
		}

		public static double CalculateMean(object[] values)
		{
			if (values.Length == 0) return double.NaN;
			var sum = values.Sum(v => Convert.ToDouble(v));
			return sum / values.Length;
		}

		public static double CalculateStandardDeviation(object[] values)
		{
			if (values.Length < 2) return double.NaN;
			var mean = CalculateMean(values);
			var sumSquaredDiff = values.Sum(v => Math.Pow(Convert.ToDouble(v) - mean, 2));
			return Math.Sqrt(sumSquaredDiff / (values.Length - 1));
		}

		// Datenbank-ähnliche Funktionen
		public static Dictionary<string, object> CreateRecord(string[] keys, object[] values)
		{
			var record = new Dictionary<string, object>();
			for (int i = 0; i < Math.Min(keys.Length, values.Length); i++)
			{
				record[keys[i]] = values[i];
			}
			return record;
		}

		public static object? GetRecordValue(Dictionary<string, object> record, string key)
		{
			return record.TryGetValue(key, out var value) ? value : null;
		}

		public static void SetRecordValue(Dictionary<string, object> record, string key, object value)
		{
			record[key] = value;
		}

		// Erweiterte hypnotische Funktionen
		public static void HypnoticPatternMatching(string pattern)
		{
			Observe($"I will now establish a pattern matching system for '{pattern}'...");
			Drift(2000);
			Observe("Your mind will automatically recognize this pattern...");
			Drift(1500);
			Observe("Every time you encounter this pattern, you will respond automatically...");
			Drift(2000);
			Observe($"The '{pattern}' pattern is now deeply embedded in your subconscious.");
		}

		public static void HypnoticTimeDilation(double factor = 2.0)
		{
			Observe($"I will now alter your perception of time by a factor of {factor}...");
			Drift(3000);
			Observe("Time will feel different to you now...");
			Drift(2000);
			Observe("Minutes will feel like hours, or hours like minutes...");
			Drift(2000);
			Observe("Your time perception has been successfully modified.");
		}

		public static void HypnoticMemoryEnhancement()
		{
			Observe("I will now enhance your memory capabilities...");
			Drift(2000);
			Observe("Your ability to remember and recall information will improve...");
			Drift(2000);
			Observe("You will find it easier to learn and retain new knowledge...");
			Drift(2000);
			Observe("Your memory enhancement is now active.");
		}

		public static void HypnoticCreativityBoost()
		{
			Observe("I will now unlock your creative potential...");
			Drift(2000);
			Observe("Your imagination will become more vivid and active...");
			Drift(2000);
			Observe("Creative solutions will come to you more easily...");
			Drift(2000);
			Observe("Your creativity is now enhanced.");
		}

		// Performance-Monitoring
		public static Dictionary<string, object> GetPerformanceMetrics()
		{
			var process = System.Diagnostics.Process.GetCurrentProcess();
			var metrics = new Dictionary<string, object>
			{
				["cpu_time"] = process.TotalProcessorTime.TotalMilliseconds,
				["memory_usage"] = process.WorkingSet64,
				["thread_count"] = process.Threads.Count,
				["start_time"] = process.StartTime.ToString(),
				["uptime"] = (DateTime.Now - process.StartTime).TotalSeconds
			};
			return metrics;
		}

		// Erweiterte Validierungsfunktionen
		public static bool IsValidEmail(string email)
		{
			try
			{
				var addr = new System.Net.Mail.MailAddress(email);
				return addr.Address == email;
			}
			catch
			{
				return false;
			}
		}

		public static bool IsValidUrl(string url)
		{
			return Uri.TryCreate(url, UriKind.Absolute, out _);
		}

		public static bool IsValidJson(string json)
		{
			try
			{
				JsonSerializer.Deserialize<object>(json);
				return true;
			}
			catch
			{
				return false;
			}
		}

		// Erweiterte Formatierungsfunktionen
		public static string FormatNumber(double number, int decimals = 2)
		{
			return number.ToString($"F{decimals}");
		}

		public static string FormatCurrency(double amount, string currency = "USD")
		{
			return $"{currency} {amount:F2}";
		}

		public static string FormatPercentage(double value)
		{
			return $"{value:F2}%";
		}

		// Erweiterte Array-Operationen
		public static object[] ArrayMap(object[] arr, Func<object, object> mapper)
		{
			return arr.Select(mapper).ToArray();
		}

		public static object ArrayReduce(object[] arr, Func<object, object, object> reducer, object initial)
		{
			return arr.Aggregate(initial, reducer);
		}

		public static object[] ArrayFlatten(object[] arr)
		{
			var result = new List<object>();
			foreach (var item in arr)
			{
				if (item is object[] subArray)
					result.AddRange(subArray);
				else
					result.Add(item);
			}
			return result.ToArray();
		}

		// Erweiterte String-Operationen
		public static string[] StringSplitByLength(string str, int maxLength)
		{
			var result = new List<string>();
			for (int i = 0; i < str.Length; i += maxLength)
			{
				var length = Math.Min(maxLength, str.Length - i);
				result.Add(str.Substring(i, length));
			}
			return result.ToArray();
		}

		public static string StringRotate(string str, int positions)
		{
			if (string.IsNullOrEmpty(str)) return str;
			positions = positions % str.Length;
			if (positions < 0) positions += str.Length;
			return str.Substring(positions) + str.Substring(0, positions);
		}

		public static string StringShuffle(string str)
		{
			var chars = str.ToCharArray();
			for (int i = chars.Length - 1; i > 0; i--)
			{
				int j = _random.Next(i + 1);
				var temp = chars[i];
				chars[i] = chars[j];
				chars[j] = temp;
			}
			return new string(chars);
		}

		// ===== WEITERE UTILITY-FUNKTIONEN =====
		public static double Clamp(double value, double min, double max) => Math.Max(min, Math.Min(max, value));
		public static int Sign(double value) => Math.Sign(value);
		public static bool IsEven(int value) => value % 2 == 0;
		public static bool IsOdd(int value) => value % 2 != 0;
		public static object[] ShuffleArray(object[] arr)
		{
			return arr.OrderBy(x => _random.Next()).ToArray();
		}
		public static double SumArray(object[] arr)
		{
			return arr.OfType<IConvertible>().Sum(x => Convert.ToDouble(x));
		}
		public static double AverageArray(object[] arr)
		{
			var nums = arr.OfType<IConvertible>().Select(x => Convert.ToDouble(x)).ToArray();
			return nums.Length > 0 ? nums.Average() : 0.0;
		}
		public static object[] Range(int start, int count)
		{
			return Enumerable.Range(start, count).Cast<object>().ToArray();
		}
		public static object[] Repeat(object value, int count)
		{
			return Enumerable.Repeat(value, count).ToArray();
		}
		public static void Swap(object[] arr, int i, int j)
		{
			var tmp = arr[i];
			arr[i] = arr[j];
			arr[j] = tmp;
		}
		public static object[][] ChunkArray(object[] arr, int chunkSize)
		{
			return arr.Select((x, i) => new { x, i })
				.GroupBy(x => x.i / chunkSize)
				.Select(g => g.Select(v => v.x).ToArray())
				.ToArray();
		}

		// ===== WEITERE UTILITY-FUNKTIONEN (Ergänzung) =====
		public static double ArraySum(object[] arr) => arr.OfType<IConvertible>().Sum(x => Convert.ToDouble(x));
		public static object? ArrayMin(object[] arr) => arr.Length == 0 ? null : arr.Min();
		public static object? ArrayMax(object[] arr) => arr.Length == 0 ? null : arr.Max();
		public static int ArrayCount(object[] arr, object? value) => arr.Count(x => Equals(x, value));
		public static object[] ArrayRemove(object[] arr, object? value) => arr.Where(x => !Equals(x, value)).ToArray();
		public static object[] ArrayDistinct(object[] arr) => arr.Distinct().ToArray();

		public static bool IsNullOrEmpty(string? str) => string.IsNullOrEmpty(str);
		public static string RepeatString(string str, int n) => string.Concat(Enumerable.Repeat(str, n));
		public static string ReverseWords(string str) => string.Join(" ", str.Split(' ').Reverse());
		public static string Truncate(string str, int length) => str.Length <= length ? str : str.Substring(0, length);
		public static string RemoveDigits(string str) => new string(str.Where(c => !char.IsDigit(c)).ToArray());

		public static bool IsPrime(int n)
		{
			if (n <= 1) return false;
			if (n == 2) return true;
			if (n % 2 == 0) return false;
			int boundary = (int)Math.Floor(Math.Sqrt(n));
			for (int i = 3; i <= boundary; i += 2)
				if (n % i == 0) return false;
			return true;
		}
		public static System.Numerics.BigInteger FactorialBig(int n)
		{
			System.Numerics.BigInteger result = 1;
			for (int i = 2; i <= n; i++) result *= i;
			return result;
		}
		public static string ToHex(long n) => n.ToString("X");
		public static string ToBinary(long n) => Convert.ToString(n, 2);
		public static int ParseInt(string str)
		{
			int.TryParse(str, out int result);
			return result;
		}

		public static Dictionary<string, string> GetEnvVars() => Environment.GetEnvironmentVariables().Cast<System.Collections.DictionaryEntry>().ToDictionary(e => (string)e.Key, e => e.Value as string ?? "");
		public static string GetTempPath() => System.IO.Path.GetTempPath();
		public static long GetTickCount() => Environment.TickCount64;
		public static void Sleep(int ms) => System.Threading.Thread.Sleep(ms);

		public static string AddDays(string date, int n) => DateTime.Parse(date).AddDays(n).ToString("yyyy-MM-dd");
		public static string AddMonths(string date, int n) => DateTime.Parse(date).AddMonths(n).ToString("yyyy-MM-dd");
		public static string AddYears(string date, int n) => DateTime.Parse(date).AddYears(n).ToString("yyyy-MM-dd");
		public static string ParseDate(string str) => DateTime.Parse(str).ToString("yyyy-MM-dd");

		public static bool IsArray(object? obj) => obj is object[];
		public static bool IsNumber(object? obj) => obj is sbyte or byte or short or ushort or int or uint or long or ulong or float or double or decimal;
		public static bool IsString(object? obj) => obj is string;
		public static bool IsBoolean(object? obj) => obj is bool;

		// ===== DICTIONARY-UTILITIES =====
		public static Dictionary<string, object> CreateDictionary() => new();
		public static string[] DictionaryKeys(Dictionary<string, object> dict) => dict.Keys.ToArray();
		public static object[] DictionaryValues(Dictionary<string, object> dict) => dict.Values.ToArray();
		public static bool DictionaryContainsKey(Dictionary<string, object> dict, string key) => dict.ContainsKey(key);
		public static object? DictionaryGet(Dictionary<string, object> dict, string key, object? defaultValue = null) => dict.TryGetValue(key, out var value) ? value : defaultValue;
		public static void DictionarySet(Dictionary<string, object> dict, string key, object value) => dict[key] = value;
		public static bool DictionaryRemove(Dictionary<string, object> dict, string key) => dict.Remove(key);
		public static int DictionaryCount(Dictionary<string, object> dict) => dict.Count;

		// ===== ERWEITERTE STRING-UTILITIES =====
		public static string Insert(string str, int index, string value) => str.Insert(index, value);
		public static string Remove(string str, int start, int count) => str.Remove(start, count);
		public static int Compare(string str1, string str2) => string.Compare(str1, str2);
		public static bool EqualsIgnoreCase(string str1, string str2) => string.Equals(str1, str2, StringComparison.OrdinalIgnoreCase);
		public static bool IsPalindrome(string str)
		{
			var clean = new string(str.Where(char.IsLetterOrDigit).ToArray()).ToLower();
			return clean == new string(clean.Reverse().ToArray());
		}
		public static int CountWords(string str) => str.Split(new[] { ' ', '\t', '\n', '\r' }, StringSplitOptions.RemoveEmptyEntries).Length;
		public static string ExtractNumbers(string str) => new string(str.Where(char.IsDigit).ToArray());
		public static string ExtractLetters(string str) => new string(str.Where(char.IsLetter).ToArray());

		// ===== ERWEITERTE ARRAY-UTILITIES =====
		public static object[] ArrayInsert(object[] arr, int index, object value)
		{
			if (arr == null)
			{
				Observe("Error: Array is null.");
				return Array.Empty<object>();
			}
			if (index < 0 || index > arr.Length)
			{
				Observe($"Error: Array insert index {index} out of bounds (length: {arr.Length}).");
				return arr;
			}
			var result = new object[arr.Length + 1];
			Array.Copy(arr, 0, result, 0, index);
			result[index] = value;
			Array.Copy(arr, index, result, index + 1, arr.Length - index);
			return result;
		}
		public static object[] ArrayRemoveAt(object[] arr, int index)
		{
			if (arr == null)
			{
				Observe("Error: Array is null.");
				return Array.Empty<object>();
			}
			if (index < 0 || index >= arr.Length)
			{
				Observe($"Error: Array remove index {index} out of bounds (length: {arr.Length}).");
				return arr;
			}
			var result = new object[arr.Length - 1];
			Array.Copy(arr, 0, result, 0, index);
			Array.Copy(arr, index + 1, result, index, arr.Length - index - 1);
			return result;
		}
		public static void ArrayClear(object[] arr) => Array.Clear(arr, 0, arr.Length);
		public static object[] ArrayCopy(object[] arr)
		{
			var result = new object[arr.Length];
			Array.Copy(arr, result, arr.Length);
			return result;
		}
		public static object[] ArrayResize(object[] arr, int newSize)
		{
			var result = new object[newSize];
			Array.Copy(arr, result, Math.Min(arr.Length, newSize));
			return result;
		}
		public static void ArrayFill(object[] arr, object value) => Array.Fill(arr, value);
		public static int ArrayIndexOf(object[] arr, object value, int startIndex) => Array.IndexOf(arr, value, startIndex);
		public static int ArrayLastIndexOf(object[] arr, object value) => Array.LastIndexOf(arr, value);
		public static object[] ArraySubArray(object[] arr, int start, int end)
		{
			var length = end - start + 1;
			var result = new object[length];
			Array.Copy(arr, start, result, 0, length);
			return result;
		}
		public static object[] ArrayRotate(object[] arr, int positions)
		{
			var result = new object[arr.Length];
			for (int i = 0; i < arr.Length; i++)
			{
				var newIndex = (i + positions) % arr.Length;
				if (newIndex < 0) newIndex += arr.Length;
				result[newIndex] = arr[i];
			}
			return result;
		}
		public static object[] ArrayShuffle(object[] arr, int seed)
		{
			var rnd = new Random(seed);
			return arr.OrderBy(x => rnd.Next()).ToArray();
		}
		public static object[][] ArrayPartition(object[] arr, Func<object, bool> predicate)
		{
			var trueItems = arr.Where(predicate).ToArray();
			var falseItems = arr.Where(x => !predicate(x)).ToArray();
			return new[] { trueItems, falseItems };
		}

		// ===== MATHEMATISCHE ERWEITERUNGEN =====
		public static double RoundToDecimal(double x, int decimals) => Math.Round(x, decimals);
		public static double CeilingToDecimal(double x, int decimals) => Math.Ceiling(x * Math.Pow(10, decimals)) / Math.Pow(10, decimals);
		public static double FloorToDecimal(double x, int decimals) => Math.Floor(x * Math.Pow(10, decimals)) / Math.Pow(10, decimals);
		public static double Modulo(double a, double b) => a % b;
		public static bool PowerOf2(int n) => n > 0 && (n & (n - 1)) == 0;
		public static int NextPowerOf2(int n)
		{
			if (n <= 1) return 1;
			n--;
			n |= n >> 1;
			n |= n >> 2;
			n |= n >> 4;
			n |= n >> 8;
			n |= n >> 16;
			return n + 1;
		}
		public static bool IsPerfectSquare(int n)
		{
			var sqrt = (int)Math.Sqrt(n);
			return sqrt * sqrt == n;
		}
		public static int SqrtInt(int n) => (int)Math.Sqrt(n);
		public static int GCDArray(object[] arr)
		{
			var nums = arr.OfType<IConvertible>().Select(x => Convert.ToInt32(x)).ToArray();
			if (nums.Length == 0) return 0;
			var result = nums[0];
			for (int i = 1; i < nums.Length; i++)
				result = (int)GCD(result, nums[i]);
			return result;
		}
		public static int LCMArray(object[] arr)
		{
			var nums = arr.OfType<IConvertible>().Select(x => Convert.ToInt32(x)).ToArray();
			if (nums.Length == 0) return 0;
			var result = nums[0];
			for (int i = 1; i < nums.Length; i++)
				result = (int)LCM(result, nums[i]);
			return result;
		}
		public static int SumOfDigits(long n) => n.ToString().Sum(c => c - '0');
		public static long ReverseNumber(long n) => long.Parse(new string(n.ToString().Reverse().ToArray()));

		// ===== DATEI/SYSTEM-ERWEITERUNGEN =====
		public static void FileCopy(string source, string dest)
		{
			try
			{
				System.IO.File.Copy(source, dest);
				Observe($"File copied from '{source}' to '{dest}'.");
			}
			catch (Exception ex)
			{
				Observe($"Error copying file from '{source}' to '{dest}': {ex.Message}");
			}
		}
		public static void FileMove(string source, string dest)
		{
			try
			{
				System.IO.File.Move(source, dest);
				Observe($"File moved from '{source}' to '{dest}'.");
			}
			catch (Exception ex)
			{
				Observe($"Error moving file from '{source}' to '{dest}': {ex.Message}");
			}
		}
		public static void FileDelete(string path)
		{
			try
			{
				System.IO.File.Delete(path);
				Observe($"File '{path}' deleted successfully.");
			}
			catch (Exception ex)
			{
				Observe($"Error deleting file '{path}': {ex.Message}");
			}
		}
		public static Dictionary<string, object> GetFileInfo(string path)
		{
			var info = new System.IO.FileInfo(path);
			return new Dictionary<string, object>
			{
				["Name"] = info.Name,
				["FullName"] = info.FullName,
				["Length"] = info.Length,
				["CreationTime"] = info.CreationTime.ToString("yyyy-MM-dd HH:mm:ss"),
				["LastWriteTime"] = info.LastWriteTime.ToString("yyyy-MM-dd HH:mm:ss"),
				["Extension"] = info.Extension,
				["Exists"] = info.Exists
			};
		}
		public static bool IsFileReadOnly(string path) => (System.IO.File.GetAttributes(path) & System.IO.FileAttributes.ReadOnly) != 0;
		public static void SetFileReadOnly(string path, bool readOnly)
		{
			var attributes = System.IO.File.GetAttributes(path);
			if (readOnly)
				attributes |= System.IO.FileAttributes.ReadOnly;
			else
				attributes &= ~System.IO.FileAttributes.ReadOnly;
			System.IO.File.SetAttributes(path, attributes);
		}
		public static string GetFileCreationTime(string path) => System.IO.File.GetCreationTime(path).ToString("yyyy-MM-dd HH:mm:ss");
		public static string GetFileLastWriteTime(string path) => System.IO.File.GetLastWriteTime(path).ToString("yyyy-MM-dd HH:mm:ss");
		public static double GetFileSizeMB(string path) => new System.IO.FileInfo(path).Length / (1024.0 * 1024.0);
		public static string GetFileNameWithoutExtension(string path) => System.IO.Path.GetFileNameWithoutExtension(path);
		public static string CombinePath(string path1, string path2) => System.IO.Path.Combine(path1, path2);

		// ===== NETZWERK/WEB-UTILITIES =====
		// (Moved to Builtins/NetworkBuiltins.cs)

		// ===== VALIDIERUNG/FORMATIERUNG =====
		public static bool IsValidPhoneNumber(string str)
		{
			var clean = new string(str.Where(char.IsDigit).ToArray());
			return clean.Length >= 10 && clean.Length <= 15;
		}
		public static bool IsValidCreditCard(string str)
		{
			var clean = new string(str.Where(char.IsDigit).ToArray());
			return clean.Length >= 13 && clean.Length <= 19;
		}
		public static bool IsValidPostalCode(string str)
		{
			var clean = new string(str.Where(char.IsLetterOrDigit).ToArray());
			return clean.Length >= 4 && clean.Length <= 10;
		}
		public static bool IsValidSSN(string str)
		{
			var clean = new string(str.Where(char.IsDigit).ToArray());
			return clean.Length == 9;
		}
		public static string FormatPhoneNumber(string str)
		{
			var clean = new string(str.Where(char.IsDigit).ToArray());
			if (clean.Length == 10)
				return $"({clean.Substring(0, 3)}) {clean.Substring(3, 3)}-{clean.Substring(6)}";
			return str;
		}
		public static string FormatCreditCard(string str)
		{
			var clean = new string(str.Where(char.IsDigit).ToArray());
			if (clean.Length >= 4)
				return new string('*', clean.Length - 4) + clean.Substring(clean.Length - 4);
			return str;
		}
		public static string MaskString(string str, char maskChar, int start, int end)
		{
			if (start >= str.Length || end < start) return str;
			var chars = str.ToCharArray();
			for (int i = start; i <= Math.Min(end, str.Length - 1); i++)
				chars[i] = maskChar;
			return new string(chars);
		}
		public static string GenerateRandomString(int length)
		{
			const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
			return new string(Enumerable.Repeat(chars, length).Select(s => s[_random.Next(s.Length)]).ToArray());
		}
		public static string GenerateUUID() => Guid.NewGuid().ToString();

		// ===== ZEIT/DATUM-ERWEITERUNGEN =====
		public static string GetTimeZone() => TimeZoneInfo.Local.DisplayName;
		public static string ConvertTimeZone(string date, string fromZone, string toZone)
		{
			try
			{
				var dt = DateTime.Parse(date);
				var fromTz = TimeZoneInfo.FindSystemTimeZoneById(fromZone);
				var toTz = TimeZoneInfo.FindSystemTimeZoneById(toZone);
				var converted = TimeZoneInfo.ConvertTime(dt, fromTz, toTz);
				return converted.ToString("yyyy-MM-dd HH:mm:ss");
			}
			catch { return date; }
		}
		public static int GetWeekOfYear(string date)
		{
			var dt = DateTime.Parse(date);
			var calendar = System.Globalization.CultureInfo.InvariantCulture.Calendar;
			return calendar.GetWeekOfYear(dt, System.Globalization.CalendarWeekRule.FirstFourDayWeek, DayOfWeek.Monday);
		}
		public static int GetQuarter(string date)
		{
			var dt = DateTime.Parse(date);
			return (dt.Month - 1) / 3 + 1;
		}
		public static bool IsWeekend(string date)
		{
			var dt = DateTime.Parse(date);
			return dt.DayOfWeek == DayOfWeek.Saturday || dt.DayOfWeek == DayOfWeek.Sunday;
		}
		public static bool IsBusinessDay(string date) => !IsWeekend(date);
		public static string AddBusinessDays(string date, int days)
		{
			var dt = DateTime.Parse(date);
			var added = 0;
			while (added < days)
			{
				dt = dt.AddDays(1);
				if (IsBusinessDay(dt.ToString("yyyy-MM-dd")))
					added++;
			}
			return dt.ToString("yyyy-MM-dd");
		}
		public static int GetDaysBetween(string date1, string date2)
		{
			var dt1 = DateTime.Parse(date1);
			var dt2 = DateTime.Parse(date2);
			return Math.Abs((dt2 - dt1).Days);
		}
		public static int GetAge(string birthDate)
		{
			var birth = DateTime.Parse(birthDate);
			var today = DateTime.Today;
			var age = today.Year - birth.Year;
			if (birth.Date > today.AddYears(-age)) age--;
			return age;
		}
		public static bool IsLeapDay(string date)
		{
			var dt = DateTime.Parse(date);
			return dt.Month == 2 && dt.Day == 29;
		}

		// ===== PERFORMANCE/DEBUG-UTILITIES =====
		public static long GetMemoryUsage() => GC.GetTotalMemory(false);
		public static double GetCPUUsage()
		{
			// Vereinfachte Implementierung - in der Praxis würde man PerformanceCounter verwenden
			return Environment.ProcessorCount * 10.0; // Simuliert 10% pro Core
		}
		public static Dictionary<string, object> GetProcessInfo()
		{
			var process = System.Diagnostics.Process.GetCurrentProcess();
			return new Dictionary<string, object>
			{
				["Id"] = process.Id,
				["ProcessName"] = process.ProcessName,
				["WorkingSet"] = process.WorkingSet64,
				["PrivateMemorySize"] = process.PrivateMemorySize64,
				["VirtualMemorySize"] = process.VirtualMemorySize64,
				["StartTime"] = process.StartTime.ToString("yyyy-MM-dd HH:mm:ss")
			};
		}
		public static Dictionary<string, object> GetSystemInfo()
		{
			return new Dictionary<string, object>
			{
				["MachineName"] = Environment.MachineName,
				["OSVersion"] = Environment.OSVersion.ToString(),
				["ProcessorCount"] = Environment.ProcessorCount,
				["WorkingSet"] = Environment.WorkingSet,
				["SystemPageSize"] = Environment.SystemPageSize,
				["TickCount"] = Environment.TickCount64
			};
		}
		public static double Benchmark(Func<object> func, int iterations)
		{
			var sw = System.Diagnostics.Stopwatch.StartNew();
			for (int i = 0; i < iterations; i++)
				func();
			return sw.Elapsed.TotalMilliseconds;
		}
		public static string[] GetCallStack()
		{
			return new System.Diagnostics.StackTrace(true).GetFrames()
				.Select(f => f.ToString())
				.ToArray();
		}
		public static Dictionary<string, object> GetExceptionInfo(Exception ex)
		{
			return new Dictionary<string, object>
			{
				["Message"] = ex.Message,
				["Type"] = ex.GetType().Name,
				["StackTrace"] = ex.StackTrace ?? "",
				["Source"] = ex.Source ?? ""
			};
		}
		public static void Log(string message, string level = "INFO")
		{
			var timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss");
			Console.WriteLine($"[{timestamp}] [{level}] {message}");
		}
		public static void Trace(string message) => Log(message, "TRACE");
	}
}
