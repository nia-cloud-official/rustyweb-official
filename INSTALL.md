# Install & Quickstart (RustyWeb)

## Prerequisites
- Rust toolchain (rustup) with stable toolchain
- Node.js + npm (or pnpm/yarn)
- wasm-pack (`cargo install wasm-pack`)
- napi-cli (`cargo install napi-cli`)
- Platform C toolchain (build-essential / Xcode / VS Build Tools)
- GitHub repository for publishing (optional)

## Local dev (quick)
1. Build CLI and run:
   ```bash
   # from repository root
   cd rust/cli
   cargo install --path .
   # create a new app
   rustyweb new myapp
   cd myapp
   # run dev server
   rustyweb run
   ```
2. In another terminal, open `http://localhost:3000`

## Full local build (produce artifacts)
1. Build wasm and copy into package:
   ```bash
   cd rust/wasm
   wasm-pack build --target web --out-dir ../../packages/rustyweb/pkg
   ```
2. Build native node binding (napi-rs):
   ```bash
   cd rust/native
   npm install
   npm run build
   mkdir -p ../../packages/rustyweb/native
   cp target/release/*.node ../../packages/rustyweb/native/ || true
   printf "module.exports = require('./index.node');\n" > ../../packages/rustyweb/native/index.js || true
   ```
3. Build CLI binaries for release:
   ```bash
   cd rust/cli
   cargo build --release
   # copy target/release/rustyweb into packages for distribution
   cp target/release/rustyweb ../../packages/rustyweb/bin/linux-rustyweb || true
   ```

## CI & Publishing
- Configure GitHub Actions secrets:
  - `NPM_TOKEN` for publishing to npm
  - `GITHUB_TOKEN` (default provided by Actions for creating releases)
- Push a Git tag `vX.Y.Z` to trigger release and npm publish.
