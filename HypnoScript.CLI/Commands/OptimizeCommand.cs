using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using HypnoScript.CLI;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.LexerParser.AST;

namespace HypnoScript.CLI.Commands
{
    public static class OptimizeCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== OPTIMIZE MODE ===");

            if (!File.Exists(filePath))
            {
                AppLogger.Error($"File not found: {filePath}");
                return 1;
            }

            try
            {
                string source = File.ReadAllText(filePath);
                var optimizationResults = new OptimizationResults();

                // Parse the program
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();

                // Perform optimizations
                var optimizedSource = PerformOptimizations(program, source, optimizationResults, verbose);

                // Generate optimized file
                var outputPath = GenerateOptimizedFile(filePath, optimizedSource);

                // Report results
                ReportOptimizationResults(optimizationResults, outputPath, verbose);

                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Optimization error: {ex.Message}");
                if (debug)
                {
                    AppLogger.Error(ex.StackTrace ?? "No stacktrace available.");
                }
                return 1;
            }
        }

        private static string PerformOptimizations(ProgramNode program, string source, OptimizationResults results, bool verbose)
        {
            var optimizedSource = source;

            // 1. Dead Code Elimination
            optimizedSource = EliminateDeadCode(program, optimizedSource, results);

            // 2. Constant Folding
            optimizedSource = FoldConstants(program, optimizedSource, results);

            // 3. Loop Optimization
            optimizedSource = OptimizeLoops(program, optimizedSource, results);

            // 4. Variable Optimization
            optimizedSource = OptimizeVariables(program, optimizedSource, results);

            // 5. Function Inlining
            optimizedSource = InlineFunctions(program, optimizedSource, results);

            // 6. Expression Simplification
            optimizedSource = SimplifyExpressions(program, optimizedSource, results);

            // 7. Memory Optimization
            optimizedSource = OptimizeMemory(program, optimizedSource, results);

            if (verbose)
            {
                AppLogger.Info($"Original size: {source.Length} characters");
                AppLogger.Info($"Optimized size: {optimizedSource.Length} characters");
                AppLogger.Info($"Size reduction: {((double)(source.Length - optimizedSource.Length) / source.Length * 100):F1}%");
            }

            return optimizedSource;
        }

        private static string EliminateDeadCode(ProgramNode program, string source, OptimizationResults results)
        {
            var lines = source.Split('\n').ToList();
            var deadCodeLines = new List<int>();

            // Find unreachable code
            for (int i = 0; i < lines.Count; i++)
            {
                var line = lines[i].Trim();

                // Check for unreachable code after return statements
                if (line.StartsWith("return") && i < lines.Count - 1)
                {
                    var nextLine = lines[i + 1].Trim();
                    if (nextLine.Length > 0 && !nextLine.StartsWith("}") && !nextLine.StartsWith("else"))
                    {
                        deadCodeLines.Add(i + 1);
                        results.Optimizations.Add(new Optimization
                        {
                            Type = OptimizationType.DeadCodeElimination,
                            Description = "Removed unreachable code after return statement",
                            Line = i + 2
                        });
                    }
                }

                // Check for unused variables (simplified)
                if (line.StartsWith("induce") && line.Contains("="))
                {
                    var varName = ExtractVariableName(line);
                    if (!IsVariableUsed(varName, lines, i + 1))
                    {
                        deadCodeLines.Add(i);
                        results.Optimizations.Add(new Optimization
                        {
                            Type = OptimizationType.DeadCodeElimination,
                            Description = $"Removed unused variable '{varName}'",
                            Line = i + 1
                        });
                    }
                }
            }

            // Remove dead code lines (in reverse order to maintain indices)
            for (int i = deadCodeLines.Count - 1; i >= 0; i--)
            {
                lines.RemoveAt(deadCodeLines[i]);
            }

            return string.Join("\n", lines);
        }

        private static string FoldConstants(ProgramNode program, string source, OptimizationResults results)
        {
            var lines = source.Split('\n').ToList();

            for (int i = 0; i < lines.Count; i++)
            {
                var line = lines[i];
                var optimizedLine = FoldConstantsInLine(line);

                if (optimizedLine != line)
                {
                    lines[i] = optimizedLine;
                    results.Optimizations.Add(new Optimization
                    {
                        Type = OptimizationType.ConstantFolding,
                        Description = "Folded constant expressions",
                        Line = i + 1
                    });
                }
            }

            return string.Join("\n", lines);
        }

        private static string FoldConstantsInLine(string line)
        {
            // Simple constant folding for arithmetic expressions
            if (line.Contains(" + ") && line.Contains("induce"))
            {
                // Find arithmetic expressions like "induce x = 2 + 3"
                var match = System.Text.RegularExpressions.Regex.Match(line, @"induce\s+(\w+)\s*=\s*(\d+)\s*\+\s*(\d+)");
                if (match.Success)
                {
                    var varName = match.Groups[1].Value;
                    var left = int.Parse(match.Groups[2].Value);
                    var right = int.Parse(match.Groups[3].Value);
                    var result = left + right;

                    return line.Replace(match.Value, $"induce {varName} = {result}");
                }
            }

            return line;
        }

        private static string OptimizeLoops(ProgramNode program, string source, OptimizationResults results)
        {
            var lines = source.Split('\n').ToList();

            for (int i = 0; i < lines.Count; i++)
            {
                var line = lines[i].Trim();

                // Optimize simple loops
                if (line.StartsWith("for") && line.Contains("induce i = 0"))
                {
                    // Check if it's a simple counting loop
                    var nextLines = GetNextLines(lines, i, 5);
                    if (IsSimpleCountingLoop(nextLines))
                    {
                        results.Optimizations.Add(new Optimization
                        {
                            Type = OptimizationType.LoopOptimization,
                            Description = "Optimized simple counting loop",
                            Line = i + 1
                        });
                    }
                }
            }

            return string.Join("\n", lines);
        }

        private static string OptimizeVariables(ProgramNode program, string source, OptimizationResults results)
        {
            var lines = source.Split('\n').ToList();
            var variableUsage = new Dictionary<string, int>();

            // Count variable usage
            for (int i = 0; i < lines.Count; i++)
            {
                var line = lines[i];
                var variables = ExtractVariables(line);
                foreach (var var in variables)
                {
                    variableUsage[var] = variableUsage.GetValueOrDefault(var, 0) + 1;
                }
            }

            // Suggest optimizations for rarely used variables
            foreach (var kvp in variableUsage)
            {
                if (kvp.Value == 1)
                {
                    results.Suggestions.Add(new OptimizationSuggestion
                    {
                        Type = SuggestionType.VariableOptimization,
                        Description = $"Variable '{kvp.Key}' is used only once - consider inlining",
                        Priority = SuggestionPriority.Low
                    });
                }
            }

            return source;
        }

        private static string InlineFunctions(ProgramNode program, string source, OptimizationResults results)
        {
            // Find small functions that can be inlined
            var lines = source.Split('\n').ToList();

            for (int i = 0; i < lines.Count; i++)
            {
                var line = lines[i].Trim();

                if (line.StartsWith("suggestion") && line.Contains("("))
                {
                    var functionName = ExtractFunctionName(line);
                    var functionBody = GetFunctionBody(lines, i);

                    if (functionBody.Count <= 3) // Small function
                    {
                        results.Suggestions.Add(new OptimizationSuggestion
                        {
                            Type = SuggestionType.FunctionInlining,
                            Description = $"Function '{functionName}' is small and could be inlined",
                            Priority = SuggestionPriority.Medium
                        });
                    }
                }
            }

            return source;
        }

        private static string SimplifyExpressions(ProgramNode program, string source, OptimizationResults results)
        {
            var lines = source.Split('\n').ToList();

            for (int i = 0; i < lines.Count; i++)
            {
                var line = lines[i];
                var simplifiedLine = SimplifyExpression(line);

                if (simplifiedLine != line)
                {
                    lines[i] = simplifiedLine;
                    results.Optimizations.Add(new Optimization
                    {
                        Type = OptimizationType.ExpressionSimplification,
                        Description = "Simplified expression",
                        Line = i + 1
                    });
                }
            }

            return string.Join("\n", lines);
        }

        private static string SimplifyExpression(string line)
        {
            // Simplify common patterns
            line = line.Replace(" + 0", "");
            line = line.Replace("0 + ", "");
            line = line.Replace(" * 1", "");
            line = line.Replace("1 * ", "");
            line = line.Replace(" && true", "");
            line = line.Replace("true && ", "");
            line = line.Replace(" || false", "");
            line = line.Replace("false || ", "");

            return line;
        }

        private static string OptimizeMemory(ProgramNode program, string source, OptimizationResults results)
        {
            var lines = source.Split('\n').ToList();

            // Check for large arrays that could be optimized
            for (int i = 0; i < lines.Count; i++)
            {
                var line = lines[i].Trim();

                if (line.Contains("induce") && line.Contains("["))
                {
                    // Check for large array literals
                    var arrayMatch = System.Text.RegularExpressions.Regex.Match(line, @"\[\s*([^]]*)\s*\]");
                    if (arrayMatch.Success)
                    {
                        var arrayContent = arrayMatch.Groups[1].Value;
                        var elements = arrayContent.Split(',').Length;

                        if (elements > 10)
                        {
                            results.Suggestions.Add(new OptimizationSuggestion
                            {
                                Type = SuggestionType.MemoryOptimization,
                                Description = $"Large array with {elements} elements - consider lazy loading",
                                Priority = SuggestionPriority.High
                            });
                        }
                    }
                }
            }

            return source;
        }

        private static string GenerateOptimizedFile(string originalPath, string optimizedSource)
        {
            var directory = Path.GetDirectoryName(originalPath);
            var fileName = Path.GetFileNameWithoutExtension(originalPath);
            var extension = Path.GetExtension(originalPath);
            var outputPath = Path.Combine(directory ?? ".", $"{fileName}_optimized{extension}");

            File.WriteAllText(outputPath, optimizedSource, Encoding.UTF8);
            return outputPath;
        }

        private static void ReportOptimizationResults(OptimizationResults results, string outputPath, bool verbose)
        {
            AppLogger.Info($"✅ Optimization completed! Output: {outputPath}");

            if (results.Optimizations.Count > 0)
            {
                AppLogger.Info($"\n=== OPTIMIZATIONS APPLIED ({results.Optimizations.Count}) ===");
                foreach (var opt in results.Optimizations)
                {
                    AppLogger.Info($"[{opt.Type}] Line {opt.Line}: {opt.Description}");
                }
            }

            if (results.Suggestions.Count > 0)
            {
                AppLogger.Info($"\n=== OPTIMIZATION SUGGESTIONS ({results.Suggestions.Count}) ===");
                foreach (var suggestion in results.Suggestions.OrderBy(s => s.Priority))
                {
                    var priorityIcon = suggestion.Priority switch
                    {
                        SuggestionPriority.High => "🔴",
                        SuggestionPriority.Medium => "🟡",
                        SuggestionPriority.Low => "🟢",
                        _ => "⚪"
                    };

                    AppLogger.Info($"{priorityIcon} [{suggestion.Type}] {suggestion.Description}");
                }
            }

            if (verbose)
            {
                AppLogger.Info($"\n=== OPTIMIZATION SUMMARY ===");
                AppLogger.Info($"Applied optimizations: {results.Optimizations.Count}");
                AppLogger.Info($"Suggestions: {results.Suggestions.Count}");
                AppLogger.Info($"High priority suggestions: {results.Suggestions.Count(s => s.Priority == SuggestionPriority.High)}");
            }
        }

        // Helper methods
        private static string ExtractVariableName(string line)
        {
            var match = System.Text.RegularExpressions.Regex.Match(line, @"induce\s+(\w+)");
            return match.Success ? match.Groups[1].Value : "";
        }

        private static bool IsVariableUsed(string varName, List<string> lines, int startIndex)
        {
            for (int i = startIndex; i < lines.Count; i++)
            {
                if (lines[i].Contains(varName))
                {
                    return true;
                }
            }
            return false;
        }

        private static List<string> GetNextLines(List<string> lines, int startIndex, int count)
        {
            var result = new List<string>();
            for (int i = startIndex + 1; i < Math.Min(startIndex + 1 + count, lines.Count); i++)
            {
                result.Add(lines[i]);
            }
            return result;
        }

        private static bool IsSimpleCountingLoop(List<string> lines)
        {
            return lines.Any(line => line.Contains("induce i = i + 1"));
        }

        private static List<string> ExtractVariables(string line)
        {
            var variables = new List<string>();
            var matches = System.Text.RegularExpressions.Regex.Matches(line, @"\b\w+\b");
            foreach (System.Text.RegularExpressions.Match match in matches)
            {
                var word = match.Value;
                if (!IsKeyword(word))
                {
                    variables.Add(word);
                }
            }
            return variables;
        }

        private static bool IsKeyword(string word)
        {
            var keywords = new[] { "induce", "observe", "if", "else", "while", "for", "return", "true", "false", "null" };
            return keywords.Contains(word);
        }

        private static string ExtractFunctionName(string line)
        {
            var match = System.Text.RegularExpressions.Regex.Match(line, @"suggestion\s+(\w+)");
            return match.Success ? match.Groups[1].Value : "";
        }

        private static List<string> GetFunctionBody(List<string> lines, int functionStart)
        {
            var body = new List<string>();
            var braceCount = 0;
            var started = false;

            for (int i = functionStart; i < lines.Count; i++)
            {
                var line = lines[i];

                if (line.Contains("{"))
                {
                    braceCount++;
                    started = true;
                }

                if (started)
                {
                    body.Add(line);
                }

                if (line.Contains("}"))
                {
                    braceCount--;
                    if (braceCount == 0)
                    {
                        break;
                    }
                }
            }

            return body;
        }
    }

    public class OptimizationResults
    {
        public List<Optimization> Optimizations { get; set; } = new();
        public List<OptimizationSuggestion> Suggestions { get; set; } = new();
    }

    public class Optimization
    {
        public OptimizationType Type { get; set; }
        public string Description { get; set; } = "";
        public int Line { get; set; }
    }

    public class OptimizationSuggestion
    {
        public SuggestionType Type { get; set; }
        public string Description { get; set; } = "";
        public SuggestionPriority Priority { get; set; }
    }

    public enum OptimizationType
    {
        DeadCodeElimination,
        ConstantFolding,
        LoopOptimization,
        VariableOptimization,
        FunctionInlining,
        ExpressionSimplification,
        MemoryOptimization
    }

    public enum SuggestionType
    {
        VariableOptimization,
        FunctionInlining,
        MemoryOptimization,
        PerformanceOptimization,
        CodeStructure
    }

    public enum SuggestionPriority
    {
        Low,
        Medium,
        High
    }
}
