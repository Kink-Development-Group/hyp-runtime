using System;
using System.Linq;

namespace HypnoScript.Runtime.Builtins
{
	/// <summary>
	/// Stellt Statistik- und Analysefunktionen für HypnoScript bereit.
	/// </summary>
	public static class StatisticsBuiltins
	{
		/// <summary>
		/// Calculates linear regression
		/// </summary>
		public static double LinearRegression(object[] x, object[] y)
		{
			if (x.Length != y.Length || x.Length < 2) return 0;

			var xValues = x.Select(v => Convert.ToDouble(v)).ToArray();
			var yValues = y.Select(v => Convert.ToDouble(v)).ToArray();

			double sumX = xValues.Sum();
			double sumY = yValues.Sum();
			double sumXY = xValues.Zip(yValues, (a, b) => a * b).Sum();
			double sumX2 = xValues.Select(v => v * v).Sum();

			int n = xValues.Length;
			double slope = (n * sumXY - sumX * sumY) / (n * sumX2 - sumX * sumX);

			return slope;
		}

		/// <summary>
		/// Calculates mean of values
		/// </summary>
		public static double CalculateMean(object[] values)
		{
			if (values.Length == 0) return 0;
			var nums = values.Select(v => Convert.ToDouble(v)).ToArray();
			return nums.Sum() / nums.Length;
		}

		/// <summary>
		/// Calculates standard deviation
		/// </summary>
		public static double CalculateStandardDeviation(object[] values)
		{
			if (values.Length < 2) return 0;

			var nums = values.Select(v => Convert.ToDouble(v)).ToArray();
			double mean = nums.Sum() / nums.Length;
			double sumSquaredDiff = nums.Sum(v => Math.Pow(v - mean, 2));

			return Math.Sqrt(sumSquaredDiff / (nums.Length - 1));
		}

		/// <summary>
		/// Calculates variance
		/// </summary>
		public static double CalculateVariance(object[] values)
		{
			if (values.Length < 2) return 0;

			var nums = values.Select(v => Convert.ToDouble(v)).ToArray();
			double mean = nums.Sum() / nums.Length;
			double sumSquaredDiff = nums.Sum(v => Math.Pow(v - mean, 2));

			return sumSquaredDiff / (nums.Length - 1);
		}

		/// <summary>
		/// Calculates median
		/// </summary>
		public static double CalculateMedian(object[] values)
		{
			if (values.Length == 0) return 0;

			var nums = values.Select(v => Convert.ToDouble(v)).OrderBy(v => v).ToArray();
			int n = nums.Length;

			if (n % 2 == 0)
			{
				return (nums[n / 2 - 1] + nums[n / 2]) / 2;
			}
			else
			{
				return nums[n / 2];
			}
		}

		/// <summary>
		/// Calculates mode
		/// </summary>
		public static double CalculateMode(object[] values)
		{
			if (values.Length == 0) return 0;

			var groups = values.GroupBy(v => Convert.ToDouble(v))
							  .OrderByDescending(g => g.Count())
							  .ThenBy(g => g.Key);

			return groups.First().Key;
		}

		/// <summary>
		/// Calculates range
		/// </summary>
		public static double CalculateRange(object[] values)
		{
			if (values.Length == 0) return 0;

			var nums = values.Select(v => Convert.ToDouble(v)).ToArray();
			return nums.Max() - nums.Min();
		}

		/// <summary>
		/// Calculates sum of array elements
		/// </summary>
		public static double ArraySum(object[] arr) => arr.OfType<IConvertible>().Sum(x => Convert.ToDouble(x));

		/// <summary>
		/// Calculates average of array elements
		/// </summary>
		public static double AverageArray(object[] arr)
		{
			if (arr == null || arr.Length == 0) return 0;
			return SumArray(arr) / arr.Length;
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
		/// Calculates correlation coefficient
		/// </summary>
		public static double CalculateCorrelation(object[] x, object[] y)
		{
			if (x.Length != y.Length || x.Length < 2) return 0;

			var xValues = x.Select(v => Convert.ToDouble(v)).ToArray();
			var yValues = y.Select(v => Convert.ToDouble(v)).ToArray();

			double meanX = xValues.Sum() / xValues.Length;
			double meanY = yValues.Sum() / yValues.Length;

			double numerator = xValues.Zip(yValues, (a, b) => (a - meanX) * (b - meanY)).Sum();
			double denominatorX = xValues.Sum(v => Math.Pow(v - meanX, 2));
			double denominatorY = yValues.Sum(v => Math.Pow(v - meanY, 2));

			if (denominatorX == 0 || denominatorY == 0) return 0;

			return numerator / Math.Sqrt(denominatorX * denominatorY);
		}

		/// <summary>
		/// Calculates percentile
		/// </summary>
		public static double CalculatePercentile(object[] values, double percentile)
		{
			if (values.Length == 0) return 0;
			if (percentile < 0 || percentile > 100) return 0;

			var nums = values.Select(v => Convert.ToDouble(v)).OrderBy(v => v).ToArray();
			double index = (percentile / 100.0) * (nums.Length - 1);

			if (index == Math.Floor(index))
			{
				return nums[(int)index];
			}
			else
			{
				int lower = (int)Math.Floor(index);
				int upper = (int)Math.Ceiling(index);
				double weight = index - lower;
				return nums[lower] * (1 - weight) + nums[upper] * weight;
			}
		}

		/// <summary>
		/// Calculates interquartile range
		/// </summary>
		public static double CalculateIQR(object[] values)
		{
			double q1 = CalculatePercentile(values, 25);
			double q3 = CalculatePercentile(values, 75);
			return q3 - q1;
		}

		/// <summary>
		/// Detects outliers using IQR method
		/// </summary>
		public static object[] DetectOutliers(object[] values)
		{
			if (values.Length < 4) return new object[0];

			double q1 = CalculatePercentile(values, 25);
			double q3 = CalculatePercentile(values, 75);
			double iqr = q3 - q1;
			double lowerBound = q1 - 1.5 * iqr;
			double upperBound = q3 + 1.5 * iqr;

			return values.Where(v =>
			{
				double val = Convert.ToDouble(v);
				return val < lowerBound || val > upperBound;
			}).ToArray();
		}
	}
}
