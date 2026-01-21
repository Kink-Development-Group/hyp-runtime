//! Native code generator for HypnoScript
//!
//! This module generates platform-specific native code for:
//! - Windows (x86_64, ARM64)
//! - macOS (x86_64, ARM64 / Apple Silicon)
//! - Linux (x86_64, ARM64, RISC-V)
//!
//! ## Architecture
//!
//! The native code generator uses Cranelift as the backend for compilation.
//! Cranelift is a fast, safe code generator that produces optimized,
//! platform-specific code with minimal runtime dependencies.
//!
//! ## Advantages of Cranelift over LLVM
//!
//! - **Faster compilation**: Cranelift is significantly faster than LLVM
//! - **Simpler integration**: Pure Rust implementation, no C++ dependencies
//! - **Smaller binary size**: Lower overhead
//! - **Security**: Memory-safe via Rust
//!
//! ## Usage
//!
//! ```rust
//! use hypnoscript_compiler::{NativeCodeGenerator, TargetPlatform, OptimizationLevel};
//! use hypnoscript_lexer_parser::ast::AstNode;
//!
//! let mut generator = NativeCodeGenerator::new();
//! generator.set_target_platform(TargetPlatform::LinuxX64);
//! generator.set_optimization_level(OptimizationLevel::Release);
//!
//! // let native_binary = generator.generate(&ast)?;
//! ```

use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use hypnoscript_lexer_parser::ast::AstNode;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use target_lexicon::Triple;
use thiserror::Error;

/// Error types for native code generation
#[derive(Error, Debug)]
pub enum NativeCodegenError {
    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),

    #[error("Cranelift initialization failed: {0}")]
    LlvmInitializationError(String),

    #[error("Code generation failed: {0}")]
    CodeGenerationError(String),

    #[error("Optimization failed: {0}")]
    OptimizationError(String),

    #[error("Linking failed: {0}")]
    LinkingError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Target platforms for native compilation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPlatform {
    /// Windows x86_64
    WindowsX64,
    /// Windows ARM64
    WindowsArm64,
    /// macOS x86_64 (Intel)
    MacOsX64,
    /// macOS ARM64 (Apple Silicon)
    MacOsArm64,
    /// Linux x86_64
    LinuxX64,
    /// Linux ARM64
    LinuxArm64,
    /// Linux RISC-V
    LinuxRiscV,
}

impl TargetPlatform {
    /// Returns the LLVM target triple
    pub fn llvm_triple(&self) -> &'static str {
        match self {
            Self::WindowsX64 => "x86_64-pc-windows-msvc",
            Self::WindowsArm64 => "aarch64-pc-windows-msvc",
            Self::MacOsX64 => "x86_64-apple-darwin",
            Self::MacOsArm64 => "aarch64-apple-darwin",
            Self::LinuxX64 => "x86_64-unknown-linux-gnu",
            Self::LinuxArm64 => "aarch64-unknown-linux-gnu",
            Self::LinuxRiscV => "riscv64gc-unknown-linux-gnu",
        }
    }

    /// Detects the current platform at build time
    pub fn current() -> Self {
        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        return Self::WindowsX64;

        #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
        return Self::WindowsArm64;

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        return Self::MacOsX64;

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        return Self::MacOsArm64;

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        return Self::LinuxX64;

        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        return Self::LinuxArm64;

        #[cfg(all(target_os = "linux", target_arch = "riscv64"))]
        return Self::LinuxRiscV;
    }
}

/// Optimization levels for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// No optimization (debug build)
    None,
    /// Moderate optimization (fast compilation)
    Less,
    /// Default optimization (balanced)
    Default,
    /// Aggressive optimization (slower compilation, faster code)
    Aggressive,
    /// Maximum optimization for releases
    Release,
}

impl OptimizationLevel {
    /// Converts to Cranelift optimization level
    pub fn to_cranelift_level(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Less => "speed",
            Self::Default => "speed",
            Self::Aggressive => "speed_and_size",
            Self::Release => "speed_and_size",
        }
    }

    /// Converts to LLVM optimization level (0-3)
    pub fn to_llvm_level(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Less => 1,
            Self::Default => 2,
            Self::Aggressive | Self::Release => 3,
        }
    }
}

