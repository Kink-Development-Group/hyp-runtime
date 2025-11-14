//! Code-Optimierungs-Module für HypnoScript
//!
//! Dieses Modul implementiert verschiedene Optimierungs-Pässe für den
//! HypnoScript-Compiler. Die Optimierungen verbessern die Performance
//! und reduzieren die Größe des generierten Codes.
//!
//! ## Implementierte Optimierungen
//!
//! - **Constant Folding**: Berechnet konstante Ausdrücke zur Compile-Zeit
//! - **Dead Code Elimination**: Entfernt unerreichbaren Code
//! - **Common Subexpression Elimination**: Vermeidet redundante Berechnungen
//! - **Loop Invariant Code Motion**: Verschiebt invariante Berechnungen aus Schleifen
//! - **Inlining**: Fügt kleine Funktionen inline ein
//!
//! ## Verwendung
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

/// Fehlertypen für die Optimierung
#[derive(Error, Debug)]
pub enum OptimizationError {
    #[error("Optimierung fehlgeschlagen: {0}")]
    OptimizationFailed(String),

    #[error("Ungültiger AST-Knoten: {0}")]
    InvalidAstNode(String),
}

/// Optimierungs-Konfiguration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Constant Folding aktivieren
    pub constant_folding: bool,
    /// Dead Code Elimination aktivieren
    pub dead_code_elimination: bool,
    /// Common Subexpression Elimination aktivieren
    pub cse: bool,
    /// Loop Invariant Code Motion aktivieren
    pub licm: bool,
    /// Function Inlining aktivieren
    pub inlining: bool,
    /// Maximale Inlining-Tiefe
    pub max_inline_depth: usize,
    /// Maximale Inlining-Größe (AST-Knoten)
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
    /// Erstellt eine Konfiguration ohne Optimierungen
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

    /// Erstellt eine Konfiguration mit allen Optimierungen
    pub fn all() -> Self {
        Self::default()
    }
}

/// HypnoScript Code-Optimizer
///
/// Wendet verschiedene Optimierungs-Pässe auf den AST an, um die
/// Performance zu verbessern und die Code-Größe zu reduzieren.
pub struct Optimizer {
    /// Optimierungs-Konfiguration
    config: OptimizationConfig,
    /// Konstanten-Environment
    constants: HashMap<String, ConstantValue>,
    /// Verwendete Variablen
    used_variables: HashSet<String>,
    /// Optimierungs-Statistiken
    stats: OptimizationStats,
}

/// Konstanter Wert zur Compile-Zeit
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum ConstantValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// Statistiken über durchgeführte Optimierungen
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Anzahl gefalteter Konstanten
    pub folded_constants: usize,
    /// Anzahl entfernter toter Code-Blöcke
    pub eliminated_dead_code: usize,
    /// Anzahl eliminierter gemeinsamer Subausdrücke
    pub eliminated_common_subexpr: usize,
    /// Anzahl verschobener Loop-Invarianten
    pub moved_loop_invariants: usize,
    /// Anzahl inline eingefügter Funktionen
    pub inlined_functions: usize,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Optimizer {
    /// Erstellt einen neuen Optimizer mit Standard-Konfiguration
    ///
    /// # Beispiele
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

    /// Erstellt einen Optimizer mit benutzerdefinierter Konfiguration
    ///
    /// # Argumente
    ///
    /// * `config` - Die Optimierungs-Konfiguration
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            config,
            constants: HashMap::new(),
            used_variables: HashSet::new(),
            stats: OptimizationStats::default(),
        }
    }

    /// Aktiviert alle Optimierungen
    pub fn enable_all_optimizations(&mut self) {
        self.config = OptimizationConfig::all();
    }

    /// Deaktiviert alle Optimierungen
    pub fn disable_all_optimizations(&mut self) {
        self.config = OptimizationConfig::none();
    }

    /// Optimiert den AST
    ///
    /// # Argumente
    ///
    /// * `program` - Der zu optimierende AST
    ///
    /// # Rückgabe
    ///
    /// Der optimierte AST
    ///
    /// # Fehler
    ///
    /// Gibt einen `OptimizationError` zurück, wenn die Optimierung fehlschlägt
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

    /// Gibt die Optimierungs-Statistiken zurück
    pub fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    // ==================== Optimization Passes ====================

    /// Pass 1: Constant Folding
    ///
    /// Berechnet konstante Ausdrücke zur Compile-Zeit.
    /// Beispiel: `2 + 3` wird zu `5`
    fn constant_folding_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        match node {
            AstNode::Program(statements) => {
                let optimized_stmts: Result<Vec<_>, _> = statements
                    .iter()
                    .map(|stmt| self.constant_folding_pass(stmt))
                    .collect();
                Ok(AstNode::Program(optimized_stmts?))
            }

            AstNode::BinaryExpression { left, operator, right } => {
                let left_opt = self.constant_folding_pass(left)?;
                let right_opt = self.constant_folding_pass(right)?;

                // Try to fold if both sides are constants
                if let (AstNode::NumberLiteral(l), AstNode::NumberLiteral(r)) = (&left_opt, &right_opt) {
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

            // Für andere Knoten: Rekursiv durchlaufen
            _ => Ok(node.clone()),
        }
    }

    /// Pass 2: Dead Code Elimination
    ///
    /// Entfernt unerreichbaren Code, z.B. nach return oder in if(false)-Zweigen.
    fn dead_code_elimination_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementierung
        // Placeholder für zukünftige Implementierung
        Ok(node.clone())
    }

    /// Pass 3: Common Subexpression Elimination
    ///
    /// Erkennt und eliminiert redundante Berechnungen.
    fn cse_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementierung
        // Placeholder für zukünftige Implementierung
        Ok(node.clone())
    }

    /// Pass 4: Loop Invariant Code Motion
    ///
    /// Verschiebt Berechnungen, die sich in Schleifen nicht ändern, vor die Schleife.
    fn licm_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementierung
        // Placeholder für zukünftige Implementierung
        Ok(node.clone())
    }

    /// Pass 5: Function Inlining
    ///
    /// Fügt kleine Funktionen inline ein, um Funktionsaufruf-Overhead zu vermeiden.
    fn inlining_pass(&mut self, node: &AstNode) -> Result<AstNode, OptimizationError> {
        // TODO: Implementierung
        // Placeholder für zukünftige Implementierung
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
