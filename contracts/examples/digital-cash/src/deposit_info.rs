use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress, TokenIdentifier},
};

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub depositor_address: ManagedAddress<M>,
    pub expiration_round: u64,
    pub token_name: TokenIdentifier<M>,
    pub nonce: u64
}
