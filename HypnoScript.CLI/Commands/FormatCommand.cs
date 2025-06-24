using System;
using System.IO;
using HypnoScript.CLI;

namespace HypnoScript.CLI.Commands
{
    public static class FormatCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== FORMAT MODE ===");

            if (!File.Exists(filePath))
            {
                AppLogger.Error($"File not found: {filePath}");
                return 2;
            }

            try
            {
                if (debug) AppLogger.Debug($"Reading file: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) AppLogger.Debug($"File read, length: {source.Length}");
                // Simple formatting - in a real implementation, this would be more sophisticated
                var formatted = source.Replace("\r\n", "\n").Replace("\r", "\n");
                File.WriteAllText(filePath, formatted);

                AppLogger.Info("✓ File formatted successfully!");
                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Formatting failed for {filePath}", ex);
                return 1;
            }
        }
    }
}
