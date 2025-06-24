using System;
using System.CommandLine;
using HypnoScript.CLI;
using HypnoScript.Core.Configuration;

namespace HypnoScript.CLI.Commands
{
    /// <summary>
    /// Command for managing HypnoScript configuration.
    /// </summary>
    public static class ConfigCommand
    {
        /// <summary>
        /// Executes the configuration command.
        /// </summary>
        /// <param name="show">Show current configuration</param>
        /// <param name="reset">Reset configuration to defaults</param>
        /// <param name="set">Set a configuration value</param>
        /// <param name="get">Get a configuration value</param>
        /// <param name="export">Export configuration to file</param>
        /// <param name="import">Import configuration from file</param>
        public static void Execute(bool show, bool reset, string? set, string? get, string? export, string? import)
        {
            try
            {
                var config = AppConfiguration.Instance;

                if (show)
                {
                    ShowConfiguration(config);
                }
                else if (reset)
                {
                    config.ResetToDefaults();
                    AppLogger.Info("Configuration reset to defaults.");
                }
                else if (!string.IsNullOrEmpty(set))
                {
                    SetConfigurationValue(config, set);
                }
                else if (!string.IsNullOrEmpty(get))
                {
                    GetConfigurationValue(config, get);
                }
                else if (!string.IsNullOrEmpty(export))
                {
                    ExportConfiguration(config, export);
                }
                else if (!string.IsNullOrEmpty(import))
                {
                    ImportConfiguration(config, import);
                }
                else
                {
                    ShowConfiguration(config);
                }
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Configuration operation failed: {ex.Message}", ex);
                Environment.Exit(1);
            }
        }

        private static void ShowConfiguration(AppConfiguration config)
        {
            AppLogger.Info("=== HypnoScript Configuration ===");

            AppLogger.Info("\n--- CLI Settings ---");
            AppLogger.Info($"Default Timeout: {config.Cli.DefaultTimeout}ms");
            AppLogger.Info($"Max Concurrent Operations: {config.Cli.MaxConcurrentOperations}");
            AppLogger.Info($"Verbose Output: {config.Cli.VerboseOutput}");
            AppLogger.Info($"Colored Output: {config.Cli.ColoredOutput}");
            AppLogger.Info($"Default Output Format: {config.Cli.DefaultOutputFormat}");
            AppLogger.Info($"Enable Auto-completion: {config.Cli.EnableAutoCompletion}");
            AppLogger.Info($"History File: {config.Cli.HistoryFilePath}");
            AppLogger.Info($"Max History Entries: {config.Cli.MaxHistoryEntries}");

            AppLogger.Info("\n--- Runtime Settings ---");
            AppLogger.Info($"Max Execution Time: {config.Runtime.MaxExecutionTime}ms");
            AppLogger.Info($"Max Memory Usage: {config.Runtime.MaxMemoryUsage}MB");
            AppLogger.Info($"Enable Garbage Collection: {config.Runtime.EnableGarbageCollection}");
            AppLogger.Info($"GC Interval: {config.Runtime.GarbageCollectionInterval}ms");
            AppLogger.Info($"Enable Stack Trace: {config.Runtime.EnableStackTrace}");
            AppLogger.Info($"Max Stack Depth: {config.Runtime.MaxStackDepth}");
            AppLogger.Info($"Enable Builtin Caching: {config.Runtime.EnableBuiltinCaching}");
            AppLogger.Info($"Builtin Cache Size: {config.Runtime.BuiltinCacheSize}");
            AppLogger.Info($"Enable Type Checking: {config.Runtime.EnableTypeChecking}");
            AppLogger.Info($"Strict Mode: {config.Runtime.StrictMode}");

            AppLogger.Info("\n--- Logging Settings ---");
            AppLogger.Info($"Log Level: {config.Logging.LogLevel}");
            AppLogger.Info($"Enable File Logging: {config.Logging.EnableFileLogging}");
            AppLogger.Info($"Log File Path: {config.Logging.LogFilePath}");
            AppLogger.Info($"Max Log File Size: {config.Logging.MaxLogFileSize}MB");
            AppLogger.Info($"Max Log Files: {config.Logging.MaxLogFiles}");
            AppLogger.Info($"Enable Console Logging: {config.Logging.EnableConsoleLogging}");
            AppLogger.Info($"Include Timestamps: {config.Logging.IncludeTimestamps}");
            AppLogger.Info($"Include Thread Info: {config.Logging.IncludeThreadInfo}");

            AppLogger.Info("\n--- Development Settings ---");
            AppLogger.Info($"Debug Mode: {config.Development.DebugMode}");
            AppLogger.Info($"Enable Profiling: {config.Development.EnableProfiling}");
            AppLogger.Info($"Detailed Error Reporting: {config.Development.DetailedErrorReporting}");
            AppLogger.Info($"Enable Source Maps: {config.Development.EnableSourceMaps}");
            AppLogger.Info($"Enable Hot Reload: {config.Development.EnableHotReload}");
            AppLogger.Info($"Enable Experimental Features: {config.Development.EnableExperimentalFeatures}");
            AppLogger.Info($"Development Server Port: {config.Development.DevelopmentServerPort}");
            AppLogger.Info($"Enable Remote Debugging: {config.Development.EnableRemoteDebugging}");
            AppLogger.Info($"Remote Debugging Port: {config.Development.RemoteDebuggingPort}");
        }

