//! Native Code Generator für HypnoScript
//!
//! Dieses Modul generiert plattformspezifischen nativen Code für:
//! - Windows (x86_64, ARM64)
//! - macOS (x86_64, ARM64 / Apple Silicon)
//! - Linux (x86_64, ARM64, RISC-V)
//!
//! ## Architektur
//!
//! Der Native Code Generator verwendet Cranelift als Backend für die Kompilierung.
//! Cranelift ist ein schneller, sicherer Code-Generator, der optimierten,
//! plattformspezifischen Code mit minimaler Runtime-Abhängigkeit erzeugt.
//!
//! ## Vorteile von Cranelift gegenüber LLVM
//!
//! - **Schnellere Kompilierung**: Cranelift ist deutlich schneller als LLVM
//! - **Einfachere Integration**: Reine Rust-Implementierung, keine C++-Abhängigkeiten
//! - **Kleinere Binary-Größe**: Geringerer Overhead
//! - **Sicherheit**: Memory-safe durch Rust
//!
//! ## Verwendung
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

/// Fehlertypen für die native Code-Generierung
#[derive(Error, Debug)]
pub enum NativeCodegenError {
    #[error("Plattform nicht unterstützt: {0}")]
    UnsupportedPlatform(String),

    #[error("Cranelift-Initialisierung fehlgeschlagen: {0}")]
    LlvmInitializationError(String),

    #[error("Code-Generierung fehlgeschlagen: {0}")]
    CodeGenerationError(String),

    #[error("Optimierung fehlgeschlagen: {0}")]
    OptimizationError(String),

    #[error("Linking fehlgeschlagen: {0}")]
    LinkingError(String),

    #[error("I/O-Fehler: {0}")]
    IoError(#[from] std::io::Error),
}

/// Zielplattformen für native Kompilierung
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
    /// Gibt den LLVM-Target-Triple zurück
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

    /// Erkennt die aktuelle Plattform zur Build-Zeit
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

/// Optimierungsstufen für die Code-Generierung
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// Keine Optimierung (Debug-Build)
    None,
    /// Moderate Optimierung (schnelle Kompilierung)
    Less,
    /// Standard-Optimierung (Balance)
    Default,
    /// Aggressive Optimierung (langsame Kompilierung, schneller Code)
    Aggressive,
    /// Maximale Optimierung für Releases
    Release,
}

impl OptimizationLevel {
    /// Konvertiert zu Cranelift-Optimierungslevel
    pub fn to_cranelift_level(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Less => "speed",
            Self::Default => "speed",
            Self::Aggressive => "speed_and_size",
            Self::Release => "speed_and_size",
        }
    }

    /// Konvertiert zu LLVM-Optimierungslevel (0-3)
    pub fn to_llvm_level(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Less => 1,
            Self::Default => 2,
            Self::Aggressive | Self::Release => 3,
        }
    }
}

/// Native Code Generator
///
/// Generiert plattformspezifischen nativen Maschinencode aus HypnoScript AST.
/// Verwendet Cranelift als Backend für optimierte Binaries.
pub struct NativeCodeGenerator {
    /// Zielplattform
    target_platform: TargetPlatform,
    /// Optimierungslevel
    optimization_level: OptimizationLevel,
    /// Ausgabepfad für die Binary
    output_path: Option<PathBuf>,
    /// Variablen-Mapping (Name -> Cranelift Variable)
    variable_map: HashMap<String, Variable>,
    /// Funktions-Mapping
    function_map: HashMap<String, usize>,
    /// Debug-Informationen generieren
    debug_info: bool,
    /// Nächste Variable-ID
    next_var_id: usize,
}

impl Default for NativeCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeCodeGenerator {
    /// Erstellt einen neuen Native Code Generator
    ///
    /// # Beispiele
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

    /// Setzt die Zielplattform
    ///
    /// # Argumente
    ///
    /// * `platform` - Die gewünschte Zielplattform
    pub fn set_target_platform(&mut self, platform: TargetPlatform) {
        self.target_platform = platform;
    }

