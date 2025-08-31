# TypeScript Rust Compiler

A TypeScript compiler implemented in Rust with WebAssembly support for modern development environments.

## Features

- 🦀 **Rust Implementation**: Core compiler written in Rust for performance and safety
- 🌐 **WebAssembly Support**: Runs in browsers and Node.js via WASM
- 🔧 **VSCode Extension**: Integrated development experience
- 🚀 **Language Server**: LSP support for editors
- ⚡ **Web Worker**: Non-blocking compilation in browsers
- 📦 **Multiple Targets**: CLI, LSP, WASM, and VSCode extension


## Quick Start

### Prerequisites

- Rust 1.70+
- Node.js 20.18.1+
- npm 10.0.0+
- wasm-pack

> **Note**: Node.js 20.18.1+ is required due to dependencies like `undici` and `cheerio` that require newer Node.js versions for proper functionality.

### Setup

```bash
# Clone the repository
git clone https://github.com/your-username/typescript-rust.git
cd typescript-rust

# Run setup script
chmod +x ./scripts/*.sh
./scripts/setup.sh
```

### Building

```bash
# Build all components
npm run build

# Or build individually
cargo build                    # Rust components
./scripts/build-wasm.sh        # WebAssembly
./scripts/build-extension.sh   # VSCode extension
```

### Testing

```bash
# Run all tests
npm test

# Or test individually
cargo test                     # Rust tests
npm test --workspaces          # JavaScript tests
```

## Usage

### Command Line

```bash
# Compile TypeScript files
tsc-rust compile src/main.ts --outdir dist

# Type check only
tsc-rust check src/**/*.ts
```

### VSCode Extension

1. Install the extension from `packages/vscode-extension/typescript-rust-0.1.0.vsix`
2. Open a TypeScript file
3. Use `Ctrl+Shift+P` → "Compile with Rust TS"

### Web Worker (Browser)

```typescript
import { WorkerCompiler } from '@typescript-rust/web-worker';

const compiler = new WorkerCompiler();
await compiler.init();

const result = await compiler.compile(`
  const greeting: string = "Hello, World!";
  console.log(greeting);
`, {
  fileName: 'example.ts',
  config: {
    target: 'es2020',
    module: 'es2020',
    strict: true,
    source_map: false
  }
});

console.log(result.code);
```

## Development

### Project Structure

- `crates/` - Rust crates
  - `ts-core/` - Core compiler library
  - `ts-wasm/` - WebAssembly bindings
  - `ts-lsp/` - Language Server Protocol implementation
  - `ts-cli/` - Command line interface
- `packages/` - JavaScript/TypeScript packages
  - `vscode-extension/` - VSCode extension
  - `web-worker/` - Web Worker package
  - `shared/` - Shared types and utilities
- `scripts/` - Build and utility scripts
- `tests/` - Integration tests
- `docs/` - Documentation

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `npm test` to ensure all tests pass
6. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.
