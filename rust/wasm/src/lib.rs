// WASM helpers for RustyWeb — small utilities exported to the browser.
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! (from Rust WASM)", name)
}
