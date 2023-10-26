multiversx_sc::derive_imports!();

use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BaseBigUint, ManagedAddress},
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum AuctionType {
    Selling,
    Siring,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Auction<M: ManagedTypeApi> {
    pub auction_type: AuctionType,
    pub starting_price: BaseBigUint<M>,
    pub ending_price: BaseBigUint<M>,
    pub deadline: u64,
    pub kitty_owner: ManagedAddress<M>,
    pub current_bid: BaseBigUint<M>,
    pub current_winner: ManagedAddress<M>,
}

impl<M: ManagedTypeApi> Auction<M> {
    pub fn new(
        auction_type: AuctionType,
        starting_price: BaseBigUint<M>,
        ending_price: BaseBigUint<M>,
        deadline: u64,
        kitty_owner: ManagedAddress<M>,
    ) -> Self {
        Auction {
            auction_type,
            starting_price,
            ending_price,
            deadline,
            kitty_owner,
            current_bid: BaseBigUint::zero(),
            current_winner: ManagedAddress::zero(),
        }
    }
}
