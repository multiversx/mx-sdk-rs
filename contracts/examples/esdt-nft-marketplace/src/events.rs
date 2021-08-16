elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::auction::{Auction, AuctionType};

#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_auction_token_event(self, auction_id: u64, auction: Auction<Self::BigUint>) {
        self.auction_token_event(
            auction_id,
            &auction.original_owner,
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            &auction.nr_auctioned_tokens,
            &auction.min_bid,
            &auction.max_bid.unwrap_or_default(),
            auction.start_time,
            auction.deadline,
            auction.payment_token.token_type,
            auction.payment_token.nonce,
            auction.auction_type,
            auction.creator_royalties_percentage,
        )
    }

    fn emit_bid_event(self, auction_id: u64, auction: Auction<Self::BigUint>, bid_time: u64) {
        self.bid_event(
            auction_id,
            &auction.current_winner,
            bid_time,
            &auction.current_bid,
        );
    }

    fn emit_end_auction_event(
        self,
        auction_id: u64,
        auction: Auction<Self::BigUint>,
        end_time: u64,
    ) {
        self.end_auction_event(
            auction_id,
            &auction.current_winner,
            &auction.current_bid,
            end_time,
        );
    }

    fn emit_buy_sft_event(self, auction_id: u64, auction: Auction<Self::BigUint>, buy_time: u64) {
        self.buy_sft_event(auction_id, &auction.current_winner, buy_time);
    }

    fn emit_withdraw_event(
        self,
        auction_id: u64,
        auction: Auction<Self::BigUint>,
        withdraw_time: u64,
    ) {
        self.withdraw_event(auction_id, &auction.original_owner, withdraw_time);
    }

    #[event("auction_token_event")]
    fn auction_token_event(
        &self,
        #[indexed] auction_id: u64,
        #[indexed] seller: &Address,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auctioned_token_amount: &Self::BigUint,
        #[indexed] min_bid: &Self::BigUint,
        #[indexed] max_bid: &Self::BigUint,
        #[indexed] start_time: u64,
        #[indexed] deadline: u64,
        #[indexed] accepted_payment_token: TokenIdentifier,
        #[indexed] accepted_payment_token_nonce: u64,
        #[indexed] auction_type: AuctionType,
        creator_royalties_percentage: Self::BigUint, // between 0 and 10,000
    );

    #[event("bid_event")]
    fn bid_event(
        &self,
        #[indexed] auction_id: u64,
        #[indexed] bidder: &Address,
        #[indexed] bid_time: u64,
        #[indexed] bid_amount: &Self::BigUint,
    );

    #[event("end_auction_event")]
    fn end_auction_event(
        &self,
        #[indexed] auction_id: u64,
        #[indexed] auction_winner: &Address,
        #[indexed] winning_bid_amount: &Self::BigUint,
        #[indexed] end_time: u64,
    );

    #[event("buy_sft_event")]
    fn buy_sft_event(
        &self,
        #[indexed] auction_id: u64,
        #[indexed] buyer: &Address,
        #[indexed] buy_time: u64,
    );

    #[event("withdraw_event")]
    fn withdraw_event(
        &self,
        #[indexed] auction_id: u64,
        #[indexed] seller: &Address,
        #[indexed] withdraw_time: u64,
    );
}
