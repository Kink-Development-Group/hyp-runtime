using System.CommandLine;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Interpreter;
using HypnoScript.Compiler.CodeGen;

var root = new RootCommand("HypnoScript CLI");

var runCmd = new Command("run", "Runs HypnoScript in interpreter or JIT")
{
	new Argument<FileInfo>("file")
};
var jitOption = new Option<bool>("--jit", "Use Reflection.Emit JIT (otherwise interpret).");
runCmd.AddOption(jitOption);

runCmd.SetHandler((FileInfo file, bool useJit) =>
{
	var source = File.ReadAllText(file.FullName);
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
		var generator = new ILCodeGenerator();
		var action = generator.Generate(program);
		action();
	}

}, new System.CommandLine.Binding.CommandHandler<FileInfo, bool>(
	(file, jit) => { } // leere Hilfsfunktion 
));

root.Add(runCmd);

var compileCmd = new Command("compile", "Compile to .wasm (not fully implemented here)")
{
	new Argument<FileInfo>("file")
};
compileCmd.SetHandler((FileInfo file) =>
{
	Console.WriteLine("WASM compilation not fully implemented in this example :-)");
}, new System.CommandLine.Binding.CommandHandler<FileInfo>(
	(file) => { }
));
root.Add(compileCmd);

return root.Invoke(args);
