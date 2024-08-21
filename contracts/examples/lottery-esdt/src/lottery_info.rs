use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier, ManagedVec},
};

use multiversx_sc::derive_imports::*;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct LotteryInfo<M: ManagedTypeApi> {
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub ticket_price: BigUint<M>,
    pub tickets_left: usize,
    pub deadline: u64,
    pub max_entries_per_user: usize,
    pub prize_distribution: ManagedVec<M, u8>,
    pub prize_pool: BigUint<M>,
}
