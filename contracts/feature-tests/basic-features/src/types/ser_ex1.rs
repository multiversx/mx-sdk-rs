use elrond_wasm::types::{BoxedBytes, Vec};
elrond_wasm::derive_imports!();

/// Copied from elrond-wasm serialization tests.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct SerExample1 {
    /// Checking nested serialization of basic types.
    pub int: u16,

    /// Bytes buffer.
    pub seq: Vec<u8>,
    pub another_byte: u8,
    pub uint_32: u32,
    pub uint_64: u64,
    pub boxed_bytes: BoxedBytes,
}
