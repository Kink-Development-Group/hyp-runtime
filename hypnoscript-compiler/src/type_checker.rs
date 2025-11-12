use hypnoscript_core::{HypnoBaseType, HypnoType};
use hypnoscript_lexer_parser::ast::{
    AstNode, SessionField, SessionMember, SessionMethod, SessionVisibility,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SessionFieldInfo {
    ty: HypnoType,
    visibility: SessionVisibility,
    is_static: bool,
}

#[derive(Debug, Clone)]
struct SessionMethodInfo {
    parameter_types: Vec<HypnoType>,
    return_type: HypnoType,
    visibility: SessionVisibility,
    is_static: bool,
    is_constructor: bool,
}

#[derive(Debug, Clone)]
struct SessionInfo {
    name: String,
    instance_fields: HashMap<String, SessionFieldInfo>,
    static_fields: HashMap<String, SessionFieldInfo>,
    instance_methods: HashMap<String, SessionMethodInfo>,
    static_methods: HashMap<String, SessionMethodInfo>,
    constructor: Option<SessionMethodInfo>,
}

impl SessionInfo {
    fn new(name: String) -> Self {
        Self {
            name,
            instance_fields: HashMap::new(),
            static_fields: HashMap::new(),
            instance_methods: HashMap::new(),
            static_methods: HashMap::new(),
            constructor: None,
        }
    }
}

/// Type checker for HypnoScript programs
pub struct TypeChecker {
    // Type environment for variables
    type_env: HashMap<String, HypnoType>,
    // Function signatures
    function_types: HashMap<String, (Vec<HypnoType>, HypnoType)>,
    // Current function return type (for return statement checking)
    current_function_return_type: Option<HypnoType>,
    // Session metadata cache
    sessions: HashMap<String, SessionInfo>,
    // Currently checked session context (if any)
    current_session: Option<String>,
    // Indicates whether we are inside a static method scope
    in_static_context: bool,
    // Error messages
    errors: Vec<String>,
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        let mut checker = Self {
            type_env: HashMap::new(),
            function_types: HashMap::new(),
            current_function_return_type: None,
            sessions: HashMap::new(),
            current_session: None,
            in_static_context: false,
            errors: Vec::new(),
        };

        // Register builtin functions
        checker.register_builtins();

        checker
    }

    /// Register builtin function signatures
    fn register_builtins(&mut self) {
        // Math
        for name in [
            "Sin", "Cos", "Tan", "Sqrt", "Log", "Log10", "Abs", "Floor", "Ceil", "Round",
        ] {
            self.register_builtin(name, vec![HypnoType::number()], HypnoType::number());
        }
        for name in ["Min", "Max", "Pow"] {
            self.register_builtin(
                name,
                vec![HypnoType::number(), HypnoType::number()],
                HypnoType::number(),
            );
        }
        for name in ["Factorial", "Gcd", "Lcm", "Fibonacci"] {
            self.register_builtin(name, vec![HypnoType::number()], HypnoType::number());
        }
        self.register_builtin("IsPrime", vec![HypnoType::number()], HypnoType::boolean());
        self.register_builtin(
            "Clamp",
            vec![
                HypnoType::number(),
                HypnoType::number(),
                HypnoType::number(),
            ],
            HypnoType::number(),
        );

        // Strings
        self.register_builtin("Length", vec![HypnoType::string()], HypnoType::number());
        for name in [
            "ToUpper",
            "ToLower",
            "Trim",
            "Reverse",
            "Capitalize",
            "RemoveDuplicates",
            "UniqueCharacters",
            "ReverseWords",
            "TitleCase",
        ] {
            self.register_builtin(name, vec![HypnoType::string()], HypnoType::string());
        }
        self.register_builtin(
            "IndexOf",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::number(),
        );
        self.register_builtin(
            "Replace",
            vec![
                HypnoType::string(),
                HypnoType::string(),
                HypnoType::string(),
            ],
            HypnoType::string(),
        );
        self.register_builtin(
            "StartsWith",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::boolean(),
        );
        self.register_builtin(
            "EndsWith",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::boolean(),
        );
        self.register_builtin(
            "Contains",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::boolean(),
        );
        self.register_builtin(
            "Split",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::create_array(HypnoType::string()),
        );
        self.register_builtin(
            "Substring",
            vec![
                HypnoType::string(),
                HypnoType::number(),
                HypnoType::number(),
            ],
            HypnoType::string(),
        );
        self.register_builtin(
            "Repeat",
            vec![HypnoType::string(), HypnoType::number()],
            HypnoType::string(),
        );
        for name in ["PadLeft", "PadRight"] {
            self.register_builtin(
                name,
                vec![
                    HypnoType::string(),
                    HypnoType::number(),
                    HypnoType::string(),
                ],
                HypnoType::string(),
            );
        }
        self.register_builtin("IsEmpty", vec![HypnoType::string()], HypnoType::boolean());
        self.register_builtin(
            "IsWhitespace",
            vec![HypnoType::string()],
            HypnoType::boolean(),
        );

        // Arrays
        let any_array = || HypnoType::create_array(HypnoType::unknown());
        let number_array = || HypnoType::create_array(HypnoType::number());
        let string_array = || HypnoType::create_array(HypnoType::string());

        self.register_builtin("ArrayLength", vec![any_array()], HypnoType::number());
        self.register_builtin("ArrayIsEmpty", vec![any_array()], HypnoType::boolean());
        self.register_builtin(
            "ArrayGet",
            vec![any_array(), HypnoType::number()],
            HypnoType::unknown(),
        );
        self.register_builtin(
            "ArrayIndexOf",
            vec![any_array(), HypnoType::unknown()],
            HypnoType::number(),
        );
        self.register_builtin(
            "ArrayContains",
            vec![any_array(), HypnoType::unknown()],
            HypnoType::boolean(),
        );
        self.register_builtin("ArrayReverse", vec![any_array()], any_array());
        for name in ["ArraySum", "ArrayAverage", "ArrayMin", "ArrayMax"] {
            self.register_builtin(name, vec![number_array()], HypnoType::number());
        }
        self.register_builtin("ArraySort", vec![number_array()], number_array());
        for name in ["ArrayFirst", "ArrayLast"] {
            self.register_builtin(name, vec![any_array()], HypnoType::unknown());
        }
        for name in ["ArrayTake", "ArraySkip"] {
            self.register_builtin(name, vec![any_array(), HypnoType::number()], any_array());
        }
        self.register_builtin(
            "ArraySlice",
            vec![any_array(), HypnoType::number(), HypnoType::number()],
            any_array(),
        );
        self.register_builtin(
            "ArrayJoin",
            vec![any_array(), HypnoType::string()],
            HypnoType::string(),
        );
        self.register_builtin(
            "ArrayCount",
            vec![any_array(), HypnoType::unknown()],
            HypnoType::number(),
        );
        self.register_builtin("ArrayDistinct", vec![any_array()], any_array());

        // Core / Hypnotic
        self.register_builtin("Observe", vec![HypnoType::unknown()], HypnoType::unknown());
        for name in ["Drift", "DeepTrance", "HypnoticCountdown"] {
            self.register_builtin(name, vec![HypnoType::number()], HypnoType::unknown());
        }
        self.register_builtin(
            "TranceInduction",
            vec![HypnoType::string()],
            HypnoType::unknown(),
        );
        self.register_builtin(
            "HypnoticVisualization",
            vec![HypnoType::string()],
            HypnoType::unknown(),
        );
        self.register_builtin("ToInt", vec![HypnoType::number()], HypnoType::number());
        self.register_builtin("ToDouble", vec![HypnoType::string()], HypnoType::number());
        self.register_builtin("ToString", vec![HypnoType::unknown()], HypnoType::string());
        self.register_builtin("ToBoolean", vec![HypnoType::string()], HypnoType::boolean());

        // File / IO
        self.register_builtin("ReadFile", vec![HypnoType::string()], HypnoType::string());
        for name in ["WriteFile", "AppendFile"] {
            self.register_builtin(
                name,
                vec![HypnoType::string(), HypnoType::string()],
                HypnoType::unknown(),
            );
        }
        for name in ["DeleteFile", "CreateDirectory"] {
            self.register_builtin(name, vec![HypnoType::string()], HypnoType::unknown());
        }
        for name in ["FileExists", "IsFile", "IsDirectory"] {
            self.register_builtin(name, vec![HypnoType::string()], HypnoType::boolean());
        }
        self.register_builtin("ListDirectory", vec![HypnoType::string()], string_array());
        self.register_builtin(
            "GetFileSize",
            vec![HypnoType::string()],
            HypnoType::number(),
        );
        self.register_builtin(
            "CopyFile",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::number(),
        );
        self.register_builtin(
            "RenameFile",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::unknown(),
        );
        for name in ["GetFileExtension", "GetFileName", "GetParentDirectory"] {
            self.register_builtin(name, vec![HypnoType::string()], HypnoType::string());
        }

        // Hashing / Utility
        self.register_builtin("HashString", vec![HypnoType::string()], HypnoType::number());
        self.register_builtin("HashNumber", vec![HypnoType::number()], HypnoType::number());
        self.register_builtin(
            "SimpleRandom",
            vec![HypnoType::number()],
            HypnoType::number(),
        );
        self.register_builtin(
            "AreAnagrams",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::boolean(),
        );
        self.register_builtin(
            "IsPalindrome",
            vec![HypnoType::string()],
            HypnoType::boolean(),
        );
        self.register_builtin(
            "CountOccurrences",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::number(),
        );

        // Statistics
        for name in [
            "Mean",
            "Median",
            "Mode",
            "StandardDeviation",
            "Variance",
            "Range",
        ] {
            self.register_builtin(name, vec![number_array()], HypnoType::number());
        }
        self.register_builtin(
            "Percentile",
            vec![number_array(), HypnoType::number()],
            HypnoType::number(),
        );
        self.register_builtin(
            "Correlation",
            vec![number_array(), number_array()],
            HypnoType::number(),
        );
        self.register_builtin(
            "LinearRegression",
            vec![number_array(), number_array()],
            number_array(),
        );

        // System
        self.register_builtin("GetCurrentDirectory", vec![], HypnoType::string());
        self.register_builtin("GetEnv", vec![HypnoType::string()], HypnoType::string());
        self.register_builtin(
            "SetEnv",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::unknown(),
        );
        self.register_builtin("GetOperatingSystem", vec![], HypnoType::string());
        self.register_builtin("GetArchitecture", vec![], HypnoType::string());
        self.register_builtin("GetCpuCount", vec![], HypnoType::number());
        self.register_builtin("GetHostname", vec![], HypnoType::string());
        self.register_builtin("GetUsername", vec![], HypnoType::string());
        self.register_builtin("GetHomeDirectory", vec![], HypnoType::string());
        self.register_builtin("GetTempDirectory", vec![], HypnoType::string());
        self.register_builtin("GetArgs", vec![], string_array());
        self.register_builtin("Exit", vec![HypnoType::number()], HypnoType::unknown());

        // Time / Date
        self.register_builtin("CurrentTimestamp", vec![], HypnoType::number());
        self.register_builtin("CurrentDate", vec![], HypnoType::string());
        self.register_builtin("CurrentTime", vec![], HypnoType::string());
        self.register_builtin("CurrentDateTime", vec![], HypnoType::string());
        self.register_builtin(
            "FormatDateTime",
            vec![HypnoType::string()],
            HypnoType::string(),
        );
        self.register_builtin("DayOfWeek", vec![], HypnoType::number());
        self.register_builtin("DayOfYear", vec![], HypnoType::number());
        self.register_builtin(
            "IsLeapYear",
            vec![HypnoType::number()],
            HypnoType::boolean(),
        );
        self.register_builtin(
            "DaysInMonth",
            vec![HypnoType::number(), HypnoType::number()],
            HypnoType::number(),
        );
        self.register_builtin("CurrentYear", vec![], HypnoType::number());
        self.register_builtin("CurrentMonth", vec![], HypnoType::number());
        self.register_builtin("CurrentDay", vec![], HypnoType::number());
        self.register_builtin("CurrentHour", vec![], HypnoType::number());
        self.register_builtin("CurrentMinute", vec![], HypnoType::number());
        self.register_builtin("CurrentSecond", vec![], HypnoType::number());

        // Validation
        for name in [
            "IsValidEmail",
            "IsValidUrl",
            "IsValidPhoneNumber",
            "IsAlphanumeric",
            "IsAlphabetic",
            "IsNumeric",
            "IsLowercase",
            "IsUppercase",
        ] {
            self.register_builtin(name, vec![HypnoType::string()], HypnoType::boolean());
        }
        self.register_builtin(
            "IsInRange",
            vec![
                HypnoType::number(),
                HypnoType::number(),
                HypnoType::number(),
            ],
            HypnoType::boolean(),
        );
        self.register_builtin(
            "MatchesPattern",
            vec![HypnoType::string(), HypnoType::string()],
            HypnoType::boolean(),
        );
    }

    fn register_builtin(
        &mut self,
        name: &str,
        parameter_types: Vec<HypnoType>,
        return_type: HypnoType,
    ) {
        self.function_types
            .insert(name.to_string(), (parameter_types, return_type));
    }

    /// Parse type annotation string to HypnoType
    fn parse_type_annotation(&self, type_str: Option<&str>) -> HypnoType {
        match type_str {
            Some("number") => HypnoType::number(),
            Some("string") => HypnoType::string(),
            Some("boolean") => HypnoType::boolean(),
            Some("trance") => HypnoType::new(HypnoBaseType::Trance, None),
            _ => HypnoType::unknown(),
        }
    }

    /// Check a program and return errors
    pub fn check_program(&mut self, program: &AstNode) -> Vec<String> {
        self.errors.clear();

        if let AstNode::Program(statements) = program {
            // Collect session metadata before type evaluation
            for stmt in statements {
                self.collect_session_signature(stmt);
            }

            // First pass: collect function declarations
            for stmt in statements {
                self.collect_function_signature(stmt);
            }

            // Second pass: type check all statements
            for stmt in statements {
                self.check_statement(stmt);
            }
        } else {
            self.errors.push("Expected program node".to_string());
        }

        self.errors.clone()
    }

    /// Collect function signatures (including triggers)
    fn collect_function_signature(&mut self, stmt: &AstNode) {
        match stmt {
            AstNode::FunctionDeclaration {
                name,
                parameters,
                return_type,
                ..
            }
            | AstNode::TriggerDeclaration {
                name,
                parameters,
                return_type,
                ..
            } => {
                let param_types: Vec<HypnoType> = parameters
                    .iter()
                    .map(|p| self.parse_type_annotation(p.type_annotation.as_deref()))
                    .collect();

                let ret_type = self.parse_type_annotation(return_type.as_deref());

                self.function_types
                    .insert(name.clone(), (param_types, ret_type));
            }
            _ => {}
        }
    }

    fn collect_session_signature(&mut self, stmt: &AstNode) {
        let AstNode::SessionDeclaration { name, members } = stmt else {
            return;
        };

        if self.sessions.contains_key(name) {
            self.errors
                .push(format!("Duplicate session declaration '{}'", name));
            return;
        }

        let mut info = SessionInfo::new(name.clone());

        for member in members {
            match member {
                SessionMember::Field(field) => {
                    let field_type = self.parse_type_annotation(field.type_annotation.as_deref());
                    let field_info = SessionFieldInfo {
                        ty: field_type,
                        visibility: field.visibility,
                        is_static: field.is_static,
                    };

                    let map = if field.is_static {
                        &mut info.static_fields
                    } else {
                        &mut info.instance_fields
                    };

                    if map.contains_key(&field.name) {
                        self.errors.push(format!(
                            "Duplicate field '{}' in session '{}'",
                            field.name, name
                        ));
                    } else {
                        map.insert(field.name.clone(), field_info);
                    }
                }
                SessionMember::Method(method) => {
                    let method_info = self.build_method_info(name, method);

                    match method_info {
                        Ok(info_item) => {
                            if info_item.is_constructor {
                                if info.constructor.is_some() {
                                    self.errors.push(format!(
                                        "Multiple constructors defined for session '{}'",
                                        name
                                    ));
                                } else {
                                    info.constructor = Some(info_item);
                                }
                                continue;
                            }

                            let target_map = if info_item.is_static {
                                &mut info.static_methods
                            } else {
                                &mut info.instance_methods
                            };

                            if target_map.contains_key(&method.name) {
                                self.errors.push(format!(
                                    "Duplicate method '{}' in session '{}'",
                                    method.name, name
                                ));
                            } else {
                                target_map.insert(method.name.clone(), info_item);
                            }
                        }
                        Err(err) => {
                            self.errors.push(err);
                        }
                    }
                }
            }
        }

        // Ensure constructor signature is registered as callable for session instantiation
        if let Some(constructor) = info.constructor.as_ref() {
            self.function_types.insert(
                name.clone(),
                (
                    constructor.parameter_types.clone(),
                    self.make_session_instance_type(name),
                ),
            );
        } else {
            // Sessions without explicit constructor accept zero arguments
            self.function_types.insert(
                name.clone(),
                (Vec::new(), self.make_session_instance_type(name)),
            );
        }

        self.sessions.insert(name.clone(), info);
        self.type_env
            .insert(name.clone(), self.make_session_type(name));
    }

    fn build_method_info(
        &self,
        session_name: &str,
        method: &SessionMethod,
    ) -> Result<SessionMethodInfo, String> {
        if method.is_constructor && method.is_static {
            return Err(format!(
                "Constructor in session '{}' cannot be static",
                session_name
            ));
        }

        let parameter_types = method
            .parameters
            .iter()
            .map(|param| self.parse_type_annotation(param.type_annotation.as_deref()))
            .collect();

        let return_type = if method.is_constructor {
            self.make_session_instance_type(session_name)
        } else {
            self.parse_type_annotation(method.return_type.as_deref())
        };

        Ok(SessionMethodInfo {
            parameter_types,
            return_type,
            visibility: method.visibility,
            is_static: method.is_static,
            is_constructor: method.is_constructor,
        })
    }

    fn make_session_type(&self, name: &str) -> HypnoType {
        HypnoType::new(HypnoBaseType::Session, Some(format!("{}::type", name)))
    }

    fn make_session_instance_type(&self, name: &str) -> HypnoType {
        HypnoType::new(HypnoBaseType::Session, Some(name.to_string()))
    }

    fn check_session_field(&mut self, session_name: &str, field: &SessionField) {
        let prev_static = self.in_static_context;
        self.in_static_context = field.is_static;

        let expected_type = self.parse_type_annotation(field.type_annotation.as_deref());
        if let Some(initializer) = field.initializer.as_ref() {
            let actual_type = self.infer_type(initializer);
            if !self.types_compatible(&expected_type, &actual_type) {
                self.errors.push(format!(
                    "Field '{}' in session '{}' expects type {}, got {}",
                    field.name, session_name, expected_type, actual_type
                ));
            }
        }

        self.in_static_context = prev_static;
    }

    fn check_session_method(&mut self, session_name: &str, method: &SessionMethod) {
        let saved_env = self.type_env.clone();
        let saved_return = self.current_function_return_type.clone();
        let saved_static = self.in_static_context;

        self.in_static_context = method.is_static;

        let return_type = if method.is_constructor {
            self.make_session_instance_type(session_name)
        } else {
            self.parse_type_annotation(method.return_type.as_deref())
        };
        self.current_function_return_type = Some(return_type);

        if !method.is_static {
            self.type_env.insert(
                "this".to_string(),
                self.make_session_instance_type(session_name),
            );
        }

        for param in &method.parameters {
            let param_type = self.parse_type_annotation(param.type_annotation.as_deref());
            self.type_env.insert(param.name.clone(), param_type);
        }

        for stmt in &method.body {
            self.check_statement(stmt);
        }

        self.type_env = saved_env;
        self.current_function_return_type = saved_return;
        self.in_static_context = saved_static;
    }

    fn session_lookup(&self, ty: &HypnoType) -> Option<(SessionInfo, bool)> {
        if ty.base_type != HypnoBaseType::Session {
            return None;
        }

        let name = ty.name.as_deref()?;
        if let Some(stripped) = name.strip_suffix("::type") {
            self.sessions
                .get(stripped)
                .cloned()
                .map(|info| (info, true))
        } else {
            self.sessions.get(name).cloned().map(|info| (info, false))
        }
    }

    fn visibility_allows(&self, session_name: &str, visibility: SessionVisibility) -> bool {
        visibility == SessionVisibility::Public
            || self
                .current_session
                .as_deref()
                .is_some_and(|current| current == session_name)
    }

    fn method_function_type(&self, method: &SessionMethodInfo) -> HypnoType {
        HypnoType::create_function(method.parameter_types.clone(), method.return_type.clone())
    }

    fn infer_session_member(&mut self, object: &AstNode, property: &str) -> HypnoType {
        let object_type = self.infer_type(object);
        let Some((session_info, is_static_reference)) = self.session_lookup(&object_type) else {
            self.errors.push(format!(
                "Cannot access member '{}' on value of type {}",
                property, object_type
            ));
            return HypnoType::unknown();
        };

        let session_name = session_info.name.clone();

        if is_static_reference {
            if let Some(field) = session_info.static_fields.get(property).cloned() {
                debug_assert!(field.is_static);
                if !self.visibility_allows(&session_name, field.visibility) {
                    self.errors.push(format!(
                        "Static field '{}' of session '{}' is not visible here",
                        property, session_name
                    ));
                    return HypnoType::unknown();
                }
                return field.ty;
            }

            if session_info.instance_fields.contains_key(property) {
                self.errors.push(format!(
                    "Cannot access instance field '{}' on session type '{}'",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            if let Some(method) = session_info.static_methods.get(property).cloned() {
                if !self.visibility_allows(&session_name, method.visibility) {
                    self.errors.push(format!(
                        "Static method '{}' of session '{}' is not visible here",
                        property, session_name
                    ));
                    return HypnoType::unknown();
                }
                return self.method_function_type(&method);
            }

            if session_info.instance_methods.contains_key(property) {
                self.errors.push(format!(
                    "Cannot access instance method '{}' on session type '{}'",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            self.errors.push(format!(
                "Session '{}' has no static member '{}'",
                session_name, property
            ));
            return HypnoType::unknown();
        }

        if let Some(field) = session_info.instance_fields.get(property).cloned() {
            debug_assert!(!field.is_static);
            if !self.visibility_allows(&session_name, field.visibility) {
                self.errors.push(format!(
                    "Field '{}' of session '{}' is not visible here",
                    property, session_name
                ));
                return HypnoType::unknown();
            }
            return field.ty;
        }

        if let Some(field) = session_info.static_fields.get(property).cloned() {
            debug_assert!(field.is_static);
            if !self.visibility_allows(&session_name, field.visibility) {
                self.errors.push(format!(
                    "Static field '{}' of session '{}' is not visible here",
                    property, session_name
                ));
                return HypnoType::unknown();
            }
            return field.ty;
        }

        if let Some(method) = session_info.instance_methods.get(property).cloned() {
            if !self.visibility_allows(&session_name, method.visibility) {
                self.errors.push(format!(
                    "Method '{}' of session '{}' is not visible here",
                    property, session_name
                ));
                return HypnoType::unknown();
            }
            return self.method_function_type(&method);
        }

        if let Some(method) = session_info.static_methods.get(property).cloned() {
            if !self.visibility_allows(&session_name, method.visibility) {
                self.errors.push(format!(
                    "Static method '{}' of session '{}' is not visible here",
                    property, session_name
                ));
                return HypnoType::unknown();
            }
            return self.method_function_type(&method);
        }

        self.errors.push(format!(
            "Session '{}' has no member '{}'",
            session_name, property
        ));
        HypnoType::unknown()
    }

    fn check_session_method_call(
        &mut self,
        object: &AstNode,
        property: &str,
        arguments: &[AstNode],
    ) -> HypnoType {
        let object_type = self.infer_type(object);
        let Some((session_info, is_static_reference)) = self.session_lookup(&object_type) else {
            self.errors.push(format!(
                "Cannot call member '{}' on value of type {}",
                property, object_type
            ));
            return HypnoType::unknown();
        };

        let session_name = session_info.name.clone();
        let instance_method = session_info.instance_methods.get(property).cloned();
        let static_method = session_info.static_methods.get(property).cloned();

        let method = if is_static_reference {
            if instance_method.is_some() {
                self.errors.push(format!(
                    "Cannot call instance method '{}' on session type '{}'",
                    property, session_name
                ));
                return HypnoType::unknown();
            }
            static_method
        } else {
            instance_method.or(static_method)
        };

        if let Some(method_info) = method {
            if !self.visibility_allows(&session_name, method_info.visibility) {
                self.errors.push(format!(
                    "Method '{}' of session '{}' is not visible here",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            if method_info.is_constructor {
                self.errors.push(format!(
                    "Constructor '{}' of session '{}' cannot be invoked as a member",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            if is_static_reference && !method_info.is_static {
                self.errors.push(format!(
                    "Cannot call instance method '{}' on session type '{}'",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            if arguments.len() != method_info.parameter_types.len() {
                self.errors.push(format!(
                    "Method '{}' of session '{}' expects {} arguments, got {}",
                    property,
                    session_name,
                    method_info.parameter_types.len(),
                    arguments.len()
                ));
            } else {
                for (idx, (arg, expected)) in arguments
                    .iter()
                    .zip(method_info.parameter_types.iter())
                    .enumerate()
                {
                    let actual = self.infer_type(arg);
                    if !self.types_compatible(expected, &actual) {
                        self.errors.push(format!(
                            "Method '{}' argument {} type mismatch: expected {}, got {}",
                            property,
                            idx + 1,
                            expected,
                            actual
                        ));
                    }
                }
            }

            return method_info.return_type.clone();
        }

        if session_info.instance_fields.contains_key(property) {
            self.errors.push(format!(
                "Member '{}' of session '{}' is a field and cannot be called",
                property, session_name
            ));
        } else if session_info.static_fields.contains_key(property) {
            if is_static_reference {
                self.errors.push(format!(
                    "Static field '{}' of session '{}' cannot be called",
                    property, session_name
                ));
            } else {
                self.errors.push(format!(
                    "Field '{}' of session '{}' cannot be called",
                    property, session_name
                ));
            }
        } else if is_static_reference {
            self.errors.push(format!(
                "Session '{}' has no static method '{}'",
                session_name, property
            ));
        } else {
            self.errors.push(format!(
                "Session '{}' has no method '{}'",
                session_name, property
            ));
        }

        HypnoType::unknown()
    }

    fn check_member_assignment(
        &mut self,
        object: &AstNode,
        property: &str,
        value: &AstNode,
    ) -> HypnoType {
        let object_type = self.infer_type(object);
        let Some((session_info, is_static_reference)) = self.session_lookup(&object_type) else {
            self.errors.push(format!(
                "Assignment target '{}' is not a session member (type: {})",
                property, object_type
            ));
            return HypnoType::unknown();
        };

        let session_name = session_info.name.clone();
        let instance_field = session_info.instance_fields.get(property).cloned();
        let static_field = session_info.static_fields.get(property).cloned();

        if is_static_reference {
            if let Some(field) = static_field {
                debug_assert!(field.is_static);
                if !self.visibility_allows(&session_name, field.visibility) {
                    self.errors.push(format!(
                        "Static field '{}' of session '{}' is not visible here",
                        property, session_name
                    ));
                    return HypnoType::unknown();
                }

                let value_type = self.infer_type(value);
                if !self.types_compatible(&field.ty, &value_type) {
                    self.errors.push(format!(
                        "Cannot assign value of type {} to static field '{}' of session '{}' (expected {})",
                        value_type, property, session_name, field.ty
                    ));
                }
                return field.ty;
            }

            if instance_field.is_some() {
                self.errors.push(format!(
                    "Cannot assign to instance field '{}' on session type '{}'",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            if session_info.static_methods.contains_key(property)
                || session_info.instance_methods.contains_key(property)
            {
                self.errors.push(format!(
                    "Cannot assign to method '{}' of session '{}'",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            self.errors.push(format!(
                "Session '{}' has no static field '{}'",
                session_name, property
            ));
            return HypnoType::unknown();
        }

        if let Some(field) = instance_field {
            debug_assert!(!field.is_static);
            if !self.visibility_allows(&session_name, field.visibility) {
                self.errors.push(format!(
                    "Field '{}' of session '{}' is not visible here",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            let value_type = self.infer_type(value);
            if !self.types_compatible(&field.ty, &value_type) {
                self.errors.push(format!(
                    "Cannot assign value of type {} to field '{}' of session '{}' (expected {})",
                    value_type, property, session_name, field.ty
                ));
            }

            return field.ty;
        }

        if let Some(field) = static_field {
            debug_assert!(field.is_static);
            if !self.visibility_allows(&session_name, field.visibility) {
                self.errors.push(format!(
                    "Static field '{}' of session '{}' is not visible here",
                    property, session_name
                ));
                return HypnoType::unknown();
            }

            self.errors.push(format!(
                "Assign static field '{}' through session '{}', not an instance",
                property, session_name
            ));
            return HypnoType::unknown();
        }

        if session_info.instance_methods.contains_key(property)
            || session_info.static_methods.contains_key(property)
        {
            self.errors.push(format!(
                "Cannot assign to method '{}' of session '{}'",
                property, session_name
            ));
            return HypnoType::unknown();
        }

        self.errors.push(format!(
            "Session '{}' has no field '{}'",
            session_name, property
        ));
        HypnoType::unknown()
    }

    /// Check a statement
    fn check_statement(&mut self, stmt: &AstNode) {
        match stmt {
            AstNode::VariableDeclaration {
                name,
                type_annotation,
                initializer,
                is_constant,
            } => {
                let expected_type = self.parse_type_annotation(type_annotation.as_deref());

                if let Some(init) = initializer {
                    let actual_type = self.infer_type(init);

                    if !self.types_compatible(&expected_type, &actual_type) {
                        self.errors.push(format!(
                            "Type mismatch for variable '{}': expected {}, got {}",
                            name, expected_type, actual_type
                        ));
                    }
                } else if *is_constant {
                    self.errors
                        .push(format!("Constant variable '{}' must be initialized", name));
                }

                self.type_env.insert(name.clone(), expected_type);
            }

            AstNode::AnchorDeclaration { name, source } => {
                let source_type = self.infer_type(source);
                self.type_env.insert(name.clone(), source_type);
            }

            AstNode::FunctionDeclaration {
                parameters,
                return_type,
                body,
                ..
            } => {
                let old_env = self.type_env.clone();
                let ret_type = self.parse_type_annotation(return_type.as_deref());
                self.current_function_return_type = Some(ret_type);

                for param in parameters {
                    let param_type = self.parse_type_annotation(param.type_annotation.as_deref());
                    self.type_env.insert(param.name.clone(), param_type);
                }

                for stmt in body {
                    self.check_statement(stmt);
                }

                self.type_env = old_env;
                self.current_function_return_type = None;
            }

            AstNode::TriggerDeclaration {
                parameters,
                return_type,
                body,
                ..
            } => {
                // Triggers are handled like functions
                let old_env = self.type_env.clone();
                let ret_type = self.parse_type_annotation(return_type.as_deref());
                self.current_function_return_type = Some(ret_type);

                for param in parameters {
                    let param_type = self.parse_type_annotation(param.type_annotation.as_deref());
                    self.type_env.insert(param.name.clone(), param_type);
                }

                for stmt in body {
                    self.check_statement(stmt);
                }

                self.type_env = old_env;
                self.current_function_return_type = None;
            }

            AstNode::EntranceBlock(statements) | AstNode::FinaleBlock(statements) => {
                for stmt in statements {
                    self.check_statement(stmt);
                }
            }

            AstNode::IfStatement {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_type = self.infer_type(condition);
                if cond_type.base_type != HypnoBaseType::Boolean {
                    self.errors
                        .push(format!("If condition must be boolean, got {}", cond_type));
                }

                for stmt in then_branch {
                    self.check_statement(stmt);
                }

                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.check_statement(stmt);
                    }
                }
            }

            AstNode::DeepFocusStatement { condition, body } => {
                let cond_type = self.infer_type(condition);
                if cond_type.base_type != HypnoBaseType::Boolean {
                    self.errors.push(format!(
                        "DeepFocus condition must be boolean, got {}",
                        cond_type
                    ));
                }

                for stmt in body {
                    self.check_statement(stmt);
                }
            }

            AstNode::WhileStatement { condition, body } => {
                let cond_type = self.infer_type(condition);
                if cond_type.base_type != HypnoBaseType::Boolean {
                    self.errors.push(format!(
                        "While condition must be boolean, got {}",
                        cond_type
                    ));
                }

                for stmt in body {
                    self.check_statement(stmt);
                }
            }

            AstNode::LoopStatement { body } => {
                for stmt in body {
                    self.check_statement(stmt);
                }
            }

            AstNode::OscillateStatement { target } => {
                let target_type = self.infer_type(target);
                if target_type.base_type != HypnoBaseType::Boolean {
                    self.errors.push(format!(
                        "Oscillate target must be boolean, got {}",
                        target_type
                    ));
                }
            }

            AstNode::SessionDeclaration { name, members } => {
                let prev_session = self.current_session.clone();
                let prev_static = self.in_static_context;

                self.current_session = Some(name.clone());
                self.in_static_context = false;

                for member in members {
                    match member {
                        SessionMember::Field(field) => self.check_session_field(name, field),
                        SessionMember::Method(method) => self.check_session_method(name, method),
                    }
                }

                self.current_session = prev_session;
                self.in_static_context = prev_static;
            }

            #[allow(clippy::collapsible_match)]
            AstNode::ReturnStatement(value) => {
                if let Some(val) = value {
                    let actual_type = self.infer_type(val);
                    if let Some(ret_type) = &self.current_function_return_type.clone() {
                        if !self.types_compatible(ret_type, &actual_type) {
                            self.errors.push(format!(
                                "Return type mismatch: expected {}, got {}",
                                ret_type, actual_type
                            ));
                        }
                    }
                }
            }

            AstNode::ExpressionStatement(expr)
            | AstNode::ObserveStatement(expr)
            | AstNode::WhisperStatement(expr)
            | AstNode::CommandStatement(expr) => {
                self.infer_type(expr);
            }

            _ => {}
        }
    }

    /// Infer the type of an expression
    fn infer_type(&mut self, expr: &AstNode) -> HypnoType {
        match expr {
            AstNode::NumberLiteral(_) => HypnoType::number(),
            AstNode::StringLiteral(_) => HypnoType::string(),
            AstNode::BooleanLiteral(_) => HypnoType::boolean(),

            AstNode::Identifier(name) => {
                if name == "this" && self.in_static_context {
                    self.errors
                        .push("Cannot use 'this' in a static context".to_string());
                    return HypnoType::unknown();
                }

                self.type_env.get(name).cloned().unwrap_or_else(|| {
                    self.errors.push(format!("Undefined variable '{}'", name));
                    HypnoType::unknown()
                })
            }

            AstNode::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left_type = self.infer_type(left);
                let right_type = self.infer_type(right);
                let normalized_op = operator.to_ascii_lowercase();

                match normalized_op.as_str() {
                    "+" | "-" | "*" | "/" | "%" => {
                        if left_type.base_type != HypnoBaseType::Number
                            || right_type.base_type != HypnoBaseType::Number
                        {
                            self.errors.push(format!(
                                "Arithmetic operator '{}' requires numeric operands, got {} and {}",
                                operator, left_type, right_type
                            ));
                        }
                        HypnoType::number()
                    }
                    "==" | "!=" | "youarefeelingverysleepy" | "youcannotresist" | "notsodeep" => {
                        HypnoType::boolean()
                    }
                    ">"
                    | "<"
                    | ">="
                    | "<="
                    | "lookatthewatch"
                    | "fallundermyspell"
                    | "youreyesaregettingheavy"
                    | "goingdeeper"
                    | "deeplygreater"
                    | "deeplyless" => {
                        if left_type.base_type != HypnoBaseType::Number
                            || right_type.base_type != HypnoBaseType::Number
                        {
                            self.errors.push(format!(
                                "Comparison operator '{}' requires numeric operands, got {} and {}",
                                operator, left_type, right_type
                            ));
                        }
                        HypnoType::boolean()
                    }
                    "&&" | "undermycontrol" | "||" | "resistanceisfutile" => {
                        if left_type.base_type != HypnoBaseType::Boolean
                            || right_type.base_type != HypnoBaseType::Boolean
                        {
                            self.errors.push(format!(
                                "Logical operator '{}' requires boolean operands, got {} and {}",
                                operator, left_type, right_type
                            ));
                        }
                        HypnoType::boolean()
                    }
                    _ => HypnoType::unknown(),
                }
            }

            AstNode::UnaryExpression { operator, operand } => {
                let operand_type = self.infer_type(operand);

                match operator.as_str() {
                    "-" => {
                        if operand_type.base_type != HypnoBaseType::Number {
                            self.errors.push(format!(
                                "Unary minus requires numeric operand, got {}",
                                operand_type
                            ));
                        }
                        HypnoType::number()
                    }
                    "!" => {
                        if operand_type.base_type != HypnoBaseType::Boolean {
                            self.errors.push(format!(
                                "Logical not requires boolean operand, got {}",
                                operand_type
                            ));
                        }
                        HypnoType::boolean()
                    }
                    _ => HypnoType::unknown(),
                }
            }

            AstNode::CallExpression { callee, arguments } => match callee.as_ref() {
                AstNode::Identifier(func_name) => {
                    let func_sig = self.function_types.get(func_name).cloned();

                    if let Some((param_types, return_type)) = func_sig {
                        if arguments.len() != param_types.len() {
                            self.errors.push(format!(
                                "Function '{}' expects {} arguments, got {}",
                                func_name,
                                param_types.len(),
                                arguments.len()
                            ));
                        } else {
                            for (i, (arg, expected_type)) in
                                arguments.iter().zip(param_types.iter()).enumerate()
                            {
                                let actual_type = self.infer_type(arg);
                                if !self.types_compatible(expected_type, &actual_type) {
                                    self.errors.push(format!(
                                        "Function '{}' argument {} type mismatch: expected {}, got {}",
                                        func_name,
                                        i + 1,
                                        expected_type,
                                        actual_type
                                    ));
                                }
                            }
                        }

                        return return_type;
                    } else {
                        self.errors
                            .push(format!("Undefined function '{}'", func_name));
                        HypnoType::unknown()
                    }
                }
                AstNode::MemberExpression { object, property } => {
                    self.check_session_method_call(object, property, arguments)
                }
                _ => {
                    let callee_type = self.infer_type(callee);
                    if callee_type.base_type != HypnoBaseType::Function {
                        self.errors
                            .push(format!("Value of type {} is not callable", callee_type));
                        return HypnoType::unknown();
                    }

                    let param_types = callee_type.parameter_types.clone().unwrap_or_default();
                    let return_type = callee_type
                        .return_type
                        .clone()
                        .map(|boxed| (*boxed).clone())
                        .unwrap_or_else(HypnoType::unknown);

                    if arguments.len() != param_types.len() {
                        self.errors.push(format!(
                            "Callable expects {} arguments, got {}",
                            param_types.len(),
                            arguments.len()
                        ));
                    } else {
                        for (i, (arg, expected_type)) in
                            arguments.iter().zip(param_types.iter()).enumerate()
                        {
                            let actual_type = self.infer_type(arg);
                            if !self.types_compatible(expected_type, &actual_type) {
                                self.errors.push(format!(
                                    "Callable argument {} type mismatch: expected {}, got {}",
                                    i + 1,
                                    expected_type,
                                    actual_type
                                ));
                            }
                        }
                    }

                    return_type
                }
            },

            AstNode::MemberExpression { object, property } => {
                self.infer_session_member(object, property)
            }

            AstNode::AssignmentExpression { target, value } => match target.as_ref() {
                AstNode::Identifier(name) => {
                    let value_type = self.infer_type(value);
                    if let Some(expected_type) = self.type_env.get(name).cloned() {
                        if !self.types_compatible(&expected_type, &value_type) {
                            self.errors.push(format!(
                                "Cannot assign value of type {} to variable '{}' of type {}",
                                value_type, name, expected_type
                            ));
                        }
                        expected_type
                    } else {
                        self.errors
                            .push(format!("Cannot assign to undefined variable '{}'", name));
                        HypnoType::unknown()
                    }
                }
                AstNode::MemberExpression { object, property } => {
                    self.check_member_assignment(object, property, value)
                }
                _ => {
                    self.errors.push("Invalid assignment target".to_string());
                    HypnoType::unknown()
                }
            },

            AstNode::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    HypnoType::create_array(HypnoType::unknown())
                } else {
                    let first_type = self.infer_type(&elements[0]);
                    for elem in &elements[1..] {
                        let elem_type = self.infer_type(elem);
                        if !self.types_compatible(&first_type, &elem_type) {
                            self.errors.push(format!(
                                "Array elements must have same type, got {} and {}",
                                first_type, elem_type
                            ));
                        }
                    }
                    HypnoType::create_array(first_type)
                }
            }

            _ => HypnoType::unknown(),
        }
    }

    /// Check if two types are compatible
    fn types_compatible(&self, expected: &HypnoType, actual: &HypnoType) -> bool {
        if expected.base_type == HypnoBaseType::Unknown
            || actual.base_type == HypnoBaseType::Unknown
        {
            return true;
        }
        expected.is_compatible_with(actual)
    }

    /// Get all errors
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hypnoscript_lexer_parser::{Lexer, Parser};

    #[test]
    fn test_type_check_simple() {
        let source = r#"
Focus {
    induce x: number = 42;
    induce y: number = 10;
    induce sum: number = x + y;
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut checker = TypeChecker::new();
        let errors = checker.check_program(&ast);
        assert!(errors.is_empty(), "Errors: {:?}", errors);
    }

    #[test]
    fn test_operator_synonym_diagnostics() {
        let source = r#"
Focus {
    induce left: string = "hello";
    induce right: string = "world";
    if (left lookAtTheWatch right) {
        observe "won't happen";
    }
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut checker = TypeChecker::new();
        let errors = checker.check_program(&ast);
        assert!(
            errors.iter().any(|msg| msg.contains("lookAtTheWatch")),
            "Expected comparison diagnostic mentioning operator, got {:?}",
            errors
        );
    }

    #[test]
    fn test_type_check_private_session_member_access() {
        let source = r#"
Focus {
    session Account {
        conceal balance: number = 0;

        expose suggestion constructor(initialBalance: number) {
            this.balance = initialBalance;
        }

        expose suggestion read(): number {
            awaken this.balance;
        }
    }

    induce leaked = Account(100).balance;
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut checker = TypeChecker::new();
        let errors = checker.check_program(&ast);
        assert!(!errors.is_empty());
        assert!(
            errors.iter().any(|msg| {
                msg.contains("Field 'balance' of session 'Account' is not visible here")
            }),
            "Expected private member visibility error, got {:?}",
            errors
        );
    }

    #[test]
    fn test_type_check_static_misuse_errors() {
        let source = r#"
Focus {
    session Config {
        expose secret: number = 42;
    }

    session Env {
        dominant expose name: string = "default";
    }

    induce secretValue = Config.secret;
    Env().name = "prod";
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut checker = TypeChecker::new();
        let errors = checker.check_program(&ast);
        assert!(
            errors.len() >= 2,
            "Expected at least two errors, got {:?}",
            errors
        );
        assert!(
            errors.iter().any(|msg| {
                msg.contains("Cannot access instance field 'secret' on session type 'Config'")
            }),
            "Expected instance vs static access error, got {:?}",
            errors
        );
        assert!(
            errors.iter().any(|msg| {
                msg.contains("Assign static field 'name' through session 'Env', not an instance")
            }),
            "Expected static assignment error, got {:?}",
            errors
        );
    }

    #[test]
    fn test_type_check_mismatch() {
        let source = r#"
Focus {
    induce x: number = "hello";
} Relax
"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut checker = TypeChecker::new();
        let errors = checker.check_program(&ast);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("Type mismatch"));
    }
}
