using System;
using System.IO;
using HypnoScript.CLI;

namespace HypnoScript.CLI.Commands
{
    public static class InfoCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== FILE INFO MODE ===");

            if (!File.Exists(filePath))
            {
                AppLogger.Error($"File not found: {filePath}");
                return 2;
            }

            try
            {
                var fileInfo = new FileInfo(filePath);
                AppLogger.Info("📁 File Information:");
                AppLogger.Info($"  Name: {fileInfo.Name}");
                AppLogger.Info($"  Size: {fileInfo.Length} bytes");
                AppLogger.Info($"  Created: {fileInfo.CreationTime}");
                AppLogger.Info($"  Modified: {fileInfo.LastWriteTime}");
                AppLogger.Info($"  Extension: {fileInfo.Extension}");

                if (verbose)
                {
                    var source = File.ReadAllText(filePath);
                    AppLogger.Info($"  Lines: {source.Split('\n').Length}");
                    AppLogger.Info($"  Characters: {source.Length}");
                }

                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Failed to get file info for {filePath}", ex);
                return 1;
            }
        }
    }
}
