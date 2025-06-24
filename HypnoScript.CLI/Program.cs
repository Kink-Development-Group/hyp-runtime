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
using System.CommandLine;
using System.CommandLine.Invocation;
using Microsoft.Extensions.Logging;

namespace HypnoScript.CLI
{
    public class Program
    {
        public static int Main(string[] args)
        {
            using var loggerFactory = LoggerFactory.Create(builder =>
            {
                builder.AddSimpleConsole(options =>
                {
                    options.SingleLine = true;
                    options.TimestampFormat = "HH:mm:ss ";
                });
                builder.SetMinimumLevel(LogLevel.Information);
            });
            var logger = loggerFactory.CreateLogger("HypnoScriptCLI");
            AppLogger.Configure(logger);

            var rootCommand = new RootCommand("HypnoScript CLI - Runtime Edition v1.0.0");

            var runFileArg = new Argument<string>("file", "The HypnoScript file to execute");
            var runDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var runVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var runCommand = new Command("run", "Execute HypnoScript code") { runFileArg, runDebugOpt, runVerboseOpt };
            runCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.RunCommand.Execute(file, debug, verbose), runFileArg, runDebugOpt, runVerboseOpt);
            rootCommand.AddCommand(runCommand);

            var compileFileArg = new Argument<string>("file", "The HypnoScript file to compile");
            var compileDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var compileVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var compileCommand = new Command("compile", "Compile to WASM (.wat)") { compileFileArg, compileDebugOpt, compileVerboseOpt };
            compileCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.CompileCommand.Execute(file, debug, verbose), compileFileArg, compileDebugOpt, compileVerboseOpt);
            rootCommand.AddCommand(compileCommand);

