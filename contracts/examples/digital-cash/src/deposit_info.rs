use multiversx_sc::{derive_imports::*, imports::*};

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub funds: ManagedVec<M, Payment<M>>,
    pub expiration: TimestampMillis,
    pub fees: Fee<M>,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Fee<M: ManagedTypeApi> {
    pub num_token_to_transfer: usize,
    pub value: Payment<M>,
}
