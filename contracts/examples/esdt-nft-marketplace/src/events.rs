elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::auction::{Auction, AuctionType};

#[allow(clippy::too_many_arguments)]
#[elrond_wasm::module]
pub trait EventsModule {
    fn emit_auction_token_event(
        self,
        auction_id: u64,
        auction: Auction<BigUint>,
        creation_time: u64,
    ) {
        self.auction_token_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.nr_auctioned_tokens,
            &auction.original_owner,
            creation_time,
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

    fn emit_bid_event(self, auction_id: u64, auction: Auction<BigUint>, bid_time: u64) {
        self.bid_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.current_winner,
            bid_time,
            &auction.current_bid,
        );
    }

    fn emit_end_auction_event(self, auction_id: u64, auction: Auction<BigUint>, end_time: u64) {
        self.end_auction_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.current_winner,
            end_time,
            &auction.current_bid,
        );
    }

    fn emit_buy_sft_event(self, auction_id: u64, auction: Auction<BigUint>, buy_time: u64) {
        self.buy_sft_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.current_winner,
            buy_time,
        );
    }

    fn emit_withdraw_event(self, auction_id: u64, auction: Auction<BigUint>, withdraw_time: u64) {
        self.withdraw_event(
            &auction.auctioned_token.token_type,
            auction.auctioned_token.nonce,
            auction_id,
            &auction.original_owner,
            withdraw_time,
        );
    }

    #[event("auction_token_event")]
    fn auction_token_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] auctioned_token_amount: &BigUint,
        #[indexed] seller: &Address,
        #[indexed] creation_time: u64,
        #[indexed] min_bid: &BigUint,
        #[indexed] max_bid: &BigUint,
        #[indexed] start_time: u64,
        #[indexed] deadline: u64,
        #[indexed] accepted_payment_token: TokenIdentifier,
        #[indexed] accepted_payment_token_nonce: u64,
        #[indexed] auction_type: AuctionType,
        creator_royalties_percentage: BigUint, // between 0 and 10,000
    );

    #[event("bid_event")]
    fn bid_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] bidder: &Address,
        #[indexed] bid_time: u64,
        #[indexed] bid_amount: &BigUint,
    );

    #[event("end_auction_event")]
    fn end_auction_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] auction_winner: &Address,
        #[indexed] end_time: u64,
        #[indexed] winning_bid_amount: &BigUint,
    );

    #[event("buy_sft_event")]
    fn buy_sft_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] buyer: &Address,
        #[indexed] buy_time: u64,
    );

    #[event("withdraw_event")]
    fn withdraw_event(
        &self,
        #[indexed] auction_token_id: &TokenIdentifier,
        #[indexed] auctioned_token_nonce: u64,
        #[indexed] auction_id: u64,
        #[indexed] seller: &Address,
        #[indexed] withdraw_time: u64,
    );
}
