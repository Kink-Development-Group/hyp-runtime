//! Single source of truth for builtin function signatures.
//!
//! Every builtin the interpreter can dispatch (see
//! `interpreter/builtins.rs`) is described here exactly once. Consumers:
//!
//! - [`crate::TypeChecker`] registers all signatures for static checking
//! - the CLI `builtins` command renders an always-up-to-date listing
//!
//! **Adding a builtin?** Add its implementation to the matching
//! `call_*_builtin` dispatcher in `interpreter/builtins.rs` *and* one entry
//! to [`BUILTINS`]. The type checker and CLI pick it up automatically.

use hypnoscript_core::HypnoType;

/// Compact, `const`-friendly builtin parameter/return type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ty {
    Number,
    String,
    Boolean,
    /// `number[]`
    NumberArray,
    /// `string[]`
    StringArray,
    /// `unknown[]` — array of arbitrary values
    AnyArray,
    /// Arbitrary value (typed as `unknown`)
    Any,
    /// No meaningful return value (typed as `unknown`)
    Void,
}

impl Ty {
    /// Converts the compact type into the type checker's [`HypnoType`].
    pub fn to_hypno_type(self) -> HypnoType {
        match self {
            Ty::Number => HypnoType::number(),
            Ty::String => HypnoType::string(),
            Ty::Boolean => HypnoType::boolean(),
            Ty::NumberArray => HypnoType::create_array(HypnoType::number()),
            Ty::StringArray => HypnoType::create_array(HypnoType::string()),
            Ty::AnyArray => HypnoType::create_array(HypnoType::unknown()),
            Ty::Any | Ty::Void => HypnoType::unknown(),
        }
    }

    /// Human-readable type name for documentation output.
    pub fn display_name(self) -> &'static str {
        match self {
            Ty::Number => "number",
            Ty::String => "string",
            Ty::Boolean => "boolean",
            Ty::NumberArray => "number[]",
            Ty::StringArray => "string[]",
            Ty::AnyArray => "unknown[]",
            Ty::Any => "unknown",
            Ty::Void => "void",
        }
    }
}

/// Declarative description of a single builtin function.
#[derive(Debug, Clone, Copy)]
pub struct BuiltinSignature {
    pub name: &'static str,
    pub category: &'static str,
    pub params: &'static [Ty],
    pub returns: Ty,
}

impl BuiltinSignature {
    /// Renders `Name(number, string) -> boolean` style documentation.
    pub fn render(&self) -> String {
        let params: Vec<&str> = self.params.iter().map(|t| t.display_name()).collect();
        format!(
            "{}({}) -> {}",
            self.name,
            params.join(", "),
            self.returns.display_name()
        )
    }
}

/// Shorthand macro keeping the table below readable.
macro_rules! builtins {
    ($($category:literal { $($name:literal ($($param:ident),*) -> $ret:ident;)+ })+) => {
        &[
            $($(BuiltinSignature {
                name: $name,
                category: $category,
                params: &[$(Ty::$param),*],
                returns: Ty::$ret,
            },)+)+
        ]
    };
}

