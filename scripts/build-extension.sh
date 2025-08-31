#!/bin/bash
# Build script for VSCode extension

set -e

echo "Building VSCode extension..."

# Navigate to extension directory
cd packages/vscode-extension

# Install dependencies
npm install

# Compile TypeScript
npm run compile

# Package extension
npm run package

echo "VSCode extension build completed!"
echo "Extension package: packages/vscode-extension/typescript-rust-0.1.0.vsix"