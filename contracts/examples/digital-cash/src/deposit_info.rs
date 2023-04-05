use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment, EsdtTokenPayment,
        ManagedAddress, ManagedVec,
    },
};

multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub payment: EgldOrEsdtTokenPayment<M>,
    pub valability: u64,
    pub expiration_round: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, ManagedVecItem)]
pub struct FundType<M: ManagedTypeApi> {
    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub nonce: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PaymentFunds<M: ManagedTypeApi> {
    pub num_token_transfer: u64,
    pub value: BigUint<M>,
}

#[derive()]
pub struct Funds<M: ManagedTypeApi> {
    pub esdt_funds: ManagedVec<M, EsdtTokenPayment<M>>,
    pub egld_funds: BigUint<M>,
}
