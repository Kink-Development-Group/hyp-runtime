using System;

namespace HypnoScript.Compiler.Error
{
    public static class ErrorReporter
    {
        // Enterprise-Level: Verwende Fehlercodes und farbliche Logausgaben (z.B. über Console.ForegroundColor)
        public static void Report(string message, int line, int column, string errorCode = "E001")
        {
            var prevColor = Console.ForegroundColor;
            Console.ForegroundColor = ConsoleColor.Red;
            Console.Error.WriteLine($"[{errorCode}] Error at {line}:{column} - {message}");
            Console.ForegroundColor = prevColor;
        }

        // Eine Erweiterungsmethode zur Abschaltung von Fehlern oder zum Sammeln in einem Log
        public static void ReportWarning(string message, int line, int column, string warningCode = "W001")
        {
            var prevColor = Console.ForegroundColor;
            Console.ForegroundColor = ConsoleColor.Yellow;
            Console.Error.WriteLine($"[{warningCode}] Warning at {line}:{column} - {message}");
            Console.ForegroundColor = prevColor;
        }
    }
}
