// napi-rs example native bindings for Node
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[napi]
pub fn greet(name: String) -> String {
    format!("Hello, {} (from native)", name)
}
