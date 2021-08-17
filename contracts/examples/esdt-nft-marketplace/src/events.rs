elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::auction::{Auction, AuctionType, EsdtToken};

#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_auction_token_event(
        self,
        auction: Auction<BigUint>,
        auction_id: u64,
        current_time: u64,
    ) {
        self.auction_token_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            current_time,
            &auction,
        );
    }

    fn emit_bid_event(self, auction: Auction<BigUint>, auction_id: u64, current_time: u64) {
        self.bid_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            current_time,
            &auction,
        );
    }

    fn emit_end_auction_event(self, auction: Auction<BigUint>, auction_id: u64, current_time: u64) {
        self.end_auction_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            current_time,
            &auction,
        );
    }

    fn emit_buy_sft_event(self, auction: Auction<BigUint>, auction_id: u64, current_time: u64) {
        self.buy_sft_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            current_time,
            &auction,
        );
    }

    fn emit_withdraw_event(self, auction: Auction<BigUint>, auction_id: u64, current_time: u64) {
        self.withdraw_event(
            &auction.original_owner,
            auction_id,
            &auction.auctioned_token,
            &auction.auction_type,
            current_time,
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
        #[indexed] current_time: u64,
        auction: &Auction<BigUint>,
    );

    #[event("bid_event")]
    fn bid_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] auction_id: u64,
        #[indexed] auction_token: &EsdtToken,
        #[indexed] auction_type: &AuctionType,
        #[indexed] current_time: u64,
        auction: &Auction<BigUint>,
    );

    #[event("end_auction_event")]
    fn end_auction_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] auction_id: u64,
        #[indexed] auction_token: &EsdtToken,
        #[indexed] auction_type: &AuctionType,
        #[indexed] current_time: u64,
        auction: &Auction<BigUint>,
    );

    #[event("buy_sft_event")]
    fn buy_sft_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] auction_id: u64,
        #[indexed] auction_token: &EsdtToken,
        #[indexed] auction_type: &AuctionType,
        #[indexed] current_time: u64,
        auction: &Auction<BigUint>,
    );

    #[event("withdraw_event")]
    fn withdraw_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] auction_id: u64,
        #[indexed] auction_token: &EsdtToken,
        #[indexed] auction_type: &AuctionType,
        #[indexed] current_time: u64,
        auction: &Auction<BigUint>,
    );
}
