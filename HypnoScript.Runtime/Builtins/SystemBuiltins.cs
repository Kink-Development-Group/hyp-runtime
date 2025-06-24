using System;
using Microsoft.Extensions.Logging;
using Microsoft.ApplicationInsights;
using Microsoft.ApplicationInsights.Extensibility;

namespace HypnoScript.Runtime.Builtins
{
    /// <summary>
    /// System Builtins für HypnoScript (ausgelagert aus HypnoBuiltins)
    /// </summary>
    public static class SystemBuiltins
    {
        private static readonly ILogger Logger = LoggerFactory.Create(builder => builder.AddConsole()).CreateLogger("SystemBuiltins");
        private static readonly TelemetryClient Telemetry = new TelemetryClient(new TelemetryConfiguration("<YOUR_INSTRUMENTATION_KEY_HERE>"));

        /// <summary>Leert die Konsole.</summary>
        public static void ClearScreen() => Console.Clear();
        /// <summary>Gibt einen Beep-Ton aus (nur Windows, sonst Sleep).</summary>
        public static void Beep(int frequency = 800, int duration = 200)
        {
#if WINDOWS
            Console.Beep(frequency, duration);
#else
            System.Threading.Thread.Sleep(duration);
#endif
        }
        /// <summary>Gibt eine Umgebungsvariable zurück.</summary>
        public static string GetEnvironmentVariable(string name) => Environment.GetEnvironmentVariable(name) ?? string.Empty;
        /// <summary>Beendet das Programm mit Exit-Code.</summary>
        public static void Exit(int code = 0) => Environment.Exit(code);
        /// <summary>Aktuelles Arbeitsverzeichnis.</summary>
        public static string GetCurrentDirectory() => Environment.CurrentDirectory;
        /// <summary>Maschinenname.</summary>
        public static string GetMachineName() => Environment.MachineName;
        /// <summary>Benutzername.</summary>
        public static string GetUserName() => Environment.UserName;
        /// <summary>Betriebssystem-Version.</summary>
        public static string GetOSVersion() => Environment.OSVersion.ToString();
        /// <summary>Anzahl Prozessoren.</summary>
        public static int GetProcessorCount() => Environment.ProcessorCount;
        /// <summary>Arbeitsspeicherverbrauch (Working Set).</summary>
        public static long GetWorkingSet() => Environment.WorkingSet;
        /// <summary>Spielt einen Sound (wie Beep).</summary>
        public static void PlaySound(int frequency = 800, int duration = 200)
        {
#if WINDOWS
            Console.Beep(frequency, duration);
#else
            System.Threading.Thread.Sleep(duration);
#endif
        }
        /// <summary>Simuliert Vibration durch Beep/Sleep.</summary>
        public static void Vibrate(int duration = 1000)
        {
            var startTime = DateTime.Now;
            while ((DateTime.Now - startTime).TotalMilliseconds < duration)
            {
#if WINDOWS
                Console.Beep(200, 50);
#else
                System.Threading.Thread.Sleep(50);
#endif
                System.Threading.Thread.Sleep(50);
            }
        }
        /// <summary>Loggt eine Info-Nachricht (Konsole & Telemetrie).</summary>
        public static void LogInfo(string message)
        {
            Logger.LogInformation(message);
            Telemetry.TrackTrace(message);
        }
        /// <summary>Sendet ein Event an Application Insights.</summary>
        public static void TrackEvent(string eventName)
        {
            Telemetry.TrackEvent(eventName);
        }
    }
}
