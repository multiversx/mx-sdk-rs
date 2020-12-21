derive_imports!();

use elrond_wasm::{Address, BigUintApi};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum AuctionType {
	Selling,
	Siring,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Auction<BigUint: BigUintApi> {
	pub auction_type: AuctionType,
	pub starting_price: BigUint,
	pub ending_price: BigUint,
	pub deadline: u64,
	pub kitty_owner: Address,
	pub current_bid: BigUint,
	pub current_winner: Address,
}

impl<BigUint: BigUintApi> Auction<BigUint> {
	pub fn new(
		auction_type: AuctionType,
		starting_price: &BigUint,
		ending_price: &BigUint,
		deadline: u64,
		kitty_owner: &Address,
	) -> Self {
		Auction {
			auction_type,
			starting_price: starting_price.clone(),
			ending_price: ending_price.clone(),
			deadline,
			kitty_owner: kitty_owner.clone(),
			current_bid: BigUint::zero(),
			current_winner: Address::zero(),
		}
	}
}