        private static void SetConfigurationValue(AppConfiguration config, string setValue)
        {
            var parts = setValue.Split('=', 2);
            if (parts.Length != 2)
            {
                AppLogger.Error("Invalid format. Use: section.key=value");
                return;
            }

            var keyPath = parts[0];
            var value = parts[1];

            if (SetConfigValue(config, keyPath, value))
            {
                config.SaveConfiguration();
                AppLogger.Info($"Configuration value '{keyPath}' set to '{value}'");
            }
            else
            {
                AppLogger.Error($"Failed to set configuration value '{keyPath}'");
            }
        }

        private static void GetConfigurationValue(AppConfiguration config, string keyPath)
        {
            var value = GetConfigValue(config, keyPath);
            if (value != null)
            {
                AppLogger.Info($"{keyPath} = {value}");
            }
            else
            {
                AppLogger.Error($"Configuration key '{keyPath}' not found");
            }
        }

        private static void ExportConfiguration(AppConfiguration config, string filePath)
        {
            try
            {
                config.SaveConfiguration();
                AppLogger.Info($"Configuration exported to: {filePath}");
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Failed to export configuration: {ex.Message}");
            }
        }

        private static void ImportConfiguration(AppConfiguration config, string filePath)
        {
            try
            {
                config.LoadConfiguration();
                AppLogger.Info($"Configuration imported from: {filePath}");
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Failed to import configuration: {ex.Message}");
            }
        }

        private static bool SetConfigValue(AppConfiguration config, string keyPath, string value)
        {
            var parts = keyPath.Split('.');
            if (parts.Length != 2)
            {
                return false;
            }

            var section = parts[0].ToLower();
            var key = parts[1];

            try
            {
                switch (section)
                {
                    case "cli":
                        return SetCliValue(config.Cli, key, value);
                    case "runtime":
                        return SetRuntimeValue(config.Runtime, key, value);
                    case "logging":
                        return SetLoggingValue(config.Logging, key, value);
                    case "development":
                        return SetDevelopmentValue(config.Development, key, value);
                    default:
                        return false;
                }
            }
            catch
            {
                return false;
            }
        }

        private static object? GetConfigValue(AppConfiguration config, string keyPath)
        {
            var parts = keyPath.Split('.');
            if (parts.Length != 2)
            {
                return null;
            }

            var section = parts[0].ToLower();
            var key = parts[1];

            switch (section)
            {
                case "cli":
                    return GetCliValue(config.Cli, key);
                case "runtime":
                    return GetRuntimeValue(config.Runtime, key);
                case "logging":
                    return GetLoggingValue(config.Logging, key);
                case "development":
                    return GetDevelopmentValue(config.Development, key);
                default:
                    return null;
            }
        }

        private static bool SetCliValue(CliSettings cli, string key, string value)
        {
            switch (key.ToLower())
            {
                case "defaulttimeout":
                    cli.DefaultTimeout = int.Parse(value);
                    return true;
                case "maxconcurrentoperations":
                    cli.MaxConcurrentOperations = int.Parse(value);
                    return true;
                case "verboseoutput":
                    cli.VerboseOutput = bool.Parse(value);
                    return true;
                case "coloredoutput":
                    cli.ColoredOutput = bool.Parse(value);
                    return true;
                case "defaultoutputformat":
                    cli.DefaultOutputFormat = value;
                    return true;
                case "enableautocompletion":
                    cli.EnableAutoCompletion = bool.Parse(value);
                    return true;
                case "historyfilepath":
                    cli.HistoryFilePath = value;
                    return true;
                case "maxhistoryentries":
                    cli.MaxHistoryEntries = int.Parse(value);
                    return true;
                default:
                    return false;
            }
        }

        private static object? GetCliValue(CliSettings cli, string key)
        {
            return key.ToLower() switch
            {
                "defaulttimeout" => cli.DefaultTimeout,
                "maxconcurrentoperations" => cli.MaxConcurrentOperations,
                "verboseoutput" => cli.VerboseOutput,
                "coloredoutput" => cli.ColoredOutput,
                "defaultoutputformat" => cli.DefaultOutputFormat,
                "enableautocompletion" => cli.EnableAutoCompletion,
                "historyfilepath" => cli.HistoryFilePath,
                "maxhistoryentries" => cli.MaxHistoryEntries,
                _ => null
            };
        }

