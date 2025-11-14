//! Native Code Generator für HypnoScript
//!
//! Dieses Modul generiert plattformspezifischen nativen Code für:
//! - Windows (x86_64, ARM64)
//! - macOS (x86_64, ARM64 / Apple Silicon)
//! - Linux (x86_64, ARM64, RISC-V)
//!
//! ## Architektur
//!
//! Der Native Code Generator verwendet ein LLVM-Backend für die Kompilierung.
//! Dies ermöglicht optimierten, plattformspezifischen Code mit minimaler
//! Runtime-Abhängigkeit.
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

use hypnoscript_lexer_parser::ast::AstNode;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

/// Fehlertypen für die native Code-Generierung
#[derive(Error, Debug)]
pub enum NativeCodegenError {
    #[error("Plattform nicht unterstützt: {0}")]
    UnsupportedPlatform(String),

    #[error("LLVM-Initialisierung fehlgeschlagen: {0}")]
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
/// Verwendet LLVM als Backend für optimierte Binaries.
pub struct NativeCodeGenerator {
    /// Zielplattform
    target_platform: TargetPlatform,
    /// Optimierungslevel
    optimization_level: OptimizationLevel,
    /// Ausgabepfad für die Binary
    output_path: Option<PathBuf>,
    /// Variablen-Mapping
    variable_map: HashMap<String, usize>,
    /// Funktions-Mapping
    function_map: HashMap<String, usize>,
    /// Debug-Informationen generieren
    debug_info: bool,
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
    pub fn generate(&mut self, _program: &AstNode) -> Result<PathBuf, NativeCodegenError> {
        // TODO: LLVM-Integration implementieren
        // Hier wird in zukünftigen Versionen das LLVM-Backend verwendet

        self.variable_map.clear();
        self.function_map.clear();

        // Placeholder: Aktuell wird eine Fehlermeldung zurückgegeben
        Err(NativeCodegenError::CodeGenerationError(
            "Native Code-Generierung ist noch nicht vollständig implementiert. \
             Verwenden Sie den Interpreter oder WASM-Compiler.".to_string()
        ))
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
    }

    #[test]
    fn test_generator_creation() {
        let generator = NativeCodeGenerator::new();
        assert_eq!(generator.target_platform, TargetPlatform::current());
        assert_eq!(generator.optimization_level, OptimizationLevel::Default);
        assert_eq!(generator.debug_info, false);
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
        assert_eq!(generator.debug_info, true);
        assert_eq!(generator.output_path, Some(PathBuf::from("output.bin")));
    }
}
