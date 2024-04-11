use multiversx_sc::{derive_imports::*, imports::*};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<'a, M: ManagedTypeApi<'a>> {
    pub depositor_address: ManagedAddress<'a, M>,
    pub esdt_funds: ManagedVec<'a, M, EsdtTokenPayment<'a, M>>,
    pub egld_funds: BigUint<'a, M>,
    pub valability: u64,
    pub expiration_round: u64,
    pub fees: Fee<'a, M>,
}

impl<'a, M> DepositInfo<'a, M>
where
    M: ManagedTypeApi<'a>,
{
    pub fn get_num_tokens(&self) -> usize {
        let mut amount = self.esdt_funds.len();
        if self.egld_funds > 0 {
            amount += 1;
        }

        amount
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Fee<'a, M: ManagedTypeApi<'a>> {
    pub num_token_to_transfer: usize,
    pub value: EgldOrEsdtTokenPayment<'a, M>,
}
