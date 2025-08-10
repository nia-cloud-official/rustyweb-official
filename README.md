# RustyWeb

[![Build and Publish](https://github.com/nia-cloud-official/rustyweb-official/actions/workflows/main.yml/badge.svg)](https://github.com/nia-cloud-official/rustyweb-official/actions/workflows/main.yml)

RustyWeb is a full-stack web framework that combines a **Rust backend** with a modern **JavaScript frontend** powered by WebAssembly (WASM). It is designed to enable developers to build fast, scalable, and maintainable web applications with an experience similar to frameworks like Next.js, but with Rust powering the server-side.

---

## Features

- 🚀 High-performance Rust backend built with [axum](https://docs.rs/axum)
- 🌐 WASM-powered frontend integration with [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
- 🔗 Native Node.js bindings using [napi-rs](https://napi.rs/)
- ⚙️ CLI tool `rustyweb` for project scaffolding, development server, production builds, and deployment helpers
- 📦 Batteries included: starter templates, build scripts, and CI/CD workflows
- 💻 Cross-platform support with GitHub Actions for Linux, macOS, and Windows
- 📚 Comprehensive documentation and examples included

---

## Getting Started

### Prerequisites

- Rust (latest stable) — install from [rustup.rs](https://rustup.rs)
- Node.js (v18+) and npm — install from [nodejs.org](https://nodejs.org)
- wasm-pack (`cargo install wasm-pack`)
- napi-cli (`cargo install napi-cli`)

### Installation

Install the RustyWeb CLI globally (once the package is published on npm):

```bash
npm install -g rustyweb
