//! Scanner implementation for tokenizing TypeScript source code

use super::{Token, TokenKind};
use super::keywords::{is_keyword, keyword_kind};
use crate::utils::span::{Span, Position};

/// Scanner for tokenizing TypeScript source code
#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    /// Create a new scanner for the given source code
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Get the current character without advancing
    fn current_char(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    /// Advance to the next character
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current_char() {
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Create a span from start position to current position
    fn make_span(&self, start_line: usize, start_column: usize) -> Span {
        Span {
            start: Position {
                line: start_line,
                column: start_column,
                offset: 0, // Will be calculated properly later
            },
            end: Position {
                line: self.line,
                column: self.column,
                offset: self.position,
            },
        }
    }

    /// Get the next token from the source
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        match self.current_char()? {
            // End of file
            '\0' | _ if self.position >= self.source.len() => {
                Some(Token {
                    kind: TokenKind::Eof,
                    span: self.make_span(start_line, start_column),
                })
            }
            // Single character tokens
            '+' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Plus,
                    span: self.make_span(start_line, start_column),
                })
            }
            '-' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Minus,
                    span: self.make_span(start_line, start_column),
                })
            }
            ';' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Semicolon,
                    span: self.make_span(start_line, start_column),
                })
            }
            // Identifiers and keywords
            ch if ch.is_alphabetic() || ch == '_' || ch == '$' => {
                let mut value = String::new();
                while let Some(ch) = self.current_char() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        value.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                let kind = if is_keyword(&value) {
                    keyword_kind(&value)
                } else {
                    TokenKind::Identifier(value)
                };

                Some(Token {
                    kind,
                    span: self.make_span(start_line, start_column),
                })
            }
            // Numbers
            ch if ch.is_ascii_digit() => {
                let mut value = String::new();
                while let Some(ch) = self.current_char() {
                    if ch.is_ascii_digit() || ch == '.' {
                        value.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                let number = value.parse::<f64>().unwrap_or(0.0);
                Some(Token {
                    kind: TokenKind::Number(number),
                    span: self.make_span(start_line, start_column),
                })
            }
            // Default case - skip unknown characters
            _ => {
                self.advance();
                self.next_token()
            }
        }
    }
}