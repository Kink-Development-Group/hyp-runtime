//! HypnoScript Runtime Library
//!
//! This module provides the runtime environment and builtin functions for HypnoScript.

pub mod advanced_string_builtins;
pub mod api_builtins;
pub mod array_builtins;
pub mod builtin_trait;
pub mod cli_builtins;
pub mod collection_builtins;
pub mod core_builtins;
pub mod data_builtins;
pub mod deepmind_builtins;
pub mod dictionary_builtins;
pub mod file_builtins;
pub mod hashing_builtins;
pub mod localization;
pub mod math_builtins;
pub mod service_builtins;
pub mod statistics_builtins;
pub mod string_builtins;
pub mod system_builtins;
pub mod time_builtins;
pub mod validation_builtins;

// Re-export builtin modules
pub use advanced_string_builtins::AdvancedStringBuiltins;
pub use api_builtins::{ApiBuiltins, ApiRequest, ApiResponse};
pub use array_builtins::ArrayBuiltins;
pub use builtin_trait::{BuiltinError, BuiltinModule, BuiltinResult};
pub use cli_builtins::{CliBuiltins, ParsedArguments};
pub use collection_builtins::CollectionBuiltins;
pub use core_builtins::CoreBuiltins;
pub use data_builtins::{CsvOptions, DataBuiltins, JsonQueryOptions};
pub use deepmind_builtins::DeepMindBuiltins;
pub use dictionary_builtins::DictionaryBuiltins;
pub use file_builtins::FileBuiltins;
pub use hashing_builtins::HashingBuiltins;
pub use localization::{Locale, LocalizedMessage, detect_locale};
pub use math_builtins::MathBuiltins;
pub use service_builtins::{RetrySchedule, ServiceBuiltins, ServiceHealthReport};
pub use statistics_builtins::StatisticsBuiltins;
pub use string_builtins::StringBuiltins;
pub use system_builtins::SystemBuiltins;
pub use time_builtins::TimeBuiltins;
pub use validation_builtins::ValidationBuiltins;
