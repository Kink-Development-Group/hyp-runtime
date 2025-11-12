use crate::token::Token;
use hypnoscript_core::HypnoType;
use serde::{Deserialize, Serialize};

/// AST node types for HypnoScript
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    // Program structure
    Program(Vec<AstNode>),
    FocusBlock(Vec<AstNode>),
    
    // Declarations
    VariableDeclaration {
        name: String,
        type_annotation: Option<String>,
        initializer: Option<Box<AstNode>>,
    },
    
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<String>,
        body: Vec<AstNode>,
    },
    
    SessionDeclaration {
        name: String,
        members: Vec<AstNode>,
    },
    
    // Statements
    ExpressionStatement(Box<AstNode>),
    ObserveStatement(Box<AstNode>),
    IfStatement {
        condition: Box<AstNode>,
        then_branch: Vec<AstNode>,
        else_branch: Option<Vec<AstNode>>,
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
                | AstNode::IfStatement { .. }
                | AstNode::WhileStatement { .. }
                | AstNode::LoopStatement { .. }
                | AstNode::ReturnStatement(_)
                | AstNode::BreakStatement
                | AstNode::ContinueStatement
        )
    }

    /// Check if the node is a declaration
    pub fn is_declaration(&self) -> bool {
        matches!(
            self,
            AstNode::VariableDeclaration { .. }
                | AstNode::FunctionDeclaration { .. }
                | AstNode::SessionDeclaration { .. }
        )
    }
}
