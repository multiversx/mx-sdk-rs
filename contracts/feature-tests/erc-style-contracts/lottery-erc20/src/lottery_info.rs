use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BaseBigUint, ManagedAddress, Vec},
};
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LotteryInfo<M: ManagedTypeApi> {
    pub ticket_price: BaseBigUint<M>,
    pub tickets_left: u32,
    pub deadline: u64,
    pub max_entries_per_user: u32,
    pub prize_distribution: Vec<u8>,
    pub whitelist: Vec<ManagedAddress<M>>,
    pub current_ticket_number: u32,
    pub prize_pool: BaseBigUint<M>,
    pub queued_tickets: u32,
}
