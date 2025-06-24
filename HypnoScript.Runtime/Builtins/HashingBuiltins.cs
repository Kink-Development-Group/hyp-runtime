using System;
using System.Security.Cryptography;
using System.Text;

namespace HypnoScript.Runtime.Builtins
{
	/// <summary>
	/// Stellt Hashing- und Encoding-Funktionen für HypnoScript bereit.
	/// </summary>
	public static class HashingBuiltins
	{
		/// <summary>
		/// Creates MD5 hash of input string
		/// </summary>
		public static string HashMD5(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";

			using var md5 = MD5.Create();
			var inputBytes = Encoding.UTF8.GetBytes(input);
			var hashBytes = md5.ComputeHash(inputBytes);

			return Convert.ToHexString(hashBytes).ToLower();
		}

		/// <summary>
		/// Creates SHA256 hash of input string
		/// </summary>
		public static string HashSHA256(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";

			using var sha256 = SHA256.Create();
			var inputBytes = Encoding.UTF8.GetBytes(input);
			var hashBytes = sha256.ComputeHash(inputBytes);

			return Convert.ToHexString(hashBytes).ToLower();
		}

		/// <summary>
		/// Creates SHA512 hash of input string
		/// </summary>
		public static string HashSHA512(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";

			using var sha512 = SHA512.Create();
			var inputBytes = Encoding.UTF8.GetBytes(input);
			var hashBytes = sha512.ComputeHash(inputBytes);

			return Convert.ToHexString(hashBytes).ToLower();
		}

		/// <summary>
		/// Base64 encodes a string
		/// </summary>
		public static string Base64Encode(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";
			var bytes = Encoding.UTF8.GetBytes(input);
			return Convert.ToBase64String(bytes);
		}

		/// <summary>
		/// Base64 decodes a string
		/// </summary>
		public static string Base64Decode(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";

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
		/// URL encodes a string
		/// </summary>
		public static string UrlEncode(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";
			return Uri.EscapeDataString(input);
		}

		/// <summary>
		/// URL decodes a string
		/// </summary>
		public static string UrlDecode(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";

			try
			{
				return Uri.UnescapeDataString(input);
			}
			catch
			{
				return input;
			}
		}

		/// <summary>
		/// HTML encodes a string
		/// </summary>
		public static string HtmlEncode(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";
			return System.Web.HttpUtility.HtmlEncode(input);
		}

		/// <summary>
		/// HTML decodes a string
		/// </summary>
		public static string HtmlDecode(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";
			return System.Web.HttpUtility.HtmlDecode(input);
		}

		/// <summary>
		/// Creates a simple hash from input
		/// </summary>
		public static int SimpleHash(string input)
		{
			if (string.IsNullOrEmpty(input)) return 0;

			int hash = 0;
			foreach (char c in input)
			{
				hash = ((hash << 5) - hash) + c;
				hash = hash & hash; // Convert to 32-bit integer
			}
			return hash;
		}

		/// <summary>
		/// Creates a checksum from input
		/// </summary>
		public static string CreateChecksum(string input)
		{
			if (string.IsNullOrEmpty(input)) return "";

			using var sha1 = SHA1.Create();
			var inputBytes = Encoding.UTF8.GetBytes(input);
			var hashBytes = sha1.ComputeHash(inputBytes);

			return Convert.ToHexString(hashBytes).ToLower();
		}

		/// <summary>
		/// Verifies a checksum
		/// </summary>
		public static bool VerifyChecksum(string input, string expectedChecksum)
		{
			var actualChecksum = CreateChecksum(input);
			return string.Equals(actualChecksum, expectedChecksum, StringComparison.OrdinalIgnoreCase);
		}
	}
}
