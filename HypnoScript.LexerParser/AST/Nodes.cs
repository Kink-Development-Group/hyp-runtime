namespace HypnoScript.LexerParser.AST
{
	// AST-Basisinterfaces
	public interface IStatement { }
	public interface IExpression { }

	// Programm-Knoten
	public record ProgramNode(List<IStatement> Statements) : IStatement;

	// Entrance-Block am Programmanfang
	public record EntranceBlockNode(List<IStatement> Statements) : IStatement;

	// Variablen-Deklaration
	public record VarDeclNode(
		string Identifier,
		string? TypeName,
		IExpression? Initializer,
		bool FromExternal
	) : IStatement;

	// Expression Statement
	public record ExpressionStatementNode(IExpression Expression) : IStatement;

	// Kontrollstrukturen
	public record IfStatementNode(IExpression Condition, List<IStatement> ThenBranch, List<IStatement>? ElseBranch) : IStatement;
	public record WhileStatementNode(IExpression Condition, List<IStatement> Body) : IStatement;
	public record LoopStatementNode(
		IStatement? Initializer,      // z.B. induce i: number = 0;
		IExpression Condition,       // z.B. i < 10;
		IStatement? Iteration,       // z.B. i = i + 1;
		List<IStatement> Body        // Body der Schleife
	) : IStatement;

	// Break und Continue
	public record SnapStatementNode() : IStatement;  // break
	public record SinkStatementNode() : IStatement;  // continue
	public record SinkToNode(string LabelName) : IStatement;  // goto

	// Labels
	public record LabelNode(string Name) : IStatement;

	// Block
	public record BlockStatementNode(List<IStatement> Statements) : IStatement;

	// Funktionen
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

	// Ein-/Ausgabe
	public record ObserveStatementNode(IExpression Expression) : IStatement;
	public record DriftStatementNode(IExpression Milliseconds) : IStatement;

	// Objektorientierung - Sessions (Klassen)
	public record SessionDeclNode(
		string Name,
		List<SessionMemberNode> Members
	) : IStatement;

	public record SessionMemberNode(
		bool IsExposed,      // expose/conceal
		bool IsDominant,     // dominant
		IStatement Declaration
	) : IStatement;

	// Strukturen - Tranceify
	public record TranceifyDeclNode(
		string Name,
		List<VarDeclNode> Members
	) : IStatement;

	// Module und Globale
	public record MindLinkNode(string FileName) : IStatement;  // import
	public record SharedTranceVarDeclNode(string Identifier, string? TypeName, IExpression? Initializer) : IStatement;  // global

	// Expression AST-Knoten
	public record BinaryExpressionNode(IExpression Left, string Operator, IExpression Right) : IExpression;
	public record LiteralExpressionNode(string Value, string LiteralType) : IExpression;
	public record IdentifierExpressionNode(string Name) : IExpression;
	public record CallExpressionNode(IExpression Callee, List<IExpression> Arguments) : IExpression;
	public record AssignmentExpressionNode(string Identifier, IExpression Value) : IExpression;

	// Objektorientierung - Methodenaufruf und Feldzugriff
	public record MethodCallExpressionNode(IExpression Target, string MethodName, List<IExpression> Arguments) : IExpression;
	public record FieldAccessExpressionNode(IExpression Target, string FieldName) : IExpression;

	// Strukturen - Record-Literal für tranceify-Instanzen
	public record RecordLiteralExpressionNode(
		string TypeName,
		Dictionary<string, IExpression> Fields
	) : IExpression;

	// Session-Instanziierung
	public record SessionInstantiationNode(
		string SessionName,
		List<IExpression> Arguments
	) : IExpression;

	// Unary Expressions
	public record UnaryExpressionNode(string Operator, IExpression Operand) : IExpression;

	// Parenthesized Expression
	public record ParenthesizedExpressionNode(IExpression Expression) : IExpression;

	// Array Access
	public record ArrayAccessExpressionNode(IExpression Array, IExpression Index) : IExpression;

	// Array Literal
	public record ArrayLiteralExpressionNode(List<IExpression> Elements) : IExpression;
}
