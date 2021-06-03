elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, TypeAbi)]
pub struct Auction<BigUint: BigUintApi> {
	pub auctioned_token: EsdtToken,
	pub nr_auctioned_tokens: BigUint,
	pub auction_type: AuctionType,
	pub auction_status: AuctionStatus,

	pub payment_token: EsdtToken,
	pub min_bid: BigUint,
	pub max_bid: BigUint,
	pub start_time: u64,
	pub deadline: u64,

	pub original_owner: Address,
	pub current_bid: BigUint,
	pub current_winner: Address,
	pub marketplace_cut_percentage: BigUint,
	pub creator_royalties_percentage: BigUint,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum AuctionType {
	None,
	Nft,
	SftAll,
	SftOnePerUser,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum AuctionStatus {
	None,
	Running,
	SftWaitingForBuyOrOwnerClaim,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone)]
pub struct EsdtToken {
	pub token_type: TokenIdentifier,
	pub nonce: u64,
}
