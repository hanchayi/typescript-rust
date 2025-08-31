#!/bin/bash
# Build script for WebAssembly module

set -e

echo "Building WebAssembly module..."

# Navigate to WASM crate directory
cd crates/ts-wasm

# Build WASM package
wasm-pack build --target web --out-dir ../../packages/web-worker/pkg

echo "WASM build completed!"

# Navigate to web worker package
cd ../../packages/web-worker

# Install dependencies and build
echo "Building Web Worker package..."
npm install
npm run build

echo "Web Worker build completed!"