            var analyzeFileArg = new Argument<string>("file", "The HypnoScript file to analyze");
            var analyzeDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var analyzeVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var analyzeCommand = new Command("analyze", "Static analysis") { analyzeFileArg, analyzeDebugOpt, analyzeVerboseOpt };
            analyzeCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.AnalyzeCommand.Execute(file, debug, verbose), analyzeFileArg, analyzeDebugOpt, analyzeVerboseOpt);
            rootCommand.AddCommand(analyzeCommand);

            var validateFileArg = new Argument<string>("file", "The HypnoScript file to validate");
            var validateDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var validateVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var validateCommand = new Command("validate", "Validate syntax") { validateFileArg, validateDebugOpt, validateVerboseOpt };
            validateCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.ValidateCommand.Execute(file, debug, verbose), validateFileArg, validateDebugOpt, validateVerboseOpt);
            rootCommand.AddCommand(validateCommand);

            var infoFileArg = new Argument<string>("file", "The HypnoScript file to show info for");
            var infoDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var infoVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var infoCommand = new Command("info", "Show file information") { infoFileArg, infoDebugOpt, infoVerboseOpt };
            infoCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.InfoCommand.Execute(file, debug, verbose), infoFileArg, infoDebugOpt, infoVerboseOpt);
            rootCommand.AddCommand(infoCommand);

            var formatFileArg = new Argument<string>("file", "The HypnoScript file to format");
            var formatDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var formatVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var formatCommand = new Command("format", "Format code") { formatFileArg, formatDebugOpt, formatVerboseOpt };
            formatCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.FormatCommand.Execute(file, debug, verbose), formatFileArg, formatDebugOpt, formatVerboseOpt);
            rootCommand.AddCommand(formatCommand);

            var testFileArg = new Argument<string>("file", () => string.Empty, "The HypnoScript file to test (optional, runs all if omitted)");
            var testDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var testVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var testCommand = new Command("test", "Run tests") { testFileArg, testDebugOpt, testVerboseOpt };
            testCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.TestCommand.Execute(file, debug, verbose), testFileArg, testDebugOpt, testVerboseOpt);
            rootCommand.AddCommand(testCommand);

            var docsFileArg = new Argument<string>("file", "The HypnoScript file to generate docs for");
            var docsDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var docsVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var docsCommand = new Command("docs", "Generate documentation") { docsFileArg, docsDebugOpt, docsVerboseOpt };
            docsCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.DocsCommand.Execute(file, debug, verbose), docsFileArg, docsDebugOpt, docsVerboseOpt);
            rootCommand.AddCommand(docsCommand);

            var benchmarkFileArg = new Argument<string>("file", "The HypnoScript file to benchmark");
            var benchmarkDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var benchmarkVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var benchmarkCommand = new Command("benchmark", "Performance benchmark") { benchmarkFileArg, benchmarkDebugOpt, benchmarkVerboseOpt };
            benchmarkCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.BenchmarkCommand.Execute(file, debug, verbose), benchmarkFileArg, benchmarkDebugOpt, benchmarkVerboseOpt);
            rootCommand.AddCommand(benchmarkCommand);

            var profileFileArg = new Argument<string>("file", "The HypnoScript file to profile");
            var profileDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var profileVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var profileCommand = new Command("profile", "Code profiling") { profileFileArg, profileDebugOpt, profileVerboseOpt };
            profileCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.ProfileCommand.Execute(file, debug, verbose), profileFileArg, profileDebugOpt, profileVerboseOpt);
            rootCommand.AddCommand(profileCommand);

            var lintFileArg = new Argument<string>("file", "The HypnoScript file to lint");
            var lintDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var lintVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var lintCommand = new Command("lint", "Code linting") { lintFileArg, lintDebugOpt, lintVerboseOpt };
            lintCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.LintCommand.Execute(file, debug, verbose), lintFileArg, lintDebugOpt, lintVerboseOpt);
            rootCommand.AddCommand(lintCommand);

            var optimizeFileArg = new Argument<string>("file", "The HypnoScript file to optimize");
            var optimizeDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var optimizeVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var optimizeCommand = new Command("optimize", "Code optimization") { optimizeFileArg, optimizeDebugOpt, optimizeVerboseOpt };
            optimizeCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.OptimizeCommand.Execute(file, debug, verbose), optimizeFileArg, optimizeDebugOpt, optimizeVerboseOpt);
            rootCommand.AddCommand(optimizeCommand);

            var webFileArg = new Argument<string>("file", "The HypnoScript file for the web server");
            var webDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var webVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var webCommand = new Command("web", "Start web server") { webFileArg, webDebugOpt, webVerboseOpt };
            webCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.WebCommand.Execute(file, debug, verbose), webFileArg, webDebugOpt, webVerboseOpt);
            rootCommand.AddCommand(webCommand);

            var apiFileArg = new Argument<string>("file", "The HypnoScript file for the API server");
            var apiDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var apiVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var apiCommand = new Command("api", "Start API server") { apiFileArg, apiDebugOpt, apiVerboseOpt };
            apiCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.ApiCommand.Execute(file, debug, verbose), apiFileArg, apiDebugOpt, apiVerboseOpt);
            rootCommand.AddCommand(apiCommand);

            var deployFileArg = new Argument<string>("file", "The HypnoScript file to deploy");
            var deployDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var deployVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var deployCommand = new Command("deploy", "Deploy application") { deployFileArg, deployDebugOpt, deployVerboseOpt };
            deployCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.DeployCommand.Execute(file, debug, verbose), deployFileArg, deployDebugOpt, deployVerboseOpt);
            rootCommand.AddCommand(deployCommand);

            var monitorFileArg = new Argument<string>("file", "The HypnoScript file to monitor");
            var monitorDebugOpt = new Option<bool>("--debug", "Enable debug output");
            var monitorVerboseOpt = new Option<bool>("--verbose", "Enable verbose output");
            var monitorCommand = new Command("monitor", "Monitor application") { monitorFileArg, monitorDebugOpt, monitorVerboseOpt };
            monitorCommand.SetHandler((string file, bool debug, bool verbose) =>
                Commands.MonitorCommand.Execute(file, debug, verbose), monitorFileArg, monitorDebugOpt, monitorVerboseOpt);
            rootCommand.AddCommand(monitorCommand);

            var configShowOpt = new Option<bool>("--show", "Show current configuration");
            var configResetOpt = new Option<bool>("--reset", "Reset configuration to defaults");
            var configSetOpt = new Option<string>("--set", "Set a configuration value (format: section.key=value)");
            var configGetOpt = new Option<string>("--get", "Get a configuration value (format: section.key)");
            var configExportOpt = new Option<string>("--export", "Export configuration to file");
            var configImportOpt = new Option<string>("--import", "Import configuration from file");
            var configCommand = new Command("config", "Manage configuration") { configShowOpt, configResetOpt, configSetOpt, configGetOpt, configExportOpt, configImportOpt };
            configCommand.SetHandler((bool show, bool reset, string? set, string? get, string? export, string? import) =>
                Commands.ConfigCommand.Execute(show, reset, set, get, export, import), configShowOpt, configResetOpt, configSetOpt, configGetOpt, configExportOpt, configImportOpt);
            rootCommand.AddCommand(configCommand);

            var versionCommand = new Command("version", "Show version");
            versionCommand.SetHandler(() => ShowVersion());
            rootCommand.AddCommand(versionCommand);

            var helpCommand = new Command("help", "Show help");
            helpCommand.SetHandler(() => ShowUsage());
            rootCommand.AddCommand(helpCommand);

            return rootCommand.Invoke(args);
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
