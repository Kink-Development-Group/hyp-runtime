using System;
using System.IO;
using HypnoScript.CLI;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Interpreter;
using System.Diagnostics;

namespace HypnoScript.CLI.Commands
{
    public static class BenchmarkCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== BENCHMARK MODE ===");
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
                var sw = Stopwatch.StartNew();
                interpreter.ExecuteProgram(program);
                sw.Stop();
                AppLogger.Info($"Execution time: {sw.ElapsedMilliseconds} ms");
                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Benchmark error: {ex.Message}");
                if (debug)
                    AppLogger.Error(ex.StackTrace ?? "No stacktrace available.");
                return 1;
            }
        }
    }
}
