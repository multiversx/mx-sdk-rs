elrond_wasm::derive_imports!();

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{Address, BigUint, ManagedType},
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum AuctionType {
    Selling,
    Siring,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Auction<M: ManagedTypeApi> {
    pub auction_type: AuctionType,
    pub starting_price: BigUint<M>,
    pub ending_price: BigUint<M>,
    pub deadline: u64,
    pub kitty_owner: Address,
    pub current_bid: BigUint<M>,
    pub current_winner: Address,
}

impl<M: ManagedTypeApi> Auction<M> {
    pub fn new(
        auction_type: AuctionType,
        starting_price: &BigUint<M>,
        ending_price: &BigUint<M>,
        deadline: u64,
        kitty_owner: &Address,
    ) -> Self {
        let type_manager = starting_price.type_manager();
        Auction {
            auction_type,
            starting_price: starting_price.clone(),
            ending_price: ending_price.clone(),
            deadline,
            kitty_owner: kitty_owner.clone(),
            current_bid: BigUint::zero(type_manager),
            current_winner: Address::zero(),
        }
    }
}
