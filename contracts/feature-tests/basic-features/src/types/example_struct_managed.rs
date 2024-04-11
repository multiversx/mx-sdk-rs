use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedBuffer},
};

multiversx_sc::derive_imports!();

#[derive(
    NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Debug, Clone,
)]
pub struct ExampleStructManaged<'a, M: ManagedTypeApi<'a>> {
    pub big_uint: BigUint<'a, M>,
    pub int: u32,
    pub bytes: ManagedBuffer<'a, M>,
}
