using System;
using System.IO;
using System.Linq;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Analysis;
using HypnoScript.Compiler.Interpreter;
using HypnoScript.CLI;

namespace HypnoScript.CLI.Commands
{
    public static class RunCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== RUN MODE ===");

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

                // Syntax validation
                if (!source.TrimStart().StartsWith("Focus"))
                {
                    AppLogger.Warn("File doesn't start with 'Focus'");
                    return 1;
                }
                AppLogger.Info("✓ File starts with 'Focus' - syntax OK");

                // Lexer
                if (debug) AppLogger.Debug("Creating lexer...");
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (debug) AppLogger.Debug($"{tokens.Count} tokens generated");
                AppLogger.Info("✓ Lexing successful!");

                if (verbose)
                {
                    AppLogger.Info("\n📋 Token Analysis:");
                    var tokenTypes = tokens.GroupBy(t => t.Type).OrderByDescending(g => g.Count());
                    foreach (var group in tokenTypes.Take(10))
                    {
                        AppLogger.Info($"  {group.Key}: {group.Count()} tokens");
                    }
                }

                // Parser
                if (debug) AppLogger.Debug("Creating parser...");
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                if (debug) AppLogger.Debug($"AST with {program.Statements.Count} statements created");
                AppLogger.Info("✓ Parsing successful!");

                if (verbose)
                {
                    AppLogger.Info("\n🌳 AST Analysis:");
                    var statementTypes = program.Statements.GroupBy(s => s.GetType().Name).OrderByDescending(g => g.Count());
                    foreach (var group in statementTypes.Take(5))
                    {
                        AppLogger.Info($"  {group.Key}: {group.Count()} statements");
                    }
                }

                // Type Checker
                if (debug) AppLogger.Debug("Running type checker...");
                var typeChecker = new TypeChecker();
                typeChecker.Check(program);
                AppLogger.Info("✓ Type checking successful!");

                // Interpreter
                if (debug) AppLogger.Debug("Starting interpreter...");
                var interpreter = new HypnoInterpreter();
                var startTime = DateTime.Now;
                interpreter.ExecuteProgram(program);
                var endTime = DateTime.Now;
                var executionTime = (endTime - startTime).TotalMilliseconds;

                var assertionFailures = interpreter.GetAssertionFailures();
                if (assertionFailures.Count > 0)
                {
                    AppLogger.Error($"{assertionFailures.Count} assertion(s) failed in {filePath}:");
                    foreach (var fail in assertionFailures)
                    {
                        AppLogger.Error($"   - {fail}");
                    }
                    return 1;
                }

                AppLogger.Info("✓ Execution completed!");
                AppLogger.Info($"⏱️  Execution time: {executionTime:F2}ms");

                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Execution failed for {filePath}", ex);
                return 1;
            }
        }
    }
}