/// All builtin functions known to the HypnoScript toolchain.
pub const BUILTINS: &[BuiltinSignature] = builtins! {
    "Math" {
        "Sin"(Number) -> Number;
        "Cos"(Number) -> Number;
        "Tan"(Number) -> Number;
        "Sqrt"(Number) -> Number;
        "Log"(Number) -> Number;
        "Log10"(Number) -> Number;
        "Abs"(Number) -> Number;
        "Floor"(Number) -> Number;
        "Ceil"(Number) -> Number;
        "Round"(Number) -> Number;
        "Min"(Number, Number) -> Number;
        "Max"(Number, Number) -> Number;
        "Pow"(Number, Number) -> Number;
        "Factorial"(Number) -> Number;
        "Gcd"(Number, Number) -> Number;
        "Lcm"(Number, Number) -> Number;
        "Fibonacci"(Number) -> Number;
        "IsPrime"(Number) -> Boolean;
        "Clamp"(Number, Number, Number) -> Number;
    }
    "String" {
        "Length"(String) -> Number;
        "ToUpper"(String) -> String;
        "ToLower"(String) -> String;
        "Trim"(String) -> String;
        "Reverse"(String) -> String;
        "Capitalize"(String) -> String;
        "RemoveDuplicates"(String) -> String;
        "UniqueCharacters"(String) -> String;
        "ReverseWords"(String) -> String;
        "TitleCase"(String) -> String;
        "IndexOf"(String, String) -> Number;
        "Replace"(String, String, String) -> String;
        "StartsWith"(String, String) -> Boolean;
        "EndsWith"(String, String) -> Boolean;
        "Contains"(String, String) -> Boolean;
        "Split"(String, String) -> StringArray;
        "Substring"(String, Number, Number) -> String;
        "Repeat"(String, Number) -> String;
        "PadLeft"(String, Number, String) -> String;
        "PadRight"(String, Number, String) -> String;
        "IsEmpty"(String) -> Boolean;
        "IsWhitespace"(String) -> Boolean;
    }
    "Array" {
        "ArrayLength"(AnyArray) -> Number;
        "ArrayIsEmpty"(AnyArray) -> Boolean;
        "ArrayGet"(AnyArray, Number) -> Any;
        "ArrayIndexOf"(AnyArray, Any) -> Number;
        "ArrayContains"(AnyArray, Any) -> Boolean;
        "ArrayReverse"(AnyArray) -> AnyArray;
        "ArraySum"(NumberArray) -> Number;
        "ArrayAverage"(NumberArray) -> Number;
        "ArrayMin"(NumberArray) -> Number;
        "ArrayMax"(NumberArray) -> Number;
        "ArraySort"(NumberArray) -> NumberArray;
        "ArrayFirst"(AnyArray) -> Any;
        "ArrayLast"(AnyArray) -> Any;
        "ArrayTake"(AnyArray, Number) -> AnyArray;
        "ArraySkip"(AnyArray, Number) -> AnyArray;
        "ArraySlice"(AnyArray, Number, Number) -> AnyArray;
        "ArrayJoin"(AnyArray, String) -> String;
        "ArrayCount"(AnyArray, Any) -> Number;
        "ArrayDistinct"(AnyArray) -> AnyArray;
    }
    "Core / Hypnotic" {
        "Observe"(Any) -> Void;
        "Drift"(Number) -> Void;
        "DeepTrance"(Number) -> Void;
        "HypnoticCountdown"(Number) -> Void;
        "TranceInduction"(String) -> Void;
        "HypnoticVisualization"(String) -> Void;
        "ToInt"(Number) -> Number;
        "ToDouble"(String) -> Number;
        "ToString"(Any) -> String;
        "ToBoolean"(String) -> Boolean;
    }
    "File / IO" {
        "ReadFile"(String) -> String;
        "WriteFile"(String, String) -> Void;
        "AppendFile"(String, String) -> Void;
        "DeleteFile"(String) -> Void;
        "CreateDirectory"(String) -> Void;
        "FileExists"(String) -> Boolean;
        "IsFile"(String) -> Boolean;
        "IsDirectory"(String) -> Boolean;
        "ListDirectory"(String) -> StringArray;
        "GetFileSize"(String) -> Number;
        "CopyFile"(String, String) -> Number;
        "RenameFile"(String, String) -> Void;
        "GetFileExtension"(String) -> String;
        "GetFileName"(String) -> String;
        "GetParentDirectory"(String) -> String;
    }
    "Hashing / Utility" {
        "HashString"(String) -> Number;
        "HashNumber"(Number) -> Number;
        "SimpleRandom"(Number) -> Number;
        "AreAnagrams"(String, String) -> Boolean;
        "IsPalindrome"(String) -> Boolean;
        "CountOccurrences"(String, String) -> Number;
    }
    "Statistics" {
        "Mean"(NumberArray) -> Number;
        "Median"(NumberArray) -> Number;
        "Mode"(NumberArray) -> Number;
        "StandardDeviation"(NumberArray) -> Number;
        "Variance"(NumberArray) -> Number;
        "Range"(NumberArray) -> Number;
        "Percentile"(NumberArray, Number) -> Number;
        "Correlation"(NumberArray, NumberArray) -> Number;
        "LinearRegression"(NumberArray, NumberArray) -> NumberArray;
    }
    "System" {
        "GetCurrentDirectory"() -> String;
        "GetEnv"(String) -> String;
        "SetEnv"(String, String) -> Void;
        "GetOperatingSystem"() -> String;
        "GetArchitecture"() -> String;
        "GetCpuCount"() -> Number;
        "GetHostname"() -> String;
        "GetUsername"() -> String;
        "GetHomeDirectory"() -> String;
        "GetTempDirectory"() -> String;
        "GetArgs"() -> StringArray;
        "Exit"(Number) -> Void;
    }
    "Time / Date" {
        "CurrentTimestamp"() -> Number;
        "CurrentDate"() -> String;
        "CurrentTime"() -> String;
        "CurrentDateTime"() -> String;
        "FormatDateTime"(String) -> String;
        "DayOfWeek"() -> Number;
        "DayOfYear"() -> Number;
        "IsLeapYear"(Number) -> Boolean;
        "DaysInMonth"(Number, Number) -> Number;
        "CurrentYear"() -> Number;
        "CurrentMonth"() -> Number;
        "CurrentDay"() -> Number;
        "CurrentHour"() -> Number;
        "CurrentMinute"() -> Number;
        "CurrentSecond"() -> Number;
    }
    "Validation" {
        "IsValidEmail"(String) -> Boolean;
        "IsValidUrl"(String) -> Boolean;
        "IsValidPhoneNumber"(String) -> Boolean;
        "IsAlphanumeric"(String) -> Boolean;
        "IsAlphabetic"(String) -> Boolean;
        "IsNumeric"(String) -> Boolean;
        "IsLowercase"(String) -> Boolean;
        "IsUppercase"(String) -> Boolean;
        "IsInRange"(Number, Number, Number) -> Boolean;
        "MatchesPattern"(String, String) -> Boolean;
    }
    "Async / Promises" {
        "delayedValue"(Number, Any) -> Any;
        "instantPromise"(Any) -> Any;
        "promiseAll"(AnyArray) -> AnyArray;
        "promiseRace"(AnyArray) -> Any;
        "isPromiseResolved"(Any) -> Boolean;
    }
};

