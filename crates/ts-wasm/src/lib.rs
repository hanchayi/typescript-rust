//! WebAssembly bindings for the TypeScript compiler

use wasm_bindgen::prelude::*;
use ts_core::{compile, CompileOptions, CompileResult};

/// WebAssembly wrapper for the TypeScript compiler
#[wasm_bindgen]
pub struct WasmCompiler;

#[wasm_bindgen]
impl WasmCompiler {
    /// Create a new WASM compiler instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmCompiler {
        console_error_panic_hook::set_once();
        WasmCompiler
    }
    
    /// Compile TypeScript source code to JavaScript
    #[wasm_bindgen]
    pub fn compile(&self, source: &str, options: JsValue) -> Result<JsValue, JsValue> {
        let opts: CompileOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse options: {}", e)))?;
            
        let result = compile(source, opts)
            .map_err(|diagnostics| {
                let error_msg = diagnostics.iter()
                    .map(|d| d.message.clone())
                    .collect::<Vec<_>>()
                    .join("; ");
                JsValue::from_str(&format!("Compilation failed: {}", error_msg))
            })?;
            
        serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
    }
    
    /// Get compiler version
    #[wasm_bindgen]
    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}