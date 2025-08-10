#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)

echo "1) Build wasm"
pushd "$ROOT/rust/wasm"
wasm-pack build --target web --out-dir ../../packages/rustyweb/pkg
popd

echo "2) Build native node binding (napi-rs)"
pushd "$ROOT/rust/native"
npm install
npm run build || true
mkdir -p "$ROOT/packages/rustyweb/native"
cp -v target/release/*.node "$ROOT/packages/rustyweb/native/" || true
# add index.js wrapper
echo "try { module.exports = require('./index.node'); } catch(e) { module.exports = { add: ()=>{ throw new Error('native not built') } } }" > "$ROOT/packages/rustyweb/native/index.js"
popd

echo "3) Build CLI"
pushd "$ROOT/rust/cli"
cargo build --release
mkdir -p "$ROOT/packages/rustyweb/bin"
cp -v target/release/rustyweb "$ROOT/packages/rustyweb/bin/rustyweb" || true
popd

echo "Artifacts copied to packages/rustyweb/"
