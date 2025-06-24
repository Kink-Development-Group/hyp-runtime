using System;
using System.Linq;
using System.Text.RegularExpressions;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// String manipulation built-in functions for HypnoScript
    /// </summary>
    public static class StringBuiltins
    {
        /// <summary>Länge eines Strings.</summary>
        public static int Length(string str) => str.Length;
        /// <summary>Substring ab Start mit Länge.</summary>
        public static string Substring(string str, int start, int length) => str.Substring(start, length);
        /// <summary>Wandelt in Großbuchstaben um.</summary>
        public static string ToUpper(string str) => str.ToUpper();
        /// <summary>Wandelt in Kleinbuchstaben um.</summary>
        public static string ToLower(string str) => str.ToLower();
        /// <summary>Prüft, ob ein String einen Teilstring enthält.</summary>
        public static bool Contains(string str, string substring) => str.Contains(substring);
        /// <summary>Ersetzt alle Vorkommen eines Teilstrings.</summary>
        public static string Replace(string str, string oldValue, string newValue) => str.Replace(oldValue, newValue);
        /// <summary>Trimmt Leerzeichen am Anfang und Ende.</summary>
        public static string Trim(string str) => str.Trim();
        /// <summary>Trimmt Leerzeichen am Anfang.</summary>
        public static string TrimStart(string str) => str.TrimStart();
        /// <summary>Trimmt Leerzeichen am Ende.</summary>
        public static string TrimEnd(string str) => str.TrimEnd();
        /// <summary>Index des ersten Vorkommens eines Teilstrings.</summary>
        public static int IndexOf(string str, string substring) => str.IndexOf(substring);
        /// <summary>Index des letzten Vorkommens eines Teilstrings.</summary>
        public static int LastIndexOf(string str, string substring) => str.LastIndexOf(substring);
        /// <summary>Teilt einen String anhand eines Separators.</summary>
        public static string[] Split(string str, string separator) => str.Split(separator);
        /// <summary>Fügt ein String-Array mit Separator zusammen.</summary>
        public static string Join(string[] array, string separator) => string.Join(separator, array);
        /// <summary>Prüft, ob ein String mit Präfix beginnt.</summary>
        public static bool StartsWith(string str, string prefix) => str.StartsWith(prefix);
        /// <summary>Prüft, ob ein String mit Suffix endet.</summary>
        public static bool EndsWith(string str, string suffix) => str.EndsWith(suffix);
        /// <summary>Links-Auffüllen auf Breite mit Zeichen.</summary>
        public static string PadLeft(string str, int width, char paddingChar = ' ') => str.PadLeft(width, paddingChar);
        /// <summary>Rechts-Auffüllen auf Breite mit Zeichen.</summary>
        public static string PadRight(string str, int width, char paddingChar = ' ') => str.PadRight(width, paddingChar);
        /// <summary>Fügt einen Wert an einer bestimmten Position in einen String ein.</summary>
        public static string Insert(string str, int index, string value)
        {
            if (str == null || value == null) return str ?? string.Empty;
            if (index < 0 || index > str.Length) return str;
            return str.Insert(index, value);
        }
        /// <summary>Entfernt eine bestimmte Anzahl von Zeichen ab einer Position.</summary>
        public static string Remove(string str, int start, int count)
        {
            if (str == null) return string.Empty;
            if (start < 0 || count < 0 || start + count > str.Length) return str;
            return str.Remove(start, count);
        }
        /// <summary>Vergleicht zwei Strings lexikografisch.</summary>
        public static int Compare(string str1, string str2)
        {
            if (str1 == null && str2 == null) return 0;
            if (str1 == null) return -1;
            if (str2 == null) return 1;
            return string.Compare(str1, str2, StringComparison.Ordinal);
        }
        /// <summary>Vergleicht zwei Strings ohne Beachtung der Groß-/Kleinschreibung.</summary>
        public static bool EqualsIgnoreCase(string str1, string str2)
        {
            if (str1 == null || str2 == null) return false;
            return string.Equals(str1, str2, StringComparison.OrdinalIgnoreCase);
        }
        /// <summary>Prüft, ob ein String ein Palindrom ist.</summary>
        public static bool IsPalindrome(string str)
        {
            if (string.IsNullOrEmpty(str)) return false;
            int len = str.Length;
            for (int i = 0; i < len / 2; i++)
                if (str[i] != str[len - i - 1]) return false;
            return true;
        }
        /// <summary>Zählt die Wörter in einem String.</summary>
        public static int CountWords(string str)
        {
            if (string.IsNullOrWhiteSpace(str)) return 0;
            return str.Split(new[] { ' ', '\t', '\n', '\r' }, StringSplitOptions.RemoveEmptyEntries).Length;
        }
        /// <summary>Extrahiert alle Ziffern aus einem String.</summary>
        public static string ExtractNumbers(string str)
        {
            if (str == null) return string.Empty;
            return new string(str.Where(char.IsDigit).ToArray());
        }
        /// <summary>Extrahiert alle Buchstaben aus einem String.</summary>
        public static string ExtractLetters(string str)
        {
            if (str == null) return string.Empty;
            return new string(str.Where(char.IsLetter).ToArray());
        }
        /// <summary>
        /// Reverses a string
        /// </summary>
        public static string Reverse(string str)
        {
            if (string.IsNullOrEmpty(str)) return str;
            return new string(str.Reverse().ToArray());
        }
        /// <summary>
        /// Capitalizes the first letter of a string
        /// </summary>
        public static string Capitalize(string str)
        {
            if (string.IsNullOrEmpty(str)) return str;
            return char.ToUpper(str[0]) + str.Substring(1).ToLower();
        }
        /// <summary>
        /// Converts string to title case
        /// </summary>
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
        /// <summary>
        /// Counts occurrences of a substring in a string
        /// </summary>
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
        /// <summary>
        /// Removes all whitespace from a string
        /// </summary>
        public static string RemoveWhitespace(string str)
        {
            if (string.IsNullOrEmpty(str)) return str;
            return new string(str.Where(c => !char.IsWhiteSpace(c)).ToArray());
        }
        /// <summary>
        /// Checks if a string is null or empty
        /// </summary>
        public static bool IsNullOrEmpty(string? str) => string.IsNullOrEmpty(str);
        /// <summary>
        /// Repeats a string n times
        /// </summary>
        public static string RepeatString(string str, int n) => string.Concat(Enumerable.Repeat(str, n));
        /// <summary>
        /// Reverses the order of words in a string
        /// </summary>
        public static string ReverseWords(string str) => string.Join(" ", str.Split(' ').Reverse());
        /// <summary>
        /// Truncates a string to specified length
        /// </summary>
        public static string Truncate(string str, int length) => str.Length <= length ? str : str.Substring(0, length);
        /// <summary>
        /// Removes all digits from a string
        /// </summary>
        public static string RemoveDigits(string str) => new string(str.Where(c => !char.IsDigit(c)).ToArray());
        /// <summary>
        /// Splits a string by length
        /// </summary>
        public static string[] StringSplitByLength(string str, int maxLength)
        {
            if (string.IsNullOrEmpty(str) || maxLength <= 0) return new string[0];

            var result = new List<string>();
            for (int i = 0; i < str.Length; i += maxLength)
            {
                int length = Math.Min(maxLength, str.Length - i);
                result.Add(str.Substring(i, length));
            }
            return result.ToArray();
        }
        /// <summary>
        /// Rotates characters in a string
        /// </summary>
        public static string StringRotate(string str, int positions)
        {
            if (string.IsNullOrEmpty(str)) return str;

            positions = positions % str.Length;
            if (positions < 0) positions += str.Length;

            return str.Substring(positions) + str.Substring(0, positions);
        }
        /// <summary>
        /// Shuffles characters in a string
        /// </summary>
        public static string StringShuffle(string str)
        {
            if (string.IsNullOrEmpty(str)) return str;

            var chars = str.ToCharArray();
            var random = new Random();

            for (int i = chars.Length - 1; i > 0; i--)
            {
                int j = random.Next(i + 1);
                char temp = chars[i];
                chars[i] = chars[j];
                chars[j] = temp;
            }

            return new string(chars);
        }
        /// <summary>
        /// Validates email format
        /// </summary>
        public static bool IsValidEmail(string email)
        {
            if (string.IsNullOrEmpty(email)) return false;

            try
            {
                var regex = new Regex(@"^[^@\s]+@[^@\s]+\.[^@\s]+$");
                return regex.IsMatch(email);
            }
            catch
            {
                return false;
            }
        }
        /// <summary>
        /// Validates URL format
        /// </summary>
        public static bool IsValidUrl(string url)
        {
            return Uri.TryCreate(url, UriKind.Absolute, out _);
        }
        /// <summary>
        /// Validates JSON format
        /// </summary>
        public static bool IsValidJson(string json)
        {
            if (string.IsNullOrEmpty(json)) return false;

            try
            {
                using var doc = System.Text.Json.JsonDocument.Parse(json);
                return true;
            }
            catch
            {
                return false;
            }
        }
        /// <summary>
        /// Formats a number with specified decimal places
        /// </summary>
        public static string FormatNumber(double number, int decimals = 2)
        {
            return number.ToString($"F{decimals}");
        }
        /// <summary>
        /// Formats a number as currency
        /// </summary>
        public static string FormatCurrency(double amount, string currency = "USD")
        {
            return $"{currency} {amount:F2}";
        }
        /// <summary>
        /// Formats a number as percentage
        /// </summary>
        public static string FormatPercentage(double value)
        {
            return $"{value:F2}%";
        }
        /// <summary>
        /// Validates phone number format
        /// </summary>
        public static bool IsValidPhoneNumber(string str)
        {
            if (string.IsNullOrEmpty(str)) return false;
            var regex = new Regex(@"^[\+]?[1-9][\d]{0,15}$");
            return regex.IsMatch(str.Replace(" ", "").Replace("-", "").Replace("(", "").Replace(")", ""));
        }
        /// <summary>
        /// Validates credit card format
        /// </summary>
        public static bool IsValidCreditCard(string str)
        {
            if (string.IsNullOrEmpty(str)) return false;
            var regex = new Regex(@"^[\d\s\-]{13,19}$");
            return regex.IsMatch(str);
        }
        /// <summary>
        /// Validates postal code format
        /// </summary>
        public static bool IsValidPostalCode(string str)
        {
            if (string.IsNullOrEmpty(str)) return false;
            var regex = new Regex(@"^[\d\w\s\-]{3,10}$");
            return regex.IsMatch(str);
        }
        /// <summary>
        /// Validates SSN format
        /// </summary>
        public static bool IsValidSSN(string str)
        {
            if (string.IsNullOrEmpty(str)) return false;
            var regex = new Regex(@"^\d{3}-?\d{2}-?\d{4}$");
            return regex.IsMatch(str);
        }
        /// <summary>
        /// Formats phone number
        /// </summary>
        public static string FormatPhoneNumber(string str)
        {
            if (string.IsNullOrEmpty(str)) return str;
            var digits = ExtractNumbers(str);
            if (digits.Length == 10)
                return $"({digits.Substring(0, 3)}) {digits.Substring(3, 3)}-{digits.Substring(6)}";
            return str;
        }
        /// <summary>
        /// Formats credit card number
        /// </summary>
        public static string FormatCreditCard(string str)
        {
            if (string.IsNullOrEmpty(str)) return str;
            var digits = ExtractNumbers(str);
            if (digits.Length >= 13 && digits.Length <= 19)
            {
                var groups = new List<string>();
                for (int i = 0; i < digits.Length; i += 4)
                {
                    groups.Add(digits.Substring(i, Math.Min(4, digits.Length - i)));
                }
                return string.Join(" ", groups);
            }
            return str;
        }
        /// <summary>
        /// Masks part of a string
        /// </summary>
        public static string MaskString(string str, char maskChar, int start, int end)
        {
            if (string.IsNullOrEmpty(str) || start < 0 || end > str.Length || start >= end)
                return str;

            return str.Substring(0, start) + new string(maskChar, end - start) + str.Substring(end);
        }
        /// <summary>
        /// Generates a random string
        /// </summary>
        public static string GenerateRandomString(int length)
        {
            const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            var random = new Random();
            return new string(Enumerable.Repeat(chars, length).Select(s => s[random.Next(s.Length)]).ToArray());
        }
    }
}
