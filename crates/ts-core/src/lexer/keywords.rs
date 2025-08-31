//! TypeScript keyword definitions and utilities

use super::TokenKind;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Map of keyword strings to their token kinds
static KEYWORDS: Lazy<HashMap<&'static str, TokenKind>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("let", TokenKind::Let);
    map.insert("const", TokenKind::Const);
    map.insert("var", TokenKind::Var);
    map.insert("function", TokenKind::Function);
    map.insert("class", TokenKind::Class);
    map.insert("interface", TokenKind::Interface);
    map.insert("type", TokenKind::Type);
    map.insert("enum", TokenKind::Enum);
    map.insert("namespace", TokenKind::Namespace);
    map.insert("import", TokenKind::Import);
    map.insert("export", TokenKind::Export);
    map.insert("from", TokenKind::From);
    map.insert("as", TokenKind::As);
    map.insert("default", TokenKind::Default);
    map.insert("if", TokenKind::If);
    map.insert("else", TokenKind::Else);
    map.insert("for", TokenKind::For);
    map.insert("while", TokenKind::While);
    map.insert("do", TokenKind::Do);
    map.insert("switch", TokenKind::Switch);
    map.insert("case", TokenKind::Case);
    map.insert("break", TokenKind::Break);
    map.insert("continue", TokenKind::Continue);
    map.insert("return", TokenKind::Return);
    map.insert("try", TokenKind::Try);
    map.insert("catch", TokenKind::Catch);
    map.insert("finally", TokenKind::Finally);
    map.insert("throw", TokenKind::Throw);
    map.insert("new", TokenKind::New);
    map.insert("this", TokenKind::This);
    map.insert("super", TokenKind::Super);
    map.insert("extends", TokenKind::Extends);
    map.insert("implements", TokenKind::Implements);
    map.insert("public", TokenKind::Public);
    map.insert("private", TokenKind::Private);
    map.insert("protected", TokenKind::Protected);
    map.insert("static", TokenKind::Static);
    map.insert("abstract", TokenKind::Abstract);
    map.insert("readonly", TokenKind::Readonly);
    map.insert("async", TokenKind::Async);
    map.insert("await", TokenKind::Await);
    map.insert("true", TokenKind::Boolean(true));
    map.insert("false", TokenKind::Boolean(false));
    map.insert("null", TokenKind::Null);
    map.insert("undefined", TokenKind::Undefined);
    map
});

/// Check if a string is a TypeScript keyword
pub fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains_key(word)
}

/// Get the token kind for a keyword
pub fn keyword_kind(word: &str) -> TokenKind {
    KEYWORDS.get(word).cloned().unwrap_or_else(|| TokenKind::Identifier(word.to_string()))
}