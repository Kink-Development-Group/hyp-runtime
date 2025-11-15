//! WebAssembly Binary Generator für HypnoScript
//!
//! Dieses Modul generiert binäres WebAssembly (.wasm) direkt aus dem AST,
//! zusätzlich zum bereits vorhandenen Text-Format (.wat) Generator.
//!
//! ## Verwendung
//!
//! ```rust,no_run
//! use hypnoscript_compiler::wasm_binary::WasmBinaryGenerator;
//! use hypnoscript_lexer_parser::ast::AstNode;
//!
//! let mut generator = WasmBinaryGenerator::new();
//! // let wasm_bytes = generator.generate(&ast)?;
//! // std::fs::write("output.wasm", wasm_bytes)?;
//! ```

use hypnoscript_lexer_parser::ast::AstNode;
use thiserror::Error;

/// Fehlertypen für die WASM-Binary-Generierung
#[derive(Error, Debug)]
pub enum WasmBinaryError {
    #[error("Ungültiger AST-Knoten: {0}")]
    InvalidAstNode(String),

    #[error("Code-Generierung fehlgeschlagen: {0}")]
    CodeGenerationError(String),

    #[error("I/O-Fehler: {0}")]
    IoError(#[from] std::io::Error),
}

/// WebAssembly Binary Generator
///
/// Generiert binäres WebAssembly (.wasm) direkt aus dem HypnoScript AST.
/// Das binäre Format ist kompakter und wird direkt von WebAssembly-Runtimes
/// ausgeführt, ohne vorheriges Parsen.
pub struct WasmBinaryGenerator {
    /// Generierte Bytes
    output: Vec<u8>,
    /// Funktions-Index
    function_index: u32,
    /// Typ-Index
    type_index: u32,
}

impl Default for WasmBinaryGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmBinaryGenerator {
    /// Erstellt einen neuen WASM Binary Generator
    ///
    /// # Beispiele
    ///
    /// ```
    /// use hypnoscript_compiler::wasm_binary::WasmBinaryGenerator;
    ///
    /// let generator = WasmBinaryGenerator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            function_index: 0,
            type_index: 0,
        }
    }

    /// Generiert WASM-Binary aus dem AST
    ///
    /// # Argumente
    ///
    /// * `program` - Der HypnoScript AST
    ///
    /// # Rückgabe
    ///
    /// Vec<u8> mit den generierten WASM-Bytes
    ///
    /// # Fehler
    ///
    /// Gibt einen `WasmBinaryError` zurück, wenn die Code-Generierung fehlschlägt
    pub fn generate(&mut self, program: &AstNode) -> Result<Vec<u8>, WasmBinaryError> {
        self.output.clear();
        self.function_index = 0;
        self.type_index = 0;

        // WASM Magic Number: \0asm
        self.write_bytes(&[0x00, 0x61, 0x73, 0x6D]);

        // WASM Version: 1
        self.write_bytes(&[0x01, 0x00, 0x00, 0x00]);

        // Type Section
        self.emit_type_section()?;

        // Import Section
        self.emit_import_section()?;

        // Function Section
        self.emit_function_section()?;

        // Memory Section
        self.emit_memory_section()?;

        // Export Section
        self.emit_export_section()?;

        // Code Section
        if let AstNode::Program(statements) = program {
            self.emit_code_section(statements)?;
        }

        Ok(self.output.clone())
    }

    /// Schreibt Bytes in den Output
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.output.extend_from_slice(bytes);
    }

    /// Schreibt einen LEB128-kodierten unsigned integer
    fn write_uleb128(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.output.push(byte);
            if value == 0 {
                break;
            }
        }
    }

    /// Emittiert die Type Section
    fn emit_type_section(&mut self) -> Result<(), WasmBinaryError> {
        let section = vec![0x60, 0x00, 0x00];

        // Write section
        self.output.push(0x01); // Type section ID
        self.write_uleb128(section.len() as u64);
        self.write_uleb128(1); // 1 type
        self.write_bytes(&section);

        Ok(())
    }

    /// Emittiert die Import Section
    fn emit_import_section(&mut self) -> Result<(), WasmBinaryError> {
        self.output.push(0x02); // Import section ID

        let mut imports = Vec::new();

        // Import console_log_f64
        self.write_import_function(
            &mut imports,
            "env",
            "console_log_f64",
            0, // Type index
        );

        // Write section length and imports
        self.write_uleb128(imports.len() as u64);
        self.write_bytes(&imports);

        Ok(())
    }

    /// Schreibt einen Import-Eintrag
    fn write_import_function(
        &mut self,
        buffer: &mut Vec<u8>,
        module: &str,
        field: &str,
        type_idx: u32,
    ) {
        // Module name
        buffer.push(module.len() as u8);
        buffer.extend_from_slice(module.as_bytes());

        // Field name
        buffer.push(field.len() as u8);
        buffer.extend_from_slice(field.as_bytes());

        // Import kind: function
        buffer.push(0x00);

        // Type index
        buffer.push(type_idx as u8);
    }

    /// Emittiert die Function Section
    fn emit_function_section(&mut self) -> Result<(), WasmBinaryError> {
        self.output.push(0x03); // Function section ID
        self.write_uleb128(1); // Section size (placeholder)
        self.write_uleb128(1); // 1 function
        self.write_uleb128(0); // Type index 0

        Ok(())
    }

    /// Emittiert die Memory Section
    fn emit_memory_section(&mut self) -> Result<(), WasmBinaryError> {
        self.output.push(0x05); // Memory section ID
        self.write_uleb128(3); // Section size
        self.write_uleb128(1); // 1 memory
        self.output.push(0x00); // No maximum
        self.write_uleb128(1); // 1 page minimum

        Ok(())
    }

    /// Emittiert die Export Section
    fn emit_export_section(&mut self) -> Result<(), WasmBinaryError> {
        self.output.push(0x07); // Export section ID

        let mut exports = Vec::new();

        // Export main function
        exports.push(4); // "main" length
        exports.extend_from_slice(b"main");
        exports.push(0x00); // Function kind
        exports.push(0x00); // Function index 0

        // Export memory
        exports.push(6); // "memory" length
        exports.extend_from_slice(b"memory");
        exports.push(0x02); // Memory kind
        exports.push(0x00); // Memory index 0

        self.write_uleb128(exports.len() as u64);
        self.write_uleb128(2); // 2 exports
        self.write_bytes(&exports);

        Ok(())
    }

    /// Emittiert die Code Section
    fn emit_code_section(&mut self, _statements: &[AstNode]) -> Result<(), WasmBinaryError> {
        self.output.push(0x0A); // Code section ID

        let mut code = Vec::new();

        // Function body
        let body = vec![
            0x00, // 0 local declarations
            0x0B, // end
        ];

        // Write function body size
        code.push(body.len() as u8);
        code.extend(body);

        // Write section
        self.write_uleb128(code.len() as u64 + 1); // +1 for function count
        self.write_uleb128(1); // 1 function
        self.write_bytes(&code);

        Ok(())
    }

    /// Gibt die generierten Bytes zurück
    pub fn get_output(&self) -> &[u8] {
        &self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_binary_magic() {
        let mut generator = WasmBinaryGenerator::new();
        let program = AstNode::Program(vec![]);

        let wasm = generator.generate(&program).unwrap();

        // Check magic number
        assert_eq!(&wasm[0..4], &[0x00, 0x61, 0x73, 0x6D]); // \0asm
        // Check version
        assert_eq!(&wasm[4..8], &[0x01, 0x00, 0x00, 0x00]); // version 1
    }

    #[test]
    fn test_wasm_binary_structure() {
        let mut generator = WasmBinaryGenerator::new();
        let program = AstNode::Program(vec![]);

        let wasm = generator.generate(&program).unwrap();

        // WASM binary should have at least header + sections
        assert!(wasm.len() > 8);
    }

    #[test]
    fn test_uleb128_encoding() {
        let mut generator = WasmBinaryGenerator::new();

        generator.write_uleb128(0);
        assert_eq!(generator.output, vec![0]);

        generator.output.clear();
        generator.write_uleb128(127);
        assert_eq!(generator.output, vec![127]);

        generator.output.clear();
        generator.write_uleb128(128);
        assert_eq!(generator.output, vec![0x80, 0x01]);
    }
}
