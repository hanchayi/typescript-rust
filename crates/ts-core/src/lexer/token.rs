//! Token definitions for the TypeScript lexer

use crate::utils::span::Span;
use serde::{Deserialize, Serialize};

/// A token in the TypeScript source code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// Source location information
    pub span: Span,
}

/// Different kinds of tokens in TypeScript
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,
    
    // Identifiers and keywords
    Identifier(String),
    
    // Keywords
    Let,
    Const,
    Var,
    Function,
    Class,
    Interface,
    Type,
    Enum,
    Namespace,
    Import,
    Export,
    From,
    As,
    Default,
    If,
    Else,
    For,
    While,
    Do,
    Switch,
    Case,
    Break,
    Continue,
    Return,
    Try,
    Catch,
    Finally,
    Throw,
    New,
    This,
    Super,
    Extends,
    Implements,
    Public,
    Private,
    Protected,
    Static,
    Abstract,
    Readonly,
    Async,
    Await,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    EqualEqual,
    EqualEqualEqual,
    BangEqual,
    BangEqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LessLess,
    GreaterGreater,
    GreaterGreaterGreater,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    Caret,
    Tilde,
    Bang,
    Question,
    QuestionQuestion,
    QuestionDot,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Arrow,
    
    // Special
    Eof,
    Newline,
    Whitespace,
    Comment(String),
}

impl Token {
    /// Create a new token
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
    
    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, 
            TokenKind::Let | TokenKind::Const | TokenKind::Var |
            TokenKind::Function | TokenKind::Class | TokenKind::Interface |
            TokenKind::Type | TokenKind::Enum | TokenKind::Namespace |
            TokenKind::Import | TokenKind::Export | TokenKind::From |
            TokenKind::As | TokenKind::Default | TokenKind::If |
            TokenKind::Else | TokenKind::For | TokenKind::While |
            TokenKind::Do | TokenKind::Switch | TokenKind::Case |
            TokenKind::Break | TokenKind::Continue | TokenKind::Return |
            TokenKind::Try | TokenKind::Catch | TokenKind::Finally |
            TokenKind::Throw | TokenKind::New | TokenKind::This |
            TokenKind::Super | TokenKind::Extends | TokenKind::Implements |
            TokenKind::Public | TokenKind::Private | TokenKind::Protected |
            TokenKind::Static | TokenKind::Abstract | TokenKind::Readonly |
            TokenKind::Async | TokenKind::Await
        )
    }
}