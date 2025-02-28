using System;
using System.CommandLine;
using System.CommandLine.Invocation;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Interpreter;
using HypnoScript.Compiler.CodeGen;
using HypnoScript.Compiler.Analysis;
using HypnoScript.Compiler.Error;

var root = new RootCommand("HypnoScript CLI - Enterprise Edition");

// ---------- "run" Kommando (Interpreter/JIT) ----------
var runFileArg = new Argument<FileInfo>("file", result => new FileInfo(result.Tokens.Single().Value))
{
    Description = "Die Quell-Datei für HypnoScript."
};
var jitOption = new Option<bool>("--jit", "Nutze Reflection.Emit JIT (ansonsten interpretieren).");
var runCmd = new Command("run", "Führt HypnoScript über den Interpreter oder JIT aus")
{
    runFileArg
};
runCmd.AddOption(jitOption);
runCmd.SetHandler(async (FileInfo file, bool useJit) =>
{
    try
    {
        var source = await File.ReadAllTextAsync(file.FullName);
        var lexer = new HypnoLexer(source);
        var tokens = lexer.Lex();
        var parser = new HypnoParser(tokens);
        var program = parser.ParseProgram();

        // Enterprise-Level: Verwende den TypeChecker, um vor der Ausführung Typ-Fehler zu erkennen.
        var typeChecker = new TypeChecker();
        typeChecker.Check(program);

        if (!useJit)
        {
            var interpreter = new HypnoInterpreter();
            interpreter.ExecuteProgram(program);
        }
        else
        {
            var dynamicMethod = new System.Reflection.Emit.DynamicMethod("", typeof(void), Type.EmptyTypes);
            var generator = new ILCodeGenerator { _il = dynamicMethod.GetILGenerator() };
            var action = generator.Generate(program);
            action();
        }
    }
    catch (Exception ex)
    {
        ErrorReporter.Report($"Execution error: {ex.Message}", 0, 0);
    }
}, runFileArg, jitOption);
root.AddCommand(runCmd);

// ---------- "compile" Kommando (WASM) ----------
var compileFileArg = new Argument<FileInfo>("file", result => new FileInfo(result.Tokens.Single().Value))
{
    Description = "Die Quell-Datei für HypnoScript (WASM-Kompilierung)."
};
var compileCmd = new Command("compile", "Kompiliert HypnoScript zu WASM (WAT-Format)")
{
    compileFileArg
};
compileCmd.SetHandler(async (FileInfo file) =>
{
    try
    {
        var source = await File.ReadAllTextAsync(file.FullName);
        var lexer = new HypnoLexer(source);
        var tokens = lexer.Lex();
        var parser = new HypnoParser(tokens);
        var program = parser.ParseProgram();

        var wasmGenerator = new WasmCodeGenerator();
        var watCode = wasmGenerator.Generate(program);

        // Ausgabe: gleiche Basis mit .wat-Erweiterung
        var outputFile = Path.ChangeExtension(file.FullName, ".wat");
        await File.WriteAllTextAsync(outputFile, watCode);

        Console.WriteLine($"WASM (WAT) code generated: {outputFile}");
    }
    catch (Exception ex)
    {
        ErrorReporter.Report($"Compilation error: {ex.Message}", 0, 0);
    }
}, compileFileArg);
root.AddCommand(compileCmd);

// ---------- "analyze" Kommando (Typprüfung und AST-Debug-Info) ----------
var analyzeFileArg = new Argument<FileInfo>("file", result => new FileInfo(result.Tokens.Single().Value))
{
    Description = "Die Quell-Datei zur statischen Analyse."
};
var analyzeCmd = new Command("analyze", "Führt statische Typprüfung und AST-Analyse durch")
{
    analyzeFileArg
};
analyzeCmd.SetHandler(async (FileInfo file) =>
{
    try
    {
        var source = await File.ReadAllTextAsync(file.FullName);
        var lexer = new HypnoLexer(source);
        var tokens = lexer.Lex();
        var parser = new HypnoParser(tokens);
        var program = parser.ParseProgram();

        var typeChecker = new TypeChecker();
        typeChecker.Check(program);

        // Enterprise-Level: Ausgabe des AST als Debug-Informationen (kann erweitert werden)
        Console.WriteLine("AST-Analyse erfolgreich. Übersicht der Top-Level Statements:");
        foreach (var stmt in program.Statements)
        {
            Console.WriteLine($" - {stmt.GetType().Name}");
        }
    }
    catch (Exception ex)
    {
        ErrorReporter.Report($"Analysis error: {ex.Message}", 0, 0);
    }
}, analyzeFileArg);
root.AddCommand(analyzeCmd);

// Weitere Befehle (z.B. "test", "deploy") können hier ergänzt werden.

// Starte die CLI
return await root.InvokeAsync(args);
