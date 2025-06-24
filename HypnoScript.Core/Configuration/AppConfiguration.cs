using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;

namespace HypnoScript.Core.Configuration
{
    /// <summary>
    /// Central configuration management for HypnoScript CLI and Runtime.
    /// </summary>
    public class AppConfiguration
    {
        private static AppConfiguration? _instance;
        private static readonly object _lock = new object();

        /// <summary>
        /// Gets the singleton instance of AppConfiguration.
        /// </summary>
        public static AppConfiguration Instance
        {
            get
            {
                if (_instance == null)
                {
                    lock (_lock)
                    {
                        _instance ??= new AppConfiguration();
                    }
                }
                return _instance;
            }
        }

        /// <summary>
        /// CLI-specific configuration settings.
        /// </summary>
        public CliSettings Cli { get; set; } = new();

        /// <summary>
        /// Runtime-specific configuration settings.
        /// </summary>
        public RuntimeSettings Runtime { get; set; } = new();

        /// <summary>
        /// Logging configuration settings.
        /// </summary>
        public LoggingSettings Logging { get; set; } = new();

        /// <summary>
        /// Development and debugging settings.
        /// </summary>
        public DevelopmentSettings Development { get; set; } = new();

        private AppConfiguration()
        {
            LoadConfiguration();
        }

        /// <summary>
        /// Loads configuration from file or creates default configuration.
        /// </summary>
        public void LoadConfiguration()
        {
            var configPath = GetConfigFilePath();

            if (File.Exists(configPath))
            {
                try
                {
                    var json = File.ReadAllText(configPath);
                    var config = JsonSerializer.Deserialize<AppConfiguration>(json);
                    if (config != null)
                    {
                        Cli = config.Cli;
                        Runtime = config.Runtime;
                        Logging = config.Logging;
                        Development = config.Development;
                    }
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"[WARN] Failed to load configuration: {ex.Message}");
                    Console.WriteLine("[INFO] Using default configuration.");
                }
            }
            else
            {
                SaveConfiguration(); // Save default configuration
            }
        }

        /// <summary>
        /// Saves the current configuration to file.
        /// </summary>
        public void SaveConfiguration()
        {
            try
            {
                var configPath = GetConfigFilePath();
                var configDir = Path.GetDirectoryName(configPath);

                if (!string.IsNullOrEmpty(configDir) && !Directory.Exists(configDir))
                {
                    Directory.CreateDirectory(configDir);
                }

                var options = new JsonSerializerOptions { WriteIndented = true };
                var json = JsonSerializer.Serialize(this, options);
                File.WriteAllText(configPath, json);
            }
            catch (Exception ex)
            {
                Console.WriteLine($"[ERROR] Failed to save configuration: {ex.Message}");
            }
        }

