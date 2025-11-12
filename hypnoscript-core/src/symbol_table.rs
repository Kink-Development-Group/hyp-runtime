use crate::symbols::{Symbol, SymbolKind};
use std::collections::HashMap;

/// Symbol table for managing scopes and variable bindings
#[derive(Debug, Clone)]
pub struct SymbolTable {
    enclosing: Option<Box<SymbolTable>>,
    symbols: HashMap<String, Symbol>,
    child_scopes: Vec<SymbolTable>,
    pub scope_name: String,
    pub scope_level: usize,
}

impl SymbolTable {
    /// Create a new symbol table
    pub fn new(enclosing: Option<Box<SymbolTable>>, scope_name: String) -> Self {
        let scope_level = enclosing.as_ref().map(|e| e.scope_level + 1).unwrap_or(0);
        Self {
            enclosing,
            symbols: HashMap::new(),
            child_scopes: Vec::new(),
            scope_name,
            scope_level,
        }
    }

    /// Create a global scope
    pub fn global() -> Self {
        Self::new(None, "Global".to_string())
    }

    /// Define a new symbol in the current scope
    pub fn define(&mut self, sym: Symbol) -> bool {
        if self.symbols.contains_key(&sym.name) {
            eprintln!(
                "[SymbolTable] Symbol '{}' is already defined in scope '{}'.",
                sym.name, self.scope_name
            );
            return false;
        }
        self.symbols.insert(sym.name.clone(), sym);
        true
    }

    /// Resolve a symbol, looking in enclosing scopes if necessary
    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.symbols
            .get(name)
            .or_else(|| self.enclosing.as_ref().and_then(|e| e.resolve(name)))
    }

    /// Resolve a symbol only in the current scope
    pub fn resolve_local(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Check if a symbol exists (locally or in enclosing scopes)
    pub fn has_symbol(&self, name: &str) -> bool {
        self.resolve(name).is_some()
    }

    /// Remove a symbol from the current scope
    pub fn remove_symbol(&mut self, name: &str) -> bool {
        self.symbols.remove(name).is_some()
    }

    /// Clear all symbols from the current scope
    pub fn clear(&mut self) {
        self.symbols.clear();
    }

    /// Get all symbols in the current scope
    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        let mut symbols: Vec<_> = self.symbols.values().collect();
        symbols.sort_by(|a, b| a.name.cmp(&b.name));
        symbols
    }

    /// Get symbols by kind
    pub fn get_symbols_by_kind(&self, kind: SymbolKind) -> Vec<&Symbol> {
        let mut symbols: Vec<_> = self.symbols.values().filter(|s| s.kind == kind).collect();
        symbols.sort_by(|a, b| a.name.cmp(&b.name));
        symbols
    }

    /// Get exported symbols
    pub fn get_exported_symbols(&self) -> Vec<&Symbol> {
        let mut symbols: Vec<_> = self.symbols.values().filter(|s| s.is_exported).collect();
        symbols.sort_by(|a, b| a.name.cmp(&b.name));
        symbols
    }

    /// Get constants
    pub fn get_constants(&self) -> Vec<&Symbol> {
        let mut symbols: Vec<_> = self.symbols.values().filter(|s| s.is_constant).collect();
        symbols.sort_by(|a, b| a.name.cmp(&b.name));
        symbols
    }

    /// Get symbol count
    pub fn symbol_count(&self) -> usize {
        self.symbols.len()
    }

    /// Get the enclosing scope
    pub fn get_enclosing_scope(&self) -> Option<&SymbolTable> {
        self.enclosing.as_ref().map(|e| e.as_ref())
    }

    /// Get child scopes
    pub fn get_child_scopes(&self) -> &[SymbolTable] {
        &self.child_scopes
    }

    /// Get the root scope
    pub fn get_root_scope(&self) -> &SymbolTable {
        let mut current = self;
        while let Some(ref enclosing) = current.enclosing {
            current = enclosing;
        }
        current
    }

    /// Get scope depth
    pub fn get_scope_depth(&self) -> usize {
        let mut depth = 0;
        let mut current = self;
        while let Some(ref enclosing) = current.enclosing {
            depth += 1;
            current = enclosing;
        }
        depth
    }

    /// Get symbol statistics
    pub fn get_symbol_statistics(&self) -> HashMap<SymbolKind, usize> {
        let mut stats = HashMap::new();
        for symbol in self.symbols.values() {
            *stats.entry(symbol.kind).or_insert(0) += 1;
        }
        stats
    }

    /// Get a scope summary
    pub fn get_scope_summary(&self) -> String {
        let stats = self.get_symbol_statistics();
        let mut summary = format!(
            "Scope '{}' (Level {}): {} symbols\n",
            self.scope_name,
            self.scope_level,
            self.symbol_count()
        );

        let mut kinds: Vec<_> = stats.keys().collect();
        kinds.sort();
        for kind in kinds {
            if let Some(count) = stats.get(kind) {
                summary.push_str(&format!("  {:?}: {}\n", kind, count));
            }
        }
        summary
    }

    /// Search symbols by pattern
    pub fn search_symbols(&self, pattern: &str, kind: Option<SymbolKind>) -> Vec<&Symbol> {
        let pattern_lower = pattern.to_lowercase();
        let mut symbols: Vec<_> = self
            .symbols
            .values()
            .filter(|s| {
                let name_match = s.name.to_lowercase().contains(&pattern_lower);
                let kind_match = kind.is_none_or(|k| s.kind == k);
                name_match && kind_match
            })
            .collect();
        symbols.sort_by(|a, b| a.name.cmp(&b.name));
        symbols
    }

    /// Validate symbols
    pub fn validate_symbols(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for symbol in self.symbols.values() {
            if symbol.name.trim().is_empty() {
                errors.push(format!(
                    "Symbol has empty name in scope '{}'",
                    self.scope_name
                ));
            }

            if symbol.kind == SymbolKind::Function && symbol.type_name.is_none() {
                errors.push(format!("Function '{}' has no return type", symbol.name));
            }
        }

        errors
    }

    /// Merge symbols from another symbol table
    pub fn merge_from(&mut self, other: &SymbolTable, overwrite: bool) {
        for (name, symbol) in &other.symbols {
            if overwrite || !self.symbols.contains_key(name) {
                self.symbols.insert(name.clone(), symbol.clone());
            }
        }
    }

    /// Export only exported symbols to a new scope
    pub fn export_scope(&self) -> SymbolTable {
        let mut exported = SymbolTable::new(None, format!("{}_Exported", self.scope_name));
        for symbol in self.symbols.values() {
            if symbol.is_exported {
                exported.define(symbol.clone());
            }
        }
        exported
    }

    /// Debug scope information
    pub fn debug_scope(&self) -> String {
        let mut result = format!(
            "Scope '{}' (Level {}):\n",
            self.scope_name, self.scope_level
        );

        let mut symbols: Vec<_> = self.symbols.iter().collect();
        symbols.sort_by(|a, b| a.0.cmp(b.0));

        for (name, symbol) in symbols {
            let const_info = if symbol.is_constant { " (const)" } else { "" };
            let export_info = if symbol.is_exported {
                " (exported)"
            } else {
                ""
            };
            result.push_str(&format!(
                "  {:?} {}: {:?}{}{}\n",
                symbol.kind, name, symbol.type_name, const_info, export_info
            ));
        }

        if let Some(ref enclosing) = self.enclosing {
            result.push_str("\nEnclosing Scope:\n");
            result.push_str(&enclosing.debug_scope());
        }

        result
    }
}
