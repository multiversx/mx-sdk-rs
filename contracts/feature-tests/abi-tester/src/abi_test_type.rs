use crate::only_nested::*;
use elrond_wasm::Box;
elrond_wasm::derive_imports!();

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
