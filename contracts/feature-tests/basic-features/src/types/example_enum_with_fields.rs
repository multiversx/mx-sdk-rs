multiversx_sc::derive_imports!();

/// Copied from multiversx-sc serialization tests.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum ExampleEnumWithFields {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct { a: u32 },
}
