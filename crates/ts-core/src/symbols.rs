//! Symbol table and scope management

use std::collections::HashMap;
use crate::types::Type;
use crate::utils::span::{Span, Position};

/// Symbol kind
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Class,
    Interface,
    Type,
    Namespace,
    Parameter,
    Property,
    Method,
}

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_info: Option<Type>,
    pub span: Span,
    pub exported: bool,
}

impl Symbol {
    pub fn format_declaration(&self) -> String {
        match &self.type_info {
            Some(t) => format!("{}: {:?}", self.name, t),
            None => self.name.clone(),
        }
    }
}

/// Scope for symbol resolution
#[derive(Debug, Clone)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    /// Create a new scope
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    /// Create a child scope
    pub fn child(parent: Scope) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Define a symbol in this scope
    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        if self.symbols.contains_key(&symbol.name) {
            Err(format!("Symbol '{}' already defined", symbol.name))
        } else {
            self.symbols.insert(symbol.name.clone(), symbol);
            Ok(())
        }
    }

    /// Look up a symbol in this scope or parent scopes
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name).or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.lookup(name))
        })
    }

    /// Get all symbols in this scope
    pub fn symbols(&self) -> &HashMap<String, Symbol> {
        &self.symbols
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

/// Symbol table for managing scopes
#[derive(Debug)]
pub struct SymbolTable {
    global_scope: Scope,
    current_scope: Option<Scope>,
}

impl SymbolTable {
    /// Create a new symbol table
    pub fn new() -> Self {
        Self {
            global_scope: Scope::new(),
            current_scope: None,
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        let new_scope = match self.current_scope.take() {
            Some(current) => Scope::child(current),
            None => Scope::child(self.global_scope.clone()),
        };
        self.current_scope = Some(new_scope);
    }

    /// Exit the current scope
    pub fn exit_scope(&mut self) {
        if let Some(scope) = self.current_scope.take() {
            self.current_scope = scope.parent.map(|p| *p);
        }
    }

    /// Define a symbol in the current scope
    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        if let Some(ref mut scope) = self.current_scope {
            scope.define(symbol)
        } else {
            self.global_scope.define(symbol)
        }
    }

    /// Look up a symbol
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.current_scope.as_ref().and_then(|scope| scope.lookup(name))
            .or_else(|| self.global_scope.lookup(name))
    }

    /// Format baseline output
    pub fn format_baseline(&self, file_name: &str) -> String {
        let mut output = String::new();
        output.push_str(&format!("=== {} ===\n", file_name));
        
        for symbol in self.global_scope.symbols().values() {
            output.push_str(&format!(
                ">{}\nSymbol({}, Decl({}, {}))\n",
                symbol.format_declaration(),
                symbol.name,
                file_name,
                symbol.span.start.line // 使用 Position 的 line 字段
            ));
        }
        
        output
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}