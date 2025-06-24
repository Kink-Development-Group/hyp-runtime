using System;
using System.IO;
using System.Linq;
using HypnoScript.LexerParser.Lexer;
using HypnoScript.LexerParser.Parser;
using HypnoScript.Compiler.Analysis;
using HypnoScript.Compiler.CodeGen;
using HypnoScript.CLI;

namespace HypnoScript.CLI.Commands
{
    public static class CompileCommand
    {
        public static int Execute(string filePath, bool debug, bool verbose)
        {
            AppLogger.Info("=== COMPILE MODE ===");

            if (!File.Exists(filePath))
            {
                AppLogger.Error($"File not found: {filePath}");
                return 2;
            }

            try
            {
                if (debug) AppLogger.Debug($"Reading file: {filePath}");
                var source = File.ReadAllText(filePath);
                if (debug) AppLogger.Debug($"File read, length: {source.Length}");

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
                AppLogger.Info($"✓ Compiled to: {outputPath}");

                if (verbose)
                {
                    AppLogger.Info($"📄 Generated {wasmCode.Length} characters of WASM code");
                }

                return 0;
            }
            catch (Exception ex)
            {
                AppLogger.Error($"Compilation failed for {filePath}", ex);
                return 1;
            }
        }
    }
}
