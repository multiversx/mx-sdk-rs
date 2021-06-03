elrond_wasm::imports!();

use crate::auction::*;

#[elrond_wasm_derive::module]
pub trait StorageModule {
	#[view(getMarketplaceCutPercentage)]
	#[storage_mapper("bidCutPerecentage")]
	fn bid_cut_percentage(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[storage_mapper("auctionForToken")]
	fn auction_for_token(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> SingleValueMapper<Self::Storage, Auction<Self::BigUint>>;
}
