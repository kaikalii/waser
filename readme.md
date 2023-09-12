`waser` is a simple Rust library for creating queryable WASM modules.

Your server defines a responder function that takes a request and returns a response.

Your client loads the WASM module and can query the server module.

## Running the example

The example is a simple server that reverses a string.

Simply run
```
cargo build -p example_server --target wasm32-unknown-unknown
cargo run -p example_client
```
