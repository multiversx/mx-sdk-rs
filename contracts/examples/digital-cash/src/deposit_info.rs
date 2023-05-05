use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, EsdtTokenPayment, ManagedAddress, ManagedVec},
};

multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub esdt_funds: ManagedVec<M, EsdtTokenPayment<M>>,
    pub egld_funds: BigUint<M>,
    pub valability: u64,
    pub expiration_round: u64,
    pub fees: Fee<M>,
}

impl<M> DepositInfo<M>
where
    M: ManagedTypeApi,
{
    pub fn get_num_tokens(&self) -> usize {
        (self.egld_funds != BigUint::zero()) as usize + self.esdt_funds.len()
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Default)]
pub struct Fee<M: ManagedTypeApi> {
    pub num_token_to_transfer: usize,
    pub value: BigUint<M>,
}
