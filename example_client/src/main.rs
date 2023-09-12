use std::fs;

use waser::*;

fn main() {
    let wasm = fs::read("target/wasm32-unknown-unknown/debug/example_server.wasm").unwrap();
    let mut module = Module::from_bytes(&wasm).unwrap();

    for input in ["hello", "world", "foo", "bar"] {
        let output = module.request(input).unwrap();
        println!("{} => {}", input, String::from_utf8_lossy(&output));
    }
}
