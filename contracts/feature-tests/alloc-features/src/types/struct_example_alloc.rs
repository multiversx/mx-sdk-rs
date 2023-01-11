use multiversx_sc::types::{BoxedBytes, Vec};
multiversx_sc::derive_imports!();

/// Example serialization for a structure that uses the heap allocator.
/// Also checking nested serialization of basic types.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct StructExampleAlloc {
    pub int: u16,

    /// Bytes buffer.
    pub seq: Vec<u8>,
    pub another_byte: u8,
    pub uint_32: u32,
    pub uint_64: u64,
    pub boxed_bytes: BoxedBytes,
}
