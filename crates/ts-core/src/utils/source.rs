//! Source code management utilities

use super::span::{Position, Span};
use std::collections::HashMap;

/// Manages source files and their content
#[derive(Debug, Clone)]
pub struct SourceManager {
    files: HashMap<String, SourceFile>,
}

/// Represents a source file
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub content: String,
    line_starts: Vec<usize>,
}

impl SourceManager {
    /// Create a new source manager
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Add a source file
    pub fn add_file(&mut self, name: String, content: String) {
        let source_file = SourceFile::new(name.clone(), content);
        self.files.insert(name, source_file);
    }

    /// Get a source file by name
    pub fn get_file(&self, name: &str) -> Option<&SourceFile> {
        self.files.get(name)
    }

    /// Convert a byte offset to line/column position
    pub fn offset_to_position(&self, file_name: &str, offset: usize) -> Option<Position> {
        let file = self.get_file(file_name)?;
        file.offset_to_position(offset)
    }
}

impl Default for SourceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceFile {
    /// Create a new source file
    pub fn new(name: String, content: String) -> Self {
        let mut line_starts = vec![0];
        for (i, ch) in content.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }
        
        Self {
            name,
            content,
            line_starts,
        }
    }

    /// Convert a byte offset to line/column position
    pub fn offset_to_position(&self, offset: usize) -> Option<Position> {
        if offset > self.content.len() {
            return None;
        }

        let line = self.line_starts
            .binary_search(&offset)
            .unwrap_or_else(|i| i.saturating_sub(1));
        
        let line_start = self.line_starts[line];
        let column = offset - line_start;

        Some(Position {
            line: line + 1,
            column: column + 1,
            offset,
        })
    }

    /// Get the content of a span
    pub fn get_span_content(&self, span: &Span) -> Option<&str> {
        let start = span.start.offset;
        let end = span.end.offset;
        
        if end <= self.content.len() {
            Some(&self.content[start..end])
        } else {
            None
        }
    }
}