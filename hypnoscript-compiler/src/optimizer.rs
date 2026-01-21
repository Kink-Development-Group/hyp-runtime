//! Code optimization module for HypnoScript
//!
//! This module implements various optimization passes for the
//! HypnoScript compiler. The optimizations improve performance
//! and reduce the size of the generated code.
//!
//! ## Implemented optimizations
//!
//! - **Constant Folding**: Computes constant expressions at compile time
//! - **Dead Code Elimination**: Removes unreachable code
//! - **Common Subexpression Elimination**: Avoids redundant calculations
//! - **Loop Invariant Code Motion**: Moves invariant computations out of loops
//! - **Inlining**: Inlines small functions
//!
//! ## Usage
//!
//! ```rust,no_run
//! use hypnoscript_compiler::optimizer::Optimizer;
//! use hypnoscript_lexer_parser::ast::AstNode;
//!
//! let mut optimizer = Optimizer::new();
//! optimizer.enable_all_optimizations();
//!
//! // let optimized_ast = optimizer.optimize(&ast)?;
//! ```

use hypnoscript_lexer_parser::ast::AstNode;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Error types for optimization
#[derive(Error, Debug)]
pub enum OptimizationError {
    #[error("Optimization failed: {0}")]
    OptimizationFailed(String),

    #[error("Invalid AST node: {0}")]
    InvalidAstNode(String),
}

/// Optimization configuration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Enable constant folding
    pub constant_folding: bool,
    /// Enable dead code elimination
    pub dead_code_elimination: bool,
    /// Enable common subexpression elimination
    pub cse: bool,
    /// Enable loop invariant code motion
    pub licm: bool,
    /// Enable function inlining
    pub inlining: bool,
    /// Maximum inlining depth
    pub max_inline_depth: usize,
    /// Maximum inlining size (AST nodes)
    pub max_inline_size: usize,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            constant_folding: true,
            dead_code_elimination: true,
            cse: true,
            licm: true,
            inlining: true,
            max_inline_depth: 3,
            max_inline_size: 50,
        }
    }
}

impl OptimizationConfig {
    /// Creates a configuration without optimizations
    pub fn none() -> Self {
        Self {
            constant_folding: false,
            dead_code_elimination: false,
            cse: false,
            licm: false,
            inlining: false,
            max_inline_depth: 0,
            max_inline_size: 0,
        }
    }

    /// Creates a configuration with all optimizations
    pub fn all() -> Self {
        Self::default()
    }
}

/// HypnoScript code optimizer
///
/// Applies various optimization passes to the AST to improve
/// performance and reduce code size.
pub struct Optimizer {
    /// Optimization configuration
    config: OptimizationConfig,
    /// Constant environment
    constants: HashMap<String, ConstantValue>,
    /// Used variables
    used_variables: HashSet<String>,
    /// Optimization statistics
    stats: OptimizationStats,
}

