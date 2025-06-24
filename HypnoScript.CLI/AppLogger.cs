using System;
using Microsoft.Extensions.Logging;

namespace HypnoScript.CLI
{
    public static class AppLogger
    {
        private static ILogger? _logger;

        public static void Configure(ILogger logger)
        {
            _logger = logger;
        }

        public static void Info(string message)
        {
            if (_logger != null)
                _logger.LogInformation(message);
            else
                Console.WriteLine($"[INFO] {message}");
        }

        public static void Warn(string message)
        {
            if (_logger != null)
                _logger.LogWarning(message);
            else
                Console.WriteLine($"[WARN] {message}");
        }

        public static void Error(string message, Exception? ex = null)
        {
            if (_logger != null)
                _logger.LogError(ex, message);
            else
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.Error.WriteLine($"[ERROR] {message}");
                if (ex != null)
                {
                    Console.Error.WriteLine($"  {ex.GetType().Name}: {ex.Message}");
                    Console.Error.WriteLine(ex.StackTrace);
                }
                Console.ResetColor();
            }
        }

        public static void Debug(string message)
        {
            if (_logger != null)
                _logger.LogDebug(message);
            else
                Console.WriteLine($"[DEBUG] {message}");
        }
    }
}
