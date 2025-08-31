//! TypeScript compiler core library

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod types;
pub mod symbols;
pub mod codegen;
pub mod diagnostics;
pub mod utils;

// Re-export commonly used types
pub use ast::{AstNode, Expression, Statement, Declaration};
pub use lexer::{Lexer, Token, TokenKind, tokenize};
pub use parser::Parser;
pub use types::{TypeChecker, Type};
pub use symbols::{SymbolTable, Symbol};
pub use codegen::CodeGenerator;
pub use diagnostics::{Diagnostic, DiagnosticKind, Severity};

use serde::{Serialize, Deserialize};

/// Compilation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOptions {
    pub target: String,
    pub module: String,
    pub source_map: bool,
    pub file_name: String,
}

/// Compilation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    pub code: String,
    pub source_map: Option<String>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Main compilation function
pub fn compile(source: &str, options: CompileOptions) -> Result<CompileResult, Vec<Diagnostic>> {
    // Tokenize
    let tokens = match tokenize(source) {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(vec![Diagnostic {
                kind: DiagnosticKind::Error,
                severity: Severity::Error,
                message: format!("Tokenization failed: {}", e),
                span: utils::span::Span::default(),
                help: None,
            }]);
        }
    };

    // Parse
    let mut parser = Parser::from_tokens(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(diagnostics) => return Err(diagnostics),
    };

    // Type check
    let mut type_checker = TypeChecker::new();
    if let Err(diagnostics) = type_checker.check_program(&ast) {
        return Err(diagnostics);
    }

    // Generate code
    let mut codegen = CodeGenerator::new();
    let code = match codegen.generate(&ast) {
        Ok(code) => code,
        Err(e) => {
            return Err(vec![Diagnostic {
                kind: DiagnosticKind::Error,
                severity: Severity::Error,
                message: format!("Code generation failed: {}", e),
                span: utils::span::Span::default(),
                help: None,
            }]);
        }
    };

    let source_map = if options.source_map {
        match codegen.generate_source_map(&options.file_name) {
            Ok(map) => Some(map),
            Err(e) => {
                return Err(vec![Diagnostic {
                    kind: DiagnosticKind::Error,
                    severity: Severity::Error,
                    message: format!("Source map generation failed: {}", e),
                    span: utils::span::Span::default(),
                    help: None,
                }]);
            }
        }
    } else {
        None
    };

    Ok(CompileResult {
        code,
        source_map,
        diagnostics: vec![], // TODO: Collect warnings
    })
}