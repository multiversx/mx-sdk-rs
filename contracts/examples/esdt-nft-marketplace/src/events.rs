elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::auction::{Auction, AuctionType, EsdtToken};

#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_auction_token_event(self, auction: Auction<Self::BigUint>, auction_id: u64) {
        self.auction_token_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            &auction,
        );
    }

    fn emit_bid_event(self, auction: Auction<Self::BigUint>, auction_id: u64) {
        self.bid_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            &auction,
        );
    }

    fn emit_end_auction(self, auction: Auction<Self::BigUint>, auction_id: u64) {
        self.end_auction_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            &auction,
        );
    }

    #[event("auction_token_event")]
    fn auction_token_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] auction_id: u64,
        #[indexed] auction_token: &EsdtToken,
        #[indexed] auction_type: &AuctionType,
        auction: &Auction<Self::BigUint>,
    );

    #[event("bid_event")]
    fn bid_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] auction_id: u64,
        #[indexed] auction_token: &EsdtToken,
        #[indexed] auction_type: &AuctionType,
        auction: &Auction<Self::BigUint>,
    );

    #[event("end_auction_event")]
    fn end_auction_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] auction_id: u64,
        #[indexed] auction_token: &EsdtToken,
        #[indexed] auction_type: &AuctionType,
        auction: &Auction<Self::BigUint>,
    );
}
