using System;
using System.Collections.Generic;
using System.Linq;

namespace HypnoScript.Runtime.Builtins
{
	/// <summary>
	/// Stellt Dictionary- und Record-Funktionen für HypnoScript bereit.
	/// </summary>
	public static class DictionaryBuiltins
	{
		/// <summary>
		/// Creates a new dictionary
		/// </summary>
		public static Dictionary<string, object> CreateDictionary() => new();

		/// <summary>
		/// Gets all keys from a dictionary
		/// </summary>
		public static string[] DictionaryKeys(Dictionary<string, object> dict) => dict.Keys.ToArray();

		/// <summary>
		/// Gets all values from a dictionary
		/// </summary>
		public static object[] DictionaryValues(Dictionary<string, object> dict) => dict.Values.ToArray();

		/// <summary>
		/// Checks if dictionary contains a key
		/// </summary>
		public static bool DictionaryContainsKey(Dictionary<string, object> dict, string key) => dict.ContainsKey(key);

		/// <summary>
		/// Gets a value from dictionary with optional default
		/// </summary>
		public static object? DictionaryGet(Dictionary<string, object> dict, string key, object? defaultValue = null) => dict.TryGetValue(key, out var value) ? value : defaultValue;

		/// <summary>
		/// Sets a value in dictionary
		/// </summary>
		public static void DictionarySet(Dictionary<string, object> dict, string key, object value) => dict[key] = value;

		/// <summary>
		/// Removes a key from dictionary
		/// </summary>
		public static bool DictionaryRemove(Dictionary<string, object> dict, string key) => dict.Remove(key);

		/// <summary>
		/// Gets count of dictionary entries
		/// </summary>
		public static int DictionaryCount(Dictionary<string, object> dict) => dict.Count;

		/// <summary>
		/// Creates a record from keys and values arrays
		/// </summary>
		public static Dictionary<string, object> CreateRecord(string[] keys, object[] values)
		{
			var record = new Dictionary<string, object>();
			for (int i = 0; i < Math.Min(keys.Length, values.Length); i++)
			{
				record[keys[i]] = values[i];
			}
			return record;
		}

		/// <summary>
		/// Gets a value from a record
		/// </summary>
		public static object? GetRecordValue(Dictionary<string, object> record, string key)
		{
			return record.TryGetValue(key, out var value) ? value : null;
		}

		/// <summary>
		/// Sets a value in a record
		/// </summary>
		public static void SetRecordValue(Dictionary<string, object> record, string key, object value)
		{
			record[key] = value;
		}

		/// <summary>
		/// Merges two dictionaries
		/// </summary>
		public static Dictionary<string, object> MergeDictionaries(Dictionary<string, object> dict1, Dictionary<string, object> dict2)
		{
			var result = new Dictionary<string, object>(dict1);
			foreach (var kvp in dict2)
			{
				result[kvp.Key] = kvp.Value;
			}
			return result;
		}

		/// <summary>
		/// Filters dictionary by predicate
		/// </summary>
		public static Dictionary<string, object> FilterDictionary(Dictionary<string, object> dict, Func<string, object, bool> predicate)
		{
			return dict.Where(kvp => predicate(kvp.Key, kvp.Value))
					  .ToDictionary(kvp => kvp.Key, kvp => kvp.Value);
		}

		/// <summary>
		/// Maps dictionary values
		/// </summary>
		public static Dictionary<string, object> MapDictionary(Dictionary<string, object> dict, Func<string, object, object> mapper)
		{
			return dict.ToDictionary(kvp => kvp.Key, kvp => mapper(kvp.Key, kvp.Value));
		}

		/// <summary>
		/// Converts dictionary to array of key-value pairs
		/// </summary>
		public static object[] DictionaryToArray(Dictionary<string, object> dict)
		{
			return dict.Select(kvp => new Dictionary<string, object> { ["key"] = kvp.Key, ["value"] = kvp.Value }).Cast<object>().ToArray();
		}

		/// <summary>
		/// Creates dictionary from array of key-value pairs
		/// </summary>
		public static Dictionary<string, object> ArrayToDictionary(object[] array)
		{
			var dict = new Dictionary<string, object>();
			foreach (var item in array)
			{
				if (item is Dictionary<string, object> kvp)
				{
					if (kvp.TryGetValue("key", out var key) && kvp.TryGetValue("value", out var value))
					{
						dict[key.ToString()!] = value;
					}
				}
			}
			return dict;
		}

		/// <summary>
		/// Checks if dictionary is empty
		/// </summary>
		public static bool IsDictionaryEmpty(Dictionary<string, object> dict) => dict.Count == 0;

		/// <summary>
		/// Clears all entries from dictionary
		/// </summary>
		public static void ClearDictionary(Dictionary<string, object> dict) => dict.Clear();

		/// <summary>
		/// Gets dictionary as sorted by keys
		/// </summary>
		public static Dictionary<string, object> SortDictionaryByKeys(Dictionary<string, object> dict)
		{
			return dict.OrderBy(kvp => kvp.Key)
					  .ToDictionary(kvp => kvp.Key, kvp => kvp.Value);
		}

		/// <summary>
		/// Gets dictionary as sorted by values
		/// </summary>
		public static Dictionary<string, object> SortDictionaryByValues(Dictionary<string, object> dict)
		{
			return dict.OrderBy(kvp => kvp.Value)
					  .ToDictionary(kvp => kvp.Key, kvp => kvp.Value);
		}
	}
}
