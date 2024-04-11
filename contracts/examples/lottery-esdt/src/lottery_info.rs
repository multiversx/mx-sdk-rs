use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrEsdtTokenIdentifier, ManagedVec},
};

use multiversx_sc::derive_imports::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LotteryInfo<'a, M: ManagedTypeApi<'a>> {
    pub token_identifier: EgldOrEsdtTokenIdentifier<'a, M>,
    pub ticket_price: BigUint<'a, M>,
    pub tickets_left: usize,
    pub deadline: u64,
    pub max_entries_per_user: usize,
    pub prize_distribution: ManagedVec<'a, M, u8>,
    pub prize_pool: BigUint<'a, M>,
}
