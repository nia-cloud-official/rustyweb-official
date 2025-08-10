# CI Notes

The provided GitHub Actions workflow does:
1. Build wasm via wasm-pack
2. Build native Node bindings (napi-rs) on each platform
3. Build CLI binary for each platform (linux, macos, windows)
4. Package artifacts and create GitHub Release
5. Publish npm package with pkg/ (wasm) and native binaries for each platform included under `native/` and `bin/`

You must set up:
- Repository secrets: `NPM_TOKEN`
- Optionally use `actions-rs/toolchain` and pre-installed build tools on runners

See `.github/workflows/publish.yml` for details.
