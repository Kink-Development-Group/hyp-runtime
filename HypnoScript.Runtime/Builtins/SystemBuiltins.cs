using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// Stellt System- und Umgebungsfunktionen für HypnoScript bereit.
    /// </summary>
    public static class SystemBuiltins
    {
        /// <summary>
        /// Clears the console screen
        /// </summary>
        public static void ClearScreen()
        {
            Console.Clear();
        }

        /// <summary>
        /// Plays a system beep
        /// </summary>
        public static void Beep(int frequency = 800, int duration = 200)
        {
#if WINDOWS
            Console.Beep(frequency, duration);
#else
            // Fallback für nicht-Windows Plattformen
            System.Threading.Thread.Sleep(duration);
#endif
        }

        /// <summary>
        /// Gets environment variable
        /// </summary>
        public static string GetEnvironmentVariable(string name)
        {
            return Environment.GetEnvironmentVariable(name) ?? "";
        }

        /// <summary>
        /// Exits the application
        /// </summary>
        public static void Exit(int code = 0)
        {
            Environment.Exit(code);
        }

        /// <summary>
        /// Gets machine name
        /// </summary>
        public static string GetMachineName() => Environment.MachineName;

        /// <summary>
        /// Gets user name
        /// </summary>
        public static string GetUserName() => Environment.UserName;

        /// <summary>
        /// Gets OS version
        /// </summary>
        public static string GetOSVersion() => Environment.OSVersion.ToString();

        /// <summary>
        /// Gets processor count
        /// </summary>
        public static int GetProcessorCount() => Environment.ProcessorCount;

        /// <summary>
        /// Gets working set memory
        /// </summary>
        public static long GetWorkingSet() => Environment.WorkingSet;

        /// <summary>
        /// Gets memory usage
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
        /// Gets all environment variables
        /// </summary>
        public static Dictionary<string, string> GetEnvVars() => Environment.GetEnvironmentVariables().Cast<System.Collections.DictionaryEntry>().ToDictionary(e => (string)e.Key, e => e.Value as string ?? "");

        /// <summary>
        /// Gets tick count
        /// </summary>
        public static long GetTickCount() => Environment.TickCount64;

        /// <summary>
        /// Sleeps for specified milliseconds
        /// </summary>
        public static void Sleep(int ms) => System.Threading.Thread.Sleep(ms);

        /// <summary>
        /// Plays a sound
        /// </summary>
        public static void PlaySound(int frequency = 800, int duration = 200)
        {
#if WINDOWS
            Console.Beep(frequency, duration);
#else
            System.Threading.Thread.Sleep(duration);
#endif
        }

        /// <summary>
        /// Simulates vibration (platform dependent)
        /// </summary>
        public static void Vibrate(int duration = 1000)
        {
            // Platform-specific vibration would go here
            // For now, just sleep
            System.Threading.Thread.Sleep(duration);
        }

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

        /// <summary>
        /// Logs a message
        /// </summary>
        public static void Log(string message, string level = "INFO")
        {
            Console.WriteLine($"[{level}] {message}");
        }

        /// <summary>
        /// Traces a message
        /// </summary>
        public static void Trace(string message) => Log(message, "TRACE");
    }
}
