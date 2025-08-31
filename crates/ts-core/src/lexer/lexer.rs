//! Main lexer implementation

use super::{Token, TokenKind, Scanner};
use crate::utils::span::{Span, Position};
use std::error::Error;

/// Main lexer for TypeScript source code
#[derive(Debug)]
pub struct Lexer {
    scanner: Scanner,
}

impl Lexer {
    /// Create a new lexer for the given source code
    pub fn new(source: &str) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }

    /// Get the next token from the source
    pub fn next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        self.scanner.next_token().ok_or_else(|| "Unexpected end of input".into())
    }

    /// Tokenize the entire source into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}