use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier, ManagedAddress},
};

multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub depositor_address: ManagedAddress<M>,
    pub expiration_round: u64,
    pub token_name: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
}
