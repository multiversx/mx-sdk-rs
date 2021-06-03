elrond_wasm::imports!();

use crate::auction::*;

#[elrond_wasm_derive::module]
pub trait ViewsModule: crate::storage::StorageModule {
	#[view(isAlreadyUpForAuction)]
	fn is_already_up_for_auction(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> bool {
		!self.auction_for_token(nft_type, nft_nonce).is_empty()
	}

	#[view(getPaymentTokenForAuctionedNft)]
	fn get_payment_token_for_auctioned_nft(
		&self,
		nft_type: &TokenIdentifier,
		nft_nonce: u64,
	) -> OptionalResult<MultiResult2<TokenIdentifier, u64>> {
		if self.is_already_up_for_auction(nft_type, nft_nonce) {
			let esdt_token = self
				.auction_for_token(nft_type, nft_nonce)
				.get()
				.payment_token;

			OptionalResult::Some((esdt_token.token_type, esdt_token.nonce).into())
		} else {
			OptionalResult::None
		}
	}

	#[view(getMinMaxBid)]
	fn get_min_max_bid(
		&self,
		nft_type: TokenIdentifier,
		nft_nonce: u64,
	) -> OptionalResult<MultiResult2<Self::BigUint, Self::BigUint>> {
		if self.is_already_up_for_auction(&nft_type, nft_nonce) {
			let auction = self.auction_for_token(&nft_type, nft_nonce).get();

			OptionalResult::Some((auction.min_bid, auction.max_bid).into())
		} else {
			OptionalResult::None
		}
	}

	#[view(getStartTime)]
	fn get_start_time(&self, nft_type: TokenIdentifier, nft_nonce: u64) -> OptionalResult<u64> {
		if self.is_already_up_for_auction(&nft_type, nft_nonce) {
			OptionalResult::Some(
				self.auction_for_token(&nft_type, nft_nonce)
					.get()
					.start_time,
			)
		} else {
			OptionalResult::None
		}
	}

	#[view(getDeadline)]
	fn get_deadline(&self, nft_type: TokenIdentifier, nft_nonce: u64) -> OptionalResult<u64> {
		if self.is_already_up_for_auction(&nft_type, nft_nonce) {
			OptionalResult::Some(self.auction_for_token(&nft_type, nft_nonce).get().deadline)
		} else {
			OptionalResult::None
		}
	}

	#[view(getOriginalOwner)]
	fn get_original_owner(
		&self,
		nft_type: TokenIdentifier,
		nft_nonce: u64,
	) -> OptionalResult<Address> {
		if self.is_already_up_for_auction(&nft_type, nft_nonce) {
			OptionalResult::Some(
				self.auction_for_token(&nft_type, nft_nonce)
					.get()
					.original_owner,
			)
		} else {
			OptionalResult::None
		}
	}

	#[view(getCurrentWinningBid)]
	fn get_current_winning_bid(
		&self,
		nft_type: TokenIdentifier,
		nft_nonce: u64,
	) -> OptionalResult<Self::BigUint> {
		if self.is_already_up_for_auction(&nft_type, nft_nonce) {
			OptionalResult::Some(
				self.auction_for_token(&nft_type, nft_nonce)
					.get()
					.current_bid,
			)
		} else {
			OptionalResult::None
		}
	}

	#[view(getCurrentWinner)]
	fn get_current_winner(
		&self,
		nft_type: TokenIdentifier,
		nft_nonce: u64,
	) -> OptionalResult<Address> {
		if self.is_already_up_for_auction(&nft_type, nft_nonce) {
			OptionalResult::Some(
				self.auction_for_token(&nft_type, nft_nonce)
					.get()
					.current_winner,
			)
		} else {
			OptionalResult::None
		}
	}

	#[view(getFullAuctionData)]
	fn get_full_auction_data(
		&self,
		nft_type: TokenIdentifier,
		nft_nonce: u64,
	) -> OptionalResult<Auction<Self::BigUint>> {
		if self.is_already_up_for_auction(&nft_type, nft_nonce) {
			OptionalResult::Some(self.auction_for_token(&nft_type, nft_nonce).get())
		} else {
			OptionalResult::None
		}
	}
}