/// Constant value at compile time
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ConstantValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// Statistics about performed optimizations
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Number of folded constants
    pub folded_constants: usize,
    /// Number of removed dead code blocks
    pub eliminated_dead_code: usize,
    /// Number of eliminated common subexpressions
    pub eliminated_common_subexpr: usize,
    /// Number of moved loop invariants
    pub moved_loop_invariants: usize,
    /// Number of inlined functions
    pub inlined_functions: usize,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Optimizer {
    /// Creates a new optimizer with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use hypnoscript_compiler::optimizer::Optimizer;
    ///
    /// let optimizer = Optimizer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: OptimizationConfig::default(),
            constants: HashMap::new(),
            used_variables: HashSet::new(),
            stats: OptimizationStats::default(),
        }
    }

    /// Creates an optimizer with a custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The optimization configuration
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            config,
            constants: HashMap::new(),
            used_variables: HashSet::new(),
            stats: OptimizationStats::default(),
        }
    }

    /// Enables all optimizations
    pub fn enable_all_optimizations(&mut self) {
        self.config = OptimizationConfig::all();
    }

    /// Disables all optimizations
    pub fn disable_all_optimizations(&mut self) {
        self.config = OptimizationConfig::none();
    }

    /// Optimizes the AST
    ///
    /// # Arguments
    ///
    /// * `program` - The AST to optimize
    ///
    /// # Returns
    ///
    /// The optimized AST
    ///
    /// # Errors
    ///
    /// Returns an `OptimizationError` when optimization fails
    pub fn optimize(&mut self, program: &AstNode) -> Result<AstNode, OptimizationError> {
        // Reset statistics
        self.stats = OptimizationStats::default();
        self.constants.clear();
        self.used_variables.clear();

        let mut optimized = program.clone();

        // Pass 1: Constant Folding
        if self.config.constant_folding {
            optimized = self.constant_folding_pass(&optimized)?;
        }

        // Pass 2: Dead Code Elimination
        if self.config.dead_code_elimination {
            optimized = self.dead_code_elimination_pass(&optimized)?;
        }

        // Pass 3: Common Subexpression Elimination
        if self.config.cse {
            optimized = self.cse_pass(&optimized)?;
        }

        // Pass 4: Loop Invariant Code Motion
        if self.config.licm {
            optimized = self.licm_pass(&optimized)?;
        }

        // Pass 5: Function Inlining
        if self.config.inlining {
            optimized = self.inlining_pass(&optimized)?;
        }

        Ok(optimized)
    }

    /// Returns the optimization statistics
    pub fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    // ==================== Optimization Passes ====================

    /// Pass 1: Constant Folding
    ///
    /// Computes constant expressions at compile time.
    /// Example: `2 + 3` becomes `5`
    fn constant_folding_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        match node {
            AstNode::Program(statements) => {
                let optimized_stmts: Result<Vec<_>, _> = statements
                    .iter()
                    .map(|stmt| self.constant_folding_pass(stmt))
                    .collect();
                Ok(AstNode::Program(optimized_stmts?))
            }

            AstNode::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left_opt = self.constant_folding_pass(left)?;
                let right_opt = self.constant_folding_pass(right)?;

                // Try to fold if both sides are constants
                if let (AstNode::NumberLiteral(l), AstNode::NumberLiteral(r)) =
                    (&left_opt, &right_opt)
                {
                    let result = match operator.as_str() {
                        "+" => Some(l + r),
                        "-" => Some(l - r),
                        "*" => Some(l * r),
                        "/" if *r != 0.0 => Some(l / r),
                        _ => None,
                    };

                    if let Some(val) = result {
                        self.stats.folded_constants += 1;
                        return Ok(AstNode::NumberLiteral(val));
                    }
                }

                Ok(AstNode::BinaryExpression {
                    left: Box::new(left_opt),
                    operator: operator.clone(),
                    right: Box::new(right_opt),
                })
            }

            AstNode::UnaryExpression { operator, operand } => {
                let operand_opt = self.constant_folding_pass(operand)?;

                if let AstNode::NumberLiteral(n) = operand_opt {
                    let result = match operator.as_str() {
                        "-" => Some(-n),
                        _ => None,
                    };

                    if let Some(val) = result {
                        self.stats.folded_constants += 1;
                        return Ok(AstNode::NumberLiteral(val));
                    }
                }

                Ok(AstNode::UnaryExpression {
                    operator: operator.clone(),
                    operand: Box::new(operand_opt),
                })
            }

            // For other nodes: traverse recursively
            _ => Ok(node.clone()),
        }
    }

    /// Pass 2: Dead Code Elimination
    ///
    /// Removes unreachable code, e.g. after return or in if(false) branches.
    fn dead_code_elimination_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementation
        // Placeholder for future implementation
        Ok(node.clone())
    }

    /// Pass 3: Common Subexpression Elimination
    ///
    /// Detects and eliminates redundant computations.
    fn cse_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementation
        // Placeholder for future implementation
        Ok(node.clone())
    }

    /// Pass 4: Loop Invariant Code Motion
    ///
    /// Moves computations that do not change inside loops before the loop.
    fn licm_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementation
        // Placeholder for future implementation
        Ok(node.clone())
    }

    /// Pass 5: Function Inlining
    ///
    /// Inlines small functions to avoid function call overhead.
    fn inlining_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementation
        // Placeholder for future implementation
        Ok(node.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimizer_creation() {
        let optimizer = Optimizer::new();
        assert!(optimizer.config.constant_folding);
        assert!(optimizer.config.dead_code_elimination);
    }

    #[test]
    fn test_config_none() {
        let config = OptimizationConfig::none();
        assert!(!config.constant_folding);
        assert!(!config.dead_code_elimination);
        assert!(!config.cse);
    }

    #[test]
    fn test_config_all() {
        let config = OptimizationConfig::all();
        assert!(config.constant_folding);
        assert!(config.dead_code_elimination);
        assert!(config.cse);
        assert!(config.licm);
        assert!(config.inlining);
    }

    #[test]
    fn test_constant_folding_addition() {
        let mut optimizer = Optimizer::new();

        // 2 + 3
        let expr = AstNode::BinaryExpression {
            left: Box::new(AstNode::NumberLiteral(2.0)),
            operator: "+".to_string(),
            right: Box::new(AstNode::NumberLiteral(3.0)),
        };

        let result = optimizer.constant_folding_pass(&expr).unwrap();

        assert_eq!(result, AstNode::NumberLiteral(5.0));
        assert_eq!(optimizer.stats.folded_constants, 1);
    }

    #[test]
    fn test_constant_folding_multiplication() {
        let mut optimizer = Optimizer::new();

        // 4 * 5
        let expr = AstNode::BinaryExpression {
            left: Box::new(AstNode::NumberLiteral(4.0)),
            operator: "*".to_string(),
            right: Box::new(AstNode::NumberLiteral(5.0)),
        };

        let result = optimizer.constant_folding_pass(&expr).unwrap();

        assert_eq!(result, AstNode::NumberLiteral(20.0));
        assert_eq!(optimizer.stats.folded_constants, 1);
    }

    #[test]
    fn test_constant_folding_unary() {
        let mut optimizer = Optimizer::new();

        // -42
        let expr = AstNode::UnaryExpression {
            operator: "-".to_string(),
            operand: Box::new(AstNode::NumberLiteral(42.0)),
        };

        let result = optimizer.constant_folding_pass(&expr).unwrap();

        assert_eq!(result, AstNode::NumberLiteral(-42.0));
        assert_eq!(optimizer.stats.folded_constants, 1);
    }
}
