using System;
using System.IO;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Analysis;
using HypnoScript.Compiler.Interpreter;
using HypnoScript.Compiler.CodeGen;

namespace HypnoScript.CLI
{
    public class Program
    {
        public static int Main(string[] args)
        {
            Console.WriteLine("=== HypnoScript CLI - Enterprise Edition ===");
            Console.WriteLine($"Args: {string.Join(" ", args)}");

            if (args.Length < 2)
            {
                ShowUsage();
                return 1;
            }

            var command = args[0];
            var filePath = args[1];
            var debug = args.Length > 2 && args[2] == "--debug";

            try
            {
                switch (command.ToLower())
                {
                    case "run":
                        return RunFile(filePath, debug);
                    case "compile":
                        return CompileFile(filePath, debug);
                    case "analyze":
                        return AnalyzeFile(filePath, debug);
                    default:
                        Console.WriteLine($"Unbekannter Befehl: {command}");
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
            Console.WriteLine("Verwendung:");
            Console.WriteLine("  dotnet run -- run <datei.hyp> [--debug]     - Führt HypnoScript-Code aus");
            Console.WriteLine("  dotnet run -- compile <datei.hyp> [--debug] - Kompiliert zu WASM (.wat)");
            Console.WriteLine("  dotnet run -- analyze <datei.hyp> [--debug] - Führt statische Analyse durch");
        }

        private static int RunFile(string filePath, bool debug)
        {
            Console.WriteLine("=== RUN MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] Datei nicht gefunden: {filePath}");
                return 2;
            }

            try
            {
                if (debug) Console.WriteLine($"[DEBUG] Lese Datei: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) Console.WriteLine($"[DEBUG] Datei gelesen, Länge: {source.Length}");

                // Syntax-Prüfung
                if (!source.TrimStart().StartsWith("Focus"))
                {
                    Console.WriteLine("⚠ Datei beginnt nicht mit 'Focus'");
                    return 1;
                }
                Console.WriteLine("✓ Datei beginnt mit 'Focus' - Syntax OK");

                // Lexer
                if (debug) Console.WriteLine("[DEBUG] Lexer wird erstellt...");
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (debug) Console.WriteLine($"[DEBUG] {tokens.Count} Tokens erzeugt");
                Console.WriteLine("✓ Lexing erfolgreich!");

                // Parser
                if (debug) Console.WriteLine("[DEBUG] Parser wird erstellt...");
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                if (debug) Console.WriteLine($"[DEBUG] AST mit {program.Statements.Count} Statements erstellt");
                Console.WriteLine("✓ Parsing erfolgreich!");

                // TypeChecker
                if (debug) Console.WriteLine("[DEBUG] TypeChecker wird ausgeführt...");
                var typeChecker = new TypeChecker();
                typeChecker.Check(program);
                Console.WriteLine("✓ TypeChecking erfolgreich!");

                // Interpreter
                if (debug) Console.WriteLine("[DEBUG] Interpreter wird gestartet...");
                var interpreter = new HypnoInterpreter();
                interpreter.ExecuteProgram(program);
                Console.WriteLine("✓ Ausführung erfolgreich!");

                Console.WriteLine("🎉 HypnoScript-Programm erfolgreich ausgeführt!");
                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int CompileFile(string filePath, bool debug)
        {
            Console.WriteLine("=== COMPILE MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] Datei nicht gefunden: {filePath}");
                return 2;
            }

            try
            {
                if (debug) Console.WriteLine($"[DEBUG] Lese Datei: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) Console.WriteLine($"[DEBUG] Datei gelesen, Länge: {source.Length}");

                // Lexer
                if (debug) Console.WriteLine("[DEBUG] Lexer wird erstellt...");
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (debug) Console.WriteLine($"[DEBUG] {tokens.Count} Tokens erzeugt");
                Console.WriteLine("✓ Lexing erfolgreich!");

                // Parser
                if (debug) Console.WriteLine("[DEBUG] Parser wird erstellt...");
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                if (debug) Console.WriteLine($"[DEBUG] AST mit {program.Statements.Count} Statements erstellt");
                Console.WriteLine("✓ Parsing erfolgreich!");

                // WASM Code Generator
                if (debug) Console.WriteLine("[DEBUG] WASM Code Generator wird gestartet...");
                var wasmGenerator = new WasmCodeGenerator();
                var watCode = wasmGenerator.Generate(program);
                if (debug) Console.WriteLine($"[DEBUG] WASM Code generiert, Länge: {watCode.Length}");
                Console.WriteLine("✓ WASM Code Generation erfolgreich!");

                // Ausgabedatei schreiben
                var outputFile = Path.ChangeExtension(filePath, ".wat");
                File.WriteAllText(outputFile, watCode);
                Console.WriteLine($"📁 WASM (WAT) Code gespeichert: {outputFile}");

                Console.WriteLine("🎉 Kompilierung erfolgreich abgeschlossen!");
                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 1;
            }
        }

        private static int AnalyzeFile(string filePath, bool debug)
        {
            Console.WriteLine("=== ANALYZE MODE ===");

            if (!File.Exists(filePath))
            {
                Console.Error.WriteLine($"[ERROR] Datei nicht gefunden: {filePath}");
                return 2;
            }

            try
            {
                if (debug) Console.WriteLine($"[DEBUG] Lese Datei: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) Console.WriteLine($"[DEBUG] Datei gelesen, Länge: {source.Length}");

                // Lexer
                if (debug) Console.WriteLine("[DEBUG] Lexer wird erstellt...");
                var lexer = new HypnoLexer(source);
                var tokens = lexer.Lex().ToList();
                if (debug) Console.WriteLine($"[DEBUG] {tokens.Count} Tokens erzeugt");
                Console.WriteLine("✓ Lexing erfolgreich!");

                // Token-Analyse
                Console.WriteLine("\n📊 TOKEN-ANALYSE:");
                var tokenTypes = tokens.GroupBy(t => t.Type).OrderByDescending(g => g.Count());
                foreach (var group in tokenTypes)
                {
                    Console.WriteLine($"  {group.Key}: {group.Count()}x");
                }

                // Parser
                if (debug) Console.WriteLine("[DEBUG] Parser wird erstellt...");
                var parser = new HypnoParser(tokens);
                var program = parser.ParseProgram();
                if (debug) Console.WriteLine($"[DEBUG] AST mit {program.Statements.Count} Statements erstellt");
                Console.WriteLine("✓ Parsing erfolgreich!");

                // AST-Analyse
                Console.WriteLine("\n🌳 AST-ANALYSE:");
                Console.WriteLine($"  Top-Level Statements: {program.Statements.Count}");
                var statementTypes = program.Statements.GroupBy(s => s.GetType().Name).OrderByDescending(g => g.Count());
                foreach (var group in statementTypes)
                {
                    Console.WriteLine($"  {group.Key}: {group.Count()}x");
                }

                // TypeChecker
                if (debug) Console.WriteLine("[DEBUG] TypeChecker wird ausgeführt...");
                var typeChecker = new TypeChecker();
                typeChecker.Check(program);
                Console.WriteLine("✓ TypeChecking erfolgreich!");

                // Code-Metriken
                Console.WriteLine("\n📈 CODE-METRIKEN:");
                var lineCount = source.Split('\n').Length;
                var charCount = source.Length;
                var tokenCount = tokens.Count;
                Console.WriteLine($"  Zeilen: {lineCount}");
                Console.WriteLine($"  Zeichen: {charCount}");
                Console.WriteLine($"  Tokens: {tokenCount}");
                Console.WriteLine($"  Statements: {program.Statements.Count}");

                Console.WriteLine("\n🎉 Statische Analyse erfolgreich abgeschlossen!");
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
