using Xunit;
using HypnoScript.Compiler.Analysis;
using HypnoScript.LexerParser.AST;
using System.Collections.Generic;
using HypnoScript.Compiler.Error;

namespace HypnoScript.Compiler.Tests
{
    public class TypeCheckerTests
    {
        [Fact]
        public void UnknownType_ShouldReportError()
        {
            // Arrange: Variable mit unbekanntem Typ
            var program = new ProgramNode(new List<IStatement>
            {
                new VarDeclNode("x", null, new IdentifierExpressionNode("y"), false)
            });
            var checker = new TypeChecker();
            ErrorReporter.ClearErrors();

            // Act
            checker.Check(program);
            var errors = ErrorReporter.GetErrors();

            // Debug-Ausgabe aller Fehler
            foreach (var err in errors)
            {
                System.Console.WriteLine($"[TEST-DEBUG] Error: {err}");
            }
            // Assert
            Assert.Contains(errors, e => e.Contains("could not be inferred (unknown type)"));
        }

        [Fact]
        public void MindLink_ShouldImportSymbols()
        {
            // Arrange: MindLink importiert Dummy-Symbole
            var program = new ProgramNode(new List<IStatement>
            {
                new MindLinkNode("dummy.hyp"),
                new VarDeclNode("z", "number", new IdentifierExpressionNode("importedVar"), false)
            });
            var checker = new TypeChecker();
            ErrorReporter.ClearErrors();

            // Act
            checker.Check(program);
            var errors = ErrorReporter.GetErrors();

            // Assert: Kein Fehler bzgl. 'importedVar' oder unknown
            Assert.DoesNotContain(errors, e => e.Contains("importedVar"));
            Assert.DoesNotContain(errors, e => e.Contains("unknown"));
        }
    }
}
