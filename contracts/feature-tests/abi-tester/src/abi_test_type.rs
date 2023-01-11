use crate::only_nested::*;
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, Box, ManagedBuffer},
};
multiversx_sc::derive_imports!();

/// Its only purpose is to test that the ABI generator works fine.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct AbiTestType {
    /// This type should only appear here.
    pub nested: OnlyShowsUpAsNested01,

    /// Tests that recursive types will not send the ABI generator into an infinite loop.
    pub next: Option<Box<AbiTestType>>,

    /// Tests that tuples tell the ABI of their component types even if they appear nowhere else.
    /// Also, just like above, recursive types need to work even when nested into a tuple.
    pub tuple_madness: (OnlyShowsUpAsNested02, Option<Box<AbiTestType>>),
}

/// Its only purpose is to test that the ABI generator works fine.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct AbiManagedType<M: ManagedTypeApi> {
    pub big_uint: BigUint<M>,
    pub integer: i32,
    pub managed_buffer: ManagedBuffer<M>,
}

/// Its only purpose is to test that the ABI generator works fine.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, ManagedVecItem)]
pub struct AbiManagedVecItem {
    pub value1: u32,
    pub value2: u32,
}
