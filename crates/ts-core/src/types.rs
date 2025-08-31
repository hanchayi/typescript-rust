//! TypeScript type system implementation

use std::collections::HashMap;
use crate::utils::span::Span;
use crate::ast::AstNode;
use crate::diagnostics::{Diagnostic, DiagnosticKind};
use serde::{Serialize, Deserialize};

/// TypeScript type representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Primitive types
    String,
    Number,
    Boolean,
    Undefined,
    Null,
    Void,
    Any,
    Unknown,
    Never,
    
    /// Object type
    Object {
        properties: HashMap<String, Type>,
    },
    
    /// Array type
    Array(Box<Type>),
    
    /// Function type
    Function {
        parameters: Vec<Parameter>,
        return_type: Box<Type>,
    },
    
    /// Union type
    Union(Vec<Type>),
    
    /// Intersection type
    Intersection(Vec<Type>),
    
    /// Type reference
    Reference {
        name: String,
        type_arguments: Vec<Type>,
    },
    
    /// Generic type parameter
    TypeParameter {
        name: String,
        constraint: Option<Box<Type>>,
    },
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
    pub optional: bool,
    pub span: Span,
}

/// Type checker context
pub struct TypeChecker {
    /// Type environment
    type_env: HashMap<String, Type>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            type_env: HashMap::new(),
        }
    }

    /// Add a type binding
    pub fn bind_type(&mut self, name: String, ty: Type) {
        self.type_env.insert(name, ty);
    }

    /// Look up a type
    pub fn lookup_type(&self, name: &str) -> Option<&Type> {
        self.type_env.get(name)
    }

    /// Check if two types are compatible
    pub fn is_assignable(&self, source: &Type, target: &Type) -> bool {
        match (source, target) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Unknown, _) => false,
            (_, Type::Unknown) => true,
            (Type::Never, _) => true,
            (_, Type::Never) => false,
            (a, b) if a == b => true,
            // Add more compatibility rules
            _ => false,
        }
    }
    
    /// Type check a program (AST)
    pub fn check_program(&mut self, _ast: &[AstNode]) -> Result<(), Vec<Diagnostic>> {
        // Placeholder implementation
        Ok(())
    }
    
    /// 格式化类型信息为基线格式
    pub fn format_for_baseline(&self) -> String {
        let mut output = String::new();
        
        // 使用实际存在的字段，而不是type_annotations
        // 这里需要根据实际的TypeChecker结构来实现
        output.push_str("// Type information not yet implemented\n");
        
        output
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}