use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BaseBigUint, ManagedBuffer},
};

multiversx_sc::derive_imports!();

#[derive(
    NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq, Debug, Clone,
)]
pub struct ExampleStructManaged<M: ManagedTypeApi> {
    pub big_uint: BaseBigUint<M>,
    pub int: u32,
    pub bytes: ManagedBuffer<M>,
}
