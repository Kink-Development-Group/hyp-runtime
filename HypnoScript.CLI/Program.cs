using System;
using System.IO;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Analysis;
using HypnoScript.Compiler.Interpreter;
using HypnoScript.Compiler.CodeGen;
using HypnoScript.LexerParser.AST;

namespace HypnoScript.CLI
{
    public class Program
    {
        public static int Main(string[] args)
        {
            Console.WriteLine("=== HypnoScript CLI - Enterprise Edition ===");
            Console.WriteLine($"Version: 2.0.0 | Args: {string.Join(" ", args)}");

            if (args.Length < 1)
            {
                ShowUsage();
                return 1;
            }

            var command = args[0].ToLower();
            var debug = args.Contains("--debug");
            var verbose = args.Contains("--verbose");

            try
            {
                switch (command)
                {
                    case "run":
                        if (args.Length < 2)
                        {
                            Console.WriteLine("Error: File path required for 'run' command");
                            return 1;
                        }
                        return RunFile(args[1], debug, verbose);
                    case "compile":
                        if (args.Length < 2)
                        {
                            Console.WriteLine("Error: File path required for 'compile' command");
                            return 1;
                        }
                        return CompileFile(args[1], debug, verbose);
                    case "analyze":
                        if (args.Length < 2)
                        {
                            Console.WriteLine("Error: File path required for 'analyze' command");
                            return 1;
                        }
                        return AnalyzeFile(args[1], debug, verbose);
                    case "info":
                        if (args.Length < 2)
                        {
                            Console.WriteLine("Error: File path required for 'info' command");
                            return 1;
                        }
                        return ShowFileInfo(args[1], debug, verbose);
                    case "validate":
                        if (args.Length < 2)
                        {
                            Console.WriteLine("Error: File path required for 'validate' command");
                            return 1;
                        }
                        return ValidateFile(args[1], debug, verbose);
                    case "format":
                        if (args.Length < 2)
                        {
                            Console.WriteLine("Error: File path required for 'format' command");
                            return 1;
                        }
                        return FormatFile(args[1], debug, verbose);
                    case "version":
                        ShowVersion();
                        return 0;
                    case "help":
                        ShowUsage();
                        return 0;
                    default:
                        Console.WriteLine($"Unknown command: {command}");
                        ShowUsage();
                        return 1;
                }
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[FATAL] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 99;
            }
        }

        private static void ShowUsage()
        {
            Console.WriteLine("HypnoScript CLI - Usage:");
            Console.WriteLine("  dotnet run -- run <file.hyp> [--debug] [--verbose]     - Execute HypnoScript code");
            Console.WriteLine("  dotnet run -- compile <file.hyp> [--debug] [--verbose] - Compile to WASM (.wat)");
            Console.WriteLine("  dotnet run -- analyze <file.hyp> [--debug] [--verbose] - Static analysis");
            Console.WriteLine("  dotnet run -- info <file.hyp> [--debug] [--verbose]    - Show file information");
            Console.WriteLine("  dotnet run -- validate <file.hyp> [--debug] [--verbose] - Validate syntax");
            Console.WriteLine("  dotnet run -- format <file.hyp> [--debug] [--verbose]  - Format code");
            Console.WriteLine("  dotnet run -- version                                 - Show version");
            Console.WriteLine("  dotnet run -- help                                    - Show this help");
            Console.WriteLine();
            Console.WriteLine("Options:");
            Console.WriteLine("  --debug   - Enable debug output");
            Console.WriteLine("  --verbose - Enable verbose output");
        }

        private static void ShowVersion()
        {
            Console.WriteLine("HypnoScript CLI v2.0.0");
            Console.WriteLine("Enterprise Edition with Advanced Features");
            Console.WriteLine("Built with .NET 8.0");
            Console.WriteLine("Features: Lexer, Parser, TypeChecker, Interpreter, WASM CodeGen");
        }

