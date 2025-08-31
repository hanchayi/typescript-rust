#!/bin/bash
# Setup script for development environment

set -e

echo "Setting up TypeScript Rust development environment..."

# Check for required tools
command -v cargo >/dev/null 2>&1 || { echo "Rust/Cargo is required but not installed. Aborting." >&2; exit 1; }
command -v node >/dev/null 2>&1 || { echo "Node.js is required but not installed. Aborting." >&2; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "npm is required but not installed. Aborting." >&2; exit 1; }

# Install wasm-pack if not present
if ! command -v wasm-pack >/dev/null 2>&1; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Install Rust targets
echo "Installing Rust WebAssembly target..."
rustup target add wasm32-unknown-unknown

# Install npm dependencies
echo "Installing npm dependencies..."
npm install

# Build everything
echo "Building project..."
./scripts/build-wasm.sh
./scripts/build-extension.sh

echo "Setup completed successfully!"
echo "You can now:"
echo "  - Run 'cargo test' to run Rust tests"
echo "  - Run 'npm test' to run JavaScript tests"
echo "  - Install the VSCode extension from packages/vscode-extension/typescript-rust-0.1.0.vsix"