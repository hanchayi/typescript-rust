//! Utility modules for the TypeScript compiler

pub mod span;
pub mod intern;
pub mod source;

// Re-export commonly used types
pub use span::{Span, Position};
pub use source::SourceManager;
// Import StringInterner directly from string-interner crate
pub use string_interner::StringInterner;