        private static int RunFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== RUN MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                if (debug) Console.WriteLine($"[DEBUG] Reading file: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) Console.WriteLine($"[DEBUG] File read, length: {source.Length}");

                // Syntax validation
                if (!source.TrimStart().StartsWith("Focus"))
                {
                    Console.WriteLine("⚠ File doesn't start with 'Focus'");
                    return 1;
                }
                Console.WriteLine("✓ File starts with 'Focus' - syntax OK");

                // Lexer
                if (debug) Console.WriteLine("[DEBUG] Creating lexer...");
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (debug) Console.WriteLine($"[DEBUG] {tokens.Count} tokens generated");
                Console.WriteLine("✓ Lexing successful!");

                if (verbose)
                {
                    Console.WriteLine("\n📋 Token Analysis:");
                    var tokenTypes = tokens.GroupBy(t => t.Type).OrderByDescending(g => g.Count());
                    foreach (var group in tokenTypes.Take(10))
                    {
                        Console.WriteLine($"  {group.Key}: {group.Count()} tokens");
                    }
                }

                // Parser
                if (debug) Console.WriteLine("[DEBUG] Creating parser...");
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                if (debug) Console.WriteLine($"[DEBUG] AST with {program.Statements.Count} statements created");
                Console.WriteLine("✓ Parsing successful!");

                if (verbose)
                {
                    Console.WriteLine("\n📊 AST Analysis:");
                    Console.WriteLine($"  Total statements: {program.Statements.Count}");
                    var statementTypes = program.Statements.GroupBy(s => s.GetType().Name).OrderByDescending(g => g.Count());
                    foreach (var group in statementTypes)
                    {
                        Console.WriteLine($"  {group.Key}: {group.Count()} statements");
                    }
                }

                // TypeChecker
                if (debug) Console.WriteLine("[DEBUG] Running type checker...");
                var typeChecker = new TypeChecker();
                typeChecker.Check(program);
                Console.WriteLine("✓ Type checking successful!");

                // Interpreter
                if (debug) Console.WriteLine("[DEBUG] Starting interpreter...");
                var interpreter = new HypnoInterpreter();
                interpreter.ExecuteProgram(program);
                Console.WriteLine("✓ Execution successful!");

                Console.WriteLine("🎉 HypnoScript program executed successfully!");
                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int CompileFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== COMPILE MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                if (debug) Console.WriteLine($"[DEBUG] Reading file: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) Console.WriteLine($"[DEBUG] File read, length: {source.Length}");

                // Lexer
                if (debug) Console.WriteLine("[DEBUG] Creating lexer...");
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (debug) Console.WriteLine($"[DEBUG] {tokens.Count} tokens generated");
                Console.WriteLine("✓ Lexing successful!");

                // Parser
                if (debug) Console.WriteLine("[DEBUG] Creating parser...");
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                if (debug) Console.WriteLine($"[DEBUG] AST with {program.Statements.Count} statements created");
                Console.WriteLine("✓ Parsing successful!");

                // WASM Code Generator
                if (debug) Console.WriteLine("[DEBUG] Starting WASM code generator...");
                var wasmGenerator = new WasmCodeGenerator();
                var watCode = wasmGenerator.Generate(program);
                if (debug) Console.WriteLine($"[DEBUG] WASM code generated, length: {watCode.Length}");
                Console.WriteLine("✓ WASM code generation successful!");

                // Write output file
                var outputFile = Path.ChangeExtension(filePath, ".wat");
                File.WriteAllText(outputFile, watCode);
                Console.WriteLine($"📁 WASM (WAT) code saved: {outputFile}");

                if (verbose)
                {
                    Console.WriteLine("\n📋 Generated WASM Statistics:");
                    var lines = watCode.Split('\n');
                    Console.WriteLine($"  Total lines: {lines.Length}");
                    Console.WriteLine($"  Functions: {lines.Count(l => l.Contains("(func"))}");
                    Console.WriteLine($"  Imports: {lines.Count(l => l.Contains("(import"))}");
                    Console.WriteLine($"  Comments: {lines.Count(l => l.Contains(";;"))}");
                }

                Console.WriteLine("🎉 Compilation completed successfully!");
                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int AnalyzeFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== ANALYZE MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                if (debug) Console.WriteLine($"[DEBUG] Reading file: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) Console.WriteLine($"[DEBUG] File read, length: {source.Length}");

                // Lexer
                if (debug) Console.WriteLine("[DEBUG] Creating lexer...");
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (debug) Console.WriteLine($"[DEBUG] {tokens.Count} tokens generated");
                Console.WriteLine("✓ Lexing successful!");

                // Token analysis
                Console.WriteLine("\n📊 TOKEN ANALYSIS:");
                var tokenTypes = tokens.GroupBy(t => t.Type).OrderByDescending(g => g.Count());
                foreach (var group in tokenTypes)
                {
                    Console.WriteLine($"  {group.Key}: {group.Count()} tokens");
                }

                // Parser
                if (debug) Console.WriteLine("[DEBUG] Creating parser...");
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                if (debug) Console.WriteLine($"[DEBUG] AST with {program.Statements.Count} statements created");
                Console.WriteLine("✓ Parsing successful!");

                // AST analysis
                Console.WriteLine("\n📊 AST ANALYSIS:");
                Console.WriteLine($"  Total statements: {program.Statements.Count}");
                var statementTypes = program.Statements.GroupBy(s => s.GetType().Name).OrderByDescending(g => g.Count());
                foreach (var group in statementTypes)
                {
                    Console.WriteLine($"  {group.Key}: {group.Count()} statements");
                }

                // TypeChecker
                if (debug) Console.WriteLine("[DEBUG] Running type checker...");
                var typeChecker = new TypeChecker();
                typeChecker.Check(program);
                Console.WriteLine("✓ Type checking successful!");

                // Code complexity analysis
                if (verbose)
                {
                    Console.WriteLine("\n📊 COMPLEXITY ANALYSIS:");
                    var functionCount = program.Statements.OfType<FunctionDeclNode>().Count();
                    var sessionCount = program.Statements.OfType<SessionDeclNode>().Count();
                    var tranceifyCount = program.Statements.OfType<TranceifyDeclNode>().Count();
                    var loopCount = program.Statements.OfType<WhileStatementNode>().Count() +
                                   program.Statements.OfType<LoopStatementNode>().Count();

                    Console.WriteLine($"  Functions: {functionCount}");
                    Console.WriteLine($"  Sessions (classes): {sessionCount}");
                    Console.WriteLine($"  Tranceify structures: {tranceifyCount}");
                    Console.WriteLine($"  Loops: {loopCount}");
                }

                Console.WriteLine("🎉 Analysis completed successfully!");
                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int ShowFileInfo(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== FILE INFO MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                var fileInfo = new FileInfo(filePath);
                Console.WriteLine($"📁 File: {filePath}");
                Console.WriteLine($"📏 Size: {fileInfo.Length} bytes");
                Console.WriteLine($"📅 Created: {fileInfo.CreationTime}");
                Console.WriteLine($"📅 Modified: {fileInfo.LastWriteTime}");

                var source = File.ReadAllText(filePath);
                var lines = source.Split('\n');
                Console.WriteLine($"📄 Lines: {lines.Length}");
                Console.WriteLine($"📝 Characters: {source.Length}");

                if (verbose)
                {
                    Console.WriteLine("\n📊 Content Analysis:");
                    Console.WriteLine($"  Non-empty lines: {lines.Count(l => !string.IsNullOrWhiteSpace(l))}");
                    Console.WriteLine($"  Comment lines: {lines.Count(l => l.TrimStart().StartsWith("//"))}");
                    Console.WriteLine($"  Induce statements: {source.Split("induce").Length - 1}");
                    Console.WriteLine($"  Observe statements: {source.Split("observe").Length - 1}");
                    Console.WriteLine($"  Function definitions: {source.Split("suggestion").Length - 1}");
                }

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int ValidateFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== VALIDATE MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                var source = File.ReadAllText(filePath);
                var errors = new List<string>();

                // Basic syntax checks
                if (!source.TrimStart().StartsWith("Focus"))
                {
                    errors.Add("File must start with 'Focus'");
                }

                if (!source.TrimEnd().EndsWith("Relax"))
                {
                    errors.Add("File must end with 'Relax'");
                }

                // Lexer validation
                try
                {
                    var lexer = new HypnoLexer(source);
                    var tokens = lexer.Lex().ToList();
                    Console.WriteLine($"✓ Lexing: {tokens.Count} tokens generated");
                }
                catch (Exception ex)
                {
                    errors.Add($"Lexer error: {ex.Message}");
                }

                // Parser validation
                try
                {
                    var lexer = new HypnoLexer(source);
                    var tokens = lexer.Lex().ToList();
                    var parser = new HypnoParser(tokens);
                    var program = parser.ParseProgram();
                    Console.WriteLine($"✓ Parsing: {program.Statements.Count} statements parsed");
                }
                catch (Exception ex)
                {
                    errors.Add($"Parser error: {ex.Message}");
                }

                // Type checker validation
                try
                {
                    var lexer = new HypnoLexer(source);
                    var tokens = lexer.Lex().ToList();
                    var parser = new HypnoParser(tokens);
                    var program = parser.ParseProgram();
                    var typeChecker = new TypeChecker();
                    typeChecker.Check(program);
                    Console.WriteLine("✓ Type checking: No type errors found");
                }
                catch (Exception ex)
                {
                    errors.Add($"Type checker error: {ex.Message}");
                }

                if (errors.Count == 0)
                {
                    Console.WriteLine("🎉 File validation successful! No errors found.");
                    return 0;
                }
                else
                {
                    Console.WriteLine($"❌ File validation failed with {errors.Count} error(s):");
                    foreach (var error in errors)
                    {
                        Console.WriteLine($"  - {error}");
                    }
                    return 1;
                }
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int FormatFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== FORMAT MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                var source = File.ReadAllText(filePath);

                // Basic formatting (this is a simplified version)
                var formatted = source
                    .Replace("\r\n", "\n")
                    .Replace("\r", "\n")
                    .Split('\n')
                    .Select(line => line.TrimEnd())
                    .Where(line => !string.IsNullOrEmpty(line) || line.Contains("Focus") || line.Contains("Relax"))
                    .ToList();

                var formattedSource = string.Join("\n", formatted);

                // Create backup
                var backupFile = filePath + ".backup";
                File.WriteAllText(backupFile, source);

                // Write formatted content
                File.WriteAllText(filePath, formattedSource);

                Console.WriteLine($"✓ File formatted successfully");
                Console.WriteLine($"📁 Backup created: {backupFile}");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }
    }
}
