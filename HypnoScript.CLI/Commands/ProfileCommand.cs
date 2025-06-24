using System;
using System.IO;
using HypnoScript.CLI;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Interpreter;
using System.Diagnostics;

namespace HypnoScript.CLI.Commands
{
    public static class ProfileCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== PROFILE MODE ===");
            if (!File.Exists(filePath))
            {
                AppLogger.Error($"File not found: {filePath}");
                return 1;
            }
            string source = File.ReadAllText(filePath);
            try
            {
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                var interpreter = new HypnoInterpreter();
                var process = Process.GetCurrentProcess();
                process.Refresh();
                long memBefore = process.PrivateMemorySize64;
                var sw = Stopwatch.StartNew();
                interpreter.ExecuteProgram(program);
                sw.Stop();
                process.Refresh();
                long memAfter = process.PrivateMemorySize64;
                AppLogger.Info($"Execution time: {sw.ElapsedMilliseconds} ms");
                AppLogger.Info($"Memory usage: {memAfter / 1024} KB (delta: {(memAfter - memBefore) / 1024} KB)");
                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Profile error: {ex.Message}");
                if (debug)
                    AppLogger.Error(ex.StackTrace ?? "No stacktrace available.");
                return 1;
            }
        }
    }
}
