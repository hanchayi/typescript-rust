//! String interning utilities

use string_interner::{StringInterner, DefaultSymbol};
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Global string interner
static INTERNER: Lazy<Mutex<StringInterner>> = Lazy::new(|| {
    Mutex::new(StringInterner::default())
});

/// Interned string symbol
pub type Symbol = DefaultSymbol;

/// Intern a string and return its symbol
pub fn intern(s: &str) -> Symbol {
    INTERNER.lock().unwrap().get_or_intern(s)
}

/// Resolve a symbol back to its string
pub fn resolve(symbol: Symbol) -> Option<String> {
    INTERNER.lock().unwrap().resolve(symbol).map(|s| s.to_string())
}

/// Get or intern a string
pub fn get_or_intern(s: &str) -> Symbol {
    intern(s)
}

/// Check if a symbol exists
pub fn contains(symbol: Symbol) -> bool {
    INTERNER.lock().unwrap().resolve(symbol).is_some()
}