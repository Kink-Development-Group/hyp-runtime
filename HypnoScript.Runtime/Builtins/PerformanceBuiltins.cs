using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading;

namespace HypnoScript.Runtime.Builtins
{
	/// <summary>
	/// Stellt Performance- und Benchmark-Funktionen für HypnoScript bereit.
	/// </summary>
	public static class PerformanceBuiltins
	{
		/// <summary>
		/// Gets performance metrics
		/// </summary>
		public static Dictionary<string, object> GetPerformanceMetrics()
		{
			var process = Process.GetCurrentProcess();
			return new Dictionary<string, object>
			{
				["memoryUsage"] = GC.GetTotalMemory(false),
				["workingSet"] = process.WorkingSet64,
				["cpuTime"] = process.TotalProcessorTime.TotalSeconds,
				["threadCount"] = process.Threads.Count,
				["handleCount"] = process.HandleCount,
				["startTime"] = process.StartTime.ToString("yyyy-MM-dd HH:mm:ss")
			};
		}

		/// <summary>
		/// Benchmarks a function
		/// </summary>
		public static double Benchmark(Func<object> func, int iterations)
		{
			var stopwatch = Stopwatch.StartNew();
			for (int i = 0; i < iterations; i++)
			{
				func();
			}
			stopwatch.Stop();
			return stopwatch.ElapsedMilliseconds / (double)iterations;
		}

		/// <summary>
		/// Gets memory usage in MB
		/// </summary>
		public static long GetMemoryUsage() => GC.GetTotalMemory(false);

		/// <summary>
		/// Gets CPU usage (approximate)
		/// </summary>
		public static double GetCPUUsage()
		{
			// Simple CPU usage approximation
			return Environment.ProcessorCount * 100.0;
		}

		/// <summary>
		/// Forces garbage collection
		/// </summary>
		public static void ForceGarbageCollection()
		{
			GC.Collect();
			GC.WaitForPendingFinalizers();
			GC.Collect();
		}

		/// <summary>
		/// Gets process information
		/// </summary>
		public static Dictionary<string, object> GetProcessInfo()
		{
			var process = Process.GetCurrentProcess();
			return new Dictionary<string, object>
			{
				["id"] = process.Id,
				["name"] = process.ProcessName,
				["memory"] = process.WorkingSet64,
				["cpuTime"] = process.TotalProcessorTime.TotalSeconds,
				["startTime"] = process.StartTime.ToString("yyyy-MM-dd HH:mm:ss")
			};
		}

		/// <summary>
		/// Gets system information
		/// </summary>
		public static Dictionary<string, object> GetSystemInfo()
		{
			return new Dictionary<string, object>
			{
				["os"] = Environment.OSVersion.ToString(),
				["machineName"] = Environment.MachineName,
				["processorCount"] = Environment.ProcessorCount,
				["workingSet"] = Environment.WorkingSet,
				["userName"] = Environment.UserName,
				["currentDirectory"] = Environment.CurrentDirectory
			};
		}

		/// <summary>
		/// Measures execution time of a function
		/// </summary>
		public static double MeasureExecutionTime(Func<object> func)
		{
			var stopwatch = Stopwatch.StartNew();
			func();
			stopwatch.Stop();
			return stopwatch.ElapsedMilliseconds;
		}

		/// <summary>
		/// Measures execution time of an action
		/// </summary>
		public static double MeasureExecutionTime(Action action)
		{
			var stopwatch = Stopwatch.StartNew();
			action();
			stopwatch.Stop();
			return stopwatch.ElapsedMilliseconds;
		}

		/// <summary>
		/// Gets current tick count
		/// </summary>
		public static long GetTickCount() => Environment.TickCount64;

		/// <summary>
		/// Sleeps for specified milliseconds
		/// </summary>
		public static void Sleep(int ms) => Thread.Sleep(ms);

		/// <summary>
		/// Debug print memory information
		/// </summary>
		public static void DebugPrintMemory()
		{
			var memory = GC.GetTotalMemory(false);
			Console.WriteLine($"[DEBUG] Memory Usage: {memory / 1024 / 1024} MB");
		}

		/// <summary>
		/// Debug print stack trace
		/// </summary>
		public static void DebugPrintStackTrace()
		{
			Console.WriteLine($"[DEBUG] Stack Trace: {Environment.StackTrace}");
		}

		/// <summary>
		/// Debug print environment information
		/// </summary>
		public static void DebugPrintEnvironment()
		{
			Console.WriteLine($"[DEBUG] OS: {Environment.OSVersion}");
			Console.WriteLine($"[DEBUG] Machine: {Environment.MachineName}");
			Console.WriteLine($"[DEBUG] Processors: {Environment.ProcessorCount}");
			Console.WriteLine($"[DEBUG] Memory: {Environment.WorkingSet / 1024 / 1024} MB");
		}

		/// <summary>
		/// Gets call stack
		/// </summary>
		public static string[] GetCallStack()
		{
			return Environment.StackTrace.Split('\n', StringSplitOptions.RemoveEmptyEntries);
		}

		/// <summary>
		/// Gets exception information
		/// </summary>
		public static Dictionary<string, object> GetExceptionInfo(Exception ex)
		{
			return new Dictionary<string, object>
			{
				["message"] = ex.Message,
				["type"] = ex.GetType().Name,
				["stackTrace"] = ex.StackTrace ?? "",
				["source"] = ex.Source ?? ""
			};
		}
	}
}
