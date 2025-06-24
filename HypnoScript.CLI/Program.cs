using System;
using System.IO;
using System.Diagnostics;
using System.Threading.Tasks;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Analysis;
using HypnoScript.Compiler.Interpreter;
using HypnoScript.Compiler.CodeGen;
using HypnoScript.LexerParser.AST;
using System.Linq;
using System.Collections.Generic;

namespace HypnoScript.CLI
{
    public class Program
    {
        public static int Main(string[] args)
        {
            Console.WriteLine("=== HypnoScript CLI - Runtime Edition v1.0.0 ===");
            Console.WriteLine($"Version: 1.0.0 | Args: {string.Join(" ", args)}");

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
                        if (!CliArgumentValidator.RequireArgs(args, 2, "run", out var err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return RunFile(args[1], debug, verbose);
                    case "compile":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "compile", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return CompileFile(args[1], debug, verbose);
                    case "analyze":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "analyze", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return AnalyzeFile(args[1], debug, verbose);
                    case "info":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "info", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return ShowFileInfo(args[1], debug, verbose);
                    case "validate":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "validate", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return ValidateFile(args[1], debug, verbose);
                    case "format":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "format", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return FormatFile(args[1], debug, verbose);
                    case "benchmark":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "benchmark", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return BenchmarkFile(args[1], debug, verbose);
                    case "profile":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "profile", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return ProfileFile(args[1], debug, verbose);
                    case "lint":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "lint", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return LintFile(args[1], debug, verbose);
                    case "optimize":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "optimize", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return OptimizeFile(args[1], debug, verbose);
                    case "web":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "web", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return StartWebServer(args[1], debug, verbose);
                    case "api":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "api", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return StartApiServer(args[1], debug, verbose);
                    case "deploy":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "deploy", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return DeployApplication(args[1], debug, verbose);
                    case "monitor":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "monitor", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return MonitorApplication(args[1], debug, verbose);
                    case "test":
                        if (args.Length < 2)
                        {
                            // Kein Dateipfad: Alle Tests ausführen
                            return RunTests(string.Empty, debug, verbose);
                        }
                        // Mit Dateipfad: Nur diese Datei testen
                        return RunTests(args[1], debug, verbose);
                    case "docs":
                        if (!CliArgumentValidator.RequireArgs(args, 2, "docs", out err) ||
                            !CliArgumentValidator.RequireFileExists(args[1], out err))
                            return err;
                        return GenerateDocs(args[1], debug, verbose);
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
            Console.WriteLine("HypnoScript CLI - Runtime Edition v1.0.0");
            Console.WriteLine("Usage:");
            Console.WriteLine("  dotnet run -- run <file.hyp> [--debug] [--verbose]     - Execute HypnoScript code");
            Console.WriteLine("  dotnet run -- compile <file.hyp> [--debug] [--verbose] - Compile to WASM (.wat)");
            Console.WriteLine("  dotnet run -- analyze <file.hyp> [--debug] [--verbose] - Static analysis");
            Console.WriteLine("  dotnet run -- info <file.hyp> [--debug] [--verbose]    - Show file information");
            Console.WriteLine("  dotnet run -- validate <file.hyp> [--debug] [--verbose] - Validate syntax");
            Console.WriteLine("  dotnet run -- format <file.hyp> [--debug] [--verbose]  - Format code");
            Console.WriteLine("  dotnet run -- benchmark <file.hyp> [--debug] [--verbose] - Performance benchmark");
            Console.WriteLine("  dotnet run -- profile <file.hyp> [--debug] [--verbose] - Code profiling");
            Console.WriteLine("  dotnet run -- lint <file.hyp> [--debug] [--verbose]    - Code linting");
            Console.WriteLine("  dotnet run -- optimize <file.hyp> [--debug] [--verbose] - Code optimization");
            Console.WriteLine("  dotnet run -- web <file.hyp> [--debug] [--verbose]     - Start web server");
            Console.WriteLine("  dotnet run -- api <file.hyp> [--debug] [--verbose]     - Start API server");
            Console.WriteLine("  dotnet run -- deploy <file.hyp> [--debug] [--verbose]  - Deploy application");
            Console.WriteLine("  dotnet run -- monitor <file.hyp> [--debug] [--verbose] - Monitor application");
            Console.WriteLine("  dotnet run -- test <file.hyp> [--debug] [--verbose]    - Run tests");
            Console.WriteLine("  dotnet run -- docs <file.hyp> [--debug] [--verbose]    - Generate documentation");
            Console.WriteLine("  dotnet run -- version                                 - Show version");
            Console.WriteLine("  dotnet run -- help                                    - Show this help");
            Console.WriteLine();
            Console.WriteLine("Runtime Features:");
            Console.WriteLine("  - Web Server with real-time compilation");
            Console.WriteLine("  - REST API Server with automatic routing");
            Console.WriteLine("  - Cloud deployment (AWS, Azure, GCP)");
            Console.WriteLine("  - Application monitoring and metrics");
            Console.WriteLine("  - Automated testing framework");
            Console.WriteLine("  - Documentation generation");
            Console.WriteLine();
            Console.WriteLine("Options:");
            Console.WriteLine("  --debug   - Enable debug output");
            Console.WriteLine("  --verbose - Enable verbose output");
        }

        private static void ShowVersion()
        {
            Console.WriteLine("HypnoScript CLI v1.0.0");
            Console.WriteLine("Runtime Edition with Advanced Features");
            Console.WriteLine("Built with .NET 8.0");
            Console.WriteLine("Features: Lexer, Parser, TypeChecker, Interpreter, WASM CodeGen");
            Console.WriteLine("Runtime: Web Server, API Server, Cloud Deployment, Monitoring");
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
                    Console.WriteLine("\n🌳 AST Analysis:");
                    var statementTypes = program.Statements.GroupBy(s => s.GetType().Name).OrderByDescending(g => g.Count());
                    foreach (var group in statementTypes.Take(5))
                    {
                        Console.WriteLine($"  {group.Key}: {group.Count()} statements");
                    }
                }

                // Type Checker
                if (debug) Console.WriteLine("[DEBUG] Running type checker...");
                var typeChecker = new TypeChecker();
                typeChecker.Check(program);
                Console.WriteLine("✓ Type checking successful!");

                // Interpreter
                if (debug) Console.WriteLine("[DEBUG] Starting interpreter...");
                var interpreter = new HypnoInterpreter();
                var startTime = DateTime.Now;
                interpreter.ExecuteProgram(program);
                var endTime = DateTime.Now;
                var executionTime = (endTime - startTime).TotalMilliseconds;

                var assertionFailures = interpreter.GetAssertionFailures();
                if (assertionFailures.Count > 0)
                {
                    Console.ForegroundColor = ConsoleColor.Red;
                    Console.WriteLine($"\n❌ {assertionFailures.Count} assertion(s) failed:");
                    foreach (var fail in assertionFailures)
                    {
                        Console.WriteLine($"   - {fail}");
                    }
                    Console.ResetColor();
                    return 1;
                }

                Console.WriteLine("✓ Execution completed!");
                Console.WriteLine($"⏱️  Execution time: {executionTime:F2}ms");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Execution failed: {ex.Message}");
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
                var source = File.ReadAllText(filePath);
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();

                var typeChecker = new TypeChecker();
                typeChecker.Check(program);

                var outputPath = Path.ChangeExtension(filePath, ".wat");
                var codeGen = new WasmCodeGenerator();
                var wasmCode = codeGen.Generate(program);

                File.WriteAllText(outputPath, wasmCode);
                Console.WriteLine($"✓ Compiled to: {outputPath}");

                if (verbose)
                {
                    Console.WriteLine($"📄 Generated {wasmCode.Length} characters of WASM code");
                }

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Compilation failed: {ex.Message}");
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
                var source = File.ReadAllText(filePath);
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();

                Console.WriteLine("📊 Analysis Results:");
                Console.WriteLine($"  File size: {source.Length} characters");
                Console.WriteLine($"  Lines of code: {source.Split('\n').Length}");
                Console.WriteLine($"  Tokens: {tokens.Count}");
                Console.WriteLine($"  Statements: {program.Statements.Count}");

                if (verbose)
                {
                    Console.WriteLine("\n🔍 Detailed Analysis:");
                    var tokenTypes = tokens.GroupBy(t => t.Type).OrderByDescending(g => g.Count());
                    foreach (var group in tokenTypes)
                    {
                        Console.WriteLine($"  {group.Key}: {group.Count()}");
                    }
                }

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Analysis failed: {ex.Message}");
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
                Console.WriteLine("📁 File Information:");
                Console.WriteLine($"  Name: {fileInfo.Name}");
                Console.WriteLine($"  Size: {fileInfo.Length} bytes");
                Console.WriteLine($"  Created: {fileInfo.CreationTime}");
                Console.WriteLine($"  Modified: {fileInfo.LastWriteTime}");
                Console.WriteLine($"  Extension: {fileInfo.Extension}");

                if (verbose)
                {
                    var source = File.ReadAllText(filePath);
                    Console.WriteLine($"  Lines: {source.Split('\n').Length}");
                    Console.WriteLine($"  Characters: {source.Length}");
                }

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Failed to get file info: {ex.Message}");
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
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();

                var typeChecker = new TypeChecker();
                typeChecker.Check(program);

                Console.WriteLine("✓ Validation successful!");
                Console.WriteLine("  ✓ Syntax: OK");
                Console.WriteLine("  ✓ Semantics: OK");
                Console.WriteLine("  ✓ Types: OK");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Validation failed: {ex.Message}");
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
                // Simple formatting - in a real implementation, this would be more sophisticated
                var formatted = source.Replace("\r\n", "\n").Replace("\r", "\n");
                File.WriteAllText(filePath, formatted);

                Console.WriteLine("✓ File formatted successfully!");
                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Formatting failed: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int BenchmarkFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== BENCHMARK MODE ===");
            Console.WriteLine("Benchmarking is not yet implemented in this version.");
            return 0;
        }

        private static int ProfileFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== PROFILE MODE ===");
            Console.WriteLine("Profiling is not yet implemented in this version.");
            return 0;
        }

