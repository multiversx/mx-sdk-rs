use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedBuffer},
};

elrond_wasm::derive_imports!();

#[derive(
    NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Debug, Clone,
)]
pub struct ExampleStructManaged<M: ManagedTypeApi> {
    pub big_uint: BigUint<M>,
    pub int: u32,
    pub bytes: ManagedBuffer<M>,
}
