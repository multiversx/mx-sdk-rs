use crate::only_nested::*;
multiversx_sc::derive_imports!();

/// Its only purpose is to test that the ABI generator works fine.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum AbiEnum {
    Nothing,
    Something(i32),
    SomethingMore(u8, OnlyShowsUpAsNested08),
    SomeStruct { a: u16, b: OnlyShowsUpAsNested09 },
}
