namespace HypnoScript.LexerParser.AST
{
	// AST-Basisinterfaces
	public interface IStatement { }
	public interface IExpression { }

	// Programm-Knoten
	public record ProgramNode(List<IStatement> Statements) : IStatement;

	// Beispiel: Variable Declaration
	public record VarDeclNode(
		string Identifier,
		string? TypeName,
		IExpression? Initializer,
		bool FromExternal
	) : IStatement;

	// Beispiel: Expression Statement
	public record ExpressionStatementNode(IExpression Expression) : IStatement;

	// If
	public record IfStatementNode(IExpression Condition, List<IStatement> ThenBranch, List<IStatement>? ElseBranch) : IStatement;

	// While
	public record WhileStatementNode(IExpression Condition, List<IStatement> Body) : IStatement;

	// Block
	public record BlockStatementNode(List<IStatement> Statements) : IStatement;

	// Function Declaration
	public record FunctionDeclNode(
		string Name,
		List<ParameterNode> Parameters,
		string? ReturnType,
		List<IStatement> Body,
		bool Imperative,
		bool Dominant
	) : IStatement;

	public record ParameterNode(string Name, string? TypeName);

	// Return (awaken)
	public record ReturnStatementNode(IExpression? Expression) : IStatement;

	// "observe"
	public record ObserveStatementNode(IExpression Expression) : IStatement;

	// Expression AST-Knoten
	public record BinaryExpressionNode(IExpression Left, string Operator, IExpression Right) : IExpression;
	public record LiteralExpressionNode(string Value, string LiteralType) : IExpression;
	public record IdentifierExpressionNode(string Name) : IExpression;
	public record CallExpressionNode(IExpression Callee, List<IExpression> Arguments) : IExpression;
	public record AssignmentExpressionNode(string Identifier, IExpression Value) : IExpression;

	// AST-Knoten für Sessions (Klassen)
	public record SessionDeclNode(
		string Name,
		List<IStatement> Members // Mitglieder können Fields und Methoden sein
	) : IStatement;

	// AST-Knoten für tranceify (Strukturdefinition)
	public record TranceifyDeclNode(
		string Name,
		List<VarDeclNode> Members
	) : IStatement;

	// AST-Knoten für Loop-Statements
	public record LoopStatementNode(
		IStatement? Initializer,      // z.B. induce i: number = 0;
		IExpression Condition,       // z.B. i < 10;
		IStatement? Iteration,       // z.B. i = i + 1;
		List<IStatement> Body        // Body der Schleife
	) : IStatement;
}
