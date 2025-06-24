using System;
using System.Collections.Generic;
using System.Linq;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Stellt Array-Funktionen für HypnoScript bereit.
    /// </summary>
    public static class ArrayBuiltins
    {
        /// <summary>
        /// Reverses an array
        /// </summary>
        public static object[] ArrayReverse(object[] arr)
        {
            if (arr == null) return new object[0];
            var result = new object[arr.Length];
            Array.Copy(arr, result, arr.Length);
            Array.Reverse(result);
            return result;
        }

        /// <summary>
        /// Sorts an array
        /// </summary>
        public static object[] ArraySort(object[] arr)
        {
            if (arr == null) return new object[0];
            var result = new object[arr.Length];
            Array.Copy(arr, result, arr.Length);
            Array.Sort(result);
            return result;
        }

        /// <summary>
        /// Removes duplicates from an array
        /// </summary>
        public static object[] ArrayUnique(object[] arr)
        {
            if (arr == null) return new object[0];
            return arr.Distinct().ToArray();
        }

        /// <summary>
        /// Filters an array using a predicate
        /// </summary>
        public static object[] ArrayFilter(object[] arr, Func<object, bool> predicate)
        {
            if (arr == null) return new object[0];
            return arr.Where(predicate).ToArray();
        }

        /// <summary>
        /// Maps an array using a function
        /// </summary>
        public static object[] ArrayMap(object[] arr, Func<object, object> mapper)
        {
            if (arr == null) return new object[0];
            return arr.Select(mapper).ToArray();
        }

        /// <summary>
        /// Reduces an array using a function
        /// </summary>
        public static object ArrayReduce(object[] arr, Func<object, object, object> reducer, object initial)
        {
            if (arr == null || arr.Length == 0) return initial;
            return arr.Aggregate(initial, reducer);
        }

        /// <summary>
        /// Flattens a nested array
        /// </summary>
        public static object[] ArrayFlatten(object[] arr)
        {
            if (arr == null) return new object[0];

            var result = new List<object>();
            foreach (var item in arr)
            {
                if (item is object[] nestedArray)
                {
                    result.AddRange(ArrayFlatten(nestedArray));
                }
                else
                {
                    result.Add(item);
                }
            }
            return result.ToArray();
        }

        /// <summary>
        /// Shuffles an array
        /// </summary>
        public static object[] ShuffleArray(object[] arr)
        {
            if (arr == null) return new object[0];
            var result = new object[arr.Length];
            Array.Copy(arr, result, arr.Length);

            var random = new Random();
            for (int i = result.Length - 1; i > 0; i--)
            {
                int j = random.Next(i + 1);
                var temp = result[i];
                result[i] = result[j];
                result[j] = temp;
            }
            return result;
        }

        /// <summary>
        /// Calculates sum of numeric array elements
        /// </summary>
        public static double SumArray(object[] arr)
        {
            if (arr == null) return 0;
            return arr.OfType<IConvertible>().Sum(x => Convert.ToDouble(x));
        }

        /// <summary>
        /// Calculates average of numeric array elements
        /// </summary>
        public static double AverageArray(object[] arr)
        {
            if (arr == null || arr.Length == 0) return 0;
            return SumArray(arr) / arr.Length;
        }

        /// <summary>
        /// Creates an array with range of numbers
        /// </summary>
        public static object[] Range(int start, int count)
        {
            return Enumerable.Range(start, count).Cast<object>().ToArray();
        }

        /// <summary>
        /// Creates an array with repeated value
        /// </summary>
        public static object[] Repeat(object value, int count)
        {
            return Enumerable.Repeat(value, count).ToArray();
        }

        /// <summary>
        /// Swaps two elements in an array
        /// </summary>
        public static void Swap(object[] arr, int i, int j)
        {
            if (arr == null || i < 0 || i >= arr.Length || j < 0 || j >= arr.Length)
                return;

            var temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }

        /// <summary>
        /// Splits array into chunks
        /// </summary>
        public static object[][] ChunkArray(object[] arr, int chunkSize)
        {
            if (arr == null || chunkSize <= 0) return new object[0][];

            var result = new List<object[]>();
            for (int i = 0; i < arr.Length; i += chunkSize)
            {
                int length = Math.Min(chunkSize, arr.Length - i);
                var chunk = new object[length];
                Array.Copy(arr, i, chunk, 0, length);
                result.Add(chunk);
            }
            return result.ToArray();
        }

        /// <summary>
        /// Calculates sum of array elements
        /// </summary>
        public static double ArraySum(object[] arr) => arr.OfType<IConvertible>().Sum(x => Convert.ToDouble(x));

        /// <summary>
        /// Finds minimum value in array
        /// </summary>
        public static object? ArrayMin(object[] arr) => arr.Length == 0 ? null : arr.Min();

        /// <summary>
        /// Finds maximum value in array
        /// </summary>
        public static object? ArrayMax(object[] arr) => arr.Length == 0 ? null : arr.Max();

        /// <summary>
        /// Counts occurrences of a value in array
        /// </summary>
        public static int ArrayCount(object[] arr, object? value) => arr.Count(x => Equals(x, value));

        /// <summary>
        /// Removes a value from array
        /// </summary>
        public static object[] ArrayRemove(object[] arr, object? value) => arr.Where(x => !Equals(x, value)).ToArray();

        /// <summary>
        /// Removes duplicates from array
        /// </summary>
        public static object[] ArrayDistinct(object[] arr) => arr.Distinct().ToArray();

        /// <summary>
        /// Inserts an element at specific index
        /// </summary>
        public static object[] ArrayInsert(object[] arr, int index, object value)
        {
            if (arr == null) return new object[] { value };
            if (index < 0) index = 0;
            if (index > arr.Length) index = arr.Length;

            var result = new object[arr.Length + 1];
            Array.Copy(arr, 0, result, 0, index);
            result[index] = value;
            Array.Copy(arr, index, result, index + 1, arr.Length - index);
            return result;
        }

        /// <summary>
        /// Removes element at specific index
        /// </summary>
        public static object[] ArrayRemoveAt(object[] arr, int index)
        {
            if (arr == null || arr.Length == 0) return new object[0];
            if (index < 0 || index >= arr.Length) return arr;

            var result = new object[arr.Length - 1];
            Array.Copy(arr, 0, result, 0, index);
            Array.Copy(arr, index + 1, result, index, arr.Length - index - 1);
            return result;
        }

        /// <summary>
        /// Clears all elements from array
        /// </summary>
        public static void ArrayClear(object[] arr) => Array.Clear(arr, 0, arr.Length);

        /// <summary>
        /// Creates a copy of array
        /// </summary>
        public static object[] ArrayCopy(object[] arr)
        {
            if (arr == null) return new object[0];
            var result = new object[arr.Length];
            Array.Copy(arr, result, arr.Length);
            return result;
        }

        /// <summary>
        /// Resizes an array
        /// </summary>
        public static object[] ArrayResize(object[] arr, int newSize)
        {
            if (newSize < 0) return new object[0];
            var result = new object[newSize];
            if (arr != null)
            {
                Array.Copy(arr, result, Math.Min(arr.Length, newSize));
            }
            return result;
        }

        /// <summary>
        /// Fills array with a value
        /// </summary>
        public static void ArrayFill(object[] arr, object value) => Array.Fill(arr, value);

        /// <summary>
        /// Finds index of first occurrence
        /// </summary>
        public static int ArrayIndexOf(object[] arr, object value, int startIndex) => Array.IndexOf(arr, value, startIndex);

        /// <summary>
        /// Finds index of last occurrence
        /// </summary>
        public static int ArrayLastIndexOf(object[] arr, object value) => Array.LastIndexOf(arr, value);

        /// <summary>
        /// Gets subarray
        /// </summary>
        public static object[] ArraySubArray(object[] arr, int start, int end)
        {
            if (arr == null) return new object[0];
            if (start < 0) start = 0;
            if (end > arr.Length) end = arr.Length;
            if (start >= end) return new object[0];

            var result = new object[end - start];
            Array.Copy(arr, start, result, 0, end - start);
            return result;
        }

        /// <summary>
        /// Rotates array elements
        /// </summary>
        public static object[] ArrayRotate(object[] arr, int positions)
        {
            if (arr == null || arr.Length == 0) return new object[0];

            positions = positions % arr.Length;
            if (positions < 0) positions += arr.Length;

            var result = new object[arr.Length];
            Array.Copy(arr, positions, result, 0, arr.Length - positions);
            Array.Copy(arr, 0, result, arr.Length - positions, positions);
            return result;
        }

        /// <summary>
        /// Shuffles array with seed
        /// </summary>
        public static object[] ArrayShuffle(object[] arr, int seed)
        {
            if (arr == null) return new object[0];
            var result = ArrayCopy(arr);
            var random = new Random(seed);

            for (int i = result.Length - 1; i > 0; i--)
            {
                int j = random.Next(i + 1);
                Swap(result, i, j);
            }
            return result;
        }

        /// <summary>
        /// Partitions array based on predicate
        /// </summary>
        public static object[][] ArrayPartition(object[] arr, Func<object, bool> predicate)
        {
            if (arr == null) return new object[0][];

            var trueItems = new List<object>();
            var falseItems = new List<object>();

            foreach (var item in arr)
            {
                if (predicate(item))
                    trueItems.Add(item);
                else
                    falseItems.Add(item);
            }

            return new object[][] { trueItems.ToArray(), falseItems.ToArray() };
        }

        /// <summary>
        /// Gets length of array
        /// </summary>
        public static int ArrayLength(object[] arr) => arr?.Length ?? 0;

        /// <summary>
        /// Gets element at index
        /// </summary>
        public static object? ArrayGet(object[] arr, int index)
        {
            if (arr == null || index < 0 || index >= arr.Length) return null;
            return arr[index];
        }

        /// <summary>
        /// Sets element at index
        /// </summary>
        public static void ArraySet(object[] arr, int index, object value)
        {
            if (arr != null && index >= 0 && index < arr.Length)
            {
                arr[index] = value;
            }
        }

        /// <summary>
        /// Gets slice of array
        /// </summary>
        public static object[] ArraySlice(object[] arr, int start, int length)
        {
            if (arr == null || start < 0 || length <= 0) return new object[0];
            if (start >= arr.Length) return new object[0];

            int actualLength = Math.Min(length, arr.Length - start);
            var result = new object[actualLength];
            Array.Copy(arr, start, result, 0, actualLength);
            return result;
        }

        /// <summary>
        /// Concatenates two arrays
        /// </summary>
        public static object[] ArrayConcat(object[] arr1, object[] arr2)
        {
            if (arr1 == null && arr2 == null) return new object[0];
            if (arr1 == null) return arr2 ?? new object[0];
            if (arr2 == null) return arr1;

            var result = new object[arr1.Length + arr2.Length];
            Array.Copy(arr1, 0, result, 0, arr1.Length);
            Array.Copy(arr2, 0, result, arr1.Length, arr2.Length);
            return result;
        }

        /// <summary>
        /// Finds index of element (without startIndex parameter)
        /// </summary>
        public static int ArrayIndexOf(object[] arr, object value) => Array.IndexOf(arr, value);

        /// <summary>
        /// Checks if array contains element
        /// </summary>
        public static bool ArrayContains(object[] arr, object value) => arr?.Contains(value) ?? false;
    }
}