        private static bool SetRuntimeValue(RuntimeSettings runtime, string key, string value)
        {
            switch (key.ToLower())
            {
                case "maxexecutiontime":
                    runtime.MaxExecutionTime = int.Parse(value);
                    return true;
                case "maxmemoryusage":
                    runtime.MaxMemoryUsage = int.Parse(value);
                    return true;
                case "enablegarbagecollection":
                    runtime.EnableGarbageCollection = bool.Parse(value);
                    return true;
                case "garbagecollectioninterval":
                    runtime.GarbageCollectionInterval = int.Parse(value);
                    return true;
                case "enablestacktrace":
                    runtime.EnableStackTrace = bool.Parse(value);
                    return true;
                case "maxstackdepth":
                    runtime.MaxStackDepth = int.Parse(value);
                    return true;
                case "enablebuiltincaching":
                    runtime.EnableBuiltinCaching = bool.Parse(value);
                    return true;
                case "builtincachesize":
                    runtime.BuiltinCacheSize = int.Parse(value);
                    return true;
                case "enabletypechecking":
                    runtime.EnableTypeChecking = bool.Parse(value);
                    return true;
                case "strictmode":
                    runtime.StrictMode = bool.Parse(value);
                    return true;
                default:
                    return false;
            }
        }

        private static object? GetRuntimeValue(RuntimeSettings runtime, string key)
        {
            return key.ToLower() switch
            {
                "maxexecutiontime" => runtime.MaxExecutionTime,
                "maxmemoryusage" => runtime.MaxMemoryUsage,
                "enablegarbagecollection" => runtime.EnableGarbageCollection,
                "garbagecollectioninterval" => runtime.GarbageCollectionInterval,
                "enablestacktrace" => runtime.EnableStackTrace,
                "maxstackdepth" => runtime.MaxStackDepth,
                "enablebuiltincaching" => runtime.EnableBuiltinCaching,
                "builtincachesize" => runtime.BuiltinCacheSize,
                "enabletypechecking" => runtime.EnableTypeChecking,
                "strictmode" => runtime.StrictMode,
                _ => null
            };
        }

        private static bool SetLoggingValue(LoggingSettings logging, string key, string value)
        {
            switch (key.ToLower())
            {
                case "loglevel":
                    logging.LogLevel = value;
                    return true;
                case "enablefilelogging":
                    logging.EnableFileLogging = bool.Parse(value);
                    return true;
                case "logfilepath":
                    logging.LogFilePath = value;
                    return true;
                case "maxlogfilesize":
                    logging.MaxLogFileSize = int.Parse(value);
                    return true;
                case "maxlogfiles":
                    logging.MaxLogFiles = int.Parse(value);
                    return true;
                case "enableconsolelogging":
                    logging.EnableConsoleLogging = bool.Parse(value);
                    return true;
                case "includetimestamps":
                    logging.IncludeTimestamps = bool.Parse(value);
                    return true;
                case "includethreadinfo":
                    logging.IncludeThreadInfo = bool.Parse(value);
                    return true;
                case "logformat":
                    logging.LogFormat = value;
                    return true;
                default:
                    return false;
            }
        }

        private static object? GetLoggingValue(LoggingSettings logging, string key)
        {
            return key.ToLower() switch
            {
                "loglevel" => logging.LogLevel,
                "enablefilelogging" => logging.EnableFileLogging,
                "logfilepath" => logging.LogFilePath,
                "maxlogfilesize" => logging.MaxLogFileSize,
                "maxlogfiles" => logging.MaxLogFiles,
                "enableconsolelogging" => logging.EnableConsoleLogging,
                "includetimestamps" => logging.IncludeTimestamps,
                "includethreadinfo" => logging.IncludeThreadInfo,
                "logformat" => logging.LogFormat,
                _ => null
            };
        }

        private static bool SetDevelopmentValue(DevelopmentSettings development, string key, string value)
        {
            switch (key.ToLower())
            {
                case "debugmode":
                    development.DebugMode = bool.Parse(value);
                    return true;
                case "enableprofiling":
                    development.EnableProfiling = bool.Parse(value);
                    return true;
                case "detailederrorreporting":
                    development.DetailedErrorReporting = bool.Parse(value);
                    return true;
                case "enablesourcemaps":
                    development.EnableSourceMaps = bool.Parse(value);
                    return true;
                case "enablehotreload":
                    development.EnableHotReload = bool.Parse(value);
                    return true;
                case "enableexperimentalfeatures":
                    development.EnableExperimentalFeatures = bool.Parse(value);
                    return true;
                case "developmentserverport":
                    development.DevelopmentServerPort = int.Parse(value);
                    return true;
                case "enableremotedebugging":
                    development.EnableRemoteDebugging = bool.Parse(value);
                    return true;
                case "remotedebuggingport":
                    development.RemoteDebuggingPort = int.Parse(value);
                    return true;
                default:
                    return false;
            }
        }

        private static object? GetDevelopmentValue(DevelopmentSettings development, string key)
        {
            return key.ToLower() switch
            {
                "debugmode" => development.DebugMode,
                "enableprofiling" => development.EnableProfiling,
                "detailederrorreporting" => development.DetailedErrorReporting,
                "enablesourcemaps" => development.EnableSourceMaps,
                "enablehotreload" => development.EnableHotReload,
                "enableexperimentalfeatures" => development.EnableExperimentalFeatures,
                "developmentserverport" => development.DevelopmentServerPort,
                "enableremotedebugging" => development.EnableRemoteDebugging,
                "remotedebuggingport" => development.RemoteDebuggingPort,
                _ => null
            };
        }
    }
}
