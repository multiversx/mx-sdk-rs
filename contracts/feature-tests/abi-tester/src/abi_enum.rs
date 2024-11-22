use crate::only_nested::*;
multiversx_sc::derive_imports!();

/// Its only purpose is to test that the ABI generator works fine.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum AbiEnum {
    Nothing,
    Something(i32),
    SomethingMore(u8, OnlyShowsUpAsNested08),
    SomeStruct { a: u16, b: OnlyShowsUpAsNested09 },
}

/// An enum with similar explicit discriminants
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum ExplicitDiscriminant {
    Zero,
    Thirty = 30,
    Twelve = 12,
    Fifty = 50,
    FiftyOne,
}

/// An enum with different explicit discriminants
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
#[repr(u8)]
pub enum ExplicitDiscriminantMixed {
    Zero,
    Unit = 3,
    Tuple(u16),
    Five,
    Struct { a: u8, b: u16 } = 1,
}
