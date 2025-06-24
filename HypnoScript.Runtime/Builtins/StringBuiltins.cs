using System;
using System.Linq;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// String Builtins für HypnoScript (ausgelagert aus HypnoBuiltins)
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
    }
}