        private static int LintFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== LINT MODE ===");
            Console.WriteLine("Linting is not yet implemented in this version.");
            return 0;
        }

        private static int OptimizeFile(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== OPTIMIZE MODE ===");
            Console.WriteLine("Optimization is not yet implemented in this version.");
            return 0;
        }

        // ===== NEUE ENTERPRISE-BEFEHLE =====

        private static int StartWebServer(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== WEB SERVER MODE ===");
            Console.WriteLine("🚀 Starting HypnoScript Web Server...");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                Console.WriteLine("📡 Web server features:");
                Console.WriteLine("  - Real-time code compilation");
                Console.WriteLine("  - Live code execution");
                Console.WriteLine("  - Interactive development environment");
                Console.WriteLine("  - WebSocket support for real-time updates");
                Console.WriteLine("  - REST API endpoints");
                Console.WriteLine("  - File upload/download");
                Console.WriteLine("  - Session management");
                Console.WriteLine("  - Performance monitoring");

                Console.WriteLine("\n🌐 Server would start on: http://localhost:8080");
                Console.WriteLine("📊 Dashboard: http://localhost:8080/dashboard");
                Console.WriteLine("🔧 API Docs: http://localhost:8080/api/docs");

                Console.WriteLine("\n⚠️  Web server is not yet fully implemented.");
                Console.WriteLine("   This is a placeholder for the Runtime Edition feature.");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Web server failed: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int StartApiServer(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== API SERVER MODE ===");
            Console.WriteLine("🔌 Starting HypnoScript API Server...");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                Console.WriteLine("🔗 API server features:");
                Console.WriteLine("  - RESTful API endpoints");
                Console.WriteLine("  - JSON request/response handling");
                Console.WriteLine("  - Authentication & authorization");
                Console.WriteLine("  - Rate limiting");
                Console.WriteLine("  - CORS support");
                Console.WriteLine("  - Request/response logging");
                Console.WriteLine("  - Health check endpoints");
                Console.WriteLine("  - Metrics collection");

                Console.WriteLine("\n🌐 Server would start on: http://localhost:5000");
                Console.WriteLine("📚 Swagger UI: http://localhost:5000/swagger");
                Console.WriteLine("💚 Health check: http://localhost:5000/health");
                Console.WriteLine("📊 Metrics: http://localhost:5000/metrics");

                Console.WriteLine("\n⚠️  API server is not yet fully implemented.");
                Console.WriteLine("   This is a placeholder for the Runtime Edition feature.");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] API server failed: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int DeployApplication(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== DEPLOY MODE ===");
            Console.WriteLine("☁️  Deploying HypnoScript Application...");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                Console.WriteLine("🚀 Deployment features:");
                Console.WriteLine("  - Multi-cloud support (AWS, Azure, GCP)");
                Console.WriteLine("  - Container deployment (Docker)");
                Console.WriteLine("  - Kubernetes orchestration");
                Console.WriteLine("  - CI/CD pipeline integration");
                Console.WriteLine("  - Environment-specific configurations");
                Console.WriteLine("  - Blue-green deployment");
                Console.WriteLine("  - Rollback capabilities");
                Console.WriteLine("  - Infrastructure as Code (Terraform)");

                Console.WriteLine("\n☁️  Supported platforms:");
                Console.WriteLine("  - AWS Lambda / ECS / EC2");
                Console.WriteLine("  - Azure Functions / AKS / VM");
                Console.WriteLine("  - Google Cloud Functions / GKE / Compute");
                Console.WriteLine("  - Docker containers");
                Console.WriteLine("  - Kubernetes clusters");

                Console.WriteLine("\n⚠️  Deployment is not yet fully implemented.");
                Console.WriteLine("   This is a placeholder for the Runtime Edition feature.");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Deployment failed: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int MonitorApplication(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== MONITOR MODE ===");
            Console.WriteLine("📊 Starting HypnoScript Application Monitor...");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                Console.WriteLine("📈 Monitoring features:");
                Console.WriteLine("  - Real-time performance metrics");
                Console.WriteLine("  - CPU, memory, and disk usage");
                Console.WriteLine("  - Request/response times");
                Console.WriteLine("  - Error rates and logs");
                Console.WriteLine("  - Custom business metrics");
                Console.WriteLine("  - Alerting and notifications");
                Console.WriteLine("  - Historical data analysis");
                Console.WriteLine("  - Dashboard visualization");

                Console.WriteLine("\n🔍 Metrics collected:");
                Console.WriteLine("  - Execution time per function");
                Console.WriteLine("  - Memory allocation patterns");
                Console.WriteLine("  - Builtin function usage");
                Console.WriteLine("  - Error frequency and types");
                Console.WriteLine("  - User interaction patterns");
                Console.WriteLine("  - System resource utilization");

                Console.WriteLine("\n⚠️  Monitoring is not yet fully implemented.");
                Console.WriteLine("   This is a placeholder for the Runtime Edition feature.");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Monitoring failed: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int RunTests(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== TEST MODE ===");
            Console.WriteLine("🧪 Running HypnoScript Tests...");

            List<string> testFiles;
            if (string.IsNullOrEmpty(filePath))
            {
                // Alle .hyp-Dateien im Projektverzeichnis rekursiv finden
                testFiles = Directory.GetFiles(Directory.GetCurrentDirectory(), "*.hyp", SearchOption.AllDirectories)
                    .OrderBy(f => f).ToList();
            }
            else
            {
                if (!File.Exists(filePath))
                {
                    Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                    return 2;
                }
                testFiles = new List<string> { filePath };
            }

            if (testFiles.Count == 0)
            {
                Console.WriteLine("[WARN] No .hyp test files found.");
                return 0;
            }

            int passed = 0, failed = 0;
            var results = new List<(string file, bool ok, TimeSpan duration, string? error)>();

            foreach (var testFile in testFiles)
            {
                var sw = System.Diagnostics.Stopwatch.StartNew();
                try
                {
                    int exitCode = RunFile(testFile, debug, verbose);
                    sw.Stop();
                    if (exitCode == 0)
                    {
                        results.Add((testFile, true, sw.Elapsed, null));
                        passed++;
                    }
                    else
                    {
                        results.Add((testFile, false, sw.Elapsed, $"Exit code: {exitCode}"));
                        failed++;
                    }
                }
                catch (Exception ex)
                {
                    sw.Stop();
                    string errorMessage = ex.Message;
                    bool isAssertionFailure = errorMessage.Contains("Assertion failed") || errorMessage.StartsWith("Assertion failed");

                    if (isAssertionFailure)
                    {
                        // Assertion-Fehler speziell hervorheben
                        errorMessage = $"ASSERTION FAILED: {errorMessage}";
                    }

                    results.Add((testFile, false, sw.Elapsed, errorMessage));
                    failed++;
                }
            }

            // Testreport
            Console.WriteLine("\n=== Test Results ===");
            foreach (var (file, ok, duration, error) in results)
            {
                if (ok)
                {
                    Console.ForegroundColor = ConsoleColor.Green;
                    Console.WriteLine($"[OK]     {Path.GetFileName(file),-30} ({duration.TotalMilliseconds:F0} ms)");
                    Console.ResetColor();
                }
                else
                {
                    Console.ForegroundColor = ConsoleColor.Red;
                    if (error?.Contains("ASSERTION FAILED") == true)
                    {
                        Console.WriteLine($"[ASSERT] {Path.GetFileName(file),-30} ({duration.TotalMilliseconds:F0} ms)");
                        Console.WriteLine($"         └─ {error}");
                    }
                    else
                    {
                        Console.WriteLine($"[FAIL]   {Path.GetFileName(file),-30} ({duration.TotalMilliseconds:F0} ms)  {error}");
                    }
                    Console.ResetColor();
                }
            }

            Console.WriteLine($"\nSummary: {passed} passed, {failed} failed, {testFiles.Count} total");
            if (failed > 0)
            {
                Console.ForegroundColor = ConsoleColor.Yellow;
                Console.WriteLine($"⚠️  {failed} test(s) failed. Check the output above for details.");
                Console.ResetColor();
            }

            return failed == 0 ? 0 : 1;
        }

        private static int GenerateDocs(string filePath, bool debug, bool verbose)
        {
            Console.WriteLine("=== DOCS MODE ===");
            Console.WriteLine("📚 Generating HypnoScript Documentation...");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                return 2;
            }

            try
            {
                Console.WriteLine("📖 Documentation features:");
                Console.WriteLine("  - API documentation generation");
                Console.WriteLine("  - Code examples and tutorials");
                Console.WriteLine("  - Function reference manual");
                Console.WriteLine("  - Best practices guide");
                Console.WriteLine("  - Troubleshooting guide");
                Console.WriteLine("  - Interactive documentation");
                Console.WriteLine("  - Multiple output formats (HTML, PDF, Markdown)");
                Console.WriteLine("  - Search functionality");

                Console.WriteLine("\n📄 Output formats:");
                Console.WriteLine("  - HTML documentation site");
                Console.WriteLine("  - PDF reference manual");
                Console.WriteLine("  - Markdown files");
                Console.WriteLine("  - JSDoc-style comments");
                Console.WriteLine("  - Swagger/OpenAPI specs");

                Console.WriteLine("\n⚠️  Documentation generation is not yet fully implemented.");
                Console.WriteLine("   This is a placeholder for the Runtime Edition feature.");

                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] Documentation generation failed: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        public static class CliArgumentValidator
        {
            public static bool RequireArgs(string[] args, int minCount, string command, out int errorCode)
            {
                if (args.Length < minCount)
                {
                    Console.WriteLine($"Error: File path required for '{command}' command");
                    errorCode = 1;
                    return false;
                }
                errorCode = 0;
                return true;
            }
            public static bool RequireFileExists(string filePath, out int errorCode)
            {
                if (!File.Exists(filePath))
                {
                    Console.Error.WriteLine($"[ERROR] File not found: {filePath}");
                    errorCode = 2;
                    return false;
                }
                errorCode = 0;
                return true;
            }
        }
    }
}
