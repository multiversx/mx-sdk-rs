use multiversx_sc::{derive_imports::*, imports::*};

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
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
        let mut amount = self.esdt_funds.len();
        if self.egld_funds > 0 {
            amount += 1;
        }

        amount
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Fee<M: ManagedTypeApi> {
    pub num_token_to_transfer: usize,
    pub value: EgldOrEsdtTokenPayment<M>,
}
