use multiversx_sc::{
    api::ManagedTypeApi,
    types::{EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, ManagedAddress},
};

multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub payment: EgldOrEsdtTokenPayment<M>,
    pub expiration_round: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, ManagedVecItem)]
pub struct FundType<M: ManagedTypeApi> {
    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
}
