using System;
using System.IO;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Analysis;
using HypnoScript.Compiler.Interpreter;

namespace HypnoScript.CLI
{
    public class Program
    {
        public static int Main(string[] args)
        {
            Console.WriteLine("=== HypnoScript CLI Test ===");
            Console.WriteLine($"Args: {string.Join(" ", args)}");

            if (args.Length < 2)
            {
                Console.WriteLine("Verwendung: dotnet run -- run <datei.hyp> [--debug]");
                return 1;
            }

            if (args[0] != "run")
            {
                Console.WriteLine("Nur 'run' Kommando wird unterstützt");
                return 1;
            }

            var filePath = args[1];
            var debug = args.Length > 2 && args[2] == "--debug";

            try
            {
                if (!File.Exists(filePath))
                {
                    Console.Error.WriteLine($"[ERROR] Datei nicht gefunden: {filePath}");
                    return 2;
                }

                if (debug) Console.WriteLine($"[DEBUG] Lese Datei: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) Console.WriteLine($"[DEBUG] Datei gelesen, Länge: {source.Length}");
                if (debug) Console.WriteLine($"[DEBUG] Inhalt: '{source}'");

                // Einfacher Test: Prüfe, ob die Datei mit "Focus" beginnt
                if (source.TrimStart().StartsWith("Focus"))
                {
                    Console.WriteLine("✓ Datei beginnt mit 'Focus' - Syntax OK");
                }
                else
                {
                    Console.WriteLine("⚠ Datei beginnt nicht mit 'Focus'");
                    return 1;
                }

                // Schritt 2: Lexer hinzufügen
                if (debug) Console.WriteLine("[DEBUG] Lexer wird erstellt...");
                var lexer = new HypnoLexer(source);
                if (debug) Console.WriteLine("[DEBUG] Lexing wird gestartet...");

                var tokens = new List<Token>();
                try
                {
                    tokens = lexer.Lex().ToList();
                    if (debug) Console.WriteLine($"[DEBUG] Lexing erfolgreich, {tokens.Count} Tokens erzeugt");

                    // Zeige die ersten 5 Tokens
                    var tokenCount = Math.Min(5, tokens.Count);
                    for (int i = 0; i < tokenCount; i++)
                    {
                        var token = tokens[i];
                        Console.WriteLine($"  Token {i}: {token.Type} '{token.Lexeme}' @ {token.Line}:{token.Column}");
                    }

                    if (tokens.Count > 5)
                    {
                        Console.WriteLine($"  ... und {tokens.Count - 5} weitere Tokens");
                    }

                    Console.WriteLine("✓ Lexing erfolgreich!");
                }
                catch (Exception lexerEx)
                {
                    Console.Error.WriteLine($"[ERROR] Lexer-Fehler: {lexerEx.Message}");
                    if (debug) Console.Error.WriteLine(lexerEx.StackTrace);
                    return 1;
                }

                // Schritt 3: Parser hinzufügen
                if (debug) Console.WriteLine("[DEBUG] Parser wird erstellt...");
                var parser = new HypnoParser(tokens);
                if (debug) Console.WriteLine("[DEBUG] Parsen wird gestartet...");

                var program = new ProgramNode(new List<IStatement>());
                try
                {
                    program = parser.ParseProgram();
                    if (debug) Console.WriteLine("[DEBUG] Parsen erfolgreich");

                    Console.WriteLine($"✓ AST erstellt mit {program.Statements.Count} Top-Level-Statements");

                    // Zeige die Statement-Typen
                    foreach (var stmt in program.Statements)
                    {
                        Console.WriteLine($"  - {stmt.GetType().Name}");
                    }

                    Console.WriteLine("✓ Parsing erfolgreich!");
                }
                catch (Exception parserEx)
                {
                    Console.Error.WriteLine($"[ERROR] Parser-Fehler: {parserEx.Message}");
                    if (debug) Console.Error.WriteLine(parserEx.StackTrace);
                    return 1;
                }

                // Schritt 4: TypeChecker hinzufügen
                if (debug) Console.WriteLine("[DEBUG] TypeChecker wird erstellt...");
                var typeChecker = new TypeChecker();
                if (debug) Console.WriteLine("[DEBUG] TypeChecking wird gestartet...");

                try
                {
                    typeChecker.Check(program);
                    if (debug) Console.WriteLine("[DEBUG] TypeChecking erfolgreich");
                    Console.WriteLine("✓ TypeChecking erfolgreich!");
                }
                catch (Exception typeEx)
                {
                    Console.Error.WriteLine($"[ERROR] TypeChecker-Fehler: {typeEx.Message}");
                    if (debug) Console.Error.WriteLine(typeEx.StackTrace);
                    return 1;
                }

                // Schritt 5: Interpreter hinzufügen
                if (debug) Console.WriteLine("[DEBUG] Interpreter wird erstellt...");
                var interpreter = new HypnoInterpreter();
                if (debug) Console.WriteLine("[DEBUG] Ausführung wird gestartet...");

                try
                {
                    interpreter.ExecuteProgram(program);
                    if (debug) Console.WriteLine("[DEBUG] Ausführung erfolgreich");
                    Console.WriteLine("✓ Ausführung erfolgreich!");
                }
                catch (Exception interpEx)
                {
                    Console.Error.WriteLine($"[ERROR] Interpreter-Fehler: {interpEx.Message}");
                    if (debug) Console.Error.WriteLine(interpEx.StackTrace);
                    return 1;
                }

                Console.WriteLine("✓ HypnoScript-Programm erfolgreich ausgeführt!");
                return 0;
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[ERROR] {ex.GetType().Name}: {ex.Message}");
                if (debug) Console.Error.WriteLine(ex.StackTrace);
                return 99;
            }
        }
    }
}
