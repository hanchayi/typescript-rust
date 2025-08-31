//! Diagnostic and error reporting utilities

use crate::utils::span::Span;
use serde::{Serialize, Deserialize};

/// Severity levels for diagnostics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Different kinds of diagnostics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Info,
    SyntaxError,
    TypeError,
    ReferenceError,
    UnreachableCode,
    UnusedVariable,
    MissingReturn,
}

/// A diagnostic message (error, warning, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub severity: Severity,
    pub message: String,
    pub span: Span,
    pub help: Option<String>,
}

impl Diagnostic {
    /// Create a new error diagnostic
    pub fn error(message: String, span: Span) -> Self {
        Self {
            kind: DiagnosticKind::Error,
            severity: Severity::Error,
            message,
            span,
            help: None,
        }
    }

    /// Create a new warning diagnostic
    pub fn warning(message: String, span: Span) -> Self {
        Self {
            kind: DiagnosticKind::Warning,
            severity: Severity::Warning,
            message,
            span,
            help: None,
        }
    }

    /// Add help text to this diagnostic
    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
}