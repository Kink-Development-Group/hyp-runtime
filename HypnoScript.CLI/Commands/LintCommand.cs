using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using HypnoScript.CLI;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.LexerParser.AST;
using HypnoScript.Compiler.Analysis;

namespace HypnoScript.CLI.Commands
{
    public static class LintCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== LINT MODE ===");

            if (!File.Exists(filePath))
            {
                AppLogger.Error($"File not found: {filePath}");
                return 1;
            }

            string source = File.ReadAllText(filePath);
            try
            {
                var lintResults = new LintResults();

                // Tokenisierung
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (tokens.Count == 0)
                {
                    AppLogger.Warn("No tokens found. File may be empty or invalid.");
                    return 1;
                }

                if (verbose)
                {
                    foreach (var token in tokens)
                    {
                        AppLogger.Debug($"Token: {token.Type} '{token.Lexeme}' @ {token.Line}:{token.Column}");
                    }
                }

                // Parsing
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();

                // Erweiterte Linting-Analyse
                PerformLintingAnalysis(program, source, lintResults, verbose);

                // Ergebnisse ausgeben
                ReportLintResults(lintResults, verbose);

                if (lintResults.Errors.Count > 0)
                {
                    AppLogger.Error($"Found {lintResults.Errors.Count} errors and {lintResults.Warnings.Count} warnings.");
                    return 1;
                }
                else if (lintResults.Warnings.Count > 0)
                {
                    AppLogger.Warn($"Found {lintResults.Warnings.Count} warnings.");
                    return 0;
                }
                else
                {
                    AppLogger.Info("No linting issues found.");
                    return 0;
                }
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Linting error: {ex.Message}");
                if (debug)
                {
                    AppLogger.Error(ex.StackTrace ?? "No stacktrace available.");
                }
                return 1;
            }
        }

        private static void PerformLintingAnalysis(ProgramNode program, string source, LintResults results, bool verbose)
        {
            var lines = source.Split('\n');

            // Syntax-Analyse
            AnalyzeSyntax(program, results);

            // Stil-Analyse
            AnalyzeStyle(program, lines, results);

            // Performance-Analyse
            AnalyzePerformance(program, results);

            // Sicherheits-Analyse
            AnalyzeSecurity(program, results);

            // Best Practices
            AnalyzeBestPractices(program, results);
        }

        private static void AnalyzeSyntax(ProgramNode program, LintResults results)
        {
            // Prüfe auf Focus/Relax-Struktur
            bool hasFocus = false;

            foreach (var stmt in program.Statements)
            {
                if (stmt is EntranceBlockNode)
                {
                    hasFocus = true;
                }
            }

            if (!hasFocus)
            {
                results.Warnings.Add(new LintIssue
                {
                    Type = LintIssueType.Warning,
                    Message = "Program should have an entrance block",
                    Line = 1,
                    Column = 1,
                    Code = "LINT001"
                });
            }
        }

        private static void AnalyzeStyle(ProgramNode program, string[] lines, LintResults results)
        {
            // Prüfe Zeilenlänge
            for (int i = 0; i < lines.Length; i++)
            {
                if (lines[i].Length > 120)
                {
                    results.Warnings.Add(new LintIssue
                    {
                        Type = LintIssueType.Warning,
                        Message = "Line is too long (>120 characters)",
                        Line = i + 1,
                        Column = 1,
                        Code = "LINT002"
                    });
                }
            }

            // Prüfe Einrückung
            for (int i = 0; i < lines.Length; i++)
            {
                var line = lines[i];
                if (line.Trim().Length > 0 && !line.StartsWith("    ") && !line.StartsWith("\t"))
                {
                    // Erste Zeile und spezielle Zeilen ausnehmen
                    if (i > 0 && !line.Trim().StartsWith("Focus") && !line.Trim().StartsWith("Relax"))
                    {
                        results.Warnings.Add(new LintIssue
                        {
                            Type = LintIssueType.Warning,
                            Message = "Inconsistent indentation",
                            Line = i + 1,
                            Column = 1,
                            Code = "LINT003"
                        });
                    }
                }
            }
        }

        private static void AnalyzePerformance(ProgramNode program, LintResults results)
        {
            int loopCount = 0;
            int functionCount = 0;

            void CountStatements(IStatement stmt)
            {
                switch (stmt)
                {
                    case WhileStatementNode:
                    case LoopStatementNode:
                        loopCount++;
                        break;
                    case FunctionDeclNode:
                        functionCount++;
                        break;
                    case BlockStatementNode block:
                        foreach (var s in block.Statements) CountStatements(s);
                        break;
                    case EntranceBlockNode entrance:
                        foreach (var s in entrance.Statements) CountStatements(s);
                        break;
                    case IfStatementNode ifStmt:
                        foreach (var s in ifStmt.ThenBranch) CountStatements(s);
                        if (ifStmt.ElseBranch != null)
                            foreach (var s in ifStmt.ElseBranch) CountStatements(s);
                        break;
                }
            }

            foreach (var stmt in program.Statements) CountStatements(stmt);

            if (loopCount > 5)
            {
                results.Warnings.Add(new LintIssue
                {
                    Type = LintIssueType.Warning,
                    Message = $"Too many loops ({loopCount}). Consider optimizing.",
                    Line = 1,
                    Column = 1,
                    Code = "LINT004"
                });
            }

            if (functionCount == 0)
            {
                results.Warnings.Add(new LintIssue
                {
                    Type = LintIssueType.Warning,
                    Message = "No functions defined. Consider modularizing your code.",
                    Line = 1,
                    Column = 1,
                    Code = "LINT005"
                });
            }
        }

        private static void AnalyzeSecurity(ProgramNode program, LintResults results)
        {
            // Prüfe auf potenzielle Sicherheitsprobleme
            void CheckSecurity(IStatement stmt)
            {
                switch (stmt)
                {
                    case VarDeclNode varDecl:
                        if (varDecl.FromExternal && varDecl.TypeName == "string")
                        {
                            results.Warnings.Add(new LintIssue
                            {
                                Type = LintIssueType.Warning,
                                Message = "External string input should be validated",
                                Line = 1,
                                Column = 1,
                                Code = "LINT006"
                            });
                        }
                        break;
                    case BlockStatementNode block:
                        foreach (var s in block.Statements) CheckSecurity(s);
                        break;
                    case EntranceBlockNode entrance:
                        foreach (var s in entrance.Statements) CheckSecurity(s);
                        break;
                    case IfStatementNode ifStmt:
                        foreach (var s in ifStmt.ThenBranch) CheckSecurity(s);
                        if (ifStmt.ElseBranch != null)
                            foreach (var s in ifStmt.ElseBranch) CheckSecurity(s);
                        break;
                }
            }

            foreach (var stmt in program.Statements) CheckSecurity(stmt);
        }

        private static void AnalyzeBestPractices(ProgramNode program, LintResults results)
        {
            // Prüfe Best Practices
            int variableCount = 0;
            var variableNames = new HashSet<string>();

            void CheckBestPractices(IStatement stmt)
            {
                switch (stmt)
                {
                    case VarDeclNode varDecl:
                        variableCount++;
                        if (!variableNames.Add(varDecl.Identifier))
                        {
                            results.Errors.Add(new LintIssue
                            {
                                Type = LintIssueType.Error,
                                Message = $"Variable '{varDecl.Identifier}' is already defined",
                                Line = 1,
                                Column = 1,
                                Code = "LINT007"
                            });
                        }

                        // Prüfe Namenskonventionen
                        if (varDecl.Identifier.Length < 2)
                        {
                            results.Warnings.Add(new LintIssue
                            {
                                Type = LintIssueType.Warning,
                                Message = $"Variable name '{varDecl.Identifier}' is too short",
                                Line = 1,
                                Column = 1,
                                Code = "LINT008"
                            });
                        }
                        break;
                    case FunctionDeclNode funcDecl:
                        if (funcDecl.Name.Length < 3)
                        {
                            results.Warnings.Add(new LintIssue
                            {
                                Type = LintIssueType.Warning,
                                Message = $"Function name '{funcDecl.Name}' is too short",
                                Line = 1,
                                Column = 1,
                                Code = "LINT009"
                            });
                        }
                        break;
                    case BlockStatementNode block:
                        foreach (var s in block.Statements) CheckBestPractices(s);
                        break;
                    case EntranceBlockNode entrance:
                        foreach (var s in entrance.Statements) CheckBestPractices(s);
                        break;
                    case IfStatementNode ifStmt:
                        foreach (var s in ifStmt.ThenBranch) CheckBestPractices(s);
                        if (ifStmt.ElseBranch != null)
                            foreach (var s in ifStmt.ElseBranch) CheckBestPractices(s);
                        break;
                }
            }

            foreach (var stmt in program.Statements) CheckBestPractices(stmt);

            if (variableCount > 20)
            {
                results.Warnings.Add(new LintIssue
                {
                    Type = LintIssueType.Warning,
                    Message = $"Too many variables ({variableCount}). Consider using records or arrays.",
                    Line = 1,
                    Column = 1,
                    Code = "LINT010"
                });
            }
        }

        private static void ReportLintResults(LintResults results, bool verbose)
        {
            if (results.Errors.Count > 0)
            {
                AppLogger.Info("\n=== ERRORS ===");
                foreach (var error in results.Errors)
                {
                    AppLogger.Error($"[{error.Code}] Line {error.Line}:{error.Column} - {error.Message}");
                }
            }

            if (results.Warnings.Count > 0)
            {
                AppLogger.Info("\n=== WARNINGS ===");
                foreach (var warning in results.Warnings)
                {
                    AppLogger.Warn($"[{warning.Code}] Line {warning.Line}:{warning.Column} - {warning.Message}");
                }
            }

            if (verbose)
            {
                AppLogger.Info("\n=== SUMMARY ===");
                AppLogger.Info($"Errors: {results.Errors.Count}");
                AppLogger.Info($"Warnings: {results.Warnings.Count}");
                AppLogger.Info($"Total Issues: {results.Errors.Count + results.Warnings.Count}");
            }
        }
    }

    public class LintResults
    {
        public List<LintIssue> Errors { get; set; } = new();
        public List<LintIssue> Warnings { get; set; } = new();
    }

    public class LintIssue
    {
        public LintIssueType Type { get; set; }
        public string Message { get; set; } = "";
        public int Line { get; set; }
        public int Column { get; set; }
        public string Code { get; set; } = "";
    }

    public enum LintIssueType
    {
        Error,
        Warning,
        Info
    }
}
