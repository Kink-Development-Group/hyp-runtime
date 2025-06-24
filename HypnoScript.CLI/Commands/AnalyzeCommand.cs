using System;
using System.IO;
using System.Linq;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.CLI;

namespace HypnoScript.CLI.Commands
{
    public static class AnalyzeCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== ANALYZE MODE ===");

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

                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();

                AppLogger.Info("📊 Analysis Results:");
                AppLogger.Info($"  File size: {source.Length} characters");
                AppLogger.Info($"  Lines of code: {source.Split('\n').Length}");
                AppLogger.Info($"  Tokens: {tokens.Count}");
                AppLogger.Info($"  Statements: {program.Statements.Count}");

                if (verbose)
                {
                    AppLogger.Info("\n🔍 Detailed Analysis:");
                    var tokenTypes = tokens.GroupBy(t => t.Type).OrderByDescending(g => g.Count());
                    foreach (var group in tokenTypes)
                    {
                        AppLogger.Info($"  {group.Key}: {group.Count()}");
                    }
                }

                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Analysis failed for {filePath}", ex);
                return 1;
            }
        }
    }
}
