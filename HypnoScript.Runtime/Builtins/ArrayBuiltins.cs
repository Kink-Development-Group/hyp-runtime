using System;
using System.Collections.Generic;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Array Builtins für HypnoScript (ausgelagert aus HypnoBuiltins)
    /// </summary>
    public static class ArrayBuiltins
    {
        /// <summary>Länge des Arrays.</summary>
        public static int ArrayLength(object[] arr) => arr.Length;
        /// <summary>Gibt das Element am Index zurück (mit Fehlerprüfung).</summary>
        public static object? ArrayGet(object[] arr, int index)
        {
            if (arr == null)
            {
                HypnoBuiltins.Observe("Error: Array is null.");
                return null;
            }
            if (index < 0 || index >= arr.Length)
            {
                HypnoBuiltins.Observe($"Error: Array index {index} out of bounds (length: {arr?.Length ?? 0}).");
                return null;
            }
            return arr[index];
        }
        /// <summary>Setzt das Element am Index (mit Fehlerprüfung).</summary>
        public static void ArraySet(object[] arr, int index, object? value)
        {
            if (arr == null)
            {
                HypnoBuiltins.Observe("Error: Array is null.");
                return;
            }
            if (index < 0 || index >= arr.Length)
            {
                HypnoBuiltins.Observe($"Error: Array index {index} out of bounds (length: {arr?.Length ?? 0}).");
                return;
            }
            arr[index] = value ?? "";
        }
        /// <summary>Gibt einen Slice des Arrays zurück (mit Fehlerprüfung).</summary>
        public static object[] ArraySlice(object[] arr, int start, int length)
        {
            if (arr == null)
            {
                HypnoBuiltins.Observe("Error: Array is null.");
                return Array.Empty<object>();
            }
            if (start < 0 || length < 0 || start + length > arr.Length)
            {
                HypnoBuiltins.Observe($"Error: Array slice out of bounds (start: {start}, length: {length}, array length: {arr.Length}).");
                return Array.Empty<object>();
            }
            var result = new object[length];
            Array.Copy(arr, start, result, 0, length);
            return result;
        }
        /// <summary>Verkettet zwei Arrays.</summary>
        public static object[] ArrayConcat(object[] arr1, object[] arr2)
        {
            var result = new object[arr1.Length + arr2.Length];
            Array.Copy(arr1, 0, result, 0, arr1.Length);
            Array.Copy(arr2, 0, result, arr1.Length, arr2.Length);
            return result;
        }
        /// <summary>Index eines Werts im Array.</summary>
        public static int ArrayIndexOf(object[] arr, object? value)
        {
            return Array.IndexOf(arr, value);
        }
        /// <summary>Prüft, ob ein Wert im Array enthalten ist.</summary>
        public static bool ArrayContains(object[] arr, object? value)
        {
            return Array.IndexOf(arr, value) >= 0;
        }
        /// <summary>Wendet eine Funktion auf jedes Element an und gibt ein neues Array zurück.</summary>
        public static object[] ArrayMap(object[] arr, Func<object, object> mapper)
        {
            if (arr == null || mapper == null) return Array.Empty<object>();
            var result = new object[arr.Length];
            for (int i = 0; i < arr.Length; i++)
                result[i] = mapper(arr[i]);
            return result;
        }
        /// <summary>Reduziert ein Array auf einen Wert mit einer Aggregatfunktion.</summary>
        public static object ArrayReduce(object[] arr, Func<object, object, object> reducer, object initial)
        {
            if (arr == null || reducer == null) return initial;
            object acc = initial;
            foreach (var item in arr)
                acc = reducer(acc, item);
            return acc;
        }
        /// <summary>Flacht ein verschachteltes Array um eine Ebene ab.</summary>
        public static object[] ArrayFlatten(object[] arr)
        {
            if (arr == null) return Array.Empty<object>();
            var list = new List<object>();
            foreach (var item in arr)
            {
                if (item is object[] subArr)
                    list.AddRange(subArr);
                else
                    list.Add(item);
            }
            return list.ToArray();
        }
    }
}