/// Looks up a builtin signature by name.
pub fn find(name: &str) -> Option<&'static BuiltinSignature> {
    BUILTINS.iter().find(|sig| sig.name == name)
}

/// Returns all categories in declaration order (deduplicated).
pub fn categories() -> Vec<&'static str> {
    let mut seen = Vec::new();
    for sig in BUILTINS {
        if !seen.contains(&sig.category) {
            seen.push(sig.category);
        }
    }
    seen
}

/// Returns all builtins belonging to `category`.
pub fn by_category(category: &str) -> impl Iterator<Item = &'static BuiltinSignature> {
    BUILTINS.iter().filter(move |sig| sig.category == category)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_no_duplicate_names() {
        let mut names: Vec<&str> = BUILTINS.iter().map(|s| s.name).collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len(), "duplicate builtin names in registry");
    }

    #[test]
    fn find_locates_builtins() {
        let sin = find("Sin").expect("Sin must be registered");
        assert_eq!(sin.params, &[Ty::Number]);
        assert_eq!(sin.returns, Ty::Number);
        assert!(find("DefinitelyNotABuiltin").is_none());
    }

    #[test]
    fn render_produces_readable_signature() {
        assert_eq!(
            find("Clamp").unwrap().render(),
            "Clamp(number, number, number) -> number"
        );
    }
}
