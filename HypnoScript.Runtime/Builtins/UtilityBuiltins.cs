using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Security.Cryptography;
using System.Text.Json;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Stellt Hilfsfunktionen und Konvertierungen für HypnoScript bereit.
    /// </summary>
    public static class UtilityBuiltins
    {
        /// <summary>
        /// Converts a value to an integer.
        /// </summary>
        /// <param name="value">The value to convert</param>
        /// <returns>The converted integer value</returns>
        public static int ToInt(object? value) => Convert.ToInt32(value);

        /// <summary>
        /// Converts a value to a double.
        /// </summary>
        /// <param name="value">The value to convert</param>
        /// <returns>The converted double value</returns>
        public static double ToDouble(object? value) => Convert.ToDouble(value);

        /// <summary>
        /// Converts a value to a string.
        /// </summary>
        /// <param name="value">The value to convert</param>
        /// <returns>The converted string value</returns>
        public static string ToString(object? value) => value?.ToString() ?? "";

        /// <summary>
        /// Converts a value to a boolean.
        /// </summary>
        /// <param name="value">The value to convert</param>
        /// <returns>The converted boolean value</returns>
        public static bool ToBoolean(object? value) => Convert.ToBoolean(value);

        /// <summary>
        /// Converts a value to a character.
        /// </summary>
        /// <param name="value">The value to convert</param>
        /// <returns>The converted character value</returns>
        public static char ToChar(object? value) => Convert.ToChar(value);

        /// <summary>
        /// Serializes an object to JSON format.
        /// </summary>
        /// <param name="obj">The object to serialize</param>
        /// <returns>JSON string representation</returns>
        public static string ToJson(object? obj)
        {
            try
            {
                return JsonSerializer.Serialize(obj, new JsonSerializerOptions { WriteIndented = true });
            }
            catch (Exception ex)
            {
                HypnoBuiltins.Observe($"Error serializing to JSON: {ex.Message}");
                return "{}";
            }
        }

        /// <summary>
        /// Deserializes a JSON string to an object.
        /// </summary>
        /// <param name="json">The JSON string to deserialize</param>
        /// <returns>The deserialized object</returns>
        public static object? FromJson(string json)
        {
            try
            {
                return JsonSerializer.Deserialize<object>(json);
            }
            catch (Exception ex)
            {
                HypnoBuiltins.Observe($"Error deserializing JSON: {ex.Message}");
                return null;
            }
        }

        /// <summary>
        /// Calculates the factorial of a number.
        /// </summary>
        /// <param name="n">The number to calculate factorial for</param>
        /// <returns>The factorial result</returns>
        public static double Factorial(int n)
        {
            if (n < 0) return double.NaN;
            if (n <= 1) return 1;
            double result = 1;
            for (int i = 2; i <= n; i++)
                result *= i;
            return result;
        }

        /// <summary>
        /// Calculates the greatest common divisor of two numbers.
        /// </summary>
        /// <param name="a">First number</param>
        /// <param name="b">Second number</param>
        /// <returns>The GCD</returns>
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

        /// <summary>
        /// Calculates the least common multiple of two numbers.
        /// </summary>
        /// <param name="a">First number</param>
        /// <param name="b">Second number</param>
        /// <returns>The LCM</returns>
        public static double LCM(double a, double b)
        {
            return Math.Abs(a * b) / GCD(a, b);
        }

        /// <summary>
        /// Converts degrees to radians.
        /// </summary>
        /// <param name="degrees">Angle in degrees</param>
        /// <returns>Angle in radians</returns>
        public static double DegreesToRadians(double degrees) => degrees * Math.PI / 180.0;

        /// <summary>
        /// Converts radians to degrees.
        /// </summary>
        /// <param name="radians">Angle in radians</param>
        /// <returns>Angle in degrees</returns>
        public static double RadiansToDegrees(double radians) => radians * 180.0 / Math.PI;

        /// <summary>
        /// Calculates arcsine in degrees.
        /// </summary>
        /// <param name="x">The value</param>
        /// <returns>Arcsine in degrees</returns>
        public static double Asin(double x) => Math.Asin(x) * 180.0 / Math.PI;

        /// <summary>
        /// Calculates arccosine in degrees.
        /// </summary>
        /// <param name="x">The value</param>
        /// <returns>Arccosine in degrees</returns>
        public static double Acos(double x) => Math.Acos(x) * 180.0 / Math.PI;

        /// <summary>
        /// Calculates arctangent in degrees.
        /// </summary>
        /// <param name="x">The value</param>
        /// <returns>Arctangent in degrees</returns>
        public static double Atan(double x) => Math.Atan(x) * 180.0 / Math.PI;

        /// <summary>
        /// Calculates arctangent of y/x in degrees.
        /// </summary>
        /// <param name="y">Y coordinate</param>
        /// <param name="x">X coordinate</param>
        /// <returns>Arctangent in degrees</returns>
        public static double Atan2(double y, double x) => Math.Atan2(y, x) * 180.0 / Math.PI;

        /// <summary>
        /// Reverses a string.
        /// </summary>
        /// <param name="str">The string to reverse</param>
        /// <returns>The reversed string</returns>
        public static string Reverse(string str)
        {
            var chars = str.ToCharArray();
            Array.Reverse(chars);
            return new string(chars);
        }

        /// <summary>
        /// Capitalizes the first letter of a string.
        /// </summary>
        /// <param name="str">The string to capitalize</param>
        /// <returns>The capitalized string</returns>
        public static string Capitalize(string str)
        {
            if (string.IsNullOrEmpty(str)) return str;
            return char.ToUpper(str[0]) + str.Substring(1).ToLower();
        }

        /// <summary>
        /// Converts a string to title case.
        /// </summary>
        /// <param name="str">The string to convert</param>
        /// <returns>The title case string</returns>
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

        /// <summary>
        /// Counts occurrences of a substring in a string.
        /// </summary>
        /// <param name="str">The main string</param>
        /// <param name="substring">The substring to count</param>
        /// <returns>Number of occurrences</returns>
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
        /// Removes all whitespace from a string.
        /// </summary>
        /// <param name="str">The string to process</param>
        /// <returns>String without whitespace</returns>
        public static string RemoveWhitespace(string str)
        {
            return string.Join("", str.Where(c => !char.IsWhiteSpace(c)));
        }

        /// <summary>
        /// Reverses an array.
        /// </summary>
        /// <param name="arr">The array to reverse</param>
        /// <returns>The reversed array</returns>
        public static object[] ArrayReverse(object[] arr)
        {
            var result = new object[arr.Length];
            Array.Copy(arr, result, arr.Length);
            Array.Reverse(result);
            return result;
        }

        /// <summary>
        /// Sorts an array.
        /// </summary>
        /// <param name="arr">The array to sort</param>
        /// <returns>The sorted array</returns>
        public static object[] ArraySort(object[] arr)
        {
            var result = new object[arr.Length];
            Array.Copy(arr, result, arr.Length);
            Array.Sort(result);
            return result;
        }

        /// <summary>
        /// Removes duplicate elements from an array.
        /// </summary>
        /// <param name="arr">The array to process</param>
        /// <returns>Array with unique elements</returns>
        public static object[] ArrayUnique(object[] arr)
        {
            return arr.Distinct().ToArray();
        }

        /// <summary>
        /// Filters an array using a predicate function.
        /// </summary>
        /// <param name="arr">The array to filter</param>
        /// <param name="predicate">The filter function</param>
        /// <returns>The filtered array</returns>
        public static object[] ArrayFilter(object[] arr, Func<object, bool> predicate)
        {
            return arr.Where(predicate).ToArray();
        }

        /// <summary>
        /// Creates an MD5 hash of a string.
        /// </summary>
        /// <param name="input">The string to hash</param>
        /// <returns>The MD5 hash</returns>
        public static string HashMD5(string input)
        {
            using (var md5 = MD5.Create())
            {
                var hash = md5.ComputeHash(Encoding.UTF8.GetBytes(input));
                return Convert.ToHexString(hash).ToLower();
            }
        }

        /// <summary>
        /// Creates a SHA256 hash of a string.
        /// </summary>
        /// <param name="input">The string to hash</param>
        /// <returns>The SHA256 hash</returns>
        public static string HashSHA256(string input)
        {
            using (var sha256 = SHA256.Create())
            {
                var hash = sha256.ComputeHash(Encoding.UTF8.GetBytes(input));
                return Convert.ToHexString(hash).ToLower();
            }
        }

        /// <summary>
        /// Encodes a string to Base64.
        /// </summary>
        /// <param name="input">The string to encode</param>
        /// <returns>The Base64 encoded string</returns>
        public static string Base64Encode(string input)
        {
            var bytes = Encoding.UTF8.GetBytes(input);
            return Convert.ToBase64String(bytes);
        }

        /// <summary>
        /// Decodes a Base64 string.
        /// </summary>
        /// <param name="input">The Base64 string to decode</param>
        /// <returns>The decoded string</returns>
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

        /// <summary>
        /// Clamps a value between a minimum and maximum.
        /// </summary>
        /// <param name="value">The value to clamp</param>
        /// <param name="min">Minimum value</param>
        /// <param name="max">Maximum value</param>
        /// <returns>The clamped value</returns>
        public static double Clamp(double value, double min, double max) => Math.Max(min, Math.Min(max, value));

        /// <summary>
        /// Gets the sign of a number.
        /// </summary>
        /// <param name="value">The number</param>
        /// <returns>The sign (-1, 0, or 1)</returns>
        public static int Sign(double value) => Math.Sign(value);

        /// <summary>
        /// Checks if a number is even.
        /// </summary>
        /// <param name="value">The number to check</param>
        /// <returns>True if even, false otherwise</returns>
        public static bool IsEven(int value) => value % 2 == 0;

        /// <summary>
        /// Checks if a number is odd.
        /// </summary>
        /// <param name="value">The number to check</param>
        /// <returns>True if odd, false otherwise</returns>
        public static bool IsOdd(int value) => value % 2 != 0;

        /// <summary>
        /// Shuffles an array randomly.
        /// </summary>
        /// <param name="arr">The array to shuffle</param>
        /// <returns>The shuffled array</returns>
        public static object[] ShuffleArray(object[] arr)
        {
            return arr.OrderBy(x => HypnoBuiltins._random.Next()).ToArray();
        }

        /// <summary>
        /// Calculates the sum of all numeric values in an array.
        /// </summary>
        /// <param name="arr">The array to sum</param>
        /// <returns>The sum</returns>
        public static double SumArray(object[] arr)
        {
            return arr.OfType<IConvertible>().Sum(x => Convert.ToDouble(x));
        }

        /// <summary>
        /// Calculates the average of all numeric values in an array.
        /// </summary>
        /// <param name="arr">The array to average</param>
        /// <returns>The average</returns>
        public static double AverageArray(object[] arr)
        {
            var nums = arr.OfType<IConvertible>().Select(x => Convert.ToDouble(x)).ToArray();
            return nums.Length > 0 ? nums.Average() : 0.0;
        }

        /// <summary>
        /// Creates an array of integers from start to start + count.
        /// </summary>
        /// <param name="start">Starting number</param>
        /// <param name="count">Number of elements</param>
        /// <returns>Array of integers</returns>
        public static object[] Range(int start, int count)
        {
            return Enumerable.Range(start, count).Cast<object>().ToArray();
        }

        /// <summary>
        /// Creates an array with a value repeated count times.
        /// </summary>
        /// <param name="value">The value to repeat</param>
        /// <param name="count">Number of repetitions</param>
        /// <returns>Array with repeated values</returns>
        public static object[] Repeat(object value, int count)
        {
            return Enumerable.Repeat(value, count).ToArray();
        }
    }
}
