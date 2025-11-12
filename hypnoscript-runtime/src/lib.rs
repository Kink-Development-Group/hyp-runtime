//! HypnoScript Runtime Library
//! 
//! This module provides the runtime environment and builtin functions for HypnoScript.

pub mod core_builtins;
pub mod math_builtins;
pub mod string_builtins;
pub mod array_builtins;
pub mod time_builtins;
pub mod validation_builtins;
pub mod file_builtins;

// Re-export builtin modules
pub use core_builtins::CoreBuiltins;
pub use math_builtins::MathBuiltins;
pub use string_builtins::StringBuiltins;
pub use array_builtins::ArrayBuiltins;
pub use time_builtins::TimeBuiltins;
pub use validation_builtins::ValidationBuiltins;
pub use file_builtins::FileBuiltins;
