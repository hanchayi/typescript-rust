//! Lexical analysis module for TypeScript

mod token;
mod scanner;
mod keywords;
mod lexer;

pub use token::{Token, TokenKind};
pub use scanner::Scanner;
pub use keywords::{is_keyword, keyword_kind};
pub use lexer::Lexer;

use crate::utils::span::Span;
use std::error::Error;

/// Tokenize a source string into a vector of tokens
pub fn tokenize(source: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}