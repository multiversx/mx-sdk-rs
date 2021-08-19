elrond_wasm::imports!();

use crate::auction::*;

#[elrond_wasm::module]
pub trait StorageModule {
    #[view(getMarketplaceCutPercentage)]
    #[storage_mapper("bidCutPerecentage")]
    fn bid_cut_percentage(&self) -> SingleValueMapper<Self::Storage, BigUint>;

    #[storage_mapper("auctionById")]
    fn auction_by_id(
        &self,
        auction_id: u64,
    ) -> SingleValueMapper<Self::Storage, Auction<Self::TypeManager>>;

    #[view(getLastValidAuctionId)]
    #[storage_mapper("lastValidAuctionId")]
    fn last_valid_auction_id(&self) -> SingleValueMapper<Self::Storage, u64>;
}
