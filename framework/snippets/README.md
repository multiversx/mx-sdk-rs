# MultiversX transaction snippets

Provides basic functionality for interacting with smart contracts directly from Rust.

It is a base on top of which little interaction programs (or snippets) can be written. Can be used in:
- interactors,
- web servers (in Rust),
- front-end (WebAssembly, via wasm-bindgen/Rust).

It is largely a wrapper around `multiversx-sdk` and `multiversx-sc`, combining the two.
