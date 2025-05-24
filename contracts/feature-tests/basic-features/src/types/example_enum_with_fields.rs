multiversx_sc::derive_imports!();

/// Copied from multiversx-sc serialization tests.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum ExampleEnumWithFields {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct { a: u32 },
}
