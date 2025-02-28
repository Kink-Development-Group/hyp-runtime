using System.CommandLine;
using System.Linq; // Neu hinzugefügt
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Interpreter;
using HypnoScript.Compiler.CodeGen;
using System.IO;

var root = new RootCommand("HypnoScript CLI");

// Run-Befehl (Interpreter/JIT)
var fileArgument = new Argument<FileInfo>("file", result => new FileInfo(result.Tokens.Single().Value));
var runCmd = new Command("run", "Runs HypnoScript in interpreter or JIT")
{
	fileArgument
};
var jitOption = new Option<bool>("--jit", "Use Reflection.Emit JIT (otherwise interpret).");
runCmd.AddOption(jitOption);
runCmd.SetHandler(async context =>
{
	var file = context.ParseResult.GetValueForArgument<FileInfo>(fileArgument);
	var useJit = context.ParseResult.GetValueForOption<bool>(jitOption);

	var source = await File.ReadAllTextAsync(file.FullName);
	var lexer = new HypnoLexer(source);
	var tokens = lexer.Lex();
	var parser = new HypnoParser(tokens);
	var program = parser.ParseProgram();

	if (!useJit)
	{
		// interpret
		var interpreter = new HypnoInterpreter();
		interpreter.ExecuteProgram(program);
	}
	else
	{
		// JIT
		var generator = new ILCodeGenerator { _il = default! };
		var action = generator.Generate(program);
		action();
	}
});

root.Add(runCmd);

// Compile-Befehl: WASM (WAT-Code generieren und in Datei schreiben)
var compileFileArgument = new Argument<FileInfo>("file", result => new FileInfo(result.Tokens.Single().Value));
var compileCmd = new Command("compile", "Compile to .wasm (WAT-Format)")
{
	compileFileArgument
};
compileCmd.SetHandler(async context =>
{
	var file = context.ParseResult.GetValueForArgument<FileInfo>(compileFileArgument);
	// Quellcode einlesen, lexen und parsen
	var source = await File.ReadAllTextAsync(file.FullName);
	var lexer = new HypnoLexer(source);
	var tokens = lexer.Lex();
	var parser = new HypnoParser(tokens);
	var program = parser.ParseProgram();

	// WASM-Code generieren
	var wasmGenerator = new WasmCodeGenerator();
	var watCode = wasmGenerator.Generate(program);

	// Ausgabedatei: gleiche Basis, aber mit .wat-Erweiterung
	var outputFile = Path.ChangeExtension(file.FullName, ".wat");
	await File.WriteAllTextAsync(outputFile, watCode);

	Console.WriteLine($"WASM (WAT) code generated: {outputFile}");
});
root.Add(compileCmd);

return root.Invoke(args);
