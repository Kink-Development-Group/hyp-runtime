use serde::{Deserialize, Serialize};

/// AST node types for HypnoScript
///
/// This enum represents all possible Abstract Syntax Tree nodes in the HypnoScript language.
/// HypnoScript is an esoteric, TypeScript-inspired language with hypnotic-themed keywords.
///
/// # Language Concepts
///
/// - **Focus/Relax**: Program boundaries (main block)
/// - **induce/implant/freeze**: Variable declarations (var/let/const equivalents)
/// - **suggestion/trigger**: Function declarations
/// - **session**: Class declarations
/// - **entrance/finale**: Constructor/destructor blocks
/// - **observe/whisper/command**: Output statements
/// - **anchor**: State snapshot/variable backup
/// - **oscillate**: Boolean toggle operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    // Program structure
    Program(Vec<AstNode>),
    FocusBlock(Vec<AstNode>),
    EntranceBlock(Vec<AstNode>),  // Constructor/setup block
    FinaleBlock(Vec<AstNode>),    // Destructor/cleanup block

    // Declarations
    VariableDeclaration {
        name: String,
        type_annotation: Option<String>,
        initializer: Option<Box<AstNode>>,
        is_constant: bool,  // true for 'freeze', false for 'induce'/'implant'
    },

    /// Anchor statement: saves the current value of a variable for later restoration
    /// Example: anchor savedValue = currentValue;
    AnchorDeclaration {
        name: String,
        source: Box<AstNode>,
    },

    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<String>,
        body: Vec<AstNode>,
    },

    /// Trigger declaration: event handler or callback function
    /// Similar to function but specifically for event handling
    TriggerDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<String>,
        body: Vec<AstNode>,
    },

    SessionDeclaration {
        name: String,
        members: Vec<SessionMember>,
    },

    // Statements
    ExpressionStatement(Box<AstNode>),

    /// observe: Output with newline (like console.log)
    ObserveStatement(Box<AstNode>),

    /// whisper: Output without newline
    WhisperStatement(Box<AstNode>),

    /// command: Imperative output (usually uppercase/emphasized)
    CommandStatement(Box<AstNode>),

    IfStatement {
        condition: Box<AstNode>,
        then_branch: Vec<AstNode>,
        else_branch: Option<Vec<AstNode>>,
    },

    /// deepFocus: Enhanced if-statement with deeper scope
    DeepFocusStatement {
        condition: Box<AstNode>,
        body: Vec<AstNode>,
    },

    WhileStatement {
        condition: Box<AstNode>,
        body: Vec<AstNode>,
    },
    LoopStatement {
        body: Vec<AstNode>,
    },
    ReturnStatement(Option<Box<AstNode>>),
    BreakStatement,
    ContinueStatement,

    /// oscillate: Toggle a boolean variable
    /// Example: oscillate myFlag;
    OscillateStatement {
        target: Box<AstNode>,
    },

    // Expressions
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),

    BinaryExpression {
        left: Box<AstNode>,
        operator: String,
        right: Box<AstNode>,
    },

    UnaryExpression {
        operator: String,
        operand: Box<AstNode>,
    },

    CallExpression {
        callee: Box<AstNode>,
        arguments: Vec<AstNode>,
    },

    MemberExpression {
        object: Box<AstNode>,
        property: String,
    },

    ArrayLiteral(Vec<AstNode>),

    IndexExpression {
        object: Box<AstNode>,
        index: Box<AstNode>,
    },

    AssignmentExpression {
        target: Box<AstNode>,
        value: Box<AstNode>,
    },
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<String>,
}

impl Parameter {
    pub fn new(name: String, type_annotation: Option<String>) -> Self {
        Self {
            name,
            type_annotation,
        }
    }
}

impl AstNode {
    /// Check if the node is an expression
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            AstNode::NumberLiteral(_)
                | AstNode::StringLiteral(_)
                | AstNode::BooleanLiteral(_)
                | AstNode::Identifier(_)
                | AstNode::BinaryExpression { .. }
                | AstNode::UnaryExpression { .. }
                | AstNode::CallExpression { .. }
                | AstNode::MemberExpression { .. }
                | AstNode::ArrayLiteral(_)
                | AstNode::IndexExpression { .. }
                | AstNode::AssignmentExpression { .. }
        )
    }

    /// Check if the node is a statement
    pub fn is_statement(&self) -> bool {
        matches!(
            self,
            AstNode::ExpressionStatement(_)
                | AstNode::ObserveStatement(_)
                | AstNode::WhisperStatement(_)
                | AstNode::CommandStatement(_)
                | AstNode::IfStatement { .. }
                | AstNode::DeepFocusStatement { .. }
                | AstNode::WhileStatement { .. }
                | AstNode::LoopStatement { .. }
                | AstNode::ReturnStatement(_)
                | AstNode::BreakStatement
                | AstNode::ContinueStatement
                | AstNode::OscillateStatement { .. }
        )
    }

    /// Check if the node is a declaration
    pub fn is_declaration(&self) -> bool {
        matches!(
            self,
            AstNode::VariableDeclaration { .. }
                | AstNode::AnchorDeclaration { .. }
                | AstNode::FunctionDeclaration { .. }
                | AstNode::TriggerDeclaration { .. }
                | AstNode::SessionDeclaration { .. }
        )
    }
}

/// Visibility for session members
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionVisibility {
    Public,
    Private,
}

/// Members that may appear inside a session declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SessionMember {
    Field(SessionField),
    Method(SessionMethod),
}

/// Session field definition within the AST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionField {
    pub name: String,
    pub type_annotation: Option<String>,
    pub initializer: Option<Box<AstNode>>,
    pub visibility: SessionVisibility,
    pub is_static: bool,
}

/// Session method definition within the AST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Vec<AstNode>,
    pub visibility: SessionVisibility,
    pub is_static: bool,
    pub is_constructor: bool,
}
