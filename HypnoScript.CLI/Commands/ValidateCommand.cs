using System;
using System.IO;
using System.Linq;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Analysis;
using HypnoScript.CLI;

namespace HypnoScript.CLI.Commands
{
    public static class ValidateCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== VALIDATE MODE ===");

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

                var typeChecker = new TypeChecker();
                typeChecker.Check(program);

                AppLogger.Info("✓ Validation successful!");
                AppLogger.Info("  ✓ Syntax: OK");
                AppLogger.Info("  ✓ Semantics: OK");
                AppLogger.Info("  ✓ Types: OK");

                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Validation failed for {filePath}", ex);
                return 1;
            }
        }
    }
}
