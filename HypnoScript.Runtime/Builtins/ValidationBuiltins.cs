using System;
using System.Text.RegularExpressions;

namespace HypnoScript.Runtime.Builtins
{
	/// <summary>
	/// Stellt Validierungsfunktionen für HypnoScript bereit.
	/// </summary>
	public static class ValidationBuiltins
	{
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
		/// Checks if a number is prime
		/// </summary>
		public static bool IsPrime(int n)
		{
			if (n < 2) return false;
			if (n == 2) return true;
			if (n % 2 == 0) return false;

			for (int i = 3; i <= Math.Sqrt(n); i += 2)
			{
				if (n % i == 0) return false;
			}
			return true;
		}

		/// <summary>
		/// Checks if a number is a power of 2
		/// </summary>
		public static bool PowerOf2(int n) => n > 0 && (n & (n - 1)) == 0;

		/// <summary>
		/// Checks if a number is a perfect square
		/// </summary>
		public static bool IsPerfectSquare(int n)
		{
			if (n < 0) return false;
			int root = (int)Math.Sqrt(n);
			return root * root == n;
		}

		/// <summary>
		/// Checks if a string is a palindrome
		/// </summary>
		public static bool IsPalindrome(string str)
		{
			if (string.IsNullOrEmpty(str)) return true;
			string clean = new string(str.Where(char.IsLetterOrDigit).ToArray()).ToLower();
			return clean == new string(clean.Reverse().ToArray());
		}

		/// <summary>
		/// Checks if a string is null or empty
		/// </summary>
		public static bool IsNullOrEmpty(string? str) => string.IsNullOrEmpty(str);

		/// <summary>
		/// Checks if an object is an array
		/// </summary>
		public static bool IsArray(object? obj) => obj is object[];

		/// <summary>
		/// Checks if an object is a number
		/// </summary>
		public static bool IsNumber(object? obj) => obj is sbyte or byte or short or ushort or int or uint or long or ulong or float or double or decimal;

		/// <summary>
		/// Checks if an object is a string
		/// </summary>
		public static bool IsString(object? obj) => obj is string;

		/// <summary>
		/// Checks if an object is a boolean
		/// </summary>
		public static bool IsBoolean(object? obj) => obj is bool;

		/// <summary>
		/// Checks if a number is even
		/// </summary>
		public static bool IsEven(int value) => value % 2 == 0;

		/// <summary>
		/// Checks if a number is odd
		/// </summary>
		public static bool IsOdd(int value) => value % 2 != 0;
	}
}