        /// <summary>
        /// Gets the configuration file path.
        /// </summary>
        /// <returns>The path to the configuration file</returns>
        private static string GetConfigFilePath()
        {
            var appData = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);
            return Path.Combine(appData, "HypnoScript", "config.json");
        }

        /// <summary>
        /// Resets configuration to default values.
        /// </summary>
        public void ResetToDefaults()
        {
            Cli = new CliSettings();
            Runtime = new RuntimeSettings();
            Logging = new LoggingSettings();
            Development = new DevelopmentSettings();
            SaveConfiguration();
        }
    }

    /// <summary>
    /// CLI-specific configuration settings.
    /// </summary>
    public class CliSettings
    {
        /// <summary>
        /// Default timeout for CLI operations in milliseconds.
        /// </summary>
        public int DefaultTimeout { get; set; } = 30000;

        /// <summary>
        /// Maximum number of concurrent operations.
        /// </summary>
        public int MaxConcurrentOperations { get; set; } = 4;

        /// <summary>
        /// Whether to show verbose output by default.
        /// </summary>
        public bool VerboseOutput { get; set; } = false;

        /// <summary>
        /// Whether to enable colored output.
        /// </summary>
        public bool ColoredOutput { get; set; } = true;

        /// <summary>
        /// Default output format for commands.
        /// </summary>
        public string DefaultOutputFormat { get; set; } = "text";

        /// <summary>
        /// Whether to enable auto-completion.
        /// </summary>
        public bool EnableAutoCompletion { get; set; } = true;

        /// <summary>
        /// History file path for command history.
        /// </summary>
        public string HistoryFilePath { get; set; } = "~/.hypnoscript_history";

        /// <summary>
        /// Maximum number of history entries to keep.
        /// </summary>
        public int MaxHistoryEntries { get; set; } = 1000;
    }

    /// <summary>
    /// Runtime-specific configuration settings.
    /// </summary>
    public class RuntimeSettings
    {
        /// <summary>
        /// Maximum execution time for scripts in milliseconds.
        /// </summary>
        public int MaxExecutionTime { get; set; } = 300000; // 5 minutes

        /// <summary>
        /// Maximum memory usage in MB.
        /// </summary>
        public int MaxMemoryUsage { get; set; } = 512;

        /// <summary>
        /// Whether to enable garbage collection during execution.
        /// </summary>
        public bool EnableGarbageCollection { get; set; } = true;

        /// <summary>
        /// Garbage collection frequency in milliseconds.
        /// </summary>
        public int GarbageCollectionInterval { get; set; } = 10000;

        /// <summary>
        /// Whether to enable stack trace collection.
        /// </summary>
        public bool EnableStackTrace { get; set; } = true;

        /// <summary>
        /// Maximum stack depth for function calls.
        /// </summary>
        public int MaxStackDepth { get; set; } = 1000;

        /// <summary>
        /// Whether to enable built-in function caching.
        /// </summary>
        public bool EnableBuiltinCaching { get; set; } = true;

        /// <summary>
        /// Cache size for built-in functions.
        /// </summary>
        public int BuiltinCacheSize { get; set; } = 1000;

        /// <summary>
        /// Whether to enable type checking during execution.
        /// </summary>
        public bool EnableTypeChecking { get; set; } = true;

        /// <summary>
        /// Whether to enable strict mode.
        /// </summary>
        public bool StrictMode { get; set; } = false;
    }

    /// <summary>
    /// Logging configuration settings.
    /// </summary>
    public class LoggingSettings
    {
        /// <summary>
        /// Minimum log level to output.
        /// </summary>
        public string LogLevel { get; set; } = "INFO";

        /// <summary>
        /// Whether to enable file logging.
        /// </summary>
        public bool EnableFileLogging { get; set; } = true;

        /// <summary>
        /// Log file path.
        /// </summary>
        public string LogFilePath { get; set; } = "logs/hypnoscript.log";

        /// <summary>
        /// Maximum log file size in MB.
        /// </summary>
        public int MaxLogFileSize { get; set; } = 10;

        /// <summary>
        /// Number of log files to keep.
        /// </summary>
        public int MaxLogFiles { get; set; } = 5;

        /// <summary>
        /// Whether to enable console logging.
        /// </summary>
        public bool EnableConsoleLogging { get; set; } = true;

        /// <summary>
        /// Whether to include timestamps in log messages.
        /// </summary>
        public bool IncludeTimestamps { get; set; } = true;

        /// <summary>
        /// Whether to include thread information in log messages.
        /// </summary>
        public bool IncludeThreadInfo { get; set; } = false;

        /// <summary>
        /// Log message format.
        /// </summary>
        public string LogFormat { get; set; } = "{Timestamp:yyyy-MM-dd HH:mm:ss} [{Level}] {Message}";
    }

    /// <summary>
    /// Development and debugging configuration settings.
    /// </summary>
    public class DevelopmentSettings
    {
        /// <summary>
        /// Whether to enable debug mode.
        /// </summary>
        public bool DebugMode { get; set; } = false;

        /// <summary>
        /// Whether to enable performance profiling.
        /// </summary>
        public bool EnableProfiling { get; set; } = false;

        /// <summary>
        /// Whether to enable detailed error reporting.
        /// </summary>
        public bool DetailedErrorReporting { get; set; } = true;

        /// <summary>
        /// Whether to enable source map generation.
        /// </summary>
        public bool EnableSourceMaps { get; set; } = false;

        /// <summary>
        /// Whether to enable hot reloading.
        /// </summary>
        public bool EnableHotReload { get; set; } = false;

        /// <summary>
        /// Whether to enable experimental features.
        /// </summary>
        public bool EnableExperimentalFeatures { get; set; } = false;

        /// <summary>
        /// Development server port.
        /// </summary>
        public int DevelopmentServerPort { get; set; } = 8080;

        /// <summary>
        /// Whether to enable remote debugging.
        /// </summary>
        public bool EnableRemoteDebugging { get; set; } = false;

        /// <summary>
        /// Remote debugging port.
        /// </summary>
        public int RemoteDebuggingPort { get; set; } = 9222;
    }
}
