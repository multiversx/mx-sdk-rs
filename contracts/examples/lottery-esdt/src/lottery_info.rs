use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress, TokenIdentifier, Vec},
};

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LotteryInfo<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub ticket_price: BigUint<M>,
    pub tickets_left: u32,
    pub deadline: u64,
    pub max_entries_per_user: u32,
    pub prize_distribution: Vec<u8>,
    pub whitelist: Vec<ManagedAddress<M>>,
    pub prize_pool: BigUint<M>,
}
