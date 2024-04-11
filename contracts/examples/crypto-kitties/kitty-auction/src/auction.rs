use multiversx_sc::derive_imports::*;

use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress},
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum AuctionType {
    Selling,
    Siring,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Auction<'a, M: ManagedTypeApi<'a>> {
    pub auction_type: AuctionType,
    pub starting_price: BigUint<'a, M>,
    pub ending_price: BigUint<'a, M>,
    pub deadline: u64,
    pub kitty_owner: ManagedAddress<'a, M>,
    pub current_bid: BigUint<'a, M>,
    pub current_winner: ManagedAddress<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> Auction<'a, M> {
    pub fn new(
        auction_type: AuctionType,
        starting_price: BigUint<'a, M>,
        ending_price: BigUint<'a, M>,
        deadline: u64,
        kitty_owner: ManagedAddress<'a, M>,
    ) -> Self {
        Auction {
            auction_type,
            starting_price,
            ending_price,
            deadline,
            kitty_owner,
            current_bid: BigUint::zero(),
            current_winner: ManagedAddress::zero(),
        }
    }
}
