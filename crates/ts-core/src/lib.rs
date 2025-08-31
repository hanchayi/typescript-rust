//! TypeScript compiler core library

use std::path::Path;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod types;
pub mod symbols;
pub mod codegen;
pub mod diagnostics;
pub mod utils;
pub mod baseline_test;

// Re-export commonly used types
pub use ast::AstNode;
pub use lexer::{Lexer, Token, TokenKind};
pub use parser::Parser;
pub use types::{Type, TypeChecker};
pub use symbols::{Symbol, SymbolTable, SymbolKind};
pub use codegen::CodeGenerator;
pub use diagnostics::Diagnostic;
pub use utils::span::{Span, Position};
pub use baseline_test::{BaselineTestRunner, BaselineTestResult};

use serde::{Deserialize, Serialize};

/// Compilation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOptions {
    /// Target JavaScript version
    pub target: String,
    /// Module system
    pub module: String,
    /// Generate source maps
    pub source_map: bool,
    /// Source file name
    pub file_name: String,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            target: "es5".to_string(),
            module: "commonjs".to_string(),
            source_map: false,
            file_name: "input.ts".to_string(),
        }
    }
}

/// Compilation result
#[derive(Debug, Clone)]
pub struct CompileResult {
    /// Generated JavaScript code
    pub code: String,
    /// Compilation diagnostics
    pub diagnostics: Vec<Diagnostic>,
    /// Source map (if enabled)
    pub source_map: Option<String>,
}

/// Compile TypeScript source code
pub fn compile(input: &Path, options: &CompileOptions) -> Result<CompileResult, Vec<Diagnostic>> {
    let source = std::fs::read_to_string(input)
        .map_err(|e| vec![Diagnostic::error(format!("Failed to read file: {}", e), Span::default())])?;
    
    let mut lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    
    let ast = parser.parse()
        .map_err(|e| vec![Diagnostic::error(format!("Parse error: {:?}", e), Span::default())])?;
    
    let mut codegen = CodeGenerator::new();
    let js_code = codegen.generate(&[ast])
        .map_err(|e| vec![Diagnostic::error(format!("Codegen error: {:?}", e), Span::default())])?;
    
    Ok(CompileResult {
        code: js_code,
        diagnostics: Vec::new(),
        source_map: None,
    })
}