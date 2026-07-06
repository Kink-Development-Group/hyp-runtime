//! Runtime error types for the HypnoScript interpreter.

use super::value::Value;
use thiserror::Error;

/// Interpreter errors that can occur during program execution.
///
/// These errors represent runtime failures in HypnoScript programs,
/// including type mismatches, undefined variables, and control flow errors.
#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Break statement outside of loop")]
    BreakOutsideLoop,

    #[error("Continue statement outside of loop")]
    ContinueOutsideLoop,

    #[error("'snap {0}' does not match any enclosing label")]
    LabeledBreak(String),

    #[error("'sink {0}' does not match any enclosing label")]
    LabeledContinue(String),

    #[error("Return from function: {0:?}")]
    Return(Value),

    #[error("Variable '{0}' not found")]
    UndefinedVariable(String),

    #[error("Type error: {0}")]
    TypeError(String),
}

/// Provide a simple locale-aware message while we prepare full i18n plumbing.
pub(crate) fn localized(en: &str, de: &str) -> String {
    format!("{} (DE: {})", en, de)
}