/// Native code generator
///
/// Generates platform-specific native machine code from the HypnoScript AST.
/// Uses Cranelift as the backend for optimized binaries.
pub struct NativeCodeGenerator {
    /// Target platform
    target_platform: TargetPlatform,
    /// Optimization level
    optimization_level: OptimizationLevel,
    /// Output path for the binary
    output_path: Option<PathBuf>,
    /// Variable mapping (name -> Cranelift variable)
    variable_map: HashMap<String, Variable>,
    /// Function mapping
    function_map: HashMap<String, usize>,
    /// Generate debug information
    debug_info: bool,
    /// Next variable ID
    next_var_id: usize,
}

impl Default for NativeCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeCodeGenerator {
    /// Creates a new native code generator
    ///
    /// # Examples
    ///
    /// ```
    /// use hypnoscript_compiler::NativeCodeGenerator;
    ///
    /// let generator = NativeCodeGenerator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            target_platform: TargetPlatform::current(),
            optimization_level: OptimizationLevel::Default,
            output_path: None,
            variable_map: HashMap::new(),
            function_map: HashMap::new(),
            debug_info: false,
            next_var_id: 0,
        }
    }

    /// Sets the target platform
    ///
    /// # Arguments
    ///
    /// * `platform` - The desired target platform
    pub fn set_target_platform(&mut self, platform: TargetPlatform) {
        self.target_platform = platform;
    }

    /// Sets the optimization level
    ///
    /// # Arguments
    ///
    /// * `level` - The desired optimization level
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }

    /// Sets the output path
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the generated binary
    pub fn set_output_path(&mut self, path: PathBuf) {
        self.output_path = Some(path);
    }

    /// Enables/disables debug information
    ///
    /// # Arguments
    ///
    /// * `enabled` - true for debug info, false otherwise
    pub fn set_debug_info(&mut self, enabled: bool) {
        self.debug_info = enabled;
    }

    /// Generates native code from the AST
    ///
    /// # Arguments
    ///
    /// * `program` - The HypnoScript AST
    ///
    /// # Returns
    ///
    /// Path to the generated binary
    ///
    /// # Errors
    ///
    /// Returns a `NativeCodegenError` when code generation fails
    pub fn generate(&mut self, program: &AstNode) -> Result<PathBuf, NativeCodegenError> {
        self.variable_map.clear();
        self.function_map.clear();
        self.next_var_id = 0;

        // Determine the target triple (will be used in the future)
        let _triple = self.get_target_triple();

        // Create ObjectModule for object file generation
        let mut flag_builder = settings::builder();
        flag_builder
            .set("opt_level", self.optimization_level.to_cranelift_level())
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        let isa_builder = cranelift_native::builder()
            .map_err(|e| NativeCodegenError::LlvmInitializationError(e.to_string()))?;
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        let obj_builder = ObjectBuilder::new(
            isa,
            "hypnoscript_program",
            cranelift_module::default_libcall_names(),
        )
        .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        let mut module = ObjectModule::new(obj_builder);

        // Create the main function
        self.generate_main_function(&mut module, program)?;

        // Finalize and write object file
        let object_product = module.finish();
        let object_bytes = object_product
            .emit()
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        // Determine output path for object file
        let obj_extension = if cfg!(target_os = "windows") {
            "obj"
        } else {
            "o"
        };
        let obj_path = PathBuf::from(format!("hypnoscript_program.{}", obj_extension));

        // Write object file
        std::fs::write(&obj_path, object_bytes)?;

        // Determine final output path for executable
        let exe_path = self.output_path.clone().unwrap_or_else(|| {
            let extension = if cfg!(target_os = "windows") {
                "exe"
            } else {
                ""
            };
            if extension.is_empty() {
                PathBuf::from("hypnoscript_output")
            } else {
                PathBuf::from(format!("hypnoscript_output.{}", extension))
            }
        });

        // Link the object file to an executable
        self.link_object_file(&obj_path, &exe_path)?;

        // Cleanup: remove object file
        let _ = std::fs::remove_file(&obj_path);

        Ok(exe_path)
    }

    /// Links an object file to an executable
    fn link_object_file(&self, obj_path: &Path, exe_path: &Path) -> Result<(), NativeCodegenError> {
        #[cfg(target_os = "windows")]
        {
            // Try different Windows linkers
            let linkers = vec![
                (
                    "link.exe",
                    vec![
                        "/OUT:".to_string() + &exe_path.to_string_lossy(),
                        "/ENTRY:main".to_string(),
                        "/SUBSYSTEM:CONSOLE".to_string(),
                        obj_path.to_string_lossy().to_string(),
                        "kernel32.lib".to_string(),
                        "msvcrt.lib".to_string(),
                    ],
                ),
                (
                    "lld-link",
                    vec![
                        "/OUT:".to_string() + &exe_path.to_string_lossy(),
                        "/ENTRY:main".to_string(),
                        "/SUBSYSTEM:CONSOLE".to_string(),
                        obj_path.to_string_lossy().to_string(),
                    ],
                ),
                (
                    "gcc",
                    vec![
                        "-o".to_string(),
                        exe_path.to_string_lossy().to_string(),
                        obj_path.to_string_lossy().to_string(),
                    ],
                ),
                (
                    "clang",
                    vec![
                        "-o".to_string(),
                        exe_path.to_string_lossy().to_string(),
                        obj_path.to_string_lossy().to_string(),
                    ],
                ),
            ];

            for (linker, args) in linkers {
                if let Ok(output) = std::process::Command::new(linker).args(&args).output()
                    && output.status.success()
                {
                    return Ok(());
                }
            }

            Err(NativeCodegenError::LinkingError(
                "No suitable linker found. Please install:\n\
                 - Visual Studio Build Tools (for link.exe)\n\
                 - GCC/MinGW (for gcc)\n\
                 - LLVM (for lld-link/clang)"
                    .to_string(),
            ))
        }

        #[cfg(not(target_os = "windows"))]
        {
            // Unix-based systems (Linux, macOS)
            let exe_path_string = exe_path.to_string_lossy().into_owned();
            let obj_path_string = obj_path.to_string_lossy().into_owned();

            let exe_arg = exe_path_string.as_str();
            let obj_arg = obj_path_string.as_str();

            let linkers = vec![
                ("cc", vec!["-o", exe_arg, obj_arg]),
                ("gcc", vec!["-o", exe_arg, obj_arg]),
                ("clang", vec!["-o", exe_arg, obj_arg]),
            ];

            for (linker, args) in linkers {
                if let Ok(output) = std::process::Command::new(linker).args(&args).output()
                    && output.status.success()
                {
                    // Make the file executable on Unix
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mut perms = std::fs::metadata(exe_path)?.permissions();
                        perms.set_mode(0o755);
                        std::fs::set_permissions(exe_path, perms)?;
                    }
                    return Ok(());
                }
            }

            Err(NativeCodegenError::LinkingError(
                "No suitable linker found. Please install gcc or clang.".to_string(),
            ))
        }
    }

    /// Converts Cranelift triple from TargetPlatform
    fn get_target_triple(&self) -> Triple {
        self.target_platform
            .llvm_triple()
            .parse()
            .unwrap_or_else(|_| Triple::host())
    }

    /// Generates the main function
    fn generate_main_function(
        &mut self,
        module: &mut ObjectModule,
        program: &AstNode,
    ) -> Result<(), NativeCodegenError> {
        // Create function signature: main() -> i32
        let mut sig = module.make_signature();
        sig.returns.push(AbiParam::new(types::I32));

        let func_id = module
            .declare_function("main", Linkage::Export, &sig)
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        let mut ctx = module.make_context();
        ctx.func.signature = sig;

        // Create function builder
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        // Create entry block
        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Generate code for the program
        if let AstNode::Program(statements) = program {
            for stmt in statements {
                self.generate_statement(&mut builder, stmt)?;
            }
        }

        // Return 0
        let zero = builder.ins().iconst(types::I32, 0);
        builder.ins().return_(&[zero]);

        // Finalize function
        builder.finalize();

        // Define function in module
        module
            .define_function(func_id, &mut ctx)
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        module.clear_context(&mut ctx);

        Ok(())
    }

    /// Generates code for a statement
    fn generate_statement(
        &mut self,
        builder: &mut FunctionBuilder,
        stmt: &AstNode,
    ) -> Result<(), NativeCodegenError> {
        match stmt {
            AstNode::VariableDeclaration {
                name, initializer, ..
            } => {
                // Create variable
                let var = Variable::new(self.next_var_id);
                self.next_var_id += 1;

                builder.declare_var(var, types::F64);
                self.variable_map.insert(name.clone(), var);

                // Initialize variable
                if let Some(init) = initializer {
                    let value = self.generate_expression(builder, init)?;
                    builder.def_var(var, value);
                } else {
                    let zero = builder.ins().f64const(0.0);
                    builder.def_var(var, zero);
                }
            }

            AstNode::AssignmentExpression { target, value } => {
                if let AstNode::Identifier(name) = target.as_ref()
                    && let Some(&var) = self.variable_map.get(name)
                {
                    let val = self.generate_expression(builder, value)?;
                    builder.def_var(var, val);
                }
            }

            AstNode::ExpressionStatement(expr) => {
                // Evaluate expression (result is discarded)
                let _value = self.generate_expression(builder, expr)?;
            }

            AstNode::FocusBlock(statements)
            | AstNode::EntranceBlock(statements)
            | AstNode::FinaleBlock(statements) => {
                for stmt in statements {
                    self.generate_statement(builder, stmt)?;
                }
            }

            _ => {
                // Ignore unsupported statements
                // TODO: Extend for full language support
            }
        }

        Ok(())
    }

    /// Generates code for an expression
    fn generate_expression(
        &mut self,
        builder: &mut FunctionBuilder,
        expr: &AstNode,
    ) -> Result<Value, NativeCodegenError> {
        match expr {
            AstNode::NumberLiteral(n) => Ok(builder.ins().f64const(*n)),

            AstNode::BooleanLiteral(b) => {
                let val = if *b { 1 } else { 0 };
                Ok(builder.ins().iconst(types::I32, val))
            }

            AstNode::Identifier(name) => {
                if let Some(&var) = self.variable_map.get(name) {
                    Ok(builder.use_var(var))
                } else {
                    Ok(builder.ins().f64const(0.0))
                }
            }

            AstNode::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let lhs = self.generate_expression(builder, left)?;
                let rhs = self.generate_expression(builder, right)?;

                let result = match operator.as_str() {
                    "+" => builder.ins().fadd(lhs, rhs),
                    "-" => builder.ins().fsub(lhs, rhs),
                    "*" => builder.ins().fmul(lhs, rhs),
                    "/" => builder.ins().fdiv(lhs, rhs),
                    "%" => {
                        // Modulo for floats: a - floor(a/b) * b
                        let div = builder.ins().fdiv(lhs, rhs);
                        let floor = builder.ins().floor(div);
                        let mul = builder.ins().fmul(floor, rhs);
                        builder.ins().fsub(lhs, mul)
                    }
                    _ => {
                        // Unknown operator -> return 0
                        builder.ins().f64const(0.0)
                    }
                };

                Ok(result)
            }

            AstNode::UnaryExpression { operator, operand } => {
                let val = self.generate_expression(builder, operand)?;

                let result = match operator.as_str() {
                    "-" => builder.ins().fneg(val),
                    "!" => {
                        // Logical negation (for integers)
                        builder.ins().bxor_imm(val, 1)
                    }
                    _ => val,
                };

                Ok(result)
            }

            _ => {
                // Unsupported expressions -> return 0
                Ok(builder.ins().f64const(0.0))
            }
        }
    }

    /// Returns information about the target platform
    pub fn target_info(&self) -> String {
        format!(
            "Target platform: {}\nLLVM triple: {}\nOptimization: {:?}",
            match self.target_platform {
                TargetPlatform::WindowsX64 => "Windows x86_64",
                TargetPlatform::WindowsArm64 => "Windows ARM64",
                TargetPlatform::MacOsX64 => "macOS x86_64 (Intel)",
                TargetPlatform::MacOsArm64 => "macOS ARM64 (Apple Silicon)",
                TargetPlatform::LinuxX64 => "Linux x86_64",
                TargetPlatform::LinuxArm64 => "Linux ARM64",
                TargetPlatform::LinuxRiscV => "Linux RISC-V",
            },
            self.target_platform.llvm_triple(),
            self.optimization_level
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hypnoscript_lexer_parser::{Lexer, Parser};

    #[test]
    fn test_target_platform_current() {
        let platform = TargetPlatform::current();

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        assert_eq!(platform, TargetPlatform::WindowsX64);

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        assert_eq!(platform, TargetPlatform::LinuxX64);

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        assert_eq!(platform, TargetPlatform::MacOsArm64);
    }

    #[test]
    fn test_llvm_triple() {
        assert_eq!(
            TargetPlatform::WindowsX64.llvm_triple(),
            "x86_64-pc-windows-msvc"
        );
        assert_eq!(
            TargetPlatform::LinuxX64.llvm_triple(),
            "x86_64-unknown-linux-gnu"
        );
        assert_eq!(
            TargetPlatform::MacOsArm64.llvm_triple(),
            "aarch64-apple-darwin"
        );
    }

    #[test]
    fn test_optimization_levels() {
        assert_eq!(OptimizationLevel::None.to_llvm_level(), 0);
        assert_eq!(OptimizationLevel::Less.to_llvm_level(), 1);
        assert_eq!(OptimizationLevel::Default.to_llvm_level(), 2);
        assert_eq!(OptimizationLevel::Aggressive.to_llvm_level(), 3);
        assert_eq!(OptimizationLevel::Release.to_llvm_level(), 3);

        assert_eq!(OptimizationLevel::None.to_cranelift_level(), "none");
        assert_eq!(OptimizationLevel::Less.to_cranelift_level(), "speed");
        assert_eq!(OptimizationLevel::Default.to_cranelift_level(), "speed");
        assert_eq!(
            OptimizationLevel::Aggressive.to_cranelift_level(),
            "speed_and_size"
        );
        assert_eq!(
            OptimizationLevel::Release.to_cranelift_level(),
            "speed_and_size"
        );
    }

    #[test]
    fn test_generator_creation() {
        let generator = NativeCodeGenerator::new();
        assert_eq!(generator.target_platform, TargetPlatform::current());
        assert_eq!(generator.optimization_level, OptimizationLevel::Default);
        assert!(!generator.debug_info);
    }

    #[test]
    fn test_generator_configuration() {
        let mut generator = NativeCodeGenerator::new();

        generator.set_target_platform(TargetPlatform::LinuxX64);
        generator.set_optimization_level(OptimizationLevel::Release);
        generator.set_debug_info(true);
        generator.set_output_path(PathBuf::from("output.bin"));

        assert_eq!(generator.target_platform, TargetPlatform::LinuxX64);
        assert_eq!(generator.optimization_level, OptimizationLevel::Release);
        assert!(generator.debug_info);
        assert_eq!(generator.output_path, Some(PathBuf::from("output.bin")));
    }

    #[test]
    fn test_simple_program_compilation() {
        let source = r#"
Focus {
    induce x: number = 42;
    induce y: number = 10;
    induce result: number = x + y;
} Relax
"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program().unwrap();

        let mut generator = NativeCodeGenerator::new();
        generator.set_optimization_level(OptimizationLevel::None);

        // Should compile without errors
        let result = generator.generate(&ast);
        assert!(result.is_ok(), "Compilation should succeed");
    }

    #[test]
    fn test_target_info() {
        let generator = NativeCodeGenerator::new();
        let info = generator.target_info();

        // Should contain information
        assert!(info.contains("Target platform:"));
        assert!(info.contains("LLVM triple:"));
        assert!(info.contains("Optimization:"));
    }
}