    /// Setzt das Optimierungslevel
    ///
    /// # Argumente
    ///
    /// * `level` - Das gewünschte Optimierungslevel
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }

    /// Setzt den Ausgabepfad
    ///
    /// # Argumente
    ///
    /// * `path` - Der Pfad für die generierte Binary
    pub fn set_output_path(&mut self, path: PathBuf) {
        self.output_path = Some(path);
    }

    /// Aktiviert/Deaktiviert Debug-Informationen
    ///
    /// # Argumente
    ///
    /// * `enabled` - true für Debug-Infos, false sonst
    pub fn set_debug_info(&mut self, enabled: bool) {
        self.debug_info = enabled;
    }

    /// Generiert nativen Code aus dem AST
    ///
    /// # Argumente
    ///
    /// * `program` - Der HypnoScript AST
    ///
    /// # Rückgabe
    ///
    /// Pfad zur generierten Binary
    ///
    /// # Fehler
    ///
    /// Gibt einen `NativeCodegenError` zurück, wenn die Code-Generierung fehlschlägt
    pub fn generate(&mut self, program: &AstNode) -> Result<PathBuf, NativeCodegenError> {
        self.variable_map.clear();
        self.function_map.clear();
        self.next_var_id = 0;

        // Bestimme das Target-Triple (wird in Zukunft verwendet)
        let _triple = self.get_target_triple();

        // Erstelle ObjectModule für die Object-Datei-Generierung
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

        // Erstelle die main-Funktion
        self.generate_main_function(&mut module, program)?;

        // Finalisiere und schreibe Object-Datei
        let object_product = module.finish();
        let object_bytes = object_product
            .emit()
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        // Bestimme Ausgabepfad für Object-Datei
        let obj_extension = if cfg!(target_os = "windows") {
            "obj"
        } else {
            "o"
        };
        let obj_path = PathBuf::from(format!("hypnoscript_program.{}", obj_extension));

        // Schreibe Object-Datei
        std::fs::write(&obj_path, object_bytes)?;

        // Bestimme finalen Ausgabepfad für ausführbare Datei
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

        // Linke die Object-Datei zu einer ausführbaren Datei
        self.link_object_file(&obj_path, &exe_path)?;

        // Cleanup: Entferne Object-Datei
        let _ = std::fs::remove_file(&obj_path);

        Ok(exe_path)
    }

    /// Linkt eine Object-Datei zu einer ausführbaren Datei
    fn link_object_file(&self, obj_path: &Path, exe_path: &Path) -> Result<(), NativeCodegenError> {
        #[cfg(target_os = "windows")]
        {
            // Versuche verschiedene Windows-Linker
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
                "Kein geeigneter Linker gefunden. Bitte installieren Sie:\n\
                 - Visual Studio Build Tools (für link.exe)\n\
                 - GCC/MinGW (für gcc)\n\
                 - LLVM (für lld-link/clang)"
                    .to_string(),
            ))
        }

        #[cfg(not(target_os = "windows"))]
        {
            // Unix-basierte Systeme (Linux, macOS)
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
                    // Mache die Datei ausführbar auf Unix
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
                "Kein geeigneter Linker gefunden. Bitte installieren Sie gcc oder clang."
                    .to_string(),
            ))
        }
    }

    /// Konvertiert Cranelift-Triple aus TargetPlatform
    fn get_target_triple(&self) -> Triple {
        self.target_platform
            .llvm_triple()
            .parse()
            .unwrap_or_else(|_| Triple::host())
    }

    /// Generiert die main-Funktion
    fn generate_main_function(
        &mut self,
        module: &mut ObjectModule,
        program: &AstNode,
    ) -> Result<(), NativeCodegenError> {
        // Erstelle Funktions-Signatur: main() -> i32
        let mut sig = module.make_signature();
        sig.returns.push(AbiParam::new(types::I32));

        let func_id = module
            .declare_function("main", Linkage::Export, &sig)
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        let mut ctx = module.make_context();
        ctx.func.signature = sig;

        // Erstelle Function Builder
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        // Erstelle Entry-Block
        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Generiere Code für das Programm
        if let AstNode::Program(statements) = program {
            for stmt in statements {
                self.generate_statement(&mut builder, stmt)?;
            }
        }

        // Return 0
        let zero = builder.ins().iconst(types::I32, 0);
        builder.ins().return_(&[zero]);

        // Finalisiere Funktion
        builder.finalize();

        // Definiere Funktion im Modul
        module
            .define_function(func_id, &mut ctx)
            .map_err(|e| NativeCodegenError::CodeGenerationError(e.to_string()))?;

        module.clear_context(&mut ctx);

        Ok(())
    }

    /// Generiert Code für ein Statement
    fn generate_statement(
        &mut self,
        builder: &mut FunctionBuilder,
        stmt: &AstNode,
    ) -> Result<(), NativeCodegenError> {
        match stmt {
            AstNode::VariableDeclaration {
                name, initializer, ..
            } => {
                // Erstelle Variable
                let var = Variable::new(self.next_var_id);
                self.next_var_id += 1;

                builder.declare_var(var, types::F64);
                self.variable_map.insert(name.clone(), var);

                // Initialisiere Variable
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
                // Evaluiere Expression (Ergebnis wird verworfen)
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
                // Nicht unterstützte Statements ignorieren
                // TODO: Erweitern für vollständige Sprachunterstützung
            }
        }

        Ok(())
    }

    /// Generiert Code für einen Expression
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
                        // Modulo für floats: a - floor(a/b) * b
                        let div = builder.ins().fdiv(lhs, rhs);
                        let floor = builder.ins().floor(div);
                        let mul = builder.ins().fmul(floor, rhs);
                        builder.ins().fsub(lhs, mul)
                    }
                    _ => {
                        // Unbekannter Operator -> Return 0
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
                        // Logische Negation (für Integers)
                        builder.ins().bxor_imm(val, 1)
                    }
                    _ => val,
                };

                Ok(result)
            }

            _ => {
                // Nicht unterstützte Expressions -> Return 0
                Ok(builder.ins().f64const(0.0))
            }
        }
    }

    /// Gibt Informationen über die Zielplattform zurück
    pub fn target_info(&self) -> String {
        format!(
            "Zielplattform: {}\nLLVM-Triple: {}\nOptimierung: {:?}",
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

        // Sollte ohne Fehler kompilieren
        let result = generator.generate(&ast);
        assert!(result.is_ok(), "Compilation should succeed");
    }

    #[test]
    fn test_target_info() {
        let generator = NativeCodeGenerator::new();
        let info = generator.target_info();

        // Sollte Informationen enthalten
        assert!(info.contains("Zielplattform:"));
        assert!(info.contains("LLVM-Triple:"));
        assert!(info.contains("Optimierung:"));
    }
